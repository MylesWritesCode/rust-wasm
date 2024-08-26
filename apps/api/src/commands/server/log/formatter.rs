use color_eyre::owo_colors::OwoColorize;

pub struct Formatter;

impl Formatter {
    pub fn new() -> Self {
        Self
    }
}

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
            tracing::Level::INFO => "INF".green().bold().to_string(),
            tracing::Level::DEBUG => "DBG".blue().bold().to_string(),
            tracing::Level::TRACE => "TRC".purple().bold().to_string(),
        };

        write!(writer, " {level}")?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                let meta = span.metadata();

                if meta.name().starts_with(super::LOG_PREFIX) {
                    let mut visitor = super::Visitor::new();
                    event.record(&mut visitor);
                    write!(writer, "{visitor}")?;
                } else {
                    write!(writer, " SPAN  name={}", meta.name())?;
                    // ctx.field_format().format_fields(writer.by_ref(), event)?;
                }
            }
        } else {
            write!(writer, " {}  ", "LOG".italic())?;
            ctx.field_format().format_fields(writer.by_ref(), event)?;
        }

        writeln!(writer)
    }
}
