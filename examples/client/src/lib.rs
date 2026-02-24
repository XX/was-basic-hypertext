use hypertext::{Renderable, rsx};
use was_basic_hypertext::hypertext_elements;
use wasm_bindgen::prelude::*;

pub mod components;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn render_root() -> String {
    rsx! {
        <div>
            (components::badge::overview())
        </div>
    }
    .render()
    .into_inner()
}
