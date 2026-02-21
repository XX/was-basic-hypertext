use std::ops::Deref;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::GlobalAttributes;
use hypertext::{Buffer, Lazy, Renderable, component, rsx};
use itertools::Itertools;

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

    pub fn class_line(&self) -> String {
        [Self::CLASS, self.variant.into_str(), self.appearance.into_str()]
            .into_iter()
            .chain(self.additional.get_classes().iter().map(Deref::deref))
            .filter(|class| !class.is_empty())
            .join(" ")
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

#[component]
pub fn badge(params: &BadgeParams, children: Lazy<fn(&mut Buffer)>) -> impl Renderable {
    let id = params.additional.get_not_empty_id();
    let class_line = params.class_line();
    let style_line = params.additional.get_style_line();

    rsx! {
        <div id=[id] class=class_line style=[style_line.as_ref()]>
            (children)
        </div>
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Badge {
    fn default() -> Self {
        Self {
            params: Default::default(),
            children: Default::default(),
        }
    }
}
