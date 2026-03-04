use dioxus::prelude::*;

#[component]
pub fn Article(slug: Vec<String>) -> Element {
    rsx! {
        div {
            class: "max-w-6xl mx-auto py-32",
            widgets::article::Article { slug }
        }
    }
}
