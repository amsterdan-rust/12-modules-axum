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

async fn get_config(State(config): State<Arc<AppConfig>>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "app_name": config.app_name,
        "version": config.version,
        "max_items_per_page": config.max_items_per_page,
    }))
}

fn app() -> Router {
    let config = Arc::new(AppConfig {
        app_name: "Axum State API".to_string(),
        version: "0.1.0".to_string(),
        max_items_per_page: 100,
    });

    let store: TodoStore = Arc::new(RwLock::new(HashMap::new()));

    Router::new()
        // Teste:
        //
        // curl -w '\n\n' http://localhost:8000/config
        .route("/config", get(get_config))
        .with_state(config)
        .merge(todo_routes(store))
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
