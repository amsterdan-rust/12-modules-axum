use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
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

async fn list_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, created_at
        FROM users
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
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
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
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
