use hypertext::prelude::GlobalAttributes;
use hypertext::{Buffer, Lazy, Renderable, component, rsx};

use crate::hypertext_elements;
use crate::variant::Variant;

#[component]
pub fn badge(variant: Variant, children: Lazy<fn(&mut Buffer)>) -> impl Renderable {
    let variant: &'static str = variant.into();

    rsx! {
        <div class=("badge ", variant)>
            (children)
        </div>
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self {
            variant: Variant::default(),
            children: Lazy::default(),
        }
    }
}
