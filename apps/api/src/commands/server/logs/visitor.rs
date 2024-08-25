use color_eyre::owo_colors::OwoColorize;

#[derive(Debug, Default)]
pub struct RequestVisitor {
    pub method: Option<String>,
    pub uri: Option<String>,
    pub body: Option<String>,
}

impl tracing::field::Visit for RequestVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "method" => self.method = Some(value.to_string()),
            "uri" => self.uri = Some(value.to_string()),
            "body" => self.body = Some(value.to_string()),
            _ => {
                println!("{}: {}", field.name(), value);
            }
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "method" => self.method = Some(format!("{:?}", value)),
            "uri" => self.uri = Some(format!("{:?}", value)),
            "body" => self.body = Some(format!("{:?}", value)),
            _ => {
                println!("{}: {:?}", field.name(), value);
            }
        }
    }
}

impl std::fmt::Display for RequestVisitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.method {
            Some(method) => {
                let method = match method.as_str() {
                    "GET" => "GET ",
                    "PUT" => "PUT ",
                    "POST" => "POST",
                    "HEAD" => "HEAD",
                    "PATCH" => "PTCH",
                    "TRACE" => "TRCE",
                    "DELETE" => "DLTE",
                    "OPTIONS" => "OPTS",
                    "CONNECT" => "CONN",
                    _ => "NONE",
                };

                write!(f, " {method} |", method = method.green())?;
            }

            None => write!(f, " {method} |", method = "NONE".purple())?,
        }

        Ok(())
    }
}
