use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub id: i32,
    pub name: String,
    pub condition: String,
}

pub async fn list_alerts() -> Json<Vec<Alert>> {
    Json(vec![])
}

pub async fn create_alert(Json(_alert): Json<Alert>) -> Json<Alert> {
    // Placeholder
    Json(Alert {
        id: 1,
        name: "Test Alert".to_string(),
        condition: "cpu > 90".to_string(),
    })
}

pub async fn delete_alert(axum::extract::Path(_id): axum::extract::Path<i32>) -> Json<()> {
    Json(())
} 