use dioxus::prelude::*;

#[component]
pub fn Project(slug: String) -> Element {
    rsx! {
        div {
            class: "max-w-6xl mx-auto py-32",
            widgets::project::Project { slug }
        }
    }
}
