# Stage 1: Build the Rust application
FROM rust:latest as builder
WORKDIR /app
COPY . .
# Compile in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime image
# Use a minimal base image for the final artifact
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates openssl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/actix /app/actix

# Expose the port your Actix app listens on
EXPOSE 3000

# Command to run your application
CMD ["./actix"]