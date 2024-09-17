use axum::http::Request;
use tower_http::trace::{DefaultMakeSpan, HttpMakeClassifier, OnRequest, TraceLayer};
use tracing::Span;

pub fn logging_layer() -> TraceLayer<HttpMakeClassifier, DefaultMakeSpan, RequestLogger> {
	TraceLayer::new_for_http()
		.make_span_with(DefaultMakeSpan::new().include_headers(true))
		.on_request(RequestLogger)
}

#[derive(Debug, Clone)]
pub struct RequestLogger;
impl<B> OnRequest<B> for RequestLogger {
	fn on_request(&mut self, request: &Request<B>, _: &Span) {
		tracing::info!(
			method = ?request.method(),
			uri = ?request.uri(),
			"{:?}",
			request.version()
		);
	}
}
