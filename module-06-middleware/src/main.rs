use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, Method, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use std::time::{Duration, Instant};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
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

async fn timing_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();

    let mut response = next.run(request).await;

    let elapsed = start.elapsed().as_millis();

    response.headers_mut().insert(
        "X-Response-Time",
        HeaderValue::from_str(&format!("{elapsed}ms")).unwrap(),
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

fn protected_routes() -> Router {
    Router::new()
        // Testes:
        //
        // Sem API key:
        //
        // curl -i -w '\n\n' http://localhost:8000/protected/data
        //
        // Com API key:
        //
        // curl -i -w '\n\n' http://localhost:8000/protected/data \
        //   -H 'X-API-Key: secret-key'
        .route("/data", get(protected_data))
        .route_layer(middleware::from_fn(auth_middleware))
}

fn public_routes() -> Router {
    Router::new()
        // Testes:
        //
        // curl -w '\n\n' http://localhost:8000/
        // curl -w '\n\n' http://localhost:8000/public
        // curl -i -w '\n\n' http://localhost:8000/slow
        .route("/", get(index))
        .route("/public", get(public_data))
        .route("/slow", get(slow_endpoint))
}

fn app() -> Router {
    Router::new()
        .merge(public_routes())
        .nest("/protected", protected_routes())
        .layer(middleware::from_fn(timing_middleware))
        .layer(middleware::from_fn(logging_middleware))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors_layer())
                .layer(CompressionLayer::new()),
        )
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

async fn auth_middleware(request: Request, next: Next) -> Result<Response, StatusCode> {
    let api_key = request
        .headers()
        .get("X-API-Key")
        .and_then(|value| value.to_str().ok());

    match api_key {
        Some("secret-key") => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn protected_data() -> impl IntoResponse {
    axum::Json(serde_json::json!({
        "message": "Secret data",
        "authorized": true,
    }))
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
}

async fn slow_endpoint() -> &'static str {
    tokio::time::sleep(Duration::from_secs(1)).await;

    "Slow operation done!"
}
