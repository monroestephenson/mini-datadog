//! logs.rs
//! Defines routes for log ingestion and retrieval.

use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::logs_handler::*;

pub fn create_route() -> Router {
    Router::new()
        .route("/api/logs", post(ingest_logs))
        .route("/api/logs/aws", post(ingest_aws_logs))
        .route("/api/logs/search", get(search_logs))
        .route("/api/logs/live", get(ws_handler))
}