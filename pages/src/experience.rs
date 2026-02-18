use dioxus::prelude::*;

#[component]
pub fn Experience() -> Element {
    rsx! {
        section {
            class: "container center-content p-32",
            id: "experience",
            features::experiences::Experiences {}
        }
    }
}
