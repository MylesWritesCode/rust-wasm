mod log;

use rand::Rng as _;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(clap::Args)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    command: Option<Commands>,
    args: Option<String>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Start {
        host: Option<String>,
        port: Option<u16>,
    },
}

pub(crate) async fn run(args: &Arguments) -> crate::Result<()> {
    match &args.command {
        Some(commands) => match commands {
            Commands::Start { host, port } => start(host.clone(), *port).await,
        },
        None => start(None, None).await,
    }
}

async fn start(host: Option<String>, port: Option<u16>) -> crate::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "api=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().event_format(log::Formatter))
        // .with(tracing_subscriber::fmt::layer())
        .init();

    // let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
    //     |request: &axum::http::Request<_>| {
    //         let method = match request.method() {
    //             &axum::http::Method::GET => " GET",
    //             _ => request.method().as_str(),
    //         };
    //         let uri = request.uri();
    //         let body = request
    //             .extensions()
    //             .get::<axum::extract::Json<serde_json::Value>>()
    //             .map(|json| format!("{}", json.0))
    //             .unwrap_or_else(|| "{}".to_string());
    //
    //         let e = tracing::error_span!(
    //             log::HTTP_REQUEST_SPAN_NAME,
    //             method,
    //             uri = uri.to_string(),
    //             body
    //         );
    //         let w = tracing::warn_span!(
    //             log::HTTP_REQUEST_SPAN_NAME,
    //             method,
    //             uri = uri.to_string(),
    //             body
    //         );
    //         let i = tracing::info_span!(
    //             log::HTTP_REQUEST_SPAN_NAME,
    //             method,
    //             uri = uri.to_string(),
    //             body
    //         );
    //         let d = tracing::debug_span!(
    //             log::HTTP_REQUEST_SPAN_NAME,
    //             method,
    //             uri = uri.to_string(),
    //             body
    //         );
    //         let t = tracing::trace_span!(
    //             log::HTTP_REQUEST_SPAN_NAME,
    //             method,
    //             uri = uri.to_string(),
    //             body
    //         );
    //
    //         tracing::error!(parent: &e, ".");
    //         tracing::warn!(parent: &w, ".");
    //         tracing::info!(parent: &i, ".");
    //         tracing::debug!(parent: &d, ".");
    //         tracing::trace!(parent: &t, ".");
    //
    //         let span = tracing::warn_span!(
    //             log::HTTP_REQUEST_SPAN_NAME,
    //             method,
    //             uri = uri.to_string(),
    //             body
    //         );
    //
    //         span
    //     },
    // );

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any);

    let app = axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/users", axum::routing::post(create_user))
        .route("/generate-graph", axum::routing::post(generate_data))
        // .layer(trace_layer)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        host.unwrap_or("0.0.0.0".to_string()),
        port.unwrap_or(5001)
    ))
    .await?;

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());

    // deleteme(myles)
    {
        let e = tracing::error_span!(
            log::HTTP_REQUEST_SPAN_NAME,
            method = "PARENT",
            uri = "/hello",
            body = "{}"
        );
        let w = tracing::warn_span!(
            log::HTTP_REQUEST_SPAN_NAME,
            method = "PARENT",
            uri = "/hello",
            body = "{}"
        );
        let i = tracing::info_span!(
            log::HTTP_REQUEST_SPAN_NAME,
            method = "PARENT",
            uri = "/hello",
            body = "{}"
        );
        let d = tracing::debug_span!(
            log::HTTP_REQUEST_SPAN_NAME,
            method = "PARENT",
            uri = "/hello",
            body = "{}"
        );
        let t = tracing::trace_span!(
            log::HTTP_REQUEST_SPAN_NAME,
            method = "PARENT",
            uri = "/hello",
            body = "{}"
        );

        let method = "GET";
        let uri = "/hello";
        let body = "{}";

        tracing::error!(parent: &e, method, uri, body);
        tracing::warn!(parent: &w, method, uri, body);
        tracing::info!(parent: &i, method, uri, body);
        tracing::debug!(parent: &d, method, uri, body);
        tracing::trace!(parent: &t, method, uri, body);
    }

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    axum::Json(payload): axum::Json<CreateUser>,
) -> impl axum::response::IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (axum::http::StatusCode::CREATED, axum::Json(user))
}

// the input to our `create_user` handler
#[derive(serde::Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(serde::Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(
    Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
struct VertexId(String);

impl std::ops::Deref for VertexId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl VertexId {
    pub fn new() -> Self {
        Self(nanoid::nanoid!().to_string())
    }
}

impl<T: std::string::ToString> From<T> for VertexId {
    fn from(id: T) -> Self {
        Self(id.to_string())
    }
}

impl std::ops::DerefMut for VertexId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, serde::Serialize)]
struct Vertex {
    /// Used to determine which edges are connected to this vertex
    id: VertexId,
}

#[derive(Debug, serde::Serialize)]
struct Edge {
    /// Random id, doesn't matter
    id: String,
    /// ID of the source vertex
    source: VertexId,
    /// ID of the target vertex
    target: VertexId,
}

#[derive(Debug)]
enum GraphElement {
    Vertex(Vertex),
    Edge(Edge),
}

impl serde::Serialize for GraphElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            GraphElement::Vertex(v) => v.serialize(serializer),
            GraphElement::Edge(e) => e.serialize(serializer),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct GenerateDataPayload {
    /// Number of vertices to generate
    vertices: i32,
    /// Number of edges to generate
    edges: i32,
}

async fn generate_data(
    axum::Json(payload): axum::Json<GenerateDataPayload>,
) -> impl axum::response::IntoResponse {
    let mut elements: Vec<GraphElement> = Vec::new();

    for _ in 0..payload.vertices {
        elements.push(GraphElement::Vertex(Vertex {
            id: VertexId::new(),
        }));
    }

    let mut rng = rand::thread_rng();

    for _ in 0..payload.edges {
        // Should always be a vertex, since from 0..payload.vertices in
        // elements should only contain vertices.
        let source = rng.gen_range(0..payload.vertices);
        let mut target = rng.gen_range(0..payload.vertices);

        while source == target {
            target = rng.gen_range(0..payload.vertices);
        }

        let source: usize = source.try_into().unwrap_or(0);
        let target: usize = target.try_into().unwrap_or(0);

        let source = match &elements[source] {
            GraphElement::Vertex(v) => v.id.clone(),
            GraphElement::Edge(_) => {
                tracing::error!("Source is an edge, this should never happen.");
                continue;
            }
        };

        let target = match &elements[target] {
            GraphElement::Vertex(v) => v.id.clone(),
            GraphElement::Edge(_) => {
                tracing::error!("Target is an edge, this should never happen.");
                continue;
            }
        };

        elements.push(GraphElement::Edge(Edge {
            id: nanoid::nanoid!().to_string(),
            source,
            target,
        }));
    }

    (axum::http::StatusCode::OK, axum::Json(elements))
}
