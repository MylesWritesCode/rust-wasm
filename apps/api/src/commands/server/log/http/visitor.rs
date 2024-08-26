use axum::http;
use std::str::FromStr;

use color_eyre::owo_colors::OwoColorize;

#[derive(Default)]
pub struct Visitor {
    // pub const RECORD_KIND: &str = "kind";
    pub kind: Option<String>,
    // pub const RECORD_METHOD: &str = "method";
    pub method: Option<String>,
    // pub const RECORD_STATUS: &str = "status";
    pub status: Option<u16>,
    // pub const RECORD_URI: &str = "uri";
    pub uri: Option<String>,
    // pub const RECORD_LATENCY: &str = "latency";
    pub latency: Option<u128>,
    // pub const RECORD_BODY: &str = "body";
}

impl Visitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl tracing_subscriber::field::Visit for Visitor {
    fn record_debug(&mut self, field: &tracing_core::Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            super::RECORD_KIND => {
                self.kind = Some(format!("{:?}", value));
            }
            super::RECORD_METHOD => {
                self.method = Some(format!("{:?}", value));
            }
            super::RECORD_STATUS => {
                if let Ok(status) = format!("{:?}", value).parse() {
                    self.status = Some(status);
                }
            }
            super::RECORD_URI => {
                self.uri = Some(format!("{:?}", value));
            }
            super::RECORD_LATENCY => {
                if let Ok(latency) = format!("{:?}", value).parse() {
                    self.latency = Some(latency);
                }
            }
            _ => {}
        };
    }

    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        match field.name() {
            super::RECORD_KIND => {
                self.kind = Some(value.into());
            }
            super::RECORD_METHOD => {
                self.method = Some(value.into());
            }
            super::RECORD_URI => {
                self.uri = Some(value.into());
            }
            _ => {}
        };
    }
}

impl std::fmt::Display for Visitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(method) = &self.method {
            let method = match axum::http::Method::from_str(method) {
                Ok(m) => match m {
                    http::Method::OPTIONS => "OPTS",
                    http::Method::GET => "GET ",
                    http::Method::POST => "POST",
                    http::Method::PUT => "PUT ",
                    http::Method::DELETE => "DEL ",
                    http::Method::HEAD => "HEAD",
                    http::Method::TRACE => "TRCE",
                    http::Method::CONNECT => "CONN",
                    http::Method::PATCH => "PTCH",
                    _ => "NONE",
                },
                _ => "NONE",
            };

            write!(f, " {}", method.bold().italic())?;
        } else {
            write!(f, " {}", "NONE".italic())?;
        }

        if let Some(kind) = &self.kind {
            write!(f, " {}", kind.bold())?;
        }

        if let Some(status) = &self.status {
            write!(f, " {}", status.bold())?;
        }

        if let Some(uri) = &self.uri {
            write!(f, " {}", uri)?;
        }

        if let Some(latency) = &self.latency {
            write!(f, " {}ms", latency)?;
        }

        Ok(())
    }
}
