use dioxus::prelude::*;
#[component]
pub fn Knowledge() -> Element {
    rsx! {
        section { id: "knowledge", class: "space-y-4",
            h1 { class: "text-3xl", "Things I know... or have known at some point" }
            div { class: "space-y-4" }
        }
    }
}
