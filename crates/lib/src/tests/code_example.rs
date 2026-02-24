use hypertext::prelude::GlobalAttributes;
use hypertext::{Lazy, Renderable, rsx};

use crate::attributes::{CommonAttributeSetters, CommonAttrs};
use crate::components::badge::Badge;
use crate::hypertext_elements;
use crate::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

#[test]
fn default() {
    let code_example_markup = r#"<div class="code-example"></div>"#;

    let code_example = CodeExample::default();
    assert_eq!(code_example.render().as_inner(), code_example_markup);

    let code_example = rsx! { <CodeExample ..></CodeExample> };
    assert_eq!(code_example.render().as_inner(), code_example_markup);

    let code_example_preview_markup = r#"<div class="code-example-preview"></div>"#;

    let code_example_preview = CodeExamplePreview::default();
    assert_eq!(code_example_preview.render().as_inner(), code_example_preview_markup);

    let code_example_preview = rsx! { <CodeExamplePreview ..></CodeExamplePreview> };
    assert_eq!(code_example_preview.render().as_inner(), code_example_preview_markup);

    let code_example_source_markup = r#"<div class="code-example-source"><pre></pre></div>"#;

    let code_example_source = CodeExampleSource::default();
    assert_eq!(code_example_source.render().as_inner(), code_example_source_markup);

    let code_example_source = rsx! { <CodeExampleSource ..></CodeExampleSource> };
    assert_eq!(code_example_source.render().as_inner(), code_example_source_markup);

    let code_example_button_markup = r#"
        <div class="code-example-buttons">
            <button class="code-example-toggle" type="button" onclick="this.closest('.code-example').classList.toggle('open')"> 
                <span class="icon">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                        <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                    </svg>
                </span>
            </button>
        </div>
    "#
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example_button = CodeExampleButton::default();
    assert_eq!(code_example_button.render().as_inner(), &code_example_button_markup);

    let code_example_button = rsx! { <CodeExampleButton ..></CodeExampleButton> };
    assert_eq!(code_example_button.render().as_inner(), &code_example_button_markup);
}

#[test]
fn empty() {
    let code_example_markup = r#"
        <div class="code-example">
            <div class="code-example-preview">
            </div>
            <div class="code-example-source">
                <pre></pre>
            </div>
            <div class="code-example-buttons">
                <button 
                    class="code-example-toggle" 
                    type="button" 
                    onclick="this.closest('.code-example').classList.toggle('open')"
                > 
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    "#
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example = rsx! {
        <CodeExample ..>
            <CodeExamplePreview ..>
            </CodeExamplePreview>
            <CodeExampleSource ..>
            </CodeExampleSource>
            <CodeExampleButton ..></CodeExampleButton>
        </CodeExample>
    };
    assert_eq!(code_example.render().as_inner(), &code_example_markup);
}

#[test]
fn attributes() {
    let code_example_markup = r#"
        <div class="code-example open">
            <div id="preview" class="code-example-preview">
                <div class="code-example-resizer">
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512">
                            <path fill="currentColor" d="M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32L64 64zm128 0c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 384c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384z"></path>
                        </svg>
                    </span>
                </div>
            </div>
            <div id="source" class="code-example-source code">
                <pre></pre>
            </div>
            <div class="code-example-buttons toggle" style="color: red">
                <button 
                    class="code-example-toggle" 
                    type="button" 
                    onclick="this.closest('.code-example').classList.toggle('open')"
                > 
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    "#
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example = CodeExample::default()
        .open(true)
        .children(Lazy::dangerously_create(|buffer| {
            CodeExamplePreview::default()
                .resize(true)
                .id("preview")
                .render_to(buffer);
            CodeExampleSource::default()
                .id("source")
                .class("code")
                .render_to(buffer);
            CodeExampleButton::default()
                .class("toggle")
                .style("color: red")
                .render_to(buffer);
        }));
    assert_eq!(code_example.render().as_inner(), &code_example_markup);

    let code_example = rsx! {
        <CodeExample open=true ..>
            <CodeExamplePreview resize=true attrs=(CommonAttrs::new().id("preview"))>
            </CodeExamplePreview>
            <CodeExampleSource attrs=(CommonAttrs::new().id("source").class("code"))>
            </CodeExampleSource>
            <CodeExampleButton attrs=(CommonAttrs::new().class("toggle").style("color: red"))>
            </CodeExampleButton>
        </CodeExample>
    };
    assert_eq!(code_example.render().as_inner(), &code_example_markup);
}

#[test]
fn children() {
    let code_example_markup = r#"
        <div class="code-example">
            <div class="code-example-preview">
                <div class="badge neutral accent">Badge</div>
                <div class="code-example-resizer">
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512">
                            <path fill="currentColor" d="M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32L64 64zm128 0c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 384c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384z"></path>
                        </svg>
                    </span>
                </div>
            </div>
            <div class="code-example-source">
                <pre>
                    <code class="language-html">
                        &lt;Badge ..&gt;"Badge"&lt;/Badge&gt;
                    </code>
                </pre>
            </div>
            <div class="code-example-buttons">
                <button 
                    class="code-example-toggle" 
                    type="button" 
                    onclick="this.closest('.code-example').classList.toggle('open')"
                >Code 
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    "#
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example = CodeExample::default().children(Lazy::dangerously_create(|buffer| {
        CodeExamplePreview::default()
            .resize(true)
            .children(Lazy::dangerously_create(|buffer| {
                rsx!(<Badge ..>"Badge"</Badge>).render_to(buffer)
            }))
            .render_to(buffer);
        CodeExampleSource::default()
            .children(Lazy::dangerously_create(|buffer| {
                rsx! {
                    <code class="language-html">
                        r#"<Badge ..>"Badge"</Badge>"#
                    </code>
                }
                .render_to(buffer);
            }))
            .render_to(buffer);
        CodeExampleButton::default()
            .children(Lazy::dangerously_create(|buffer| "Code".render_to(buffer)))
            .render_to(buffer);
    }));
    assert_eq!(code_example.render().as_inner(), &code_example_markup);

    let code_example = rsx! {
        <CodeExample ..>
            <CodeExamplePreview resize=true ..>
                <Badge ..>"Badge"</Badge>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">
                    r#"<Badge ..>"Badge"</Badge>"#
                </code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>
    };
    assert_eq!(code_example.render().as_inner(), &code_example_markup);
}
