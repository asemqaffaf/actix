# Stage 1: Build the Rust application with static linking
FROM rust:latest as builder
WORKDIR /workspace

# Install musl-tools for static linking and PostgreSQL dependencies
RUN apt-get update && \
    apt-get install -y musl-tools gcc-aarch64-linux-gnu musl-dev libpq-dev pkg-config && \
    rustup target add aarch64-unknown-linux-musl && \
    rm -rf /var/lib/apt/lists/*
    
# Set up environment for cross-compilation
ENV CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc

COPY . .
# Compile in release mode with static linking
WORKDIR /workspace/app
RUN cargo build --release --target aarch64-unknown-linux-musl

# Stage 2: Create a minimal runtime image
# Use Alpine for a lightweight final image - works well with static binaries
FROM alpine:latest
RUN apk add --no-cache ca-certificates

WORKDIR /app
COPY --from=builder /workspace/app/target/aarch64-unknown-linux-musl/release/actix_app /app/actix_app

# Set the PORT environment variable for the application
ENV PORT=3000

# Expose the port your Actix app listens on
EXPOSE 3000

# Make the binary executable
RUN chmod +x /app/actix_app

# Command to run your application
ENTRYPOINT ["/app/actix_app"]
CMD []
