use mini_datadog_backend::models::Metric;
use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;
use rand::Rng;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut rng = rand::thread_rng();

    println!("Starting metrics generator...");
    loop {
        let metric = Metric {
            time: chrono::Utc::now().format("%H:%M:%S").to_string(),
            cpu: rng.gen_range(0.0..100.0),
            memory: rng.gen_range(0.0..32.0), // Simulating up to 32GB memory
            latency: rng.gen_range(10.0..500.0), // Latency in ms
            request_count: rng.gen_range(0..1000),
        };

        match client.post("http://localhost:8080/api/metrics")
            .json(&metric)
            .send()
            .await
        {
            Ok(_) => println!("Sent metric: {:?}", metric),
            Err(e) => eprintln!("Failed to send metric: {}", e),
        }

        sleep(Duration::from_secs(1)).await;
    }
} 