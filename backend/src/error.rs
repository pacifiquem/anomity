use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;
use serde_with::DisplayFromStr;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an internal database error occured")]
    Sqlx(#[from] sqlx::Error),

    #[error("an internal server error occured")]
    Anyhow(#[from] anyhow::Error),

    #[error("validation error")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("{0}")]
    UnprocessableEntity(String),

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    TokenCreation(String),

    #[error("{0}")]
    InvalidToken(String),

    #[error("{0}")]
    Validation(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(Serialize)]
        struct ErrorResponse<'a> {
            #[serde_as(as = "DisplayFromStr")]
            message: &'a Error,

            errors: Option<&'a ValidationErrors>,
        }

        let errors = match &self {
            Error::InvalidEntity(errors) => Some(errors),
            _ => None,
        };

        (
            self.status_code(),
            Json(ErrorResponse {
                message: &self,
                errors,
            }),
        )
            .into_response()
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Sqlx(_) | Anyhow(_) | TokenCreation(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Conflict(_) => StatusCode::CONFLICT,
            NotFound(_) => StatusCode::NOT_FOUND,
            Unauthorized(_) | InvalidToken(_) => StatusCode::UNAUTHORIZED,
            Validation(_) => StatusCode::BAD_REQUEST,
        }
    }
}
