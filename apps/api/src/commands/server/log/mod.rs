use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

mod formatter;
mod http;

pub const LOG_PREFIX: &str = "log::http";
pub const REQ_PREFIX: &str = "log::http::req";
pub const RES_PREFIX: &str = "log::http::res";

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
                    "api=trace,tower_http=debug,axum::rejection=trace".into()
                }),
            )
            // Global event formatter
            .with(tracing_subscriber::fmt::layer().event_format(formatter::Formatter))
            // HTTP event formatter
            .with(tracing_subscriber::fmt::layer().with_filter(
                tracing_subscriber::filter::filter_fn(|metadata| {
                    metadata.name().starts_with(LOG_PREFIX)
                }),
            ))
            .init();
    }
}
