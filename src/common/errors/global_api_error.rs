use axum::response::{IntoResponse, Response};
use crate::common::errors::application_error::ApplicationError;
use crate::common::errors::authentication_error::AuthenticationError;
use crate::common::errors::request_error::RequestError;

#[derive(Debug)]
pub enum ApiError {
    ApplicationError(ApplicationError),
    RequestError(RequestError),
    AuthenticationError(AuthenticationError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ApplicationError(e) => e.into_response(),
            ApiError::RequestError(e) => e.into_response(),
            ApiError::AuthenticationError(e) => e.into_response()
        }
    }
}

impl From<RequestError> for ApiError {
    fn from(error: RequestError) -> Self {
        ApiError::RequestError(error)
    }
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        ApiError::ApplicationError(error)
    }
}

impl From<AuthenticationError> for ApiError {
    fn from(error: AuthenticationError) -> Self {
        ApiError::AuthenticationError(error)
    }
}