//! alert.rs
//! Data model for alerts.

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub id: i32,
    pub name: String,
    pub condition: String,
    // Additional fields for threshold, channels, etc.
}