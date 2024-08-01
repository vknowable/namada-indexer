use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::response::api::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum BlocksError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Invalid range: {0}")]
    InvalidRange(String),
}

impl IntoResponse for BlocksError {
    fn into_response(self) -> Response {
        let status_code = match self {
            BlocksError::Unknown(_) | BlocksError::Database(_) | BlocksError::InvalidRange(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
