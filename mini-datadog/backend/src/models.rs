// Data models will go here 

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metric {
    pub time: String,
    pub cpu: f64,
    pub memory: f64,
    pub latency: f64,
    pub request_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
} 