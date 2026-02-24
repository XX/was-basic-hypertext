use hypertext::prelude::GlobalAttributes;
use hypertext::{Renderable, rsx};
use was_basic_hypertext::components::badge::Badge;
use was_basic_hypertext::hypertext_elements;
use was_basic_hypertext::layouts::code_example::{
    CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource,
};
use was_basic_hypertext::variant::Variant;

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
                    <Badge variant=(Variant::Neutral) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Brand) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Success) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Warning) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Danger) ..>"Badge"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource ..>
                <code class="language-html">r#"
                    <Badge variant=(Variant::Neutral) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Brand) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Success) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Warning) ..>"Badge"</Badge>
                    <Badge variant=(Variant::Danger) ..>"Badge"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton ..>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
