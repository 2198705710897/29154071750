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

# Create dummy main.rs and lib.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs && \
    cargo build --release --no-default-features --features postgres --bin postgres && \
    rm -rf src

# Copy source code
COPY src ./src
# COPY config.toml ./  # Removed as it's missing; using env vars instead

# Build the actual postgres binary
RUN cargo build --release --no-default-features --features postgres --bin postgres

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the postgres binary from builder
COPY --from=builder /app/target/release/postgres /app/xcommunity-bot

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Run the application
CMD ["/app/xcommunity-bot"]
