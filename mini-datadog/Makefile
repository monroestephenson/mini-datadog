.PHONY: up down build logs ps clean restart help debug up-build up-postgres prune aws-data init

# Default target
help:
	@echo "Available commands:"
	@echo "  make init      - Initialize the project (first time setup)"
	@echo "  make up        - Start all services"
	@echo "  make up-build  - Build and start all services"
	@echo "  make down      - Stop all services"
	@echo "  make build     - Rebuild all services"
	@echo "  make logs      - View logs from all services"
	@echo "  make ps        - List running services"
	@echo "  make clean     - Remove all containers, volumes, and build artifacts"
	@echo "  make restart   - Restart all services"
	@echo "  make debug     - Show status and logs of all services"
	@echo "  make up-postgres - Start only PostgreSQL (useful for development)"
	@echo "  make prune     - Remove all unused Docker resources"
	@echo "  make aws-data  - Start AWS-like data generator"

# Initialize project
init: clean
	@echo "Initializing project..."
	@docker system prune -f
	@docker volume create mini_datadog_postgres_data || true
	@make build

# Start all services
up:
	@echo "Starting services..."
	@docker compose pull postgres || true
	docker compose up -d postgres
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 5
	docker compose up -d backend
	@echo "Waiting for backend to be ready..."
	@sleep 5
	docker compose up -d
	@echo "Waiting for all services to be ready..."
	@sleep 5
	@make health

# Build and start all services
up-build:
	@echo "Building and starting services..."
	@make down || true
	@make clean || true
	@docker compose build --no-cache
	@make up

# Stop all services
down:
	@echo "Stopping services..."
	docker compose down --remove-orphans

# Build all services
build:
	@echo "Building services..."
	docker compose build --pull

# View logs
logs:
	docker compose logs -f

# List running services
ps:
	@echo "=== Service Status ==="
	docker compose ps
	@echo "\n=== Container Health ==="
	@docker compose ps --format "table {{.Name}}\t{{.Status}}\t{{.Health}}"

# Clean everything
clean: down
	@echo "Cleaning up..."
	docker compose down -v
	docker system prune -f
	rm -rf backend/target
	rm -rf frontend/node_modules frontend/dist

# Restart all services
restart: down up

# Show debug information
debug:
	@echo "=== Docker Compose Status ==="
	docker compose ps
	@echo "\n=== Container Health ==="
	docker compose ps --format "table {{.Name}}\t{{.Status}}\t{{.Health}}"
	@echo "\n=== Container Logs ==="
	docker compose logs --tail=50
	@echo "\n=== Container Stats ==="
	docker stats --no-stream

# Start only PostgreSQL
up-postgres:
	@echo "Starting PostgreSQL..."
	docker compose up -d postgres
	@echo "Waiting for PostgreSQL to be ready..."
	@sleep 5
	@make health

# Development commands
dev-backend:
	cd backend && cargo run

dev-frontend:
	cd frontend && npm install && npm start

dev-metrics:
	cd backend && cargo run --bin generate_metrics

# Start AWS data generator
aws-data:
	@echo "Starting AWS-like data generator..."
	cd backend && cargo run --bin generate_aws_data

# Individual service logs
logs-backend:
	docker compose logs -f backend

logs-frontend:
	docker compose logs -f frontend

logs-postgres:
	docker compose logs -f postgres

logs-metrics:
	docker compose logs -f metrics-generator

logs-aws:
	docker compose logs -f aws-data-generator

# Check service health
health:
	@echo "=== Service Health Check ==="
	@echo "\nChecking PostgreSQL..."
	@docker compose exec -T postgres pg_isready -U postgres || echo "Database is not responding"
	@echo "\nChecking Backend..."
	@curl -s http://localhost:8080/health || echo "Backend is not responding"
	@echo "\nChecking Frontend..."
	@curl -s -I http://localhost || echo "Frontend is not responding"

# Show container stats
stats:
	docker stats --no-stream

# Remove all unused Docker resources
prune:
	@echo "Removing unused Docker resources..."
	docker system prune -af --volumes 