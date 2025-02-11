//! metrics_handler.rs
//! HTTP handler logic for metrics.

use axum::{
    extract::Extension,
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::handlers::metrics_handler::{Metric, CustomMetric};
use crate::services::db_service::DbService;

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
    name: String,
    value: f64,
    labels: std::collections::HashMap<String, String>,
    timestamp: DateTime<Utc>,
}

pub async fn ingest_metrics(
    Extension(db): Extension<Arc<DbService>>,
    Json(metric): Json<Metric>,
) -> Json<Metric> {
    if let Err(e) = db.store_metric(&metric).await {
        tracing::error!("Failed to store metric: {:?}", e);
    }
    
    Json(metric)
}

pub async fn get_metrics(
    Extension(db): Extension<Arc<DbService>>,
) -> Json<Vec<Metric>> {
    match db.get_recent_metrics(20).await {
        Ok(metrics) => Json(metrics),
        Err(e) => {
            tracing::error!("Failed to fetch metrics: {:?}", e);
            Json(vec![])
        }
    }
}

pub async fn ingest_custom_metric(
    Json(metric): Json<CustomMetric>
) -> Json<CustomMetric> {
    // Store custom metric
    store_custom_metric(&metric).await;
    Json(metric)
}

async fn store_metric(metric: &Metric) {
    // TODO: Implement database storage
    tracing::info!("Storing metric: {:?}", metric);
}

async fn store_custom_metric(metric: &CustomMetric) {
    // TODO: Implement database storage
    tracing::info!("Storing custom metric: {:?}", metric);
}

pub async fn query_metrics(Query(params): Query<serde_json::Value>) -> String {
    // Placeholder: query metrics from DB or time-series store
    format!("Query metrics with params: {:?}", params)
}

pub struct MetricsService {
    tx: Arc<broadcast::Sender<Metric>>,
}

impl MetricsService {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx: Arc::new(tx),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Metric> {
        self.tx.subscribe()
    }

    pub fn get_sender(&self) -> Arc<broadcast::Sender<Metric>> {
        self.tx.clone()
    }
}