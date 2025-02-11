//! main.rs
//! Entry point for the Mini Datadog backend.

use axum::{
    routing::{get, post, delete},
    Router, Extension,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

use crate::handlers::{metrics_handler, logs_handler, alerts_handler};

mod handlers;
mod models;
mod db;
mod utils;
mod services;
mod config;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize config
    let config = config::Config::from_env();

    // Initialize database
    let db = Arc::new(
        services::db_service::DbService::new(&config.database_url)
            .await
            .expect("Failed to initialize database")
    );

    // Set up metrics cleanup task
    let cleanup_db = db.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400)); // Daily
        loop {
            interval.tick().await;
            if let Err(e) = cleanup_db.cleanup_old_metrics(config.metrics_retention_days).await {
                tracing::error!("Failed to cleanup old metrics: {:?}", e);
            }
        }
    });

    // Create router with routes
    let app = Router::new()
        .route("/api/metrics", post(metrics_handler::ingest_metrics))
        .route("/api/metrics", get(metrics_handler::get_metrics))
        .route("/api/metrics/custom", post(metrics_handler::ingest_custom_metric))
        .route("/logs/live", get(logs_handler::ws_handler))
        .route("/api/alerts", get(alerts_handler::list_alerts))
        .route("/api/alerts", post(alerts_handler::create_alert))
        .route("/api/alerts/:id", delete(alerts_handler::delete_alert))
        .layer(Extension(db))
        .layer(CorsLayer::permissive());

    // Start server
    let addr = "0.0.0.0:8080";
    tracing::info!("Starting server on {}", addr);
    axum::serve(
        TcpListener::bind(addr)
            .await
            .expect("Failed to bind to address"),
        app,
    )
    .await
    .expect("Failed to start server");
}