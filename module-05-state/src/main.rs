mod metricx;
mod todo;

use axum::{Extension, Json, Router, extract::State, routing::get};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    metricx::{Metrics, get_metrics, track_request},
    todo::{TodoStore, todo_routes},
};

#[derive(Clone)]
struct AppConfig {
    app_name: String,
    version: String,
    max_items_per_page: usize,
}

#[derive(Clone)]
pub struct AppState {
    config: Arc<AppConfig>,
    todos: TodoStore,
    metrics: Arc<RwLock<Metrics>>,
    db: DbPool,
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
        db: DbPool::new("postgres://localhost/my_app"),
    };

    let current_user = CurrentUser {
        id: "user-123".to_string(),
        name: "Demo User".to_string(),
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
        .route("/db/users", get(list_users_from_db))
        .route("/me", get(get_current_user))
        .merge(todo_routes())
        .with_state(state)
        .layer(Extension(current_user))
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
struct DbPool {
    connection_string: String,
    max_connections: u32,
}

impl DbPool {
    fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            max_connections: 10,
        }
    }

    async fn query_users(&self) -> Vec<String> {
        vec!["Ana".to_string(), "Bruno".to_string(), "Carla".to_string()]
    }
}

async fn list_users_from_db(State(state): State<AppState>) -> Json<Vec<String>> {
    let users = state.db.query_users().await;

    Json(users)
}

#[derive(Clone)]
struct CurrentUser {
    id: String,
    name: String,
}

async fn get_current_user(Extension(user): Extension<CurrentUser>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "id": user.id,
        "name": user.name,
    }))
}
