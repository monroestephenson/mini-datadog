pub mod aws_metrics;
pub mod aws_logs;
pub mod aws_traces;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemMetric {
    pub timestamp: DateTime<Utc>,
    pub instance_id: String,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_in: f64,
    pub network_out: f64,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationLog {
    pub timestamp: DateTime<Utc>,
    pub log_level: String,
    pub message: String,
    pub service_name: String,
    pub user_id: Option<String>,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApmTrace {
    pub timestamp: DateTime<Utc>,
    pub trace_id: String,
    pub span_id: String,
    pub service_name: String,
    pub operation: String,
    pub latency_ms: i32,
    pub status: String,
    pub error_message: Option<String>,
} 