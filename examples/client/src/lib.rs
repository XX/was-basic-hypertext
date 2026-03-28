use hypertext::prelude::GlobalAttributes;
use hypertext::{RenderableExt, rsx};
use was_basic_hypertext::appearance::Appearance::*;
use was_basic_hypertext::attributes::CommonAttributeSetters;
use was_basic_hypertext::components::button::Button;
use was_basic_hypertext::hypertext_elements;
use was_basic_hypertext::layouts::page::{Page, PageAside, PageBody, PageFooter, PageHeader, PageMain, PageMenu};
use was_basic_hypertext::link::LinkSetters;
use was_basic_hypertext::link::Target::*;
use was_basic_hypertext::variant::Variant::*;
use wasm_bindgen::prelude::*;

pub mod components;
pub mod fontawesome;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn render_root() -> String {
    rsx! {
        <Page>
            <PageHeader class="wa-split">
                <div class="wa-cluster">
                    <span class="icon" style="color: var(--wa-color-brand-fill-loud); font-size: 1.5em; --rotate-angle: 0deg;">
                        (fontawesome::icon("puzzle-piece"))
                    </span>
                    <span id="brand-name" class="wa-heading-m wa-desktop-only">"WAS Basic Hypertext"</span>
                    <a href="#">"Example Link"</a>
                    <a href="#">"Example Link"</a>
                </div>
                <div class="wa-cluster wa-gap-xs">
                    <Button variant=Brand appearance=Accent class="size-small" href="https://github.com/XX/was-basic-hypertext" target=Blank>
                        <span class="start icon">
                            (fontawesome::icon("github"))
                        </span>
                        "GitHub"
                    </Button>
                </div>
            </PageHeader>
            <PageBody>
                <PageMenu>
                    <nav class="page-menu-nav border-end">
                        <div class="wa-flank"><span class="wa-heading-m">"Components"</span></div>
                    </nav>
                    <nav class="page-menu-nav border-end">
                        <a class="wa-flank" href="#"><span>"Badge"</span></a>
                        <a class="wa-flank" href="#"><span>"Button"</span></a>
                    </nav>
                    <nav class="page-menu-nav border-end">
                        <div class="wa-flank"><span class="wa-heading-m">"Layouts"</span></div>
                    </nav>
                    <nav class="page-menu-nav">
                        <a class="wa-flank" href="#"><span>"Code Example"</span></a>
                        <a class="wa-flank" href="#"><span>"Page"</span></a>
                    </nav>
                </PageMenu>
                <PageMain class="main-content">
                    (components::badge::overview())
                </PageMain>
                <PageAside>
                </PageAside>
            </PageBody>
            <PageFooter class="wa-grid wa-gap-xl">
                <div class="wa-cluster" style="flex-wrap: nowrap">
                    <span class="icon">
                        (fontawesome::icon("puzzle-piece"))
                    </span>
                    <span id="brand-name" class="wa-heading-m">"WAS Basic Hypertext"</span>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">Our Work</h3>
                    <a href="#">Habitat Restoration</a>
                    <a href="#">Migration Science</a>
                    <a href="#">Advocacy</a>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">About Us</h3>
                    <a href="#">Our History</a>
                    <a href="#">Leadership</a>
                    <a href="#">Fiscal Reports</a>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">Discover</h3>
                    <a href="#">Field Guides</a>
                    <a href="#">Photo Search</a>
                    <a href="#">Gear and Resources</a>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">Get Involved</h3>
                    <a href="#">Adopt a Bird</a>
                    <a href="#">Your Local Audubon</a>
                    <a href="#">Youth Audubon Camps</a>
                </div>
            </PageFooter>
        </Page>
    }
    .render()
    .into_inner()
}
