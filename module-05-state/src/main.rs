mod todo;

use axum::{Json, Router, extract::State, routing::get};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::todo::{TodoStore, todo_routes};

#[derive(Clone)]
struct AppConfig {
    app_name: String,
    version: String,
    max_items_per_page: usize,
}

async fn get_config(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "app_name": state.config.app_name,
        "version": state.config.version,
        "max_items_per_page": state.config.max_items_per_page,
    }))
}

fn app() -> Router {
    let config = Arc::new(AppConfig {
        app_name: "Axum State API".to_string(),
        version: "0.1.0".to_string(),
        max_items_per_page: 100,
    });

    let todos: TodoStore = Arc::new(RwLock::new(HashMap::new()));

    let state = AppState {
        config,
        todos,
        metrics: Arc::new(RwLock::new(Metrics::default())),
    };

    Router::new()
        // Teste:
        //
        // curl -w '\n\n' http://localhost:8000/config
        .route("/config", get(get_config))
        // Testes:
        //
        // curl -w '\n\n' http://localhost:8000/metrics
        //
        // curl -w '\n\n' http://localhost:8000/track
        //
        // curl -w '\n\n' http://localhost:8000/metrics
        .route("/metrics", get(get_metrics))
        .route("/track", get(track_request))
        .merge(todo_routes())
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("failed to bind");

    println!("🚀 Module 05: State Management");
    println!("Server running on http://localhost:8000");

    axum::serve(listener, app()).await.expect("server failed");
}

#[derive(Clone)]
pub struct AppState {
    config: Arc<AppConfig>,
    todos: TodoStore,
    metrics: Arc<RwLock<Metrics>>,
}

#[derive(Debug, Default)]
struct Metrics {
    request_count: u64,
    error_count: u64,
}

async fn get_metrics(State(state): State<AppState>) -> Json<serde_json::Value> {
    let metrics = state.metrics.read().await;

    Json(serde_json::json!({
        "requests": metrics.request_count,
        "errors": metrics.error_count,
        "app_version": state.config.version,
    }))
}

async fn track_request(State(state): State<AppState>) -> &'static str {
    let mut metrics = state.metrics.write().await;

    metrics.request_count += 1;

    "Request counted!"
}
