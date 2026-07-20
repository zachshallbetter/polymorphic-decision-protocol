# Stage 1: Build environment
FROM rust:1.80-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire workspace configuration and source
COPY Cargo.toml Cargo.lock ./
COPY crates/pdp-core/Cargo.toml crates/pdp-core/Cargo.toml
COPY crates/pdp-core/src crates/pdp-core/src

# Compile release build for the workspace
RUN cargo build --release --workspace

# Stage 2: Slim runtime container
FROM debian:bookworm-slim

# Install runtime SSL dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binaries from builder stage
# (Ready to copy other crates like pdp-server as they are implemented)
COPY --from=builder /app/target/release/ /app/bin/

ENV PORT=8080
EXPOSE 8080

# Keep container running or execute pdp-server when implemented
CMD ["/app/bin/pdp-core"]
