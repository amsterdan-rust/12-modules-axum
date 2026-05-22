use axum::{Router, extract::Path, routing::get};

pub fn post_routes() -> Router {
    Router::new()
        .route("/", get(list_posts).post(create_post))
        .route("/{id}", get(read_post).delete(delete_post))
}

// curl http://localhost:8000/api/posts
async fn list_posts() -> &'static str {
    "Listando posts"
}

// curl -X POST http://localhost:8000/api/posts
async fn create_post() -> &'static str {
    "Criando post"
}

// curl http://localhost:8000/api/posts/10
async fn read_post(Path(id): Path<u64>) -> String {
    format!("Buscando post {id}")
}

// curl -X DELETE http://localhost:8000/api/posts/10
async fn delete_post(Path(id): Path<u64>) -> String {
    format!("Removendo post {id}")
}
