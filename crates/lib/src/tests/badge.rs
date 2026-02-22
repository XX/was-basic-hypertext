use hypertext::{Renderable, rsx};

use crate::appearance::AppearanceConstructor;
use crate::attributes::CommonAttributeSetters;
use crate::components::badge::{Badge, BadgeParams};
use crate::variant::{Variant, VariantConstructor};

#[test]
fn default() {
    let badge_markup = r#"<div class="badge neutral accent"></div>"#;

    let badge = Badge::default();
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge ../> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge ..></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn variant() {
    let badge_markup = r#"<div class="badge success accent"></div>"#;

    let badge = rsx! { <Badge params=(BadgeParams::success()) ../> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge params=(BadgeParams::success())></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn children() {
    let badge_markup = r#"<div class="badge neutral accent">Hello, world!</div>"#;

    let badge = rsx! { <Badge ..>"Hello, world!"</Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn nested() {
    let badge_markup =
        r#"<div class="badge neutral accent"><div class="badge success accent">Hello, world!</div></div>"#;

    let badge = rsx! { <Badge ..><Badge params=(BadgeParams::success())>"Hello, world!"</Badge></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn additional_attributes() {
    let badge_markup = r#"<div id="the-badge" class="badge neutral accent test alarm" style="color: red"></div>"#;

    let params = BadgeParams::new()
        .id("the-badge")
        .class("test")
        .class("alarm")
        .style("color: red");
    let badge = rsx! { <Badge params=(params.clone())></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge_markup =
        r#"<div id="bad" class="badge danger filled-outlined test" style="color: red; background-color: green"></div>"#;

    let params = BadgeParams::filled_outlined()
        .variant(Variant::Danger)
        .id("bad")
        .class("test")
        .style("color: red")
        .style("background-color: green");
    let badge = rsx! { <Badge params=(params.clone())></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}
