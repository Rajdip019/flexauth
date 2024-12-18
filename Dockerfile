# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.77
ARG APP_NAME=inhouse-auth

################################################################################
# Create a development stage for building the application.
FROM rust:${RUST_VERSION}-alpine AS dev

# Set the working directory
WORKDIR /app

# Install system dependencies and required libraries for the build
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static

# Install cargo-watch for auto-reloading
RUN cargo install cargo-watch --locked

# Copy the Cargo.toml and Cargo.lock files separately to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file and build dependencies to cache them
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Copy the actual source code
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


RUN cargo install cargo-watch --locked


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
