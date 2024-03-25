# Build stage
FROM rust:latest AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release
RUN cargo test --release

# Runtime stage
FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/yx248-mini8 /usr/local/bin/yx248-mini8

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Use non-root user
RUN useradd -m myuser
USER myuser

ENTRYPOINT ["yx248-mini8"]
