# Mini Datadog

A simplified version of Datadog with metrics collection, visualization, and monitoring capabilities.

## Prerequisites

- Docker (with Docker Compose V2)
- Make (optional, for using Makefile commands)

## Quick Start

1. Clone the repository:
```bash
git clone <repository-url>
cd mini-datadog
```

2. Start all services:
```bash
make up
```

The application will be available at:
- Frontend: http://localhost
- Backend API: http://localhost:8080
- PostgreSQL: localhost:5432

## Available Make Commands

- `make help` - Show available commands
- `make up` - Start all services
- `make down` - Stop all services
- `make build` - Rebuild all services
- `make logs` - View logs from all services
- `make ps` - List running services
- `make clean` - Remove all containers, volumes, and build artifacts
- `make restart` - Restart all services

### Development Commands

- `make dev-backend` - Run backend in development mode
- `make dev-frontend` - Run frontend in development mode
- `make dev-metrics` - Run metrics generator in development mode

### Individual Service Logs

- `make logs-backend` - View backend logs
- `make logs-frontend` - View frontend logs
- `make logs-postgres` - View database logs
- `make logs-metrics` - View metrics generator logs

## Architecture

The application consists of four main components:

1. **Frontend**: React application with Material-UI and Recharts for visualization
2. **Backend**: Rust-based API server using Axum framework
3. **Database**: PostgreSQL for storing metrics
4. **Metrics Generator**: Simulates system metrics collection

## Development

### Running Services Individually

You can run each service in development mode:

1. Start the database:
```bash
make up postgres
```

2. Start the backend:
```bash
make dev-backend
```

3. Start the frontend:
```bash
make dev-frontend
```

4. Start the metrics generator:
```bash
make dev-metrics
```

### Building for Production

```bash
make build
make up
```

## Troubleshooting

1. If services fail to start, check logs:
```bash
make logs
```

2. To reset everything:
```bash
make clean
make build
make up
```

3. If the database connection fails:
```bash
make down
make up postgres  # Start postgres first
make up          # Start other services
```