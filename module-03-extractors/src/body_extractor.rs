use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidatedUser {
    pub name: String,
    pub email: String,
}

pub struct ValidatedJson<T>(pub T);

#[derive(Debug)]
pub enum ValidationError {
    InvalidJson(String),
    NameTooShort,
    InvalidEmail,
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ValidationError::InvalidJson(error) => {
                (StatusCode::BAD_REQUEST, format!("JSON inválido: {error}"))
            }
            ValidationError::NameTooShort => (
                StatusCode::BAD_REQUEST,
                "O nome precisa ter pelo menos 2 caracteres".to_string(),
            ),
            ValidationError::InvalidEmail => (
                StatusCode::BAD_REQUEST,
                "O email precisa conter @".to_string(),
            ),
        };

        (status, message).into_response()
    }
}

impl<S> FromRequest<S> for ValidatedJson<ValidatedUser>
where
    S: Send + Sync,
{
    type Rejection = ValidationError;

    fn from_request(
        req: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Json(user): Json<ValidatedUser> = Json::from_request(req, state)
                .await
                .map_err(|error| ValidationError::InvalidJson(error.to_string()))?;

            if user.name.len() < 2 {
                return Err(ValidationError::NameTooShort);
            }

            if !user.email.contains('@') {
                return Err(ValidationError::InvalidEmail);
            }

            Ok(ValidatedJson(user))
        }
    }
}
