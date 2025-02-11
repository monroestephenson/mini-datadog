//! main.rs
//! Entry point for the Mini Datadog backend.

use axum::{
    routing::{get, post},
    Router, Extension,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::sync::Arc;

mod routes;
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
        .route("/api/metrics", post(handlers::metrics_handler::ingest_metrics))
        .route("/api/metrics", get(handlers::metrics_handler::get_metrics))
        .route("/api/metrics/custom", post(handlers::metrics_handler::ingest_custom_metric))
        .route("/logs/live", get(handlers::logs::ws_handler))
        .route("/api/alerts", get(handlers::alerts::list_alerts))
        .route("/api/alerts", post(handlers::alerts::create_alert))
        .route("/api/alerts/:id", delete(handlers::alerts::delete_alert))
        .layer(Extension(db))
        .layer(CorsLayer::permissive());

    // Start server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    tracing::info!("Server running on http://127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();
}