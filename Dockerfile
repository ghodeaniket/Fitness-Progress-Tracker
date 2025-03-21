FROM rust:1.79 as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Create dummy source for dependency caching
RUN mkdir -p src && \
    echo "fn main() { println!(\"Dummy placeholder\"); }" > src/main.rs

# Build dependencies (this will be cached if dependencies don't change)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release || echo "Initial build failed, but that's expected"

# Copy actual source code
COPY . .

# Build the actual application
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/fitness-progress-tracker /app/fitness-progress-tracker

# Copy migrations directory
COPY --from=builder /app/migrations /app/migrations

# Expose the port
EXPOSE 8080

# Command to run the application
CMD ["./fitness-progress-tracker"]
