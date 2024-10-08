use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub(super) mod formatter;
pub(super) mod http;

pub(super) use http::layer::Layer;
pub(super) use http::layer::LOG_PREFIX;
pub(super) use http::visitor::Visitor;

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&self) {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    // axum logs rejections from built-in extractors with the `axum::rejection`
                    // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                    "api=trace,tower_http=trace,axum::rejection=trace".into()
                }),
            )
            .with(tracing_subscriber::fmt::layer().event_format(formatter::Formatter::new()))
            .init();
    }
}
