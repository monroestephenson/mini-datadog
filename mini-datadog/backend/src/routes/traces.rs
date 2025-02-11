//! traces.rs
//! Defines routes for distributed tracing data.

use axum::{
    routing::{post, get},
    Router,
};

use crate::handlers::traces_handler::*;

pub fn create_route() -> Router {
    Router::new()
        .route("/traces/ingest", post(ingest_traces))
        .route("/traces/query", get(query_traces))
}