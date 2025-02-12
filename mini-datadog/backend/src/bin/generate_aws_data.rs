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
        // Generate and send metrics (every 10 seconds)
        let metrics = metrics_gen.generate();
        for metric in metrics {
            match client.post("http://backend:8080/api/metrics/aws")
                .json(&metric)
                .send()
                .await
            {
                Ok(_) => println!("Sent metric for instance {} in {}", metric.instance_id, metric.region),
                Err(e) => eprintln!("Failed to send metric: {}", e),
            }
        }

        // Generate and send logs (100 per batch, every 10 seconds)
        let logs = logs_gen.generate(100);
        for log in logs {
            match client.post("http://backend:8080/api/logs/aws")
                .json(&log)
                .send()
                .await
            {
                Ok(_) => println!("Sent log: [{}] {} - {}", log.log_level, log.service_name, log.message),
                Err(e) => eprintln!("Failed to send log: {}", e),
            }
        }

        // Generate and send traces (200 per batch, every 10 seconds)
        let traces = traces_gen.generate(200);
        for trace in traces {
            match client.post("http://backend:8080/api/traces/aws")
                .json(&trace)
                .send()
                .await
            {
                Ok(_) => {
                    if trace.status == "failed" {
                        println!("Sent failed trace: {} - {} ({}ms) - {}", 
                            trace.service_name, 
                            trace.operation,
                            trace.latency_ms,
                            trace.error_message.unwrap_or_default()
                        );
                    } else {
                        println!("Sent trace: {} - {} ({}ms)", 
                            trace.service_name, 
                            trace.operation,
                            trace.latency_ms
                        );
                    }
                },
                Err(e) => eprintln!("Failed to send trace: {}", e),
            }
        }

        // Wait for 10 seconds before next batch
        sleep(Duration::from_secs(10)).await;
    }
} 