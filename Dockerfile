# X Community Bot - Dockerfile for Railway
FROM rust:1.75-slim

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the postgres binary
RUN cargo build --release --bin postgres

# Copy the binary to a simpler location
RUN cp target/release/postgres /app/xcommunity-bot

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Run the application
CMD ["/app/xcommunity-bot"]
