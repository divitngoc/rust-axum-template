use axum::extract::{MatchedPath, Request};
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;
use tower_http::trace::{HttpMakeClassifier, TraceLayer};
use tracing::Span;
use crate::common::utils::constants::TRACING_ID_HEADER;
use crate::common::utils::header_utils;
use crate::middleware::services::ForceSetRequestIdLayer;

pub fn set_span() -> TraceLayer<HttpMakeClassifier, fn(&Request) -> Span> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            let tracing_id = header_utils::extract_tracing_id_from_request(request);

            tracing::info_span!(
                        "http_request",
                        tracing_id = tracing_id,
                        method = ?request.method(),
                        path
                    )
        })
}

pub fn set_tracing_id() -> ServiceBuilder<Stack<ForceSetRequestIdLayer, Identity>> {
    ServiceBuilder::new()
        .layer(ForceSetRequestIdLayer::new(TRACING_ID_HEADER.clone()))
}