use dioxus::prelude::*;
use kernel::lang::t;

#[component]
pub fn Home() -> Element {
    rsx! {
        section { id: "home", class: "space-y-4",
            h6 { {t!("home_welcome")} }
        }
    }
}
