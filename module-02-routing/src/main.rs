use axum::{
    Router,
    extract::Path,
    routing::{delete, get, patch, post, put},
};

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
