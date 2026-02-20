use dioxus::prelude::*;

#[component]
pub fn Experience() -> Element {
    rsx! {
        section {
            class: "max-w-5xl mx-auto py-32 center-content",
            id: "experience",
            features::experiences::Experiences {}
        }
    }
}
