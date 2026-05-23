use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;

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
        .route("/json/user", get(json_user))
        .route("/json/created", get(json_created_user))
        .route("/json/users", get(json_users))
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

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

// curl -w '\n\n' 'http://localhost:8000/json/user'
async fn json_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "Ana".to_string(),
        email: "ana@example.com".to_string(),
        active: true,
    })
}

// curl -i 'http://localhost:8000/json/created'
//
// echo
async fn json_created_user() -> (StatusCode, Json<User>) {
    (
        StatusCode::CREATED,
        Json(User {
            id: 2,
            name: "Bruno".to_string(),
            email: "bruno@example.com".to_string(),
            active: true,
        }),
    )
}

#[derive(Serialize)]
struct UsersResponse {
    users: Vec<User>,
    total: usize,
    page: u32,
}

// curl -w '\n\n' 'http://localhost:8000/json/users'
async fn json_users() -> Json<UsersResponse> {
    let users = vec![
        User {
            id: 1,
            name: "Ana".to_string(),
            email: "ana@example.com".to_string(),
            active: true,
        },
        User {
            id: 2,
            name: "Bruno".to_string(),
            email: "bruno@example.com".to_string(),
            active: false,
        },
    ];

    Json(UsersResponse {
        total: users.len(),
        users,
        page: 1,
    })
}
