use axum::{
    Router,
    extract::{Path, Query},
    routing::get,
};
use serde::Deserialize;

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
        .route("/users", get(list_users))
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
