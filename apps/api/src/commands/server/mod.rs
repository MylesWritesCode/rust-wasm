mod cors;
mod log;

use rand::Rng as _;

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
    log::Logger::new().init();

    let app = axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/users", axum::routing::post(create_user))
        .route("/generate-graph", axum::routing::post(generate_data))
        .layer(log::Layer::new().get_layer())
        .layer(cors::Cors::new().get_layer());

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        host.unwrap_or("0.0.0.0".to_string()),
        port.unwrap_or(5001)
    ))
    .await?;

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());

    tracing::error!(
        x = "value",
        complex.foo = "foo",
        complex.bar = "bar",
        "message",
    );
    tracing::warn!(
        x = "value",
        complex.foo = "foo",
        complex.bar = "bar",
        "message",
    );
    tracing::info!(
        x = "value",
        complex.foo = "foo",
        complex.bar = "bar",
        "message",
    );
    tracing::debug!(
        x = "value",
        complex.foo = "foo",
        complex.bar = "bar",
        "message",
    );
    tracing::trace!(
        x = "value",
        complex.foo = "foo",
        complex.bar = "bar",
        "message",
    );

    // deleteme(myles)
    {
        let uri = "/hello";
        let body = "{}";

        let kind = "REQ";
        let method = axum::http::Method::GET.to_string();
        let e = tracing::error_span!(log::LOG_PREFIX, method, uri, body, kind);
        let kind = "RES";
        tracing::error!(parent: &e, kind, "has error");

        let method = axum::http::Method::PUT.to_string();
        let w = tracing::warn_span!(log::LOG_PREFIX, method, uri, body, kind);
        tracing::warn!(parent: &w, "has warning");

        let method = axum::http::Method::POST.to_string();
        let i = tracing::info_span!(log::LOG_PREFIX, method, uri, body, kind);
        tracing::info!(parent: &i, "has info");

        let method = axum::http::Method::HEAD.to_string();
        let d = tracing::debug_span!(log::LOG_PREFIX, method, uri, body, kind);
        tracing::debug!(parent: &d, "has debug");

        let method = axum::http::Method::PATCH.to_string();
        let t = tracing::trace_span!(log::LOG_PREFIX, method, uri, body, kind);
        tracing::trace!(parent: &t, "has trace");

        let span = tracing::info_span!(log::LOG_PREFIX, method, uri, body, kind);

        let method = axum::http::Method::TRACE.to_string();
        tracing::info!(parent: &span, method, uri, body, kind);

        let method = axum::http::Method::DELETE.to_string();
        tracing::info!(parent: &span, method, uri, body, kind);

        let method = axum::http::Method::OPTIONS.to_string();
        tracing::info!(parent: &span, method, uri, body, kind);

        let method = axum::http::Method::CONNECT.to_string();
        tracing::info!(parent: &span, method, uri, body, kind);
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
