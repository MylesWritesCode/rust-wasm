mod http;
mod layer;
pub mod visitor;

use color_eyre::owo_colors::OwoColorize;

pub const HTTP_REQUEST_SPAN_NAME: &str = "api::http::request";
pub const HTTP_RESPONSE_SPAN_NAME: &str = "api::http::response";

pub struct Formatter;

// impl<'writer> tracing_subscriber::fmt::FormatFields<'writer> for Formatter {
//     fn format_fields<R: tracing_subscriber::field::RecordFields>(
//         &self,
//         mut writer: tracing_subscriber::fmt::format::Writer<'writer>,
//         fields: R,
//     ) -> std::fmt::Result {
//         let mut visitor = request::RequestFields::default();
//         fields.record(&mut visitor);
//
//
//         write!(writer, "some field")
//     }
// }

impl<S, N> tracing_subscriber::fmt::FormatEvent<S, N> for Formatter
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: 'static + for<'a> tracing_subscriber::fmt::FormatFields<'a>,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        write!(writer, "{timestamp}")?;

        let level: String = match *event.metadata().level() {
            tracing::Level::ERROR => "ERR".red().bold().to_string(),
            tracing::Level::WARN => "WRN".yellow().bold().to_string(),
            tracing::Level::INFO => "INF".blue().bold().to_string(),
            tracing::Level::DEBUG => "DBG".green().bold().to_string(),
            tracing::Level::TRACE => "TRC".purple().bold().to_string(),
        };

        write!(writer, " | {level} |")?;

        write!(writer, "we are going over the event scope")?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                let meta = span.metadata();

                #[allow(clippy::single_match)] // This is a placeholder for future use
                match meta.name() {
                    HTTP_REQUEST_SPAN_NAME => {
                        let mut visitor = visitor::RequestVisitor::default();
                        event.record(&mut visitor);
                        write!(writer, "{visitor}")?;
                    }
                    _ => {
                        write!(writer, " {name} ", name = meta.name())?;
                    }
                }
            }
        } else {
            write!(writer, " ")?;
            ctx.field_format().format_fields(writer.by_ref(), event)?;
        }

        writeln!(writer)
    }
}
