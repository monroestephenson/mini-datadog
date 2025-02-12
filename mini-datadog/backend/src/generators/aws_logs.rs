use chrono::Utc;
use rand::Rng;
use uuid::Uuid;
use crate::generators::ApplicationLog;

const SERVICES: &[&str] = &["auth-service", "db-service", "api-gateway", "billing-service"];
const LOG_LEVELS: &[&str] = &["INFO", "WARNING", "ERROR"];

struct LogTemplate {
    level: &'static str,
    message: &'static str,
}

const LOG_TEMPLATES: &[LogTemplate] = &[
    LogTemplate { level: "INFO", message: "User login successful" },
    LogTemplate { level: "WARNING", message: "Failed login attempt, incorrect password" },
    LogTemplate { level: "ERROR", message: "Database connection timeout" },
    LogTemplate { level: "INFO", message: "Payment processing completed" },
    LogTemplate { level: "ERROR", message: "Payment processing failed, insufficient funds" },
    LogTemplate { level: "WARNING", message: "API request took too long, possible bottleneck" },
    LogTemplate { level: "INFO", message: "User profile updated successfully" },
];

pub struct LogsGenerator;

impl LogsGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, count: usize) -> Vec<ApplicationLog> {
        let mut rng = rand::thread_rng();
        let now = Utc::now();

        (0..count).map(|_| {
            let template = &LOG_TEMPLATES[rng.gen_range(0..LOG_TEMPLATES.len())];
            let service = SERVICES[rng.gen_range(0..SERVICES.len())];

            // 70% chance of having a user_id
            let user_id = if rng.gen_bool(0.7) {
                Some(Uuid::new_v4().to_string())
            } else {
                None
            };

            ApplicationLog {
                timestamp: now,
                log_level: template.level.to_string(),
                message: template.message.to_string(),
                service_name: service.to_string(),
                user_id,
                request_id: Uuid::new_v4().to_string(),
            }
        }).collect()
    }
} 