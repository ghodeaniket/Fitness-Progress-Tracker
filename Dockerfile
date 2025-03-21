FROM rust:1.79 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the actual source code
COPY . .

# Force cargo to rebuild with the actual source code
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install SSL certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/fitness-progress-tracker /app/fitness-progress-tracker

# Copy migrations directory
COPY --from=builder /app/migrations /app/migrations

# Expose the port
EXPOSE 8080

# Command to run the application
CMD ["./fitness-progress-tracker"]
