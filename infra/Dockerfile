# Stage 1: Build the Rust application with static linking
FROM ekidd/rust-musl-builder:latest AS builder
WORKDIR /home/rust/src

# Add PostgreSQL support
RUN apt-get update && \
    apt-get install -y libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy your source code
COPY --chown=rust:rust . .

# Change to app directory and build in release mode
WORKDIR /home/rust/src/app
RUN cargo build --release

# Stage 2: Create a minimal runtime image
# Use Alpine for a lightweight final image - works well with static binaries
FROM alpine:latest
RUN apk add --no-cache ca-certificates

WORKDIR /app
COPY --from=builder /home/rust/src/app/target/x86_64-unknown-linux-gnu/release/actix_app /app/actix_app

# Set the PORT environment variable for the application
ENV PORT=3000

# Expose the port your Actix app listens on
EXPOSE 3000

# Make the binary executable
RUN chmod +x /app/actix_app

# Command to run your application
ENTRYPOINT ["/app/actix_app"]
CMD []
