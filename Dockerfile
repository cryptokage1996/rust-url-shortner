# Build stage
FROM rust:1.80 AS builder

WORKDIR /usr/src/app
COPY . .

# Build release binary
RUN cargo build --release

# Final image
FROM debian:bookworm-slim

# Install sqlite3 CLI (optional for debugging)
RUN apt-get update && apt-get install -y sqlite3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/url-shortner ./url-shortner

# Volume for SQLite DB
VOLUME ["/app/data"]

EXPOSE 8000

CMD ["./url-shortner"]
