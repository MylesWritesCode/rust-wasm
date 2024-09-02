use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {name}!"));
}

#[wasm_bindgen(js_name = transformRs)]
pub fn transform_rs(value: JsValue) -> JsValue {
    log(&format!("transform_rs: {value:?}"));

    let elements: Vec<graph::GraphElement> = serde_wasm_bindgen::from_value(value).unwrap();
    // do stuff with elements

    serde_wasm_bindgen::to_value(&elements).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
