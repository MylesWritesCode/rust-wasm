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
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Glyph {
    pub label: String,
    pub angle: u8,
}

/// This is copied because we're testing the transform speed, and we want this
/// struct to be a little different than the one exported from `graph`.
#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
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

// notes(myles)
// - Check serialize and deserialize speeds, could be the reason this is slow.
// - Figure out how to accept and return a Rust type here. It'd be nice if we
//   can also get the type returned from the API for the front, but that's
//   lower priority for now - really just a nice to have should this go into
//   the very real world work project.
//   - Might be a little hard, because we can only use C-style enums with
//     wasm_bindgen. Something we can toy around with is sending back a vec of
//     the same type, but the struct will just have optional Vertex and Edge on
//     it. Seems gross imo, but if that's what we got, that's what we got. If
//     so, that struct will definitely have to have custom serialization and
//     deserialization impls.
//   - Figure out if there's a better way than above, though I suspect not with
//     the current limitations of wasm_bindgen (or maybe it's just wasm, idk).
// - Verify that the lib is running in --release mode, though I suspect it
//   already is when running with `wasm-pack build --release`.
// - LATER Make the transform more complicated, as I would have at work. I know
//   that the JS side transform isn't the greatest when dealing with so many
//   fields on the JS interface, so I'd like to see if more realistic data will
//   show a smaller delta between the WASM and JS transforms.
#[wasm_bindgen(js_name = transformRs)]
pub fn transform_rs(value: JsValue) -> JsValue {
    let elements: Vec<graph::GraphElement> = serde_wasm_bindgen::from_value(value).unwrap();
    let mut res: Vec<Element> = Vec::new();

    for (i, element) in elements.iter().enumerate() {
        match element {
            graph::GraphElement::Vertex(v) => {
                let glyphs = if i % 8 == 0 {
                    let g = vec![Glyph {
                        label: "some-glyph".to_string(),
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

// notes(myles)
// - Look into DOM manipulations, specifically with the canvas, thought I'm
//   unsure if setting up cytoscape + rendering the graph is within the scope
//   of this experiment. Perhaps if `transform_rs` is fast enough, I can look
//   into it.
//   - Draw SVG from WASM using data strings could work, but after the initial
//     drawing, I'm unsure (and I doubt) the WASM side manipulating the
//     positions is a good idea, considering the lag we get from crossing the
//     WASM/JS boundary.
//   - For the initial drawing, I know we get a bounding box per node, defined
//     by x1, x2, y1, y2, w, h. The math seems simple enough here, but is it
//     worth it to do this in WASM? Consider, cytoscape will have to send the
//     bb to WASM, and then WASM will have to translate then send back to the
//     DOM. The render flush fires very often (on move, on click), so there's
//     probably technical limitations there, but testing it seems fun. The JS
//     side doesn't actually do a lot of work on this front because it the
//     event fires so often, so maybe just taking the most expensive part from
//     the JS side (calculating world coords) of glyphs and doing that in WASM
//     could be a potential win.
//   - Someone suggested accessing the WASM memory on the JS side directly, so
//     that's a potential avenue to explore. We'd have to create the pointer
//     on the Rust side and send it to the front, and the front would have to
//     read the pointer for those addresses, but right now this is much more
//     easily solved with cytoscape-layers.
// - LATER Definitely out of scope for this project, but using Bevy to build a
//   graphing library seems like another fun side-project. The problem is I'm
//   not sure how much work needs to be done to build out a graphing library in
//   Bevy, or if graph libraries already exist. Again, fun side project, but
//   definitely out of scope for this.
//
//

#[cfg(test)]
mod tests {
    use super::*;
}
