use std::fmt;
use axum::extract::{rejection::JsonRejection};
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use crate::common::errors::api_error_response::{ErrorMessage, ErrorResponse};

#[derive(Debug)]
pub enum RequestError {
    ValidationError(Vec<ErrorMessage>),
    JsonRejection(JsonRejection),
    HeaderNotFound(String),
    InvalidUUIDHeaderFormat(String),
}


impl From<JsonRejection> for RequestError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        match self {
            RequestError::ValidationError(messages) =>
                ErrorResponse::builder()
                    .status_code(StatusCode::BAD_REQUEST)
                    .errors(messages)
                    .build()
                    .into_response(),
            RequestError::JsonRejection(e) => ErrorResponse::build(e.status(), e.body_text()).into_response(),
            RequestError::HeaderNotFound(header_name) => ErrorResponse::build(StatusCode::BAD_REQUEST, format!("Required header {header_name} is missing")).into_response(),
            RequestError::InvalidUUIDHeaderFormat(header_name) => ErrorResponse::build(StatusCode::BAD_REQUEST, format!("Header {header_name} must be in UUID format")).into_response(),
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::ValidationError(messages) => write!(f, "Validation Error: {:?}", messages),
            RequestError::JsonRejection(_) => write!(f, "JSON Parsing Error"),
            RequestError::HeaderNotFound(message) => write!(f, "Required header not found: {:?}", message),
            RequestError::InvalidUUIDHeaderFormat(header_name) => write!(f, "Invalid UUID header format: {}", header_name),
        }
    }
}