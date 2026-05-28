use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
struct CreateUser {
    name: String,
    email: String,
}

type UserStore = Arc<RwLock<HashMap<u64, User>>>;

#[utoipa::path(
    get,
    path="/health",
    responses(
        (status = 200, description = "Application is healthy", body = String)
    )
)]
async fn health() -> &'static str {
    "OK"
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = Vec<User>)
    )
)]
async fn list_users(State(store): State<UserStore>) -> Json<Vec<User>> {
    let users = store.read().unwrap();

    Json(users.values().cloned().collect())
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
async fn create_user(
    State(store): State<UserStore>,
    Json(input): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut users = store.write().unwrap();

    let id = users.len() as u64 + 1;

    let user = User {
        id,
        name: input.name,
        email: input.email,
    };

    users.insert(id, user.clone());

    (StatusCode::CREATED, Json(user))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = u64, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(
    State(store): State<UserStore>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = store.read().unwrap();

    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

fn create_app(store: UserStore) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user))
        .with_state(store)
}

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(HashMap::new()));

    let app = create_app(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("🚀 Module 13: OpenAPI");
    println!("Server: http://localhost:8000");
    println!("GET  /health");
    println!("GET  /users");
    println!("POST /users");
    println!("GET  /users/{{id}}");

    axum::serve(listener, app).await.unwrap();
}

/*
Teste com:

cargo r13

curl -i -w '\n\n' http://localhost:8000/health

curl -i -w '\n\n' http://localhost:8000/users

curl -i -w '\n\n' -X POST http://localhost:8000/users \
  -H 'Content-Type: application/json' \
  -d '{"name":"Alice","email":"alice@example.com"}'

curl -i -w '\n\n' http://localhost:8000/users/1
*/
