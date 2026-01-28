# ============================================================================
# X Community Bot - Dockerfile for Railway
# ============================================================================

# Build stage
FROM rust:1.82-slim as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --features postgres && \
    rm -rf src

# Copy source code
COPY src ./src
COPY config.toml ./

# Build the actual application
RUN touch src/main.rs && \
    cargo build --release --features postgres

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/xcommunity-bot /app/xcommunity-bot

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Run the application
CMD ["/app/xcommunity-bot"]
