mod post;
mod user;

use axum::{
    Router,
    extract::{Path, Query},
    routing::{delete, get, patch, post, put},
};
use serde::Deserialize;

use crate::{post::post_routes, user::user_routes};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("não foi possível abrir a porta 8000");

    println!("Module 02: Routing");
    println!("Servidor rodando em http://localhost:8000");

    axum::serve(listener, app())
        .await
        .expect("erro ao iniciar o servidor");
}

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/resource", get(read_resources))
        .route("/resource", post(create_resource))
        .route("/resource/{id}", get(read_resource))
        .route("/resource/{id}", put(update_resource))
        .route("/resource/{id}", patch(patch_resource))
        .route("/resource/{id}", delete(delete_resource))
        .route("/users/{user_id}/posts/{post_id}", get(read_user_post))
        .route(
            "/users/{user_id}/posts/{post_id}/comments/{comment_id}",
            get(read_comment),
        )
        .route("/items", get(list_items))
        .route("/search", get(search))
        .route("/files/{*path}", get(read_file))
        .nest("/api/users", user_routes())
        .nest("/api/posts", post_routes())
}

// curl http://localhost:8000
async fn home() -> &'static str {
    "Module 02: Routing"
}

// curl http://localhost:8000/resource
async fn read_resources() -> &'static str {
    "GET - listar recursos"
}

// curl -X POST http://localhost:8000/resource
async fn create_resource() -> &'static str {
    "POST - criar recurso"
}

// curl http://localhost:8000/resource/10
async fn read_resource(Path(id): Path<u64>) -> String {
    format!("GET - buscar recurso {id}")
}

// curl -X PUT http://localhost:8000/resource/10
async fn update_resource(Path(id): Path<u64>) -> String {
    format!("PUT - atualizar recurso completo {id}")
}

// curl -X PATCH http://localhost:8000/resource/10
async fn patch_resource(Path(id): Path<u64>) -> String {
    format!("PATCH - atualizar parcialmente recurso {id}")
}

// curl -X DELETE http://localhost:8000/resource/10
async fn delete_resource(Path(id): Path<u64>) -> String {
    format!("DELETE - remover recurso {id}")
}

// curl http://localhost:8000/users/7/posts/99
async fn read_user_post(Path((user_id, post_id)): Path<(u64, u64)>) -> String {
    format!("Usuário {user_id} - Post {post_id}")
}

#[derive(Deserialize)]
struct CommentPath {
    user_id: u64,
    post_id: u64,
    comment_id: u64,
}

// curl http://localhost:8000/users/7/posts/99/comments/3
async fn read_comment(Path(params): Path<CommentPath>) -> String {
    format!(
        "Usuário {} - Post {} - Comentário {}",
        params.user_id, params.post_id, params.comment_id,
    )
}

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

// curl "http://localhost:8000/items"
// curl "http://localhost:8000/items?page=2"
// curl "http://localhost:8000/items?page=2&limit=20"
async fn list_items(Query(pagination): Query<Pagination>) -> String {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);

    format!("Listando itens - página {page}, limite {limit}")
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    category: Option<String>,
    sort: Option<String>,
}

// curl "http://localhost:8000/search?q=rust"
// curl "http://localhost:8000/search?q=rust&category=backend"
// curl "http://localhost:8000/search?q=rust&category=backend&sort=recent"
async fn search(Query(params): Query<SearchParams>) -> String {
    let category = params.category.unwrap_or_else(|| "all".to_string());
    let sort = params.sort.unwrap_or_else(|| "relevance".to_string());

    format!(
        "Buscando por '{}' na categoria '{}', ordenado por '{}'",
        params.q, category, sort
    )
}

// curl http://localhost:8000/files/readme.md
// curl http://localhost:8000/files/docs/rust/axum.md
async fn read_file(Path(path): Path<String>) -> String {
    format!("Acessando arquivo: {path}")
}
