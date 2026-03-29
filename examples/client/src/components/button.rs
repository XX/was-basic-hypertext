use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use was_basic_hypertext::appearance::Appearance::*;
use was_basic_hypertext::attributes::CommonAttributeSetters;
use was_basic_hypertext::components::button::Button;
use was_basic_hypertext::layouts::code_example::{
    CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource,
};
use was_basic_hypertext::variant::Variant::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <h1>"Button"</h1>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Button>"Button"</Button>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"<Button>"Button"</Button>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h2>"Examples"</h2>

        <h3>"Variants"</h3>
        <p>"Set the "<code>variant</code>" property to change the button's variant."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs">
                    <Button variant=Neutral>"Neutral"</Button>
                    <Button variant=Brand>"Brand"</Button>
                    <Button variant=Success>"Success"</Button>
                    <Button variant=Warning>"Warning"</Button>
                    <Button variant=Danger>"Danger"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <Button variant=Neutral>"Neutral"</Button>
                    <Button variant=Brand>"Brand"</Button>
                    <Button variant=Success>"Success"</Button>
                    <Button variant=Warning>"Warning"</Button>
                    <Button variant=Danger>"Danger"</Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Appearance"</h3>
        <p>"Use the "<code>appearance</code>" property to change the button's visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Neutral appearance=Accent>"Accent"</Button>
                    <Button variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Neutral appearance=Filled>"Filled"</Button>
                    <Button variant=Neutral appearance=Outlined>"Outlined"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Brand appearance=Accent>"Accent"</Button>
                    <Button variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Brand appearance=Filled>"Filled"</Button>
                    <Button variant=Brand appearance=Outlined>"Outlined"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Success appearance=Accent>"Accent"</Button>
                    <Button variant=Success appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Success appearance=Filled>"Filled"</Button>
                    <Button variant=Success appearance=Outlined>"Outlined"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Warning appearance=Accent>"Accent"</Button>
                    <Button variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Warning appearance=Filled>"Filled"</Button>
                    <Button variant=Warning appearance=Outlined>"Outlined"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Button variant=Danger appearance=Accent>"Accent"</Button>
                    <Button variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Danger appearance=Filled>"Filled"</Button>
                    <Button variant=Danger appearance=Outlined>"Outlined"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Neutral appearance=Accent>"Accent"</Button>
                        <Button variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Neutral appearance=Filled>"Filled"</Button>
                        <Button variant=Neutral appearance=Outlined>"Outlined"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Brand appearance=Accent>"Accent"</Button>
                        <Button variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Brand appearance=Filled>"Filled"</Button>
                        <Button variant=Brand appearance=Outlined>"Outlined"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Success appearance=Accent>"Accent"</Button>
                        <Button variant=Success appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Success appearance=Filled>"Filled"</Button>
                        <Button variant=Success appearance=Outlined>"Outlined"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Warning appearance=Accent>"Accent"</Button>
                        <Button variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Warning appearance=Filled>"Filled"</Button>
                        <Button variant=Warning appearance=Outlined>"Outlined"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Button variant=Danger appearance=Accent>"Accent"</Button>
                        <Button variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Danger appearance=Filled>"Filled"</Button>
                        <Button variant=Danger appearance=Outlined>"Outlined"</Button>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Size"</h3>
        <p>"Buttons are sized relative to the current font size. You can set "
            <code>"font-size"</code>
            " style or the corresponding "
            <code>"size-*"</code>
            " class on any button (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Brand class="size-extra-small">"Brand"</Button>
                    <Button variant=Brand class="size-small">"Brand"</Button>
                    <Button variant=Brand class="size-medium">"Brand"</Button>
                    <Button variant=Brand class="size-large">"Brand"</Button>
                    <Button variant=Brand class="size-extra-large">"Brand"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Button variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Brand class="size-extra-small">"Brand"</Button>
                        <Button variant=Brand class="size-small">"Brand"</Button>
                        <Button variant=Brand class="size-medium">"Brand"</Button>
                        <Button variant=Brand class="size-large">"Brand"</Button>
                        <Button variant=Brand class="size-extra-large">"Brand"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Button variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Button>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
