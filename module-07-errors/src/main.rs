use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("User not found: {0}")]
    UserNotFound(u64),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::UserNotFound(_) => StatusCode::NOT_FOUND,
        };

        let body = ErrorResponse {
            error: self.to_string(),
            code: status.as_u16(),
        };

        (status, Json(body)).into_response()
    }
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

async fn get_user(Path(id): Path<u64>) -> Result<Json<User>, AppError> {
    match id {
        1 => Ok(Json(User {
            id: 1,
            name: "Alice".to_string(),
        })),
        2 => Ok(Json(User {
            id: 2,
            name: "Bob".to_string(),
        })),
        _ => Err(AppError::UserNotFound(id)),
    }
}

fn app() -> Router {
    Router::new()
        // Testes:
        //
        // curl -i -w '\n\n' http://localhost:8000/users/1
        // curl -i -w '\n\n' http://localhost:8000/users/2
        // curl -i -w '\n\n' http://localhost:8000/users/999
        .route("/users/{id}", get(get_user))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("failed to bind");

    println!("🚀 Module 07: Error Handling");
    println!("Server running on http://localhost:8000");

    axum::serve(listener, app()).await.expect("server failed");
}
