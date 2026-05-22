use axum::{Router, extract::Path, routing::get};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route(
            "/{id}",
            get(read_user)
                .put(update_user)
                .patch(patch_user)
                .delete(delete_user),
        )
}

// curl http://localhost:8000/api/users
async fn list_users() -> &'static str {
    "Listando usuários"
}

// curl -X POST http://localhost:8000/api/users
async fn create_user() -> &'static str {
    "Criando usuário"
}

// curl http://localhost:8000/api/users/10
async fn read_user(Path(id): Path<u64>) -> String {
    format!("Buscando usuário {id}")
}

// curl -X PUT http://localhost:8000/api/users/10
async fn update_user(Path(id): Path<u64>) -> String {
    format!("Atualizando usuário completo {id}")
}

// curl -X PATCH http://localhost:8000/api/users/10
async fn patch_user(Path(id): Path<u64>) -> String {
    format!("Atualizando usuário parcialmente {id}")
}

// curl -X DELETE http://localhost:8000/api/users/10
async fn delete_user(Path(id): Path<u64>) -> String {
    format!("Removendo usuário {id}")
}
