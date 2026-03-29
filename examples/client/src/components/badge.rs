use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use was_basic_hypertext::appearance::Appearance::*;
use was_basic_hypertext::attributes::CommonAttributeSetters;
use was_basic_hypertext::components::badge::Badge;
use was_basic_hypertext::layouts::code_example::{
    CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource,
};
use was_basic_hypertext::variant::Variant::*;

use crate::fontawesome;

pub fn overview() -> impl Renderable {
    rsx! {
        <h1>"Badge"</h1>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Badge>"Badge"</Badge>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"<Badge>"Badge"</Badge>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h2>"Examples"</h2>

        <h3>"Variants"</h3>
        <p>"Set the "<code>variant</code>" property to change the badge's variant."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Neutral>"Neutral"</Badge>
                    <Badge variant=Brand>"Brand"</Badge>
                    <Badge variant=Success>"Success"</Badge>
                    <Badge variant=Warning>"Warning"</Badge>
                    <Badge variant=Danger>"Danger"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <Badge variant=Neutral>"Neutral"</Badge>
                    <Badge variant=Brand>"Brand"</Badge>
                    <Badge variant=Success>"Success"</Badge>
                    <Badge variant=Warning>"Warning"</Badge>
                    <Badge variant=Danger>"Danger"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Appearance"</h3>
        <p>"Use the "<code>appearance</code>" property to change the badge's visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Neutral appearance=Accent>"Accent"</Badge>
                    <Badge variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Neutral appearance=Filled>"Filled"</Badge>
                    <Badge variant=Neutral appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Brand appearance=Accent>"Accent"</Badge>
                    <Badge variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Brand appearance=Filled>"Filled"</Badge>
                    <Badge variant=Brand appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Success appearance=Accent>"Accent"</Badge>
                    <Badge variant=Success appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Success appearance=Filled>"Filled"</Badge>
                    <Badge variant=Success appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Warning appearance=Accent>"Accent"</Badge>
                    <Badge variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Warning appearance=Filled>"Filled"</Badge>
                    <Badge variant=Warning appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Danger appearance=Accent>"Accent"</Badge>
                    <Badge variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Danger appearance=Filled>"Filled"</Badge>
                    <Badge variant=Danger appearance=Outlined>"Outlined"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Neutral appearance=Accent>"Accent"</Badge>
                        <Badge variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Neutral appearance=Filled>"Filled"</Badge>
                        <Badge variant=Neutral appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Brand appearance=Accent>"Accent"</Badge>
                        <Badge variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Brand appearance=Filled>"Filled"</Badge>
                        <Badge variant=Brand appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Success appearance=Accent>"Accent"</Badge>
                        <Badge variant=Success appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Success appearance=Filled>"Filled"</Badge>
                        <Badge variant=Success appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Warning appearance=Accent>"Accent"</Badge>
                        <Badge variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Warning appearance=Filled>"Filled"</Badge>
                        <Badge variant=Warning appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Badge variant=Danger appearance=Accent>"Accent"</Badge>
                        <Badge variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Danger appearance=Filled>"Filled"</Badge>
                        <Badge variant=Danger appearance=Outlined>"Outlined"</Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Size"</h3>
        <p>"Badges are sized relative to the current font size. You can set "
            <code>"font-size"</code>
            " style or the corresponding "
            <code>"size-*"</code>
            " class on any badge (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Brand class="size-extra-small">"Brand"</Badge>
                    <Badge variant=Brand class="size-small">"Brand"</Badge>
                    <Badge variant=Brand class="size-medium">"Brand"</Badge>
                    <Badge variant=Brand class="size-large">"Brand"</Badge>
                    <Badge variant=Brand class="size-extra-large">"Brand"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Brand class="size-extra-small">"Brand"</Badge>
                        <Badge variant=Brand class="size-small">"Brand"</Badge>
                        <Badge variant=Brand class="size-medium">"Brand"</Badge>
                        <Badge variant=Brand class="size-large">"Brand"</Badge>
                        <Badge variant=Brand class="size-extra-large">"Brand"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Badge variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Pill Badges"</h3>
        <p>"Use the "<code>"pill"</code>" property to give badges rounded edges."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Neutral pill=true>"Neutral"</Badge>
                    <Badge variant=Brand pill=true>"Brand"</Badge>
                    <Badge variant=Success pill=true>"Success"</Badge>
                    <Badge variant=Warning pill=true>"Warning"</Badge>
                    <Badge variant=Danger pill=true>"Danger"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <Badge variant=Neutral pill=true>"Neutral"</Badge>
                    <Badge variant=Brand pill=true>"Brand"</Badge>
                    <Badge variant=Success pill=true>"Success"</Badge>
                    <Badge variant=Warning pill=true>"Warning"</Badge>
                    <Badge variant=Danger pill=true>"Danger"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <h3>"Start & End Decorations"</h3>
        <p>"Use the "<code>"start"</code>" and "<code>"end"</code>" classes to add presentational elements like icons alongside the badge's label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Badge variant=Brand>
                        <span class="start icon">
                            (fontawesome::icon("check"))
                        </span>
                        "Start"
                    </Badge>
                    <Badge variant=Brand>
                        "End"
                        <span class="end icon">
                            (fontawesome::icon("bookmark-solid"))
                        </span>
                    </Badge>
                    <Badge variant=Brand>
                        <span class="start icon">
                            (fontawesome::icon("check"))
                        </span>
                        "Both"
                        <span class="end icon">
                            (fontawesome::icon("bookmark-solid"))
                        </span>
                    </Badge>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Badge variant=Brand appearance=Outlined class="size-extra-small">
                        <span class="start icon">
                            (fontawesome::icon("check"))
                        </span>
                        "Both"
                        <span class="end icon">
                            (fontawesome::icon("bookmark-solid"))
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class="size-small">
                        <span class="start icon">
                            (fontawesome::icon("check"))
                        </span>
                        "Both"
                        <span class="end icon">
                            (fontawesome::icon("bookmark-solid"))
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class="size-medium">
                        <span class="start icon">
                            (fontawesome::icon("check"))
                        </span>
                        "Both"
                        <span class="end icon">
                            (fontawesome::icon("bookmark-solid"))
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class="size-large">
                        <span class="start icon">
                            (fontawesome::icon("check"))
                        </span>
                        "Both"
                        <span class="end icon">
                            (fontawesome::icon("bookmark-solid"))
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class="size-extra-large">
                        <span class="start icon">
                            (fontawesome::icon("check"))
                        </span>
                        "Both"
                        <span class="end icon">
                            (fontawesome::icon("bookmark-solid"))
                        </span>
                    </Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Badge>
                            <span class="start">...</span>
                            "Start"
                        </Badge>
                        <Badge>
                            "End"
                            <span class="end">...</span>
                        </Badge>
                        <Badge>
                            <span class="start">...</span>
                            "Both"
                            <span class="end">...</span>
                        </Badge>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Badge variant=Brand appearance=Outlined  class="size-extra-small">
                            <span class="start">...</span>
                            "Both"
                            <span class="end">...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class="size-small">
                            <span class="start">...</span>
                            "Both"
                            <span class="end">...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class="size-medium">
                            <span class="start">...</span>
                            "Both"
                            <span class="end">...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class="size-large">
                            <span class="start">...</span>
                            "Both"
                            <span class="end">...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class="size-extra-large">
                            <span class="start">...</span>
                            "Both"
                            <span class="end">...</span>
                        </Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
