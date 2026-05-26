use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

type UserStore = Arc<RwLock<HashMap<u64, User>>>;

async fn health() -> impl IntoResponse {
    "OK"
}

async fn list_users(State(store): State<UserStore>) -> Json<Vec<User>> {
    let users = store.read().unwrap();

    Json(users.values().cloned().collect())
}

async fn create_user(
    State(store): State<UserStore>,
    Json(input): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut users = store.write().unwrap();

    let id = users.len() as u64 + 1;

    let user = User {
        id,
        name: input.name,
    };

    users.insert(id, user.clone());

    (StatusCode::CREATED, Json(user))
}

fn create_app(store: UserStore) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .with_state(store)
}

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(HashMap::new()));
    let app = create_app(store);

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

    fn test_store() -> UserStore {
        Arc::new(RwLock::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = create_app(test_store());

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

    #[tokio::test]
    async fn test_list_users_empty() {
        let app = create_app(test_store());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();

        let users: Vec<User> = serde_json::from_slice(&body).unwrap();

        assert_eq!(users, Vec::<User>::new());
    }
}
