use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use std::time::Instant;
use tracing::Level;

async fn logging_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    let response = next.run(request).await;

    tracing::info!(
        method = %method,
        uri = %uri,
        status = %response.status().as_u16(),
        duration_ms = %start.elapsed().as_millis(),
        "request completed"
    );

    response
}

async fn index() -> &'static str {
    "Welcome to Axum Middleware Module!"
}

async fn public_data() -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "message": "Public data",
        "accessible": true,
    }))
}

fn app() -> Router {
    Router::new()
        // Testes:
        //
        // curl -w '\n\n' http://localhost:8000/
        // curl -w '\n\n' http://localhost:8000/public
        .route("/", get(index))
        .route("/public", get(public_data))
        .layer(middleware::from_fn(logging_middleware))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("failed to bind");

    println!("🚀 Module 06: Middleware & Layers");
    println!("Server running on http://localhost:8000");

    axum::serve(listener, app()).await.expect("server failed");
}
