//! alerts.rs
//! Defines routes for alert management.

use axum::{
    routing::{post, get},
    Router,
};

use crate::handlers::alerts_handler::*;

pub fn create_route() -> Router {
    Router::new()
        .route("/alerts/create", post(create_alert))
        .route("/alerts/list", get(list_alerts))
        .route("/alerts/delete", post(delete_alert))
}