use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("não foi possível abrir a porta 8000");

    println!("Servidor rodando em http://localhost:8000");

    axum::serve(listener, app())
        .await
        .expect("erro ao iniciar o servidor");
}

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/created", get(created))
        .route("/status", get(status))
        .route("/echo", post(echo))
}

// curl http://localhost:8000
async fn home() -> &'static str {
    "Hello, Axum!"
}

// curl http://localhost:8000/health
async fn health() -> &'static str {
    "OK"
}

// curl http://localhost:8000/version
async fn version() -> String {
    format!("Versão da aplicação: {}", env!("CARGO_PKG_VERSION"))
}

// curl -i http://localhost:8000/created
async fn created() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Recurso criado")
}

// curl -i http://localhost:8000/status
async fn status() -> (StatusCode, &'static str) {
    let service_is_ok = true;

    if service_is_ok {
        (StatusCode::OK, "Serviço funcionando")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "Serviço indisponível")
    }
}

// curl -X POST -d "Aprendendo Axum" http://localhost:8000/echo
async fn echo(body: String) -> String {
    format!("Você enviou: {body}")
}
