# Define the list of required environment variables for the root .env file
REQUIRED_ENV_VARS = PORT SERVER_KEK EMAIL_PASSWORD EMAIL MAIL_NAME SMTP_DOMAIN SMTP_PORT MONGO_INITDB_ROOT_USERNAME MONGO_INITDB_ROOT_PASSWORD

# Default target to check and update .env file
.PHONY: setup
setup: update-root-env update-ui-env check-private-key

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

# Target to create or overwrite the .env file in ./ui folder
.PHONY: update-ui-env
update-ui-env:
	@if [ ! -f .env ]; then \
		echo "Root .env file does not exist. Please run 'make check-env' first."; \
		exit 1; \
	fi; \
	PORT=$$(grep -E "^PORT=" .env | cut -d '=' -f 2); \
	X_API_KEY=$$(grep -E "^X_API_KEY=" .env | cut -d '=' -f 2); \
	if [ -z "$$PORT" ] || [ -z "$$X_API_KEY" ]; then \
		echo "Required variables PORT or X_API_KEY are missing in the root .env file."; \
		exit 1; \
	fi; \
	echo "Creating ./ui/.env file with the necessary environment variables..."; \
	echo "X_API_KEY=$$X_API_KEY" > ./ui/.env; \
	echo "NEXT_PUBLIC_API_BASE_URL=http://localhost:$$PORT" >> ./ui/.env; \
	echo "NEXT_PUBLIC_ENDPOINT=http://localhost:3000" >> ./ui/.env; \
	echo "✅ UI .env file created successfully."


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

# Target to run the server using Docker Compose with --build option
.PHONY: run-server
build-run-server: setup
	docker-compose up --build

# Target to run the server using Docker Compose without --build option
.PHONY: run-server
run-server: setup
	docker-compose up

# Target to run the ui / next app using npm run dev
.PHONY: run-ui
run-ui: setup
	@if lsof -i:3000 -t > /dev/null ; then \
		echo "PORT:3000 is busy, so can't start the dashboard. Kill the process if there's anything running."; \
		exit 1; \
	else \
		cd ui; \
		if [ ! -d node_modules ]; then \
			npm i; \
		fi; \
		npm run dev; \
	fi
