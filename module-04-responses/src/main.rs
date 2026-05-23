use axum::{
    Json, Router,
    body::Body,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
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
        .route("/html", get(html_page))
        .route("/html/dynamic", get(dynamic_html))
        .route("/headers", get(with_headers))
        .route("/full", get(full_response))
        .route("/redirect/permanent", get(redirect_permanent))
        .route("/redirect/temporary", get(redirect_temporary))
        .route("/redirect/after-post", get(redirect_after_post))
        .route("/new-location", get(new_location))
        .route("/temporary-location", get(temporary_location))
        .route("/success", get(success))
        .route("/custom", get(custom_response))
        .route("/api/success", get(api_success))
        .route("/api/error", get(api_error))
        .route("/maybe-user", get(maybe_user))
        .route("/maybe-error", get(maybe_error))
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

// curl -w '\n\n' 'http://localhost:8000/html'
async fn html_page() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="pt-BR">
        <head>
            <meta charset="UTF-8">
            <title>Axum HTML</title>
        </head>
        <body>
            <h1>Resposta HTML com Axum</h1>
            <p>Essa página veio diretamente de um handler.</p>
        </body>
        </html>
        "#,
    )
}

// curl -w '\n\n' 'http://localhost:8000/html/dynamic'
async fn dynamic_html() -> Html<String> {
    let topics = ["Routing", "Extractors", "Responses"];

    let items: String = topics
        .iter()
        .map(|topic| format!("<li>{topic}</li>"))
        .collect();

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html lang="pt-BR">
        <head>
            <meta charset="UTF-8">
            <title>Módulos Axum</title>
        </head>
        <body>
            <h1>Módulos estudados</h1>
            <ul>{items}</ul>
        </body>
        </html>
        "#
    ))
}

// curl -i 'http://localhost:8000/headers'
//
// echo
async fn with_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );

    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("max-age=3600"),
    );

    headers.insert("x-module", HeaderValue::from_static("module-04-responses"));

    (headers, "Resposta com headers customizados")
}

// curl -i 'http://localhost:8000/full'
//
// echo
async fn full_response() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );

    headers.insert("x-request-id", HeaderValue::from_static("req-123"));

    (
        StatusCode::OK,
        headers,
        "Resposta com status, headers e body",
    )
}

// curl -i 'http://localhost:8000/redirect/permanent'
//
// echo
async fn redirect_permanent() -> Redirect {
    Redirect::permanent("/new-location")
}

// curl -i 'http://localhost:8000/redirect/temporary'
//
// echo
async fn redirect_temporary() -> Redirect {
    Redirect::temporary("/temporary-location")
}

// curl -i 'http://localhost:8000/redirect/after-post'
//
// echo
async fn redirect_after_post() -> Redirect {
    Redirect::to("/success")
}

// curl -w '\n\n' 'http://localhost:8000/new-location'
async fn new_location() -> &'static str {
    "Você chegou na nova localização permanente"
}

// curl -w '\n\n' 'http://localhost:8000/temporary-location'
async fn temporary_location() -> &'static str {
    "Você chegou na localização temporária"
}

// curl -w '\n\n' 'http://localhost:8000/success'
async fn success() -> &'static str {
    "Operação concluída com sucesso"
}

struct CustomResponse {
    message: String,
    status: StatusCode,
}

impl IntoResponse for CustomResponse {
    fn into_response(self) -> Response {
        let body = format!(
            r#"{{"message":"{}","status":{}}}"#,
            self.message,
            self.status.as_u16(),
        );

        Response::builder()
            .status(self.status)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .expect("erro ao construir resposta")
    }
}

// curl -i 'http://localhost:8000/custom'
//
// echo
async fn custom_response() -> CustomResponse {
    CustomResponse {
        message: "Resposta criada com IntoResponse".to_string(),
        status: StatusCode::OK,
    }
}

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        (status, Json(self)).into_response()
    }
}

// curl -i 'http://localhost:8000/api/success'
//
// echo
async fn api_success() -> ApiResponse<User> {
    ApiResponse {
        success: true,
        data: Some(User {
            id: 1,
            name: "Ana".to_string(),
            email: "ana@example.com".to_string(),
            active: true,
        }),
        error: None,
    }
}

// curl -i 'http://localhost:8000/api/error'
//
// echo
async fn api_error() -> ApiResponse<()> {
    ApiResponse {
        success: false,
        data: None,
        error: Some("Algo deu errado".to_string()),
    }
}

// curl -i 'http://localhost:8000/maybe-user'
//
// echo
async fn maybe_user() -> Result<Json<User>, (StatusCode, String)> {
    Ok(Json(User {
        id: 1,
        name: "Ana".to_string(),
        email: "ana@example.com".to_string(),
        active: true,
    }))
}

// curl -i 'http://localhost:8000/maybe-error'
//
// echo
async fn maybe_error() -> Result<Json<User>, (StatusCode, String)> {
    Err((StatusCode::NOT_FOUND, "Usuário não encontrado".to_string()))
}
