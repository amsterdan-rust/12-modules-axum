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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(&body[..], b"OK");
    }
}
