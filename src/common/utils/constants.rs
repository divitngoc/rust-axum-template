use http::HeaderName;

pub static TRACING_ID_HEADER: HeaderName = HeaderName::from_static("x-tracing-id");
pub static ORGANISATION_ID_HEADER: HeaderName = HeaderName::from_static("x-organisation-id");