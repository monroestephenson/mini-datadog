# Mini Datadog

A simplified version of Datadog with metrics collection, visualization, and monitoring capabilities. This project aims to replicate core functionalities of Datadog, providing a learning platform for understanding observability systems.

## Overview

Mini Datadog is a full-stack observability platform that collects, processes, and visualizes:
- System metrics (CPU, Memory, Latency, Request Count)
- AWS-like infrastructure metrics
- Application logs
- APM (Application Performance Monitoring) traces
- Custom metrics

## Features

### Current Functionality
1. **Metrics Collection**
   - System metrics (CPU, Memory, Latency, Request Counts)
   - AWS-like infrastructure metrics from multiple simulated instances
   - Custom metric support with labels
   - Real-time metric ingestion and storage

2. **Logging System**
   - Structured log ingestion
   - Multi-service log aggregation
   - Real-time log streaming via WebSocket
   - Log search capabilities
   - AWS-style log formats with metadata

3. **APM (Traces)**
   - Distributed tracing support
   - Service-to-service communication tracking
   - Latency monitoring
   - Error tracking and analysis

4. **Data Generators**
   - System metrics generator (every 1 second)
   - AWS-like data generator (every 10 seconds)
     - Simulates multiple EC2 instances
     - Generates realistic service logs
     - Creates distributed traces

5. **Data Retention**
   - Configurable metrics retention (default: 30 days)
   - Automatic data cleanup
   - Efficient storage management

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

### Core Commands
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
- `make aws-data` - Start AWS-like data generator

### Individual Service Logs
- `make logs-backend` - View backend logs
- `make logs-frontend` - View frontend logs
- `make logs-postgres` - View database logs
- `make logs-metrics` - View metrics generator logs
- `make logs-aws` - View AWS data generator logs

## Architecture

The application consists of five main components:

1. **Frontend**
   - React application with Material-UI
   - Real-time data visualization using Recharts
   - Interactive dashboards
   - WebSocket connections for live updates

2. **Backend**
   - Rust-based API server using Axum framework
   - WebSocket support for real-time data streaming
   - Efficient data processing and storage
   - RESTful API endpoints for data ingestion and retrieval

3. **Database**
   - PostgreSQL for persistent storage
   - Optimized schema for metrics and metadata
   - Efficient querying capabilities

4. **Metrics Generator**
   - Simulates system metrics
   - Generates realistic load patterns
   - Configurable generation frequency

5. **AWS Data Generator**
   - Simulates AWS infrastructure
   - Generates realistic service logs
   - Creates distributed traces
   - Simulates multi-region deployment

## API Endpoints

### Metrics
- `POST /api/metrics` - Ingest system metrics
- `POST /api/metrics/custom` - Ingest custom metrics
- `GET /api/metrics` - Retrieve recent metrics
- `POST /api/metrics/aws` - Ingest AWS infrastructure metrics

### Logs
- `POST /api/logs` - Ingest application logs
- `POST /api/logs/aws` - Ingest AWS service logs
- `GET /api/logs/search` - Search logs with filters
- `GET /api/logs/live` - WebSocket endpoint for live log streaming

### Traces
- `POST /api/traces` - Ingest trace data
- `POST /api/traces/aws` - Ingest AWS service traces

### Alerts
- `GET /api/alerts` - List configured alerts
- `POST /api/alerts` - Create new alert
- `DELETE /api/alerts/:id` - Delete alert

## Development

### Running Services Individually

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

5. Start the AWS data generator:
```bash
make aws-data
```

### Building for Production

```bash
make build
make up
```

## Data Generators

### System Metrics Generator
Generates the following metrics every second:
- CPU usage (0-100%)
- Memory usage (0-32GB)
- Latency (10-500ms)
- Request count (0-1000)

### AWS Data Generator
Generates the following data every 10 seconds:
1. Infrastructure Metrics
   - CPU, Memory, Disk usage
   - Network I/O
   - Multiple regions and instances

2. Service Logs
   - Multiple service types (auth, db, api-gateway, etc.)
   - Various log levels (INFO, WARN, ERROR, DEBUG)
   - Realistic error scenarios
   - Rich metadata

3. APM Traces
   - Service-to-service communication
   - Latency measurements
   - Error tracking
   - Distributed trace correlation

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

4. To check service health:
```bash
make health
```

5. To view resource usage:
```bash
make stats
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.