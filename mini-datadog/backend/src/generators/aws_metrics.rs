use chrono::{DateTime, Utc};
use rand::Rng;
use uuid::Uuid;
use crate::generators::SystemMetric;

const REGIONS: &[&str] = &["us-east-1", "us-west-2", "eu-central-1"];
const INSTANCE_COUNT: usize = 10;

pub struct MetricsGenerator {
    instances: Vec<String>,
}

impl MetricsGenerator {
    pub fn new() -> Self {
        let instances = (0..INSTANCE_COUNT)
            .map(|_| Uuid::new_v4().to_string())
            .collect();
        Self { instances }
    }

    pub fn generate(&self) -> Vec<SystemMetric> {
        let mut rng = rand::thread_rng();
        let now = Utc::now();

        self.instances.iter().map(|instance_id| {
            // Add occasional spikes in CPU usage (10% chance of spike)
            let cpu_spike = rng.gen_bool(0.1);
            let cpu_usage = if cpu_spike {
                rng.gen_range(90.0..100.0)
            } else {
                rng.gen_range(10.0..95.0)
            };

            SystemMetric {
                timestamp: now,
                instance_id: instance_id.clone(),
                cpu_usage,
                memory_usage: rng.gen_range(20.0..98.0),
                disk_usage: rng.gen_range(50.0..500.0),
                network_in: rng.gen_range(10.0..500.0),
                network_out: rng.gen_range(10.0..300.0),
                region: REGIONS[rng.gen_range(0..REGIONS.len())].to_string(),
            }
        }).collect()
    }
} 