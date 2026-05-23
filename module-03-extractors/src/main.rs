use axum::{
    Json, Router,
    body::Bytes,
    extract::{Path, Query},
    http::HeaderMap,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("não foi possível abrir a porta 8000");

    println!("Module 03: Extractors");
    println!("Servidor rodando em http://localhost:8000");

    axum::serve(listener, app())
        .await
        .expect("erro ao iniciar o servidor");
}

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/users/{id}", get(get_user))
        .route("/users", get(list_users).post(create_user))
        .route("/headers", get(show_headers))
        .route("/raw", post(raw_body))
        .route("/users/{id}/update", post(update_user_with_extractors))
}

#[derive(Debug, Deserialize)]
struct ListUsersParams {
    page: Option<u32>,
    limit: Option<u32>,
    sort: Option<String>,
}

// curl http://localhost:8000
async fn home() -> &'static str {
    "Module 03: Extractors"
}

// curl http://localhost:8000/users/10
async fn get_user(Path(id): Path<u64>) -> String {
    format!("Buscando usuário com id {id}")
}

// curl "http://localhost:8000/users"
// curl "http://localhost:8000/users?page=2"
// curl "http://localhost:8000/users?page=2&limit=5&sort=name"
async fn list_users(Query(params): Query<ListUsersParams>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let sort = params.sort.unwrap_or_else(|| "id".to_string());

    format!("Listando usuários - página {page}, limite {limit}, ordenação {sort}")
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct CreateUserResponse {
    id: u64,
    name: String,
    email: String,
}

// curl -X POST http://localhost:8000/users \
//   -H "Content-Type: application/json" \
//   -d '{"name":"Ana","email":"ana@example.com"}'
async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<CreateUserResponse> {
    Json(CreateUserResponse {
        id: 1,
        name: payload.name,
        email: payload.email,
    })
}

// curl -w "\n\n" http://localhost:8000/headers
// curl -w "\n\n" http://localhost:8000/headers \
//   -H "User-Agent: Meu Cliente Rust"
// curl -w "\n\n" http://localhost:8000/headers \
//   -H "Content-Type: application/json"
async fn show_headers(headers: HeaderMap) -> String {
    let user_agent = headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("desconhecido");

    let content_type = headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("não informado");

    format!("User-Agent: {user_agent}\nContent-Type: {content_type}")
}

// curl -w "\n\n" -X POST http://localhost:8000/raw \
//   -d "Olá, corpo bruto!"
//
// curl -w "\n\n" -X POST http://localhost:8000/raw \
//   -H "Content-Type: application/json" \
//   -d '{"message":"Olá"}'
async fn raw_body(body: Bytes) -> String {
    format!("Recebi {} bytes", body.len())
}

// curl -w '\n\n' -X POST 'http://localhost:8000/users/10/update?page=2&limit=5&sort=name' \
//   -H 'Content-Type: application/json' \
//   -H 'User-Agent: Meu Cliente Rust' \
//   -d '{"name":"Ana","email":"ana@example.com"}'
async fn update_user_with_extractors(
    Path(id): Path<u64>,
    Query(params): Query<ListUsersParams>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let sort = params.sort.unwrap_or_else(|| "id".to_string());

    let user_agent = headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("desconhecido");

    format!(
        "Atualizando usuário {id}\nPágina: {page}\nLimite: {limit}\nOrdenação: {sort}\nUser-Agent: {user_agent}\nNome: {}\nEmail: {}",
        payload.name, payload.email,
    )
}
