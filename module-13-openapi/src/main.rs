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
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Clone, Serialize, ToSchema)]
struct User {
    #[schema(example = 1)]
    id: u64,

    #[schema(example = "Alice")]
    name: String,

    #[schema(example = "alice@example.com")]
    email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
struct CreateUser {
    #[schema(example = "Alice")]
    name: String,

    #[schema(example = "alice@example.com")]
    email: String,
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Module 13 OpenAPI",
        description = "Example Axum API documented with OpenAPI and Swagger UI",
        version = "0.1.0",
        contact(
            name = "Amsterdan Vasconcelos",
            email = "amsterdan.rust@gmail.com"
        ),
        license(
            name = "MIT"
        )
        ),
    paths(
        health,
        list_users,
        create_user,
        get_user
    ),
    components(
        schemas(User, CreateUser, ErrorResponse)
    ),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "health", description = "Application health endpoints")
    )
)]
struct ApiDoc;

#[derive(Debug, Serialize, ToSchema)]
struct ErrorResponse {
    #[schema(example = "User not found")]
    message: String,

    #[schema(example = 404)]
    status: u16,
}

type UserStore = Arc<RwLock<HashMap<u64, User>>>;

fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            message: message.to_string(),
            status: status.as_u16(),
        }),
    )
}

#[utoipa::path(
    get,
    path="/health",
    tag = "health",
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
    tag = "users",
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
    tag = "users",
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
    tag = "users",
    params(
        ("id" = u64, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found", body = ErrorResponse)
    )
)]
async fn get_user(
    State(store): State<UserStore>,
    Path(id): Path<u64>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    let users = store.read().unwrap();

    users
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| error_response(StatusCode::NOT_FOUND, "User not found"))
}

fn create_app(store: UserStore) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
    println!("GET  /api-docs/openapi.json");
    println!("GET  /swagger-ui");

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
