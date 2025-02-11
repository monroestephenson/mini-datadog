//! alerts_handler.rs
//! HTTP handler logic for real-time alerts.

use axum::Json;
use serde_json::Value;

pub async fn create_alert(Json(payload): Json<Value>) -> String {
    format!("Create alert: {:?}", payload)
}

pub async fn list_alerts() -> String {
    "List of alerts".to_string()
}

pub async fn delete_alert(Json(payload): Json<Value>) -> String {
    format!("Delete alert: {:?}", payload)
}