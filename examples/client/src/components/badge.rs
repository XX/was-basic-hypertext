use hypertext::prelude::GlobalAttributes;
use hypertext::{Renderable, rsx};
use was_basic_hypertext::appearance::Appearance::*;
use was_basic_hypertext::attributes::{CommonAttributeSetters, CommonAttrs};
use was_basic_hypertext::components::badge::Badge;
use was_basic_hypertext::hypertext_elements;
use was_basic_hypertext::layouts::code_example::{
    CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource,
};
use was_basic_hypertext::variant::Variant::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <h1>"Badge"</h1>
        <CodeExample ..>
            <CodeExamplePreview resize=true ..>
                <Badge ..>"Badge"</Badge>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"<Badge ..>"Badge"</Badge>"#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>

        <h2>"Examples"</h2>

        <h3>"Variants"</h3>
        <p>"Set the "<code>variant</code>" property to change the badge's variant."</p>
        <CodeExample ..>
            <CodeExamplePreview resize=true ..>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Neutral ..>"Neutral"</Badge>
                    <Badge variant=Brand ..>"Brand"</Badge>
                    <Badge variant=Success ..>"Success"</Badge>
                    <Badge variant=Warning ..>"Warning"</Badge>
                    <Badge variant=Danger ..>"Danger"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"
                    <Badge variant=Neutral ..>"Neutral"</Badge>
                    <Badge variant=Brand ..>"Brand"</Badge>
                    <Badge variant=Success ..>"Success"</Badge>
                    <Badge variant=Warning ..>"Warning"</Badge>
                    <Badge variant=Danger ..>"Danger"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Appearance"</h3>
        <p>"Use the "<code>appearance</code>" property to change the badge's visual appearance."</p>
        <CodeExample ..>
            <CodeExamplePreview resize=true ..>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Neutral appearance=Accent ..>"Accent"</Badge>
                    <Badge variant=Neutral appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                    <Badge variant=Neutral appearance=Filled ..>"Filled"</Badge>
                    <Badge variant=Neutral appearance=Outlined ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Brand appearance=Accent ..>"Accent"</Badge>
                    <Badge variant=Brand appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                    <Badge variant=Brand appearance=Filled ..>"Filled"</Badge>
                    <Badge variant=Brand appearance=Outlined ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Success appearance=Accent ..>"Accent"</Badge>
                    <Badge variant=Success appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                    <Badge variant=Success appearance=Filled ..>"Filled"</Badge>
                    <Badge variant=Success appearance=Outlined ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Warning appearance=Accent ..>"Accent"</Badge>
                    <Badge variant=Warning appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                    <Badge variant=Warning appearance=Filled ..>"Filled"</Badge>
                    <Badge variant=Warning appearance=Outlined ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Danger appearance=Accent ..>"Accent"</Badge>
                    <Badge variant=Danger appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                    <Badge variant=Danger appearance=Filled ..>"Filled"</Badge>
                    <Badge variant=Danger appearance=Outlined ..>"Outlined"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Neutral appearance=Accent ..>"Accent"</Badge>
                        <Badge variant=Neutral appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                        <Badge variant=Neutral appearance=Filled ..>"Filled"</Badge>
                        <Badge variant=Neutral appearance=Outlined ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Brand appearance=Accent ..>"Accent"</Badge>
                        <Badge variant=Brand appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                        <Badge variant=Brand appearance=Filled ..>"Filled"</Badge>
                        <Badge variant=Brand appearance=Outlined ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Success appearance=Accent ..>"Accent"</Badge>
                        <Badge variant=Success appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                        <Badge variant=Success appearance=Filled ..>"Filled"</Badge>
                        <Badge variant=Success appearance=Outlined ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Warning appearance=Accent ..>"Accent"</Badge>
                        <Badge variant=Warning appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                        <Badge variant=Warning appearance=Filled ..>"Filled"</Badge>
                        <Badge variant=Warning appearance=Outlined ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Badge variant=Danger appearance=Accent ..>"Accent"</Badge>
                        <Badge variant=Danger appearance=FilledOutlined ..>"Filled-Outlined"</Badge>
                        <Badge variant=Danger appearance=Filled ..>"Filled"</Badge>
                        <Badge variant=Danger appearance=Outlined ..>"Outlined"</Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Size"</h3>
        <p>"Badges are sized relative to the current font size. You can set "<code>"font-size"</code>" on any badge (or an ancestor element) to change it."</p>
        <CodeExample ..>
            <CodeExamplePreview resize=true ..>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-xs)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-s)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-m)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-l)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-xl)")
                    ) ..>"Brand"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-xs)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-s)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-m)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-l)")
                    ) ..>"Brand"</Badge>
                    <Badge variant=Brand attrs=(
                        CommonAttrs::new().style("font-size: var(--wa-font-size-xl)")
                    ) ..>"Brand"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Pill Badges"</h3>
        <p>"Use the "<code>"pill"</code>" property to give badges rounded edges."</p>
        <CodeExample ..>
            <CodeExamplePreview resize=true ..>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Neutral pill=true ..>"Neutral"</Badge>
                    <Badge variant=Brand pill=true ..>"Brand"</Badge>
                    <Badge variant=Success pill=true ..>"Success"</Badge>
                    <Badge variant=Warning pill=true ..>"Warning"</Badge>
                    <Badge variant=Danger pill=true ..>"Danger"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"
                    <Badge variant=Neutral pill=true ..>"Neutral"</Badge>
                    <Badge variant=Brand pill=true ..>"Brand"</Badge>
                    <Badge variant=Success pill=true ..>"Success"</Badge>
                    <Badge variant=Warning pill=true ..>"Warning"</Badge>
                    <Badge variant=Danger pill=true ..>"Danger"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
