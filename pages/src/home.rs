use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    rsx! {
        section { id: "home", class: "space-y-4",
            h1 { class: "text-3xl", "Welcome to my world." }
        }
    }
}
