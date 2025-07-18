# Use the official Rust image as a builder
FROM docker.io/rust:1.88 as builder

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src/ ./src/

# Build the application in release mode
RUN cargo build --release

# Use a slim base image for the final container
FROM debian:bullseye-slim

# Install OpenSSL dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/content /usr/local/bin/content

# Expose the port the app runs on
EXPOSE 3000

# Command to run the binary
CMD ["content"]
