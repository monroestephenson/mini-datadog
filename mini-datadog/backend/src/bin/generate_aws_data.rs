use mini_datadog_backend::generators::{
    aws_metrics::MetricsGenerator,
    aws_logs::LogsGenerator,
    aws_traces::TracesGenerator,
};
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let metrics_gen = MetricsGenerator::new();
    let logs_gen = LogsGenerator::new();
    let traces_gen = TracesGenerator::new();

    println!("Starting AWS-like data generator...");

    loop {
        // Generate and send metrics (10 instances)
        let metrics = metrics_gen.generate();
        for metric in metrics {
            match client.post("http://backend:8080/api/metrics/aws")
                .json(&metric)
                .send()
                .await
            {
                Ok(_) => println!("Sent metric for instance {}", metric.instance_id),
                Err(e) => eprintln!("Failed to send metric: {}", e),
            }
        }

        // Generate and send logs (50 per minute)
        let logs = logs_gen.generate(50);
        for log in logs {
            match client.post("http://backend:8080/api/logs/aws")
                .json(&log)
                .send()
                .await
            {
                Ok(_) => println!("Sent log: {} - {}", log.service_name, log.message),
                Err(e) => eprintln!("Failed to send log: {}", e),
            }
        }

        // Generate and send traces (100 per minute)
        let traces = traces_gen.generate(100);
        for trace in traces {
            match client.post("http://backend:8080/api/traces/aws")
                .json(&trace)
                .send()
                .await
            {
                Ok(_) => println!("Sent trace: {} - {}", trace.service_name, trace.operation),
                Err(e) => eprintln!("Failed to send trace: {}", e),
            }
        }

        // Wait for 1 minute before next batch
        sleep(Duration::from_secs(60)).await;
    }
} 