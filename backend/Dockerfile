# Stage 1: Base image with cargo-chef for caching dependencies
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Accept the build time argument
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Stage 2: Dependency planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder
FROM chef AS builder
WORKDIR /app
# Use cached dependencies
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy application source and build the binary
COPY . .
RUN cargo build --release --bin backend

# Stage 4: Runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install necessary libraries
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  libssl3 \
  ca-certificates && \
  rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/backend /usr/local/bin

# Set environment variables
ENV RUST_LOG="backend=debug,info"

# Entry point for the application
ENTRYPOINT ["backend"]

# Expose the application port
EXPOSE 8000/tcp
