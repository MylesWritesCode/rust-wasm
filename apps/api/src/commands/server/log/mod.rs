mod layer;
pub mod request;

use color_eyre::owo_colors::OwoColorize;

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

        self.format_event(ctx, writer.by_ref(), event)?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                let meta = span.metadata();

                #[allow(clippy::single_match)] // This is a placeholder for future use
                match meta.name() {
                    request::SPAN_NAME => {
                        let mut visitor = request::RequestFields::default();
                        event.record(&mut visitor);

                        write!(
                            writer,
                            "found method: {method:#?} ",
                            method = event.fields()
                        )?;
                    }
                    _ => {
                        write!(writer, " {name} ", name = meta.name())?;
                    }
                }
            }
        }

        write!(writer, " ")?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
