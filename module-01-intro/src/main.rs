use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/created", get(created))
        .route("/status", get(status))
        .route("/echo", post(echo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("não foi possível abrir a porta 8000");

    println!("Servidor rodando em http://localhost:8000");

    axum::serve(listener, app)
        .await
        .expect("erro ao iniciar o servidor");
}

async fn home() -> &'static str {
    "Hello, Axum!"
}

async fn health() -> &'static str {
    "OK"
}

async fn echo(body: String) -> String {
    format!("Você enviou: {body}")
}

async fn created() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Recurso criado")
}

async fn status() -> (StatusCode, &'static str) {
    let service_is_ok = true;

    if service_is_ok {
        (StatusCode::OK, "Serviço funcionando")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "Serviço indisponível")
    }
}
