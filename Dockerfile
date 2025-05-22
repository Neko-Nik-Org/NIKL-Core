# === Stage 1: Build ===
FROM rust:1.85-slim AS builder

WORKDIR /nikl

# Install dependencies for building
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

# Pre-cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Copy actual source and build
COPY . .
RUN cargo build --release

# === Stage 2: Runtime ===
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Add a nikl user (non-root)
RUN useradd -ms /bin/bash nikl

# Copy the built binary
COPY --from=builder /nikl/target/release/nikl /usr/local/bin/nikl

# Set workdir and default user
WORKDIR /usr/src/app
USER nikl

# Add entrypoint so users can do: docker run nikl script.nk
ENTRYPOINT ["nikl"]
