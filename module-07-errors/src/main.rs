use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("User not found: {0}")]
    UserNotFound(u64),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal server error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::UserNotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
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

async fn validate_input(Path(value): Path<String>) -> Result<String, AppError> {
    if value.len() < 3 {
        return Err(AppError::InvalidInput(
            "Value must be at least 3 characters".to_string(),
        ));
    }

    Ok(format!("Valid input: {value}"))
}

async fn protected_resource() -> Result<&'static str, AppError> {
    let is_authenticated = false;

    if !is_authenticated {
        return Err(AppError::Unauthorized);
    }

    Ok("Secret data!")
}

async fn database_operation() -> Result<&'static str, AppError> {
    Err(AppError::DatabaseError("Connection timeout".to_string()))
}

async fn complex_operation(Path(id): Path<u64>) -> Result<Json<User>, AppError> {
    let user = find_user(id)?;

    validate_user(&user)?;

    Ok(Json(user))
}

fn find_user(id: u64) -> Result<User, AppError> {
    if id == 0 {
        Err(AppError::InvalidInput("ID cannot be zero".to_string()))
    } else if id > 100 {
        Err(AppError::UserNotFound(id))
    } else {
        Ok(User {
            id,
            name: format!("User{id}"),
        })
    }
}

fn validate_user(user: &User) -> Result<(), AppError> {
    if user.name.is_empty() {
        Err(AppError::InvalidInput("Name cannot be empty".to_string()))
    } else {
        Ok(())
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
        // Testes:
        //
        // curl -i -w '\n\n' http://localhost:8000/validate/abc
        // curl -i -w '\n\n' http://localhost:8000/validate/ab
        .route("/validate/{value}", get(validate_input))
        // Teste:
        //
        // curl -i -w '\n\n' http://localhost:8000/protected
        .route("/protected", get(protected_resource))
        // Teste:
        //
        // curl -i -w '\n\n' http://localhost:8000/database
        .route("/database", get(database_operation))
        // Testes:
        //
        // curl -i -w '\n\n' http://localhost:8000/complex/1
        // curl -i -w '\n\n' http://localhost:8000/complex/0
        // curl -i -w '\n\n' http://localhost:8000/complex/999
        .route("/complex/{id}", get(complex_operation))
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
