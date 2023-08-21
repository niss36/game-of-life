use wasm_bindgen::prelude::wasm_bindgen;

use crate::universe::Universe;

#[wasm_bindgen]
impl Universe {
    pub fn render_to_text(&self) -> String {
        self.to_string()
    }
}
