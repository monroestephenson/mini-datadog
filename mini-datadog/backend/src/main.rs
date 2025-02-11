//! main.rs
//! Entry point for the Mini Datadog backend.

use axum::{Router};
use tracing_subscriber::fmt::SubscriberBuilder;

mod lib;
mod config;

use crate::lib::{get_app_router};

#[tokio::main]
async fn main() {
    // Initialize tracing/logging
    SubscriberBuilder::default()
        .with_env_filter("mini_datadog=debug,tower_http=debug")
        .init();

    // Load configuration
    let cfg = config::AppConfig::load();

    // Build the app router
    let app = get_app_router(cfg);

    // Run server
    let addr = "0.0.0.0:8080".parse().unwrap();
    tracing::info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}