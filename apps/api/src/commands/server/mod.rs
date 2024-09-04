mod cors;
mod docs;
mod log;

use rand::Rng;
use utoipa::OpenApi;

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

    let router = axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/users", axum::routing::post(create_user))
        .route("/generate-graph", axum::routing::post(generate_graph))
        .merge(
            utoipa_swagger_ui::SwaggerUi::new("/docs").url("/openapi.json", docs::Docs::openapi()),
        )
        .layer(log::Layer::new().get_layer())
        .layer(cors::Cors::new().get_layer());

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        host.unwrap_or("0.0.0.0".to_string()),
        port.unwrap_or(5001)
    ))
    .await?;

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await?;

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

#[derive(Debug, serde::Deserialize)]
struct GenerateDataPayload {
    /// Number of vertices to generate
    vertices: u32,
    /// Number of edges to generate
    edges: u32,
}

#[derive(Debug, utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
struct GenerateDataResponse {
    vertices: Vec<graph::Vertex>,
    edges: Vec<graph::Edge>,
}

#[utoipa::path(get, path = "generate-graph")]
async fn generate_graph(
    axum::Json(payload): axum::Json<GenerateDataPayload>,
) -> impl axum::response::IntoResponse {
    // These should be safe to unwrap, since they are u32 and we're converting them to usize
    let mut vertices: Vec<graph::Vertex> =
        Vec::with_capacity(payload.vertices.try_into().unwrap_or(0));
    let mut edges: Vec<graph::Edge> = Vec::with_capacity(payload.edges.try_into().unwrap_or(0));

    let mut rng = rand::thread_rng();

    for _ in 0..payload.vertices {
        let mut vertex = graph::Vertex {
            id: graph::VertexId::new(),
            label: Box::from(nanoid::nanoid!()),
            parent: None,
        };

        // 20% chance to spawn a vertex with a parent, as long as we have at least one vertex in the vec
        if !vertices.is_empty() && rng.gen_range(0..100) > 80 {
            let i = rng.gen_range(0..vertices.len());

            if let Some(parent) = vertices.get(i) {
                vertex.parent = Some(Box::from(parent.id.to_string()));
            };
        }

        vertices.push(vertex);
    }

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

        // Only push an edge if both the source and target are valid vertices
        if let (Some(s), Some(t)) = (&vertices.get(source), &vertices.get(target)) {
            let edge = graph::Edge {
                id: Box::from(nanoid::nanoid!()),
                source: s.id.clone(),
                target: t.id.clone(),
            };

            edges.push(edge);
        }
    }

    tracing::info!(
        "Generated {} vertices and {} edges",
        payload.vertices,
        payload.edges
    );

    (
        axum::http::StatusCode::OK,
        axum::Json(GenerateDataResponse { vertices, edges }),
    )
}
