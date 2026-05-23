use axum::{Json, extract::State};

use crate::AppState;

#[derive(Debug, Default)]
pub struct Metrics {
    request_count: u64,
    error_count: u64,
}

pub async fn get_metrics(State(state): State<AppState>) -> Json<serde_json::Value> {
    let metrics = state.metrics.read().await;

    Json(serde_json::json!({
        "requests": metrics.request_count,
        "errors": metrics.error_count,
        "app_version": state.config.version,
    }))
}

pub async fn track_request(State(state): State<AppState>) -> &'static str {
    let mut metrics = state.metrics.write().await;

    metrics.request_count += 1;

    "Request counted!"
}
