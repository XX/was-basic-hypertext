use derive_more::{AsMut, AsRef};
use hypertext::prelude::GlobalAttributes;
use hypertext::{Buffer, Lazy, Renderable, rsx};
use was_basic_hypertext_macros::{Params, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::hypertext_elements;
use crate::variant::Variant;

#[derive(Default, AsRef, AsMut, Params)]
#[const_str(CLASS = "badge")]
pub struct Badge {
    #[param(setters, from)]
    pub variant: Variant,

    #[param(setters, from)]
    pub appearance: Appearance,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[param(setters)]
    pub children: Lazy<fn(&mut Buffer)>,
}

impl Renderable for Badge {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS, self.variant.into_str(), self.appearance.into_str()]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}
