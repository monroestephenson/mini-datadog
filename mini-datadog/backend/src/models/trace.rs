//! trace.rs
//! Data model for distributed traces.

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trace {
    pub trace_id: String,
    pub span_id: String,
    pub parent_id: Option<String>,
    pub operation_name: String,
    pub start_time: String,
    pub end_time: String,
    // Additional fields for metadata
}