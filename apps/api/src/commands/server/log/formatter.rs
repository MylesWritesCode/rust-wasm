use color_eyre::owo_colors::OwoColorize;
use tracing::instrument::WithSubscriber;

pub struct Formatter;

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

        write!(writer, " {level}")?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                let meta = span.metadata();

                write!(writer, " {name} ", name = meta.name())?;

                match meta.name() {
                    super::REQ_PREFIX => {
                        write!(writer, " send to req formatter ")?;
                        // let mut visitor = visitor::RequestVisitor::default();
                        // event.record(&mut visitor);
                        // write!(writer, "{visitor}")?;
                    }
                    super::RES_PREFIX => {
                        write!(writer, " send to res formatter ")?;
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
