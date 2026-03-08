use dioxus::prelude::*;

static GITHUB_LOGO: Asset = asset!("/assets/github.svg");

#[component]
pub fn Github(class: Option<String>) -> Element {
    rsx! {
        img {
            class: format!("dark:invert shrink-0 {}", class.unwrap_or_default()),
            src: GITHUB_LOGO,
            alt: "GitHub Logo"
        }
    }
}
