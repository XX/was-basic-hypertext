use derive_more::{AsMut, AsRef};
use hypertext::prelude::GlobalAttributes;
use hypertext::{Buffer, Lazy, Renderable, rsx};

use crate::appearance::{Appearance, AppearanceSetters};
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::hypertext_elements;
use crate::variant::{Variant, VariantSetters};

#[derive(Clone, Debug, Default, PartialEq, Eq, AsRef, AsMut)]
pub struct BadgeParams {
    pub variant: Variant,
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

impl VariantSetters for BadgeParams {
    fn set_variant(&mut self, variant: Variant) {
        self.variant = variant;
    }
}

impl AppearanceSetters for BadgeParams {
    fn set_appearance(&mut self, appearance: Appearance) {
        self.appearance = appearance;
    }
}

impl From<Variant> for BadgeParams {
    fn from(variant: Variant) -> Self {
        Self::default().variant(variant)
    }
}

impl From<Appearance> for BadgeParams {
    fn from(appearance: Appearance) -> Self {
        Self::default().appearance(appearance)
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
