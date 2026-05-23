use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};

pub struct ApiKey(pub String);

#[derive(Debug)]
pub struct ApiKeyError;

impl IntoResponse for ApiKeyError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            "API key ausente ou inválida. Envie o header X-API-Key.",
        )
            .into_response()
    }
}

impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = ApiKeyError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let api_key = parts
            .headers
            .get("x-api-key")
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());

        async move {
            match api_key {
                Some(key) if !key.is_empty() => Ok(ApiKey(key)),
                _ => Err(ApiKeyError),
            }
        }
    }
}
