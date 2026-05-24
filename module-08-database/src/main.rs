use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

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

fn app(pool: PgPool) -> Router {
    Router::new()
        // Teste:
        //
        // curl -i -w '\n\n' http://localhost:8000/health/database
        .route("/health/database", get(database_health))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = create_pool().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("failed to bind");

    println!("🚀 Module 08: Database Integration");
    println!("Server running on http://localhost:8000");

    axum::serve(listener, app(pool))
        .await
        .expect("server failed");
}
