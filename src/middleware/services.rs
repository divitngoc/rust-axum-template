use std::task::{Context, Poll};
use axum::body::Body;
use axum::extract::Request;
use axum::response::Response;
use http::{HeaderName, HeaderValue};
use tower::Service;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct ForceSetRequestIdLayer {
    header_name: HeaderName,
}

impl ForceSetRequestIdLayer {
    pub fn new(header_name: HeaderName) -> Self {
        Self { header_name }
    }
}

impl<S> tower::Layer<S> for ForceSetRequestIdLayer {
    type Service = ForceSetRequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ForceSetRequestIdService {
            inner,
            header_name: self.header_name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForceSetRequestIdService<S> {
    inner: S,
    header_name: HeaderName,
}

impl<S, ReqBody> Service<Request<ReqBody>> for ForceSetRequestIdService<S>
    where
        S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
        S::Future: Send,
        ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
        let tracing_id = Uuid::new_v4().to_string();
        request.headers_mut().insert(
            self.header_name.clone(),
            HeaderValue::from_str(&tracing_id).unwrap(),
        );
        self.inner.call(request)
    }
}