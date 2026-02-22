use derive_more::{AsMut, AsRef};
use hypertext::prelude::GlobalAttributes;
use hypertext::{Buffer, Lazy, Renderable, rsx};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::hypertext_elements;
use crate::macros::Params;
use crate::variant::Variant;

#[derive(Clone, Debug, Default, PartialEq, Eq, AsRef, AsMut, Params)]
pub struct BadgeParams {
    #[param(setters, from)]
    pub variant: Variant,

    #[param(setters, from)]
    pub appearance: Appearance,

    #[as_ref]
    #[as_mut]
    pub additional: CommonAttrs,
}

impl BadgeParams {
    pub const CLASS: &str = "badge";

    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct Badge {
    pub params: BadgeParams,
    pub children: Lazy<fn(&mut Buffer)>,
}

impl Renderable for Badge {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.params.id();
        let class_line = self.params.class_line_with([
            BadgeParams::CLASS,
            self.params.variant.into_str(),
            self.params.appearance.into_str(),
        ]);
        let style_line = self.params.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}
