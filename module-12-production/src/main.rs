use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    ready: Arc<AtomicBool>,
    request_count: Arc<AtomicU64>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            ready: Arc::new(AtomicBool::new(true)),
            request_count: Arc::new(AtomicU64::new(0)),
        }
    }
}

async fn index(State(state): State<AppState>) -> &'static str {
    state.request_count.fetch_add(1, Ordering::SeqCst);

    "Hello from production-ready Axum!"
}

async fn health() -> &'static str {
    "OK"
}

async fn ready(State(state): State<AppState>) -> Result<&'static str, (StatusCode, &'static str)> {
    if state.ready.load(Ordering::SeqCst) {
        Ok("ready")
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, "not ready"))
    }
}

async fn metrics(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "requests": state.request_count.load(Ordering::SeqCst),
        "ready": state.ready.load(Ordering::SeqCst),
    }))
}

fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(metrics))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(true)
                .with_current_span(true),
        )
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let state = AppState::default();

    let app = create_app(state);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    tracing::info!("🚀 Module 12: Production Ready");
    tracing::info!("Server: http://localhost:8000");
    tracing::info!("GET /");
    tracing::info!("GET /health");
    tracing::info!("GET /ready");
    tracing::info!("GET /metrics");

    axum::serve(listener, app).await.unwrap();
}

/*
Teste com:

cargo r12

curl -i -w '\n\n' http://localhost:8000/

curl -i -w '\n\n' http://localhost:8000/health

curl -i -w '\n\n' http://localhost:8000/ready

curl -i -w '\n\n' http://localhost:8000/metrics
*/
