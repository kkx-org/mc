use actix_web::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Auth failed: invalid credentials")]
    InvalidCredentials,
    #[error("Auth failed: email already registered")]
    EmailAlreadyRegistered,
    #[error("Auth failed: validation failed, field: {0}")]
    ValidationFailed(String),
}

impl From<AuthError> for crate::error::Error {
    fn from(err: AuthError) -> Self {
        crate::error::Error::Api(match err {
            AuthError::InvalidCredentials => crate::error::ApiError {
                kind: String::from("invalid_credentials"),
                status_code: StatusCode::UNAUTHORIZED,
                message: String::from("invalid email or password"),
            },
            AuthError::EmailAlreadyRegistered => crate::error::ApiError {
                kind: String::from("email_already_registered"),
                status_code: StatusCode::CONFLICT,
                message: String::from("this email is already registered"),
            },
            AuthError::ValidationFailed(_) => crate::error::ApiError {
                kind: String::from("validation_failed"),
                status_code: StatusCode::BAD_REQUEST,
                message: err.to_string(),
            },
        })
    }
}

