use dioxus::prelude::*;

#[component]
pub fn Knowledge() -> Element {
    rsx! {
        section {
            class: "max-w-5xl mx-auto py-32 center-content",
            id: "knowledge",
            widgets::articles::ArticleGrid {}
        }
    }
}
