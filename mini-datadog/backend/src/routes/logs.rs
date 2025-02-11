//! logs.rs
//! Defines routes for log ingestion and retrieval.

use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::logs_handler::*;

pub fn create_route() -> Router {
    Router::new()
        .route("/logs/ingest", post(ingest_logs))
        .route("/logs/search", get(search_logs))
        .route("/logs/live", get(stream_logs_ws))
}