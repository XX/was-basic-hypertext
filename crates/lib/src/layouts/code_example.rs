use derive_more::{AsMut, AsRef};
use hypertext::prelude::{EventHandlerAttributes, GlobalAttributes};
use hypertext::{Buffer, Lazy, Renderable, rsx};
use was_basic_hypertext_macros::{Params, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::hypertext_elements;

#[derive(Default, AsRef, AsMut, Params)]
#[const_str(CLASS = "code-example")]
pub struct CodeExample {
    #[param(setters)]
    pub open: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[param(setters)]
    pub children: Lazy<fn(&mut Buffer)>,
}

impl Renderable for CodeExample {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let classes = [Self::CLASS, if self.open { "open" } else { "" }];
        let class_line = self.class_line_with(classes);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Params)]
#[const_str(CLASS = "code-example-preview")]
pub struct CodeExamplePreview {
    #[param(setters)]
    pub resize: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[param(setters)]
    pub children: Lazy<fn(&mut Buffer)>,
}

impl Renderable for CodeExamplePreview {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
                @if self.resize {
                    <div class=(CodeExample::CLASS, "-resizer")>
                        <span class="icon">
                            // grip-lines-vertical
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512">
                                // ! Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32L64 64zm128 0c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 384c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384z"></path>
                            </svg>
                        </span>
                    </div>
                }
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Params)]
#[const_str(CLASS = "code-example-source")]
pub struct CodeExampleSource {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[param(setters)]
    pub children: Lazy<fn(&mut Buffer)>,
}

impl Renderable for CodeExampleSource {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                <pre>
                    (self.children)
                </pre>
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Params)]
#[const_str(CLASS = "code-example-buttons")]
pub struct CodeExampleButton {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[param(setters)]
    pub children: Lazy<fn(&mut Buffer)>,
}

impl Renderable for CodeExampleButton {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                <button class=(CodeExample::CLASS, "-toggle") type="button" onclick=("this.closest('.", CodeExample::CLASS, "').classList.toggle('open')")>
                    (self.children)
                    " "
                    <span class="icon">
                        // chevron-down
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            // ! Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                            // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        }
        .render_to(buffer);
    }
}
