use axum::{body, http};
use tower_http::{classify, trace};
use tracing::field;

type TraceLayer = trace::TraceLayer<
    classify::SharedClassifier<classify::ServerErrorsAsFailures>,
    MakeSpanWith,
    OnRequest,
    OnResponse,
    OnBodyChunk,
    OnEos,
    OnFailure,
>;

pub struct Layer(TraceLayer);

impl Layer {
    pub fn new() -> Self {
        let layer = trace::TraceLayer::new_for_http()
            .make_span_with(MakeSpanWith::default())
            .on_request(OnRequest::default())
            .on_response(OnResponse::default())
            .on_body_chunk(OnBodyChunk::default())
            .on_eos(OnEos::default())
            .on_failure(OnFailure::default());

        Self(layer)
    }

    pub fn get_layer(self) -> TraceLayer {
        self.0
    }
}

pub const LOG_PREFIX: &str = "log::http";
pub const RECORD_KIND_RES: &str = "RES";
pub const RECORD_KIND_REQ: &str = "REQ";
pub const RECORD_KIND: &str = "kind";
pub const RECORD_METHOD: &str = "method";
pub const RECORD_STATUS: &str = "status";
pub const RECORD_URI: &str = "uri";
pub const RECORD_LATENCY: &str = "latency";
pub const RECORD_BODY: &str = "body";

#[derive(Clone, Default)]
pub struct MakeSpanWith {}

impl<B> trace::MakeSpan<B> for MakeSpanWith {
    fn make_span(&mut self, _request: &http::Request<B>) -> tracing::Span {
        let span = tracing::info_span!(
            LOG_PREFIX,
            { RECORD_KIND } = field::Empty,
            { RECORD_METHOD } = field::Empty,
            { RECORD_STATUS } = field::Empty,
            { RECORD_URI } = field::Empty,
            { RECORD_BODY } = field::Empty,
        );

        span
    }
}

#[derive(Clone, Default)]
pub struct OnRequest {}

impl<B> tower_http::trace::OnRequest<B> for OnRequest
where
    B: 'static + body::HttpBody<Data = body::Bytes> + Send,
{
    fn on_request(&mut self, request: &http::Request<B>, span: &tracing::Span) {
        tracing::info!(
            parent: span,
            kind = RECORD_KIND_REQ,
            method = request.method().to_string(),
            uri = request.uri().to_string(),
        );
    }
}

#[derive(Clone, Default)]
pub struct OnResponse {}

impl<B> trace::OnResponse<B> for OnResponse {
    fn on_response(
        self,
        response: &http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        tracing::info!(
            parent: span,
            kind = RECORD_KIND_RES,
            latency = latency.as_millis(),
            status = response.status().as_u16()
        );
    }
}

#[derive(Clone, Default)]
pub struct OnBodyChunk {}

impl<B> trace::OnBodyChunk<B> for OnBodyChunk {
    fn on_body_chunk(&mut self, _chunk: &B, _latency: std::time::Duration, _span: &tracing::Span) {
        // do nothing for now
    }
}

#[derive(Clone, Default)]
pub struct OnEos {}

impl trace::OnEos for OnEos {
    fn on_eos(
        self,
        _trailers: Option<&http::HeaderMap>,
        _stream_duration: std::time::Duration,
        _span: &tracing::Span,
    ) {
        // do nothing for now
    }
}

#[derive(Clone, Default)]
pub struct OnFailure {}

impl<FailureClass> trace::OnFailure<FailureClass> for OnFailure {
    fn on_failure(
        &mut self,
        _failure_classification: FailureClass,
        _latency: std::time::Duration,
        _span: &tracing::Span,
    ) {
        // do nothing for now
    }
}
