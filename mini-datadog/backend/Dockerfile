FROM rust:latest as builder

WORKDIR /usr/src/app

# First, create a dummy project and build dependencies
RUN USER=root cargo new --bin mini-datadog-backend
WORKDIR /usr/src/app/mini-datadog-backend

# Copy only the dependency files first
COPY Cargo.toml Cargo.lock ./

# Create dummy source files for all binaries
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/bin/generate_metrics.rs && \
    echo "fn main() {}" > src/bin/generate_aws_data.rs && \
    SQLX_OFFLINE=true cargo build --release && \
    rm -rf src/

# Now copy the real source code
COPY . .

# Build the application with SQLx offline mode
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    curl \
    postgresql-client \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the built binaries
COPY --from=builder /usr/src/app/mini-datadog-backend/target/release/mini-datadog-backend .
COPY --from=builder /usr/src/app/mini-datadog-backend/target/release/generate_metrics .
COPY --from=builder /usr/src/app/mini-datadog-backend/target/release/generate_aws_data .

# Copy the migrations directory
COPY --from=builder /usr/src/app/mini-datadog-backend/migrations /usr/local/bin/migrations

# Copy the .env file
COPY --from=builder /usr/src/app/mini-datadog-backend/.env .

# Create a script to wait for services and start the application
RUN echo '#!/bin/bash\n\
echo "Waiting for PostgreSQL..."\n\
while ! nc -z postgres 5432; do\n\
  sleep 1\n\
done\n\
echo "PostgreSQL is up - starting application"\n\
exec "$@"' > /usr/local/bin/docker-entrypoint.sh && \
chmod +x /usr/local/bin/docker-entrypoint.sh

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 8080

# Add healthcheck
HEALTHCHECK --interval=10s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["mini-datadog-backend"] 