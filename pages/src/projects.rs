use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section {
            class: "max-w-5xl mx-auto py-32 center-content",
            id: "projects",
            widgets::projects::ProjectGrid {}
        }
    }
}
