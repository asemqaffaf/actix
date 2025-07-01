# Stage 1: Build the Rust application with static linking
FROM rust:latest as builder
WORKDIR /workspace

# Install musl-tools for static linking
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rustup target add aarch64-unknown-linux-musl && \
    rm -rf /var/lib/apt/lists/*

COPY . .
# Compile in release mode with static linking
WORKDIR /workspace/app
RUN cargo build --release --target aarch64-unknown-linux-musl

# Stage 2: Create a minimal runtime image
# Use Alpine for a lightweight final image - works well with static binaries
FROM alpine:latest
RUN apk add --no-cache ca-certificates

WORKDIR /app
COPY --from=builder /workspace/app/target/aarch64-unknown-linux-musl/release/actix /app/actix

# Set the PORT environment variable for the application
ENV PORT=3000

# Expose the port your Actix app listens on
EXPOSE 3000

# Make the binary executable
RUN chmod +x /app/actix_app

# Command to run your application
ENTRYPOINT ["/app/actix_app"]
CMD []
