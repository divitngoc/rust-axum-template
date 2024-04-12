use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AuthenticationError {
    InvalidToken,
    SerializationError,
    ExpiredToken,
}

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidToken => StatusCode::UNAUTHORIZED.into_response(),
            Self::SerializationError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::ExpiredToken => StatusCode::UNAUTHORIZED.into_response()
        }
    }
}