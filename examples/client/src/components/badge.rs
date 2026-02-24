use hypertext::prelude::GlobalAttributes;
use hypertext::{Renderable, rsx};
use was_basic_hypertext::appearance::Appearance::*;
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
                    <Badge variant=Neutral ..>"Badge"</Badge>
                    <Badge variant=Brand ..>"Badge"</Badge>
                    <Badge variant=Success ..>"Badge"</Badge>
                    <Badge variant=Warning ..>"Badge"</Badge>
                    <Badge variant=Danger ..>"Badge"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"
                    <Badge variant=Neutral ..>"Badge"</Badge>
                    <Badge variant=Brand ..>"Badge"</Badge>
                    <Badge variant=Success ..>"Badge"</Badge>
                    <Badge variant=Warning ..>"Badge"</Badge>
                    <Badge variant=Danger ..>"Badge"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Appearance"</h3>
        <p>"Use the "<code>appearance</code>" property to change the badge's visual appearance."</p>
        <CodeExample ..>
            <CodeExamplePreview resize=true ..>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge appearance=Accent variant=Neutral ..>"Accent"</Badge>
                    <Badge appearance=FilledOutlined variant=Neutral ..>"Filled-Outlined"</Badge>
                    <Badge appearance=Filled variant=Neutral ..>"Filled"</Badge>
                    <Badge appearance=Outlined variant=Neutral ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge appearance=Accent variant=Brand ..>"Accent"</Badge>
                    <Badge appearance=FilledOutlined variant=Brand ..>"Filled-Outlined"</Badge>
                    <Badge appearance=Filled variant=Brand ..>"Filled"</Badge>
                    <Badge appearance=Outlined variant=Brand ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge appearance=Accent variant=Success ..>"Accent"</Badge>
                    <Badge appearance=FilledOutlined variant=Success ..>"Filled-Outlined"</Badge>
                    <Badge appearance=Filled variant=Success ..>"Filled"</Badge>
                    <Badge appearance=Outlined variant=Success ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge appearance=Accent variant=Warning ..>"Accent"</Badge>
                    <Badge appearance=FilledOutlined variant=Warning ..>"Filled-Outlined"</Badge>
                    <Badge appearance=Filled variant=Warning ..>"Filled"</Badge>
                    <Badge appearance=Outlined variant=Warning ..>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge appearance=Accent variant=Danger ..>"Accent"</Badge>
                    <Badge appearance=FilledOutlined variant=Danger ..>"Filled-Outlined"</Badge>
                    <Badge appearance=Filled variant=Danger ..>"Filled"</Badge>
                    <Badge appearance=Outlined variant=Danger ..>"Outlined"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge appearance=Accent variant=Neutral ..>"Accent"</Badge>
                        <Badge appearance=FilledOutlined variant=Neutral ..>"Filled-Outlined"</Badge>
                        <Badge appearance=Filled variant=Neutral ..>"Filled"</Badge>
                        <Badge appearance=Outlined variant=Neutral ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge appearance=Accent variant=Brand ..>"Accent"</Badge>
                        <Badge appearance=FilledOutlined variant=Brand ..>"Filled-Outlined"</Badge>
                        <Badge appearance=Filled variant=Brand ..>"Filled"</Badge>
                        <Badge appearance=Outlined variant=Brand ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge appearance=Accent variant=Success ..>"Accent"</Badge>
                        <Badge appearance=FilledOutlined variant=Success ..>"Filled-Outlined"</Badge>
                        <Badge appearance=Filled variant=Success ..>"Filled"</Badge>
                        <Badge appearance=Outlined variant=Success ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge appearance=Accent variant=Warning ..>"Accent"</Badge>
                        <Badge appearance=FilledOutlined variant=Warning ..>"Filled-Outlined"</Badge>
                        <Badge appearance=Filled variant=Warning ..>"Filled"</Badge>
                        <Badge appearance=Outlined variant=Warning ..>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge appearance=Accent variant=Danger ..>"Accent"</Badge>
                        <Badge appearance=FilledOutlined variant=Danger ..>"Filled-Outlined"</Badge>
                        <Badge appearance=Filled variant=Danger ..>"Filled"</Badge>
                        <Badge appearance=Outlined variant=Danger ..>"Outlined"</Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
