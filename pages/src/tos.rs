use dioxus::prelude::*;

#[component]
pub fn TermsOfUse() -> Element {
    rsx! {
        section { id: "terms-of-use", class: "space-y-8", "Legal & CGU rendered here" }
    }
}
