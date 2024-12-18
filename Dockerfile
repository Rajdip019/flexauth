# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.77
ARG APP_NAME=inhouse-auth

################################################################################
# Development Stage
FROM rust:${RUST_VERSION}-alpine AS dev

WORKDIR /app

RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static curl

RUN cargo install cargo-watch --locked

COPY Cargo.toml Cargo.lock ./   

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

COPY . .

VOLUME /app/src

CMD ["cargo", "watch", "-q", "-c" ,"-x", "run"]

################################################################################
# Build Stage
FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git curl pkgconfig openssl-dev openssl-libs-static

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

################################################################################
# Final Stage
FROM alpine:3.18 AS final

ARG PORT
ARG UID=10001

# Install curl to fetch the global-bundle.pem
RUN apk add --no-cache curl

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Download the global-bundle.pem for MongoDB TLS
WORKDIR /app
RUN curl -o /app/global-bundle.pem https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem

# Copy the application executable
COPY --from=build /bin/server /bin/server

# Expose the application port
EXPOSE ${PORT}

CMD ["/bin/server"]
