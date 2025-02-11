//! lib.rs
//! Library module that builds the main Axum router and merges sub-routers.

use axum::Router;

use crate::config::AppConfig;
use crate::routes::{logs, metrics, traces, alerts};

pub fn get_app_router(cfg: AppConfig) -> Router {
    Router::new()
        .merge(logs::create_route())
        .merge(metrics::create_route())
        .merge(traces::create_route())
        .merge(alerts::create_route())
        // You can pass config to each route if needed
        .layer(axum::middleware::from_fn(move |req, next| {
            // Middleware placeholder
            next.run(req)
        }))
}