use dioxus::prelude::*;

#[component]
pub fn Connect() -> Element {
    rsx! {
        section { id: "connect", class: "space-y-4",
            h1 { class: "text-3xl", "Engage with me" }
        }
    }
}
