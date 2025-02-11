//! traces_handler.rs
//! HTTP handler logic for distributed tracing.

use axum::{Json, extract::Query};
use serde_json::Value;

pub async fn ingest_traces(Json(payload): Json<Value>) -> String {
    format!("Received traces: {:?}", payload)
}

pub async fn query_traces(Query(params): Query<serde_json::Value>) -> String {
    format!("Query traces with params: {:?}", params)
}