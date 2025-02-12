use chrono::Utc;
use rand::Rng;
use uuid::Uuid;
use crate::generators::ApplicationLog;
use serde_json::json;

const SERVICES: &[&str] = &[
    "auth-service", 
    "db-service", 
    "api-gateway", 
    "billing-service",
    "ec2-instance",
    "rds-postgres",
    "elasticache-redis",
    "lambda-function",
    "s3-bucket",
    "cloudfront-cdn"
];

const LOG_LEVELS: &[&str] = &["INFO", "WARN", "ERROR", "DEBUG"];

struct LogTemplate {
    level: &'static str,
    message: &'static str,
    service: &'static str,
}

const LOG_TEMPLATES: &[LogTemplate] = &[
    // Auth Service Logs
    LogTemplate { level: "INFO", message: "User login successful", service: "auth-service" },
    LogTemplate { level: "WARN", message: "Failed login attempt from IP {}", service: "auth-service" },
    LogTemplate { level: "ERROR", message: "Token validation failed", service: "auth-service" },
    LogTemplate { level: "INFO", message: "Password reset requested", service: "auth-service" },
    
    // Database Service Logs
    LogTemplate { level: "INFO", message: "Query executed successfully in {}ms", service: "db-service" },
    LogTemplate { level: "ERROR", message: "Database connection timeout", service: "db-service" },
    LogTemplate { level: "WARN", message: "Slow query detected: {}", service: "db-service" },
    LogTemplate { level: "ERROR", message: "Failed to execute transaction", service: "db-service" },
    
    // API Gateway Logs
    LogTemplate { level: "INFO", message: "Request processed successfully", service: "api-gateway" },
    LogTemplate { level: "ERROR", message: "Rate limit exceeded for API key", service: "api-gateway" },
    LogTemplate { level: "WARN", message: "High latency detected on endpoint {}", service: "api-gateway" },
    
    // Billing Service Logs
    LogTemplate { level: "INFO", message: "Payment processed successfully", service: "billing-service" },
    LogTemplate { level: "ERROR", message: "Payment processing failed: {}", service: "billing-service" },
    LogTemplate { level: "WARN", message: "Unusual transaction pattern detected", service: "billing-service" },
    
    // EC2 Instance Logs
    LogTemplate { level: "INFO", message: "Instance started successfully", service: "ec2-instance" },
    LogTemplate { level: "WARN", message: "High CPU utilization: {}%", service: "ec2-instance" },
    LogTemplate { level: "ERROR", message: "Instance termination due to health check failure", service: "ec2-instance" },
    
    // RDS Logs
    LogTemplate { level: "INFO", message: "Database backup completed", service: "rds-postgres" },
    LogTemplate { level: "WARN", message: "High memory usage: {}%", service: "rds-postgres" },
    LogTemplate { level: "ERROR", message: "Failover initiated", service: "rds-postgres" },
    
    // ElastiCache Logs
    LogTemplate { level: "INFO", message: "Cache eviction completed", service: "elasticache-redis" },
    LogTemplate { level: "WARN", message: "Cache hit ratio below threshold: {}%", service: "elasticache-redis" },
    LogTemplate { level: "ERROR", message: "Node failure detected", service: "elasticache-redis" },
    
    // Lambda Logs
    LogTemplate { level: "INFO", message: "Function execution completed in {}ms", service: "lambda-function" },
    LogTemplate { level: "ERROR", message: "Function timeout after {}ms", service: "lambda-function" },
    LogTemplate { level: "WARN", message: "Memory usage approaching limit: {}MB", service: "lambda-function" },
    
    // S3 Logs
    LogTemplate { level: "INFO", message: "Object uploaded successfully: {}", service: "s3-bucket" },
    LogTemplate { level: "ERROR", message: "Failed to delete object: {}", service: "s3-bucket" },
    LogTemplate { level: "WARN", message: "Bucket size approaching limit", service: "s3-bucket" },
    
    // CloudFront Logs
    LogTemplate { level: "INFO", message: "Cache refresh completed for {}", service: "cloudfront-cdn" },
    LogTemplate { level: "ERROR", message: "Origin server error for {}", service: "cloudfront-cdn" },
    LogTemplate { level: "WARN", message: "Increased error rate detected in region {}", service: "cloudfront-cdn" },
];

pub struct LogsGenerator;

impl LogsGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_placeholder(&self) -> String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..6) {
            0 => format!("{}.{}.{}.{}", rng.gen_range(1..255), rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)), // IP
            1 => format!("{}ms", rng.gen_range(1..1000)), // Latency
            2 => format!("{}%", rng.gen_range(0..100)), // Percentage
            3 => format!("{}MB", rng.gen_range(1..1024)), // Memory
            4 => format!("{}", Uuid::new_v4()), // UUID
            _ => format!("{}", ["us-east-1", "us-west-2", "eu-west-1", "ap-southeast-1"][rng.gen_range(0..4)]), // Region
        }
    }

    pub fn generate(&self, count: usize) -> Vec<ApplicationLog> {
        let mut rng = rand::thread_rng();
        let now = Utc::now();

        (0..count).map(|_| {
            let template = &LOG_TEMPLATES[rng.gen_range(0..LOG_TEMPLATES.len())];
            let message = template.message.replace("{}", &self.generate_placeholder());

            // Generate random metadata based on the service
            let metadata = match template.service {
                "ec2-instance" => json!({
                    "instance_type": ["t2.micro", "t2.small", "t2.medium"][rng.gen_range(0..3)],
                    "availability_zone": ["a", "b", "c"][rng.gen_range(0..3)],
                }),
                "rds-postgres" => json!({
                    "db_identifier": format!("db-{}", Uuid::new_v4()),
                    "engine_version": ["13.7", "14.3", "15.1"][rng.gen_range(0..3)],
                }),
                "lambda-function" => json!({
                    "function_name": format!("lambda-{}", Uuid::new_v4()),
                    "runtime": ["nodejs18.x", "python3.9", "java11"][rng.gen_range(0..3)],
                }),
                _ => json!({}),
            };

            ApplicationLog {
                timestamp: now,
                log_level: template.level.to_string(),
                message,
                service_name: template.service.to_string(),
                user_id: if rng.gen_bool(0.3) { Some(Uuid::new_v4().to_string()) } else { None },
                request_id: Uuid::new_v4().to_string(),
                metadata: Some(metadata),
            }
        }).collect()
    }
} 