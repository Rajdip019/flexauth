# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.77
ARG APP_NAME=inhouse-auth

################################################################################
# Create a development stage for building the application.
FROM rust:${RUST_VERSION}-alpine AS dev

# Set the working directory
WORKDIR /app

# Install system dependencies
RUN apk add --no-cache musl-dev

# Install OpenSSL development libraries
RUN apk add --no-cache pkgconfig openssl-dev

# -lssl -lcrypto are required for the openssl crate
RUN apk add --no-cache openssl-libs-static

# Install cargo-watch for auto-reloading
RUN cargo install cargo-watch

# Copy the source code into the container
COPY . .

# Mount the source code into the container
VOLUME /app/src

# Entrypoint command
CMD ["cargo", "watch", "-q", "-c" ,"-x", "run"]

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

# Install OpenSSL development libraries
RUN apk add --no-cache pkgconfig openssl-dev

# -lssl -lcrypto are required for the openssl crate
RUN apk add --no-cache openssl-libs-static


RUN cargo install cargo-watch


RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

################################################################################`

## Final build file

FROM alpine:3.18 AS final

ARG PORT

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE ${PORT}

# What the container should run when it is started.
CMD ["/bin/server"]


################################################################################
# Docker File for SMTP Server

FROM ubuntu:latest AS smtp

ARG EMAIL
ARG EMAIL_PASSWORD
ARG MAIL_NAME
ARG SMTP_DOMAIN
ARG SMTP_PORT

RUN apt-get update && \
    apt-get install -y mailutils && \
    apt install -y postfix

COPY /main.cf /etc/postfix/main.cf

RUN sh -c 'echo "root: ${EMAIL}" >> /etc/aliases' && \
    sh -c 'echo "${MAIL_NAME}" >> /etc/mailname' && \
    sh -c 'echo "[${SMTP_DOMAIN}]:${SMTP_PORT} ${EMAIL}:${EMAIL_PASSWORD}" >> /etc/postfix/sasl_passwd' && \
    postmap /etc/postfix/sasl_passwd && \
    chmod 0600 /etc/postfix/sasl_passwd /etc/postfix/sasl_passwd.db

CMD service postfix restart && tail -f /dev/null