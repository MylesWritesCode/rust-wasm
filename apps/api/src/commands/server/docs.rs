#[derive(utoipa::OpenApi)]
#[openapi(
    tags(
        (name = "docs", description = "Documentation endpoints"),
    ),
    components(schemas(super::GenerateDataResponse, Vertex, Edge))
)]
pub struct Docs;

// note(myles) We have to declare the below types like this, because newtype
// support isn't that great in utoipa yet. I'm unsure if it's even possible in
// rust, given the orphan rule preventing consumers of libraries from
// implementing traits on types they don't own. This isn't great because it can
// possibly get out of sync with the actual types in the extenral crate.

#[allow(dead_code)] // This is only used for documentation
#[derive(utoipa::ToSchema, serde::Deserialize)]
#[schema(as = graph::Vertex)]
struct Vertex {
    /// Used to determine which edges are connected to this vertex
    id: String,
    /// A human-readable identifier for the vertex
    label: String,
    /// An optional parent vertex
    parent: String,
}

#[allow(dead_code)] // This is only used for documentation
#[derive(utoipa::ToSchema, serde::Deserialize)]
#[schema(as = graph::Edge)]
pub struct Edge {
    /// Random id, doesn't matter
    pub id: String,
    /// ID of the source vertex
    pub source: String,
    /// ID of the target vertex
    pub target: String,
}

// impl<'a> utoipa::ToSchema<'a> for Vertex {
//     fn schema() -> (
//         &'a str,
//         utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
//     ) {
//         (
//             "Pet",
//             utoipa::openapi::ObjectBuilder::new()
//                 .property(
//                     "id",
//                     utoipa::openapi::ObjectBuilder::new()
//                         .schema_type(utoipa::openapi::SchemaType::Integer)
//                         .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
//                             utoipa::openapi::KnownFormat::Int64,
//                         ))),
//                 )
//                 .required("id")
//                 .property(
//                     "name",
//                     utoipa::openapi::ObjectBuilder::new()
//                         .schema_type(utoipa::openapi::SchemaType::String),
//                 )
//                 .required("name")
//                 .property(
//                     "age",
//                     utoipa::openapi::ObjectBuilder::new()
//                         .schema_type(utoipa::openapi::SchemaType::Integer)
//                         .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
//                             utoipa::openapi::KnownFormat::Int32,
//                         ))),
//                 )
//                 .example(Some(serde_json::json!({
//                   "name":"bob the cat","id":1
//                 })))
//                 .into(),
//         )
//     }
// }
