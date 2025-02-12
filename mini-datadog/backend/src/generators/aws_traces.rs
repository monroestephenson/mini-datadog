use chrono::Utc;
use rand::Rng;
use uuid::Uuid;
use crate::generators::ApmTrace;

const SERVICES: &[&str] = &["api-gateway", "auth-service", "db-service"];
const OPERATIONS: &[&str] = &["POST /login", "GET /profile", "POST /payment"];
const ERROR_MESSAGES: &[&str] = &[
    "Connection timeout",
    "Internal server error",
    "Service unavailable",
    "Database query failed",
    "Rate limit exceeded",
];

pub struct TracesGenerator;

impl TracesGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, count: usize) -> Vec<ApmTrace> {
        let mut rng = rand::thread_rng();
        let now = Utc::now();

        (0..count).map(|_| {
            // 5% chance of high latency
            let latency_spike = rng.gen_bool(0.05);
            let latency_ms = if latency_spike {
                rng.gen_range(2000..5000)
            } else {
                rng.gen_range(20..500)
            };

            // 10% chance of failure
            let failed = rng.gen_bool(0.1);
            let status = if failed { "failed" } else { "success" };
            let error_message = if failed {
                Some(ERROR_MESSAGES[rng.gen_range(0..ERROR_MESSAGES.len())].to_string())
            } else {
                None
            };

            ApmTrace {
                timestamp: now,
                trace_id: Uuid::new_v4().to_string(),
                span_id: Uuid::new_v4().to_string(),
                service_name: SERVICES[rng.gen_range(0..SERVICES.len())].to_string(),
                operation: OPERATIONS[rng.gen_range(0..OPERATIONS.len())].to_string(),
                latency_ms,
                status: status.to_string(),
                error_message,
            }
        }).collect()
    }
} 