use hypertext::{Renderable, rsx};
use was_basic_hypertext::components::badge::Badge;
use was_basic_hypertext::hypertext_elements;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn render_root() -> String {
    rsx! {
        <div>
            <Badge ..>"Hello, world!"</Badge>
        </div>
    }
    .render()
    .into_inner()
}
