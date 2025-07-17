# Stage 1: Build
FROM rust:1.77 AS builder

WORKDIR /app

# Pre-copy for dependency caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-web-dev /usr/local/bin/app

EXPOSE 8080

CMD ["app"]
