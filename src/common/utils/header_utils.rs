use axum::extract::Request;
use http::{HeaderMap, HeaderName};
use uuid::Uuid;
use crate::common::errors::request_error::RequestError;
use crate::common::utils::constants::TRACING_ID_HEADER;

pub fn extract_from_request(req: &Request, header_name: &HeaderName) -> Option<String> {
    extract_from_header(req.headers(), header_name)
}

pub fn extract_tracing_id_from_request(req: &Request) -> String {
    extract_from_request(&req, &TRACING_ID_HEADER).expect("Could not find tracing id in request")
}

pub fn extract_from_header(headers: &HeaderMap, header_name: &HeaderName) -> Option<String> {
    headers.get(header_name.clone())
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

pub fn extract_from_required_header(headers: &HeaderMap, header_name: &HeaderName) -> Result<String, RequestError> {
    extract_from_header(headers, header_name)
        .ok_or_else(|| RequestError::HeaderNotFound(header_name.as_str().to_string()))
}

pub fn extract_uuid_from_required_header(headers: &HeaderMap, header_name: &HeaderName) -> Result<Uuid, RequestError> {
    extract_from_required_header(headers, header_name)
        .and_then(|value|
            Uuid::parse_str(&value).map_err(|_| RequestError::InvalidUUIDHeaderFormat(header_name.as_str().to_string()))
        )
}