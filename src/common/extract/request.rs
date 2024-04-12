use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};
use crate::common::errors::request_error::RequestError;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(RequestError))]
pub struct SwiftJson<T>(pub T);

impl<T> IntoResponse for SwiftJson<T>
    where
        axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

