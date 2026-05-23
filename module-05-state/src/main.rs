use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

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

fn todo_routes(store: TodoStore) -> Router {
    Router::new()
        // Testes:
        //
        // curl -w '\n\n' http://localhost:8000/todos
        //
        // curl -w '\n\n' -X POST http://localhost:8000/todos \
        //   -H 'Content-Type: application/json' \
        //   -d '{"title":"Aprender state no Axum"}'
        .route("/todos", get(list_todos).post(create_todo))
        .with_state(store)
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

type TodoStore = Arc<RwLock<HashMap<String, Todo>>>;

async fn list_todos(State(store): State<TodoStore>) -> Json<Vec<Todo>> {
    let todos = store.read().unwrap();

    let todos_vec = todos.values().cloned().collect();

    Json(todos_vec)
}

async fn create_todo(
    State(store): State<TodoStore>,
    Json(input): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: input.title,
        completed: false,
    };

    store.write().unwrap().insert(todo.id.clone(), todo.clone());

    (StatusCode::CREATED, Json(todo))
}
