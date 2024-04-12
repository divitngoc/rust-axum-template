use axum::body::Body;
use axum::response::{IntoResponse, Response};
use http::{StatusCode};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub support_contact: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorMessage>>
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    pub message: String
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        let json_body = serde_json::to_string(&self)
            .unwrap_or_else(|_| "{\"errors\":\"Internal server errors\"}".to_string());

        Response::builder()
            .status(self.status_code)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(json_body))
            .expect("Unable to build response from ErrorResponse")
    }
}

// The builder for `ErrorResponse`
pub struct ErrorResponseBuilder {
    status_code: StatusCode,
    support_contact: String,
    errors: Option<Vec<ErrorMessage>>,
}

impl ErrorResponse {

    pub fn build(status_code: StatusCode, message: String) -> Self {
        return ErrorResponseBuilder::default()
            .status_code(status_code)
            .error(ErrorMessage { field: None, message })
            .build()
    }

    pub fn builder() -> ErrorResponseBuilder {
        ErrorResponseBuilder::default()
    }
}

impl ErrorResponseBuilder {
    // Initialize the builder with default values
    pub fn default() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            support_contact: "support@swiftapi.com".to_string(),
            errors: None,
        }
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn error(mut self, error: ErrorMessage) -> Self {
        self.errors = Some(vec![error]);
        self
    }

    pub fn errors(mut self, errors: Vec<ErrorMessage>) -> Self {
        self.errors = Some(errors);
        self
    }

    pub fn build(self) -> ErrorResponse {
        ErrorResponse {
            status_code: self.status_code,
            support_contact: self.support_contact,
            errors: self.errors,
        }
    }
}