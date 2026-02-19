use dioxus::prelude::*;

#[component]
pub fn Knowledge() -> Element {
    rsx! {
        section {
            class: "container center-content",
            id: "knowledge",
            widgets::articles::ArticleGrid {}
        }
    }
}
