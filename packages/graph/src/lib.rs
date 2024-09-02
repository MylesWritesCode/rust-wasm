use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(
    Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct VertexId(Box<str>);

impl std::ops::Deref for VertexId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl VertexId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for VertexId {
    fn default() -> Self {
        Self(Box::from(nanoid::nanoid!()))
    }
}

impl<T: std::string::ToString> From<T> for VertexId {
    fn from(id: T) -> Self {
        Self(Box::from(id.to_string()))
    }
}

impl std::ops::DerefMut for VertexId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vertex {
    /// Used to determine which edges are connected to this vertex
    pub id: VertexId,
    pub label: Box<str>,
    pub parent: Option<Box<str>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Edge {
    /// Random id, doesn't matter
    pub id: Box<str>,
    /// ID of the source vertex
    pub source: VertexId,
    /// ID of the target vertex
    pub target: VertexId,
}

#[derive(Debug)]
pub enum GraphElement {
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

impl<'de> serde::Deserialize<'de> for GraphElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Advantages of Using a Helper Enum
        // Clarity and Organization
        //     A helper enum can encapsulate the deserialization logic for
        //     multiple variants, making the code cleaner and easier to
        //     understand. This separation allows you to manage deserialization
        //     logic for different types without cluttering the main enum's
        //     implementation.
        // Support for Complex Deserialization
        //     If the deserialization logic for each variant is complex or
        //     requires different handling (like different field names or
        //     types), a helper enum can simplify this. It allows you to define
        //     untagged variants or use different attributes for each variant
        //     without complicating the main enum.
        // Error Handling
        //     Using a helper enum can make error handling more
        //     straightforward. If deserialization fails for one variant, you
        //     can easily return specific error messages based on which variant
        //     failed to deserialize.
        // Flexibility
        //     The helper enum can be designed to handle various input formats
        //     or structures, allowing for more flexible deserialization
        //     strategies. For example, it can manage cases where the input
        //     data might not clearly indicate which variant to deserialize
        //     into.
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum DeGraphElement {
            Vertex(Vertex),
            Edge(Edge),
        }

        let element = DeGraphElement::deserialize(deserializer)?;

        match element {
            DeGraphElement::Vertex(v) => Ok(GraphElement::Vertex(v)),
            DeGraphElement::Edge(e) => Ok(GraphElement::Edge(e)),
        }
    }
}
