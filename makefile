################################################# Initial Setup #################################################


# Define the list of required environment variables for the root .env file
REQUIRED_ENV_VARS = PORT SERVER_KEK EMAIL_PASSWORD EMAIL MAIL_NAME SMTP_DOMAIN SMTP_PORT MONGO_INITDB_ROOT_USERNAME MONGO_INITDB_ROOT_PASSWORD

# Default target to check and update .env file
.PHONY: setup
setup: update-root-env check-private-key

# Target to check and update the root .env file
.PHONY: update-root-env
update-root-env:
	@if [ -f .env ]; then \
		echo ".env file exists."; \
	else \
		echo ".env file does not exist. Creating .env file...."; \
		touch .env; \
	fi; \
	for var in $(REQUIRED_ENV_VARS); do \
		if ! grep -q "^$${var}=" .env; then \
			read -p "Enter value for $$var: " value; \
			echo "$${var}=$$value" >> .env; \
		else \
			echo "✅ $${var}"; \
		fi; \
	done; \
	if ! grep -q "^X_API_KEY=" .env; then \
		X_API_KEY=$$(openssl rand -base64 32 | tr -d '='); \
		echo "X_API_KEY=$$X_API_KEY" >> .env; \
		echo "✅ Generated X_API_KEY=$$X_API_KEY"; \
	else \
		echo "✅ X_API_KEY"; \
	fi


# Target to check and generate private_key.pem if it doesn't exist
.PHONY: check-private-key
check-private-key:
	@if [ -f private_key.pem ]; then \
		echo "🔑 private_key.pem exists."; \
	else \
		echo "private_key.pem does not exist. Generating private_key.pem."; \
		openssl genpkey -algorithm RSA -out private_key.pem -pkeyopt rsa_keygen_bits:2048; \
		echo "🔑 Generated private_key.pem"; \
	fi

################################################# Docker Setups #################################################

# Build the Docker image for the server target dev only
.PHONY: flexauth-build-docker
build-server: check-private-key
	docker build -f Dockerfile . -t flexauth-server:dev --target dev

# Target to run the server using Docker Compose without --build option
.PHONY: flexauth-up-docker
flexauth-up-docker: setup
	docker compose up

# Target to run the server using Docker Compose with --build option
.PHONY: flexauth-build-up-docker
flexauth-build-up-docker: setup
	docker compose up --build

################################################# Kubernetes Setups #################################################

# Define the .env file and the skaffold files
ENV_FILE := .env
SKAFFOLD_TEMPLATE := skaffold.template.yaml
SKAFFOLD_GENERATED := skaffold.generated.yaml
NAMESPACE=flexauth
SECRET=flexauth-secrets

# Load .env file and export all variables for Makefile
include $(ENV_FILE)
export $(shell sed 's/=.*//' $(ENV_FILE))

# Generate the skaffold.yaml file with envsubst
$(SKAFFOLD_GENERATED): $(SKAFFOLD_TEMPLATE)
	@echo "Generating $(SKAFFOLD_GENERATED) with environment variables..."
	@envsubst '$$EMAIL $$EMAIL_PASSWORD $$MAIL_NAME $$SMTP_DOMAIN $$SMTP_PORT' < $(SKAFFOLD_TEMPLATE) > $(SKAFFOLD_GENERATED)
	@echo "$(SKAFFOLD_GENERATED) generated successfully."

create-namespace:
	@echo "Creating namespace $(NAMESPACE)..."
	@if kubectl get namespace $(NAMESPACE) >/dev/null 2>&1; then \
		echo "Namespace $(NAMESPACE) already exists."; \
	else \
		kubectl create namespace $(NAMESPACE) || (echo "Failed to create namespace." && exit 1); \
	fi

# Take envs from .env then encode them to base64 and create a secret in k8s using bash
.PHONY: create-secret
create-secret:
	@echo "Creating secret in k8s..."
	@if kubectl get secret $(SECRET) -n $(NAMESPACE) >/dev/null 2>&1; then \
		echo "Secret $(SECRET) already exists. Overwriting..."; \
		kubectl delete secret $(SECRET) -n $(NAMESPACE); \
	fi && \
	kubectl create secret generic $(SECRET) --from-env-file=.env -n $(NAMESPACE) || (echo "Failed to create secret." && exit 1)

# Run Minikube
.PHONY: minikube-up
minikube-up:
		@echo "Running Skaffold..."
		@echo "Checking Minikube status..."
	@if minikube status | grep -q "host: Running"; then \
		echo "Minikube is already running."; \
	else \
		echo "Starting Minikube..."; \
		minikube start --driver=docker || (echo "Minikube failed to start." && exit 1); \
	fi

# Clean up generated files
.PHONY: clean
clean:
	@echo "Cleaning up generated files..."
	@rm -f $(SKAFFOLD_GENERATED)
	@echo "Clean-up complete."

# Run flexauth using Skaffold and start tunneling with minikube but don't occupy the terminal
.PHONY: flexauth-up-k8s
up-k8s:
	@skaffold run -f $(SKAFFOLD_GENERATED)

# start warching the logs of the flexauth server using kubectl
.PHONY: flexauth-logs-k8s
logs-k8s:
	@kubectl logs -n $(NAMESPACE) -l app=flexauth-server -f

# Get the local address of the flexauth server and mongo-express server in minikube
.PHONY: flexauth-address-k8s
flexauth-address-k8s:
	@echo "Flexauth is running in minikube. Write "minikube tunnel" to start tunneling."
	@echo "Then you will be able to see your servers are running at the following addresses:"
	@echo "Flexauth server address: http://127.0.0.1:8080"
	@echo "Mongo-express address: http://127.0.0.1:8081"

# Close minikube tunnel first check if it's running
.PHONY: flexauth-down-tunnel
down-tunnel:
	@echo "Checking if Minikube tunnel is running..."
	@if pgrep -f "minikube tunnel" > /dev/null; then \
		echo "Minikube tunnel is running. Closing it now..."; \
		pkill -f "minikube tunnel"; \
		echo "Minikube tunnel closed."; \
	else \
		echo "Minikube tunnel is not running."; \
	fi

# Delete all the resources
.PHONY: flexauth-down-k8s
down-k8s:
	@echo "Deleting all resources..."
	@kubectl delete -f k8s/local
	@kubectl delete secret flexauth-secrets -n $(NAMESPACE)
	@kubectl delete namespace $(NAMESPACE)
	@echo "All resources deleted."

# Final targets
flexauth-up-k8s: setup minikube-up create-namespace create-secret $(SKAFFOLD_GENERATED) up-k8s clean logs-k8s
flexauth-down-k8s: down-k8s clean
