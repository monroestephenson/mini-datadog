//! metric.rs
//! Data model for metrics.

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: String,
    // Additional fields for tags, host info, etc.
}