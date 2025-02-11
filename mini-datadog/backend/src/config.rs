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