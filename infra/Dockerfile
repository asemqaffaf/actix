# Stage 1: Build the Rust application with static linking
FROM rust:latest as builder
WORKDIR /workspace

# Install PostgreSQL client, diesel CLI and musl-tools for static linking
RUN apt-get update && \
    apt-get install -y musl-tools libpq-dev postgresql-client && \
    rustup target add aarch64-unknown-linux-musl && \
    cargo install diesel_cli --no-default-features --features postgres && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests first to leverage Docker cache
COPY app/Cargo.toml app/Cargo.lock ./app/

# Create a dummy main.rs to build dependencies
RUN mkdir -p app/src && \
    echo "fn main() {println!(\"Placeholder\");}" > app/src/main.rs && \
    cd app && cargo build --release --target aarch64-unknown-linux-musl && \
    rm -f src/main.rs

# Copy the rest of the application
COPY . .

# Compile in release mode with static linking
WORKDIR /workspace/app
RUN cargo build --release --target aarch64-unknown-linux-musl

# Stage 2: Create a minimal runtime image
# Use Alpine for a lightweight final image - works well with static binaries
FROM alpine:latest

# Install PostgreSQL client libraries
RUN apk add --no-cache ca-certificates postgresql-client libpq

WORKDIR /app
COPY --from=builder /workspace/app/target/aarch64-unknown-linux-musl/release/actix_app /app/actix_app
COPY --from=builder /workspace/app/migrations /app/migrations
COPY --from=builder /workspace/app/.env /app/.env

# Set the PORT environment variable for the application
ENV PORT=3000

# Expose the port your Actix app listens on
EXPOSE 3000

# Make the binary executable
RUN chmod +x /app/actix_app

# Command to run your application
ENTRYPOINT ["/app/actix_app"]
CMD []
