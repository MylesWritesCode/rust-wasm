use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {name}!"));
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Glyph {
    label: String,
    angle: u8,
}

/// This is copied because we're testing the transform speed, and we want this
/// struct to be a little different than the one exported from `graph`.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Vertex {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
    pub glyphs: Option<Vec<Glyph>>,
}

/// This is copied because we're testing the transform speed, and we want this
/// struct to be a little different than the one exported from `graph`.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Edge {
    /// Random id, doesn't matter
    pub id: String,
    /// ID of the source vertex
    pub source: String,
    /// ID of the target vertex
    pub target: String,
}

/// This is copied because we're testing the transform speed, and we want this
/// struct to be a little different than the one exported from `graph`.
enum Element {
    Vertex(Vertex),
    Edge(Edge),
}

impl serde::Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Element::Vertex(v) => v.serialize(serializer),
            Element::Edge(e) => e.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum DeGraphElement {
            Vertex(Vertex),
            Edge(Edge),
        }

        let element = DeGraphElement::deserialize(deserializer)?;

        match element {
            DeGraphElement::Vertex(v) => Ok(Element::Vertex(v)),
            DeGraphElement::Edge(e) => Ok(Element::Edge(e)),
        }
    }
}

#[wasm_bindgen(js_name = transformRs)]
pub fn transform_rs(value: JsValue) -> JsValue {
    let elements: Vec<graph::GraphElement> = serde_wasm_bindgen::from_value(value).unwrap();
    let mut res: Vec<Element> = Vec::new();

    for (i, element) in elements.iter().enumerate() {
        match element {
            graph::GraphElement::Vertex(v) => {
                let glyphs = if i % 8 == 0 {
                    let g = vec![Glyph {
                        label: "a".to_string(),
                        angle: 45,
                    }];
                    Some(g)
                } else {
                    None
                };

                let vertex = Vertex {
                    id: v.id.to_string(),
                    label: v.label.to_string(),
                    parent: v.parent.clone().map(String::from),
                    glyphs,
                };
                let element = Element::Vertex(vertex);

                res.push(element);
            }
            graph::GraphElement::Edge(e) => {
                let edge = Edge {
                    id: e.id.to_string(),
                    source: e.source.to_string(),
                    target: e.target.to_string(),
                };
                let element = Element::Edge(edge);

                res.push(element);
            }
        };
    }

    serde_wasm_bindgen::to_value(&res).unwrap()

    // let elements: Vec<graph::GraphElement> = serde_wasm_bindgen::from_value(value).unwrap();
    // // do stuff with elements
    //
    // serde_wasm_bindgen::to_value(&elements).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
