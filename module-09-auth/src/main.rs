use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use axum::{
    Json, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
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

#[derive(Debug, Clone)]
struct CurrentUser {
    id: String,
    role: String,
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

fn verify_token(config: &AuthConfig, token: &str) -> Result<Claims, StatusCode> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| StatusCode::UNAUTHORIZED)
}

async fn auth_middleware(
    State(config): State<Arc<AuthConfig>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(&config, token)?;

    let current_user = CurrentUser {
        id: claims.sub,
        role: claims.role,
    };

    request.extensions_mut().insert(current_user);

    Ok(next.run(request).await)
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
    let fake_password_hash = hash_password("password123");

    let fake_user = match input.email.as_str() {
        "test@example.com" => Some(("user-1", "user")),
        "admin@example.com" => Some(("admin-1", "admin")),
        _ => None,
    };

    let Some((user_id, role)) = fake_user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let password_is_valid = verify_password(&input.password, &fake_password_hash);

    if password_is_valid {
        let token = create_token(&config, user_id, role)?;

        Ok(Json(LoginResponse {
            token,
            expires_in: config.jwt_expiry_hours * 3600,
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn me(axum::Extension(user): axum::Extension<CurrentUser>) -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "Access granted",
        "user_id": user.id,
        "role": user.role
    }))
}

async fn admin(axum::Extension(user): axum::Extension<CurrentUser>) -> impl IntoResponse {
    if user.role != "admin" {
        return (StatusCode::FORBIDDEN, "Admin access required").into_response();
    }

    Json(serde_json::json!({
        "message": "Admin area",
        "user_id": user.id,
        "role": user.role
    }))
    .into_response()
}

#[tokio::main]
async fn main() {
    let config = Arc::new(AuthConfig {
        jwt_secret: "super-secret-key-change-in-production".to_string(),
        jwt_expiry_hours: 24,
    });

    let protected_routes = Router::new()
        .route("/me", get(me))
        .route("/admin", get(admin))
        .route_layer(middleware::from_fn_with_state(
            config.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .nest("/protected", protected_routes)
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
