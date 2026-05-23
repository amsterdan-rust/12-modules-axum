use std::{collections::HashMap, sync::Arc};

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::AppState;

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        // Testes:
        //
        // curl -w '\n\n' http://localhost:8000/todos
        //
        // curl -w '\n\n' -X POST http://localhost:8000/todos \
        //   -H 'Content-Type: application/json' \
        //   -d '{"title":"Aprender state no Axum"}'
        //
        // curl -w '\n\n' http://localhost:8000/todos/{id}
        //
        // curl -w '\n\n' -X PUT http://localhost:8000/todos/{id} \
        //   -H 'Content-Type: application/json' \
        //   -d '{"completed":true}'
        //
        // curl -w '\n\n' -X DELETE http://localhost:8000/todos/{id}
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

pub type TodoStore = Arc<RwLock<HashMap<String, Todo>>>;

async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.read().await;

    let todos_vec = todos.values().cloned().collect();

    Json(todos_vec)
}

async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: input.title,
        completed: false,
    };

    state
        .todos
        .write()
        .await
        .insert(todo.id.clone(), todo.clone());

    (StatusCode::CREATED, Json(todo))
}

async fn get_todo(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = state.todos.read().await;

    todos
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_todo(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.todos.write().await;

    if let Some(todo) = todos.get_mut(&id) {
        if let Some(title) = input.title {
            todo.title = title;
        }

        if let Some(completed) = input.completed {
            todo.completed = completed;
        }

        Ok(Json(todo.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_todo(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> StatusCode {
    let mut todos = state.todos.write().await;

    if todos.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
