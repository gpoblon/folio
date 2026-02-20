use dioxus::prelude::*;
// use kernel::lang;

#[component]
pub fn Article(slug: Vec<String>) -> Element {
    rsx! {
        div {
            class: "max-w-5xl mx-auto py-32 center-content",
            entities::article::Article { slug }
        }
    }
}
