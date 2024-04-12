use axum::response::IntoResponse;
use http::StatusCode;

pub async fn handle() -> impl IntoResponse {
    StatusCode::NO_CONTENT.into_response()
}