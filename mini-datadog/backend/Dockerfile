FROM rust:1.75 as builder

WORKDIR /usr/src/app

# Create a dummy project to cache dependencies
RUN cargo new --bin mini-datadog-backend
WORKDIR /usr/src/app/mini-datadog-backend
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copy the real source code
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    curl \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the built binaries
COPY --from=builder /usr/src/app/mini-datadog-backend/target/release/mini-datadog-backend .
COPY --from=builder /usr/src/app/mini-datadog-backend/target/release/generate_metrics .

# Copy the migrations directory
COPY --from=builder /usr/src/app/mini-datadog-backend/migrations /usr/local/bin/migrations

# Copy the .env file
COPY --from=builder /usr/src/app/mini-datadog-backend/.env .

# Copy the wait-for script
COPY --from=builder /usr/src/app/mini-datadog-backend/scripts/wait-for-it.sh .
RUN chmod +x wait-for-it.sh

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 8080

# Add healthcheck
HEALTHCHECK --interval=10s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Use wait-for-it script to wait for postgres
CMD ["./wait-for-it.sh", "postgres:5432", "--timeout=30", "--strict", "--", "./mini-datadog-backend"] 