-- Create metrics table
CREATE TABLE IF NOT EXISTS system_metrics (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    cpu DOUBLE PRECISION NOT NULL,
    memory DOUBLE PRECISION NOT NULL,
    latency DOUBLE PRECISION NOT NULL,
    request_count INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create custom metrics table
CREATE TABLE IF NOT EXISTS custom_metrics (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    labels JSONB NOT NULL DEFAULT '{}',
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_system_metrics_timestamp ON system_metrics(timestamp);
CREATE INDEX idx_custom_metrics_timestamp ON custom_metrics(timestamp);
CREATE INDEX idx_custom_metrics_name ON custom_metrics(name); 