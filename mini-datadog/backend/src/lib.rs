//! lib.rs
//! Library module that builds the main Axum router and merges sub-routers.

use axum::Router;

use crate::config::AppConfig;
use crate::routes::{logs, metrics, traces, alerts};

pub mod handlers;
pub mod models;
pub mod db;
pub mod utils;
pub mod services;
pub mod config;

// Re-export commonly used items
pub use config::Config;
pub use services::db_service::DbService;

mod routes;

pub fn get_app_router(cfg: AppConfig) -> Router {
    Router::new()
        .merge(logs::create_route())
        .merge(metrics::create_route())
        .merge(traces::create_route())
        .merge(alerts::create_route())
        .layer(axum::middleware::from_fn(move |req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next<axum::body::Body>| {
            next.run(req)
        }))
}