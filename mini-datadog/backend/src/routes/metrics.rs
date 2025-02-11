//! metrics.rs
//! Defines routes for metrics ingestion and retrieval.

use axum::{
    routing::{post, get},
    Router,
};

use crate::handlers::metrics_handler::*;

pub fn create_route() -> Router {
    Router::new()
        .route("/metrics/ingest", post(ingest_metrics))
        .route("/metrics/query", get(query_metrics))
}