//! logs_handler.rs
//! HTTP handler logic for logs.

use axum::{Json, extract::Query, extract::ws::{WebSocket, WebSocketUpgrade}, response::Response};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use std::sync::Arc;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub service: String,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub metadata: Option<Value>,
}

static LOGS_CHANNEL: tokio::sync::OnceCell<broadcast::Sender<LogEntry>> = tokio::sync::OnceCell::const_new();

pub async fn ingest_logs(Json(log): Json<LogEntry>) -> Json<LogEntry> {
    if let Some(sender) = LOGS_CHANNEL.get() {
        let _ = sender.send(log.clone());
    }
    Json(log)
}

pub async fn ingest_aws_logs(Json(log): Json<LogEntry>) -> Json<LogEntry> {
    if let Some(sender) = LOGS_CHANNEL.get() {
        let _ = sender.send(log.clone());
    }
    Json(log)
}

pub async fn search_logs(Query(params): Query<Value>) -> Json<Vec<LogEntry>> {
    // TODO: Implement log search from database
    Json(vec![])
}

pub async fn stream_logs_ws() -> String {
    // Placeholder: upgrade to WebSocket for streaming
    "WebSocket log tailing endpoint".to_string()
}

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let sender = LOGS_CHANNEL.get_or_init(|| {
        let (tx, _) = broadcast::channel(1000);
        tx
    }).clone();
    
    let mut receiver = sender.subscribe();

    while let Ok(log) = receiver.recv().await {
        if let Ok(json) = serde_json::to_string(&log) {
            if socket.send(axum::extract::ws::Message::Text(json)).await.is_err() {
                break;
            }
        }
    }
}