use hypertext::{Renderable, rsx};

use crate::components::badge::Badge;
use crate::variant::Variant;

#[test]
fn default() {
    let badge_markup = r#"<div class="badge neutral"></div>"#;

    let badge = Badge::default();
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge ../> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge ..></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn variant() {
    let badge_markup = r#"<div class="badge success"></div>"#;

    let badge = rsx! { <Badge variant=(Variant::Success) ../> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge variant=(Variant::Success)></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn children() {
    let badge_markup = r#"<div class="badge neutral">Hello, world!</div>"#;

    let badge = rsx! { <Badge ..>"Hello, world!"</Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn nested() {
    let badge_markup = r#"<div class="badge neutral"><div class="badge success">Hello, world!</div></div>"#;

    let badge = rsx! { <Badge ..><Badge variant=(Variant::Success)>"Hello, world!"</Badge></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}
