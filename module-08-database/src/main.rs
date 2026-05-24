use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, thiserror::Error)]
enum DbError {
    #[error("User not found")]
    NotFound,

    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let status = match self {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = ErrorResponse {
            error: self.to_string(),
            code: status.as_u16(),
        };

        (status, Json(body)).into_response()
    }
}

#[derive(Serialize)]
struct HealthResponse {
    database: &'static str,
}

async fn database_health(State(pool): State<PgPool>) -> Json<HealthResponse> {
    let _ = sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("database health check failed");

    Json(HealthResponse { database: "ok" })
}

async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, DbError> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, created_at
        FROM users
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(users))
}

async fn get_user(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<Json<User>, DbError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(DbError::NotFound)?;

    Ok(Json(user))
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), DbError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, name, email)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&input.name)
    .bind(&input.email)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateUser>,
) -> Result<Json<User>, DbError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET
            name = COALESCE($2, name),
            email = COALESCE($3, email)
        WHERE id = $1
        RETURNING id, name, email, created_at
        "#,
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.email)
    .fetch_optional(&pool)
    .await?
    .ok_or(DbError::NotFound)?;

    Ok(Json(user))
}

async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, DbError> {
    let result = sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        Err(DbError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn create_pool() -> PgPool {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/axum_course".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database")
}

async fn run_migrations(pool: &PgPool) {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("failed to create users table");
}

fn app(pool: PgPool) -> Router {
    Router::new()
        // Teste:
        //
        // curl -i -w '\n\n' http://localhost:8000/health/database
        .route("/health/database", get(database_health))
        // Testes:
        //
        // curl -i -w '\n\n' http://localhost:8000/users
        //
        // curl -i -w '\n\n' -X POST http://localhost:8000/users \
        //   -H 'Content-Type: application/json' \
        //   -d '{"name":"Alice","email":"alice@example.com"}'
        .route("/users", get(list_users).post(create_user))
        // Testes:
        //
        // curl -i -w '\n\n' http://localhost:8000/users/COLE_O_ID_AQUI
        //
        // curl -i -w '\n\n' -X PUT http://localhost:8000/users/COLE_O_ID_AQUI \
        //   -H 'Content-Type: application/json' \
        //   -d '{"name":"Carla Atualizada"}'
        //
        // curl -i -w '\n\n' -X PUT http://localhost:8000/users/COLE_O_ID_AQUI \
        //   -H 'Content-Type: application/json' \
        //   -d '{"email":"carla.updated@example.com"}'
        //
        // UUID inexistente:
        //
        // curl -i -w '\n\n' http://localhost:8000/users/00000000-0000-0000-0000-000000000000
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = create_pool().await;
    run_migrations(&pool).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("failed to bind");

    println!("🚀 Module 08: Database Integration");
    println!("Server running on http://localhost:8000");

    axum::serve(listener, app(pool))
        .await
        .expect("server failed");
}
