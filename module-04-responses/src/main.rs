use axum::{Router, http::StatusCode, routing::get};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("não foi possível abrir a porta 8000");

    println!("Module 04: Responses");
    println!("Servidor rodando em http://localhost:8000");

    axum::serve(listener, app())
        .await
        .expect("erro ao iniciar o servidor");
}

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/static", get(static_text))
        .route("/owned", get(owned_text))
        .route("/created", get(created))
}

// curl -w '\n\n' 'http://localhost:8000'
async fn home() -> &'static str {
    "Module 04: Responses"
}

// curl -w '\n\n' 'http://localhost:8000/static'
async fn static_text() -> &'static str {
    "Resposta com &'static str"
}

// curl -w '\n\n' 'http://localhost:8000/owned'
async fn owned_text() -> String {
    let timestamp = current_timestamp();

    format!("Resposta com String criada em runtime. Timestamp: {timestamp}")
}

// curl -i 'http://localhost:8000/created'
//
// echo
async fn created() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Recurso criado com sucesso")
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("erro ao calcular timestamp")
        .as_secs()
}
