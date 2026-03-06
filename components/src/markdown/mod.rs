mod parser;
mod preprocess;
mod rewrite;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct MarkdownProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// The markdown content to render
    /// Example: "# Hello World"
    /// Default: ""
    #[props(into, default = String::new())]
    content: String,
}

/// Uses `dangerous_inner_html` to render markdown content as HTML
#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = parser::string_to_html(props.content.clone());

    rsx! {
        div {
            class: "prose dark:prose-invert max-w-none",
            dangerous_inner_html: "{content}",
            ..props.attributes
        }
    }
}
