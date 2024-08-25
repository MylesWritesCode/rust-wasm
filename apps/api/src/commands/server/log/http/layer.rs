use axum::http;
use tower_http::{classify, trace};

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
            .make_span_with(MakeSpanWith)
            .on_request(OnRequest)
            .on_response(OnResponse)
            .on_body_chunk(OnBodyChunk)
            .on_eos(OnEos)
            .on_failure(OnFailure);

        Self(layer)
    }

    pub fn get_layer(self) -> TraceLayer {
        self.0
    }
}

#[derive(Clone)]
pub struct MakeSpanWith;

impl<B> trace::MakeSpan<B> for MakeSpanWith {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        todo!()
    }
}

#[derive(Clone)]
pub struct OnRequest;

impl<B> tower_http::trace::OnRequest<B> for OnRequest {
    fn on_request(&mut self, request: &http::Request<B>, span: &tracing::Span) {
        todo!()
    }
    // fn on_request(&self, request: &http::Request<B>, span: &tracing::Span) {
    //     let method = request.method().to_string();
    //     let uri = request.uri().to_string();
    //     let body = String::from_utf8_lossy(request.body()).to_string();
    //     let span = tracing::debug_span!(log::REQ_PREFIX, method, uri, body);
    //     let _enter = span.enter();
    // }
}

#[derive(Clone)]
pub struct OnResponse;

impl<B> trace::OnResponse<B> for OnResponse {
    fn on_response(
        self,
        response: &http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        todo!()
    }
}

#[derive(Clone)]
pub struct OnBodyChunk;

impl<B> trace::OnBodyChunk<B> for OnBodyChunk {
    fn on_body_chunk(&mut self, chunk: &B, latency: std::time::Duration, span: &tracing::Span) {
        todo!()
    }
}

#[derive(Clone)]
pub struct OnEos;

impl trace::OnEos for OnEos {
    fn on_eos(
        self,
        trailers: Option<&http::HeaderMap>,
        stream_duration: std::time::Duration,
        span: &tracing::Span,
    ) {
        todo!()
    }
}

#[derive(Clone)]
pub struct OnFailure;

impl<FailureClass> trace::OnFailure<FailureClass> for OnFailure {
    fn on_failure(
        &mut self,
        failure_classification: FailureClass,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        todo!()
    }
}
