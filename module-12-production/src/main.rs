use axum::{
    Json, Router,
    extract::State,
    http::{Method, StatusCode, Uri},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Instant,
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

async fn request_logger(request: axum::extract::Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let started_at = Instant::now();

    tracing::info!("→ {} {}", method, uri);

    let response = next.run(request).await;

    let status = response.status();
    let elapsed = started_at.elapsed();

    let status_icon = if status.is_success() {
        "✅"
    } else if status.is_client_error() {
        "⚠️"
    } else if status.is_server_error() {
        "💥"
    } else {
        "ℹ️"
    };

    tracing::info!(
        "{} {} {} {}ms",
        status_icon,
        status.as_u16(),
        format_request(&method, &uri),
        elapsed.as_millis()
    );

    response
}

fn format_request(method: &Method, uri: &Uri) -> String {
    format!("{method} {uri}")
}

fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(metrics))
        .with_state(state)
        .layer(middleware::from_fn(request_logger))
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(false)
                .with_line_number(false),
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
