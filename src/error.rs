use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Api error: {0}")]
    Api(#[from] ApiError),
    #[error("Sqlx error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Argon2 error: {0}")]
    Argon2(argon2::Error),
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Error::Argon2(err)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ApiError {
    pub status_code: StatusCode,
    pub kind: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Error::Api(err) => err.status_code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(match &self {
            Error::Api(err) => ApiErrorResponse {
                error: err.kind.clone(),
                message: err.to_string(),
            },
            _ => ApiErrorResponse {
                error: String::from("internal_server_error"),
                message: String::from("An unexpected error occured"),
            },
        })
    }
}
