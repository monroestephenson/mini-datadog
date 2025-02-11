//! config.rs
//! Configuration struct for the application.

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub kafka_brokers: String,
    pub nats_url: String,
    // Additional configuration fields
}

impl AppConfig {
    pub fn load() -> Self {
        // Placeholder: load from environment or file
        Self {
            database_url: "postgres://user:pass@localhost/db".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            kafka_brokers: "localhost:9092".to_string(),
            nats_url: "localhost:4222".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub metrics_retention_days: i32,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/mini_datadog".to_string()),
            metrics_retention_days: std::env::var("METRICS_RETENTION_DAYS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }
}