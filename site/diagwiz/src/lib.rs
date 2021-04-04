use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(input: String) -> Result<String, JsValue> {
    let input = format!("{}\n", input);
    diagram_seq::transform(input).map_err(|e| e.to_string().into())
}
