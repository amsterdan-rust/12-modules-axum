use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AuthConfig {
    jwt_secret: String,
    jwt_expiry_hours: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    expires_in: i64,
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut rand::rngs::OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn create_token(config: &AuthConfig, user_id: &str, role: &str) -> Result<String, StatusCode> {
    let expiry = Utc::now() + Duration::hours(config.jwt_expiry_hours);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiry.timestamp() as usize,
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn register(Json(input): Json<RegisterRequest>) -> impl IntoResponse {
    let hashed_password = hash_password(&input.password);

    Json(serde_json::json!({
        "message": "User registered",
        "name": input.name,
        "email": input.email,
        "password_hash": hashed_password
    }))
}

async fn login(
    State(config): State<Arc<AuthConfig>>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let fake_user_email = "test@example.com";
    let fake_user_id = "user-1";
    let fake_user_role = "user";

    let fake_password_hash = hash_password("password123");

    let email_is_valid = input.email == fake_user_email;
    let password_is_valid = verify_password(&input.password, &fake_password_hash);

    if email_is_valid && password_is_valid {
        let token = create_token(&config, fake_user_id, fake_user_role)?;

        Ok(Json(LoginResponse {
            token,
            expires_in: config.jwt_expiry_hours * 3600,
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[tokio::main]
async fn main() {
    let config = Arc::new(AuthConfig {
        jwt_secret: "super-secret-key-change-in-production".to_string(),
        jwt_expiry_hours: 24,
    });

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("🚀 Module 09: Authentication");
    println!("Server: http://localhost:8000");
    println!("POST /register");
    println!("POST /login");

    axum::serve(listener, app).await.unwrap();
}

/*
Teste com:

cargo r09

curl -w '\n\n' -X POST http://localhost:8000/register \
  -H 'Content-Type: application/json' \
  -d '{"name":"Amsterdan","email":"test@example.com","password":"password123"}'

curl -w '\n\n' -X POST http://localhost:8000/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"test@example.com","password":"password123"}'
*/
