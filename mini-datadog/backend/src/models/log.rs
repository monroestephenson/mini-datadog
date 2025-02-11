//! log.rs
//! Data model for logs.

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: i32,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    // Additional fields for structured logging
}