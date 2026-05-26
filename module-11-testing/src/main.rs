use axum::{Router, response::IntoResponse, routing::get};

async fn health() -> impl IntoResponse {
    "OK"
}

fn create_app() -> Router {
    Router::new().route("/health", get(health))
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("🚀 Module 11: Testing");
    println!("Server: http://localhost:8000");
    println!("GET /health");

    axum::serve(listener, app).await.unwrap();
}

/*
Teste manual com:

cargo r11

curl -i -w '\n\n' http://localhost:8000/health
*/
