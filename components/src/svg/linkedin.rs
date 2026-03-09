use dioxus::prelude::*;

static LINKEDIN_LOGO: Asset = asset!("/assets/linkedin.svg");

#[component]
pub fn Linkedin(class: Option<String>) -> Element {
    rsx! {
        img {
            class: format!("dark:invert shrink-0 {}", class.unwrap_or_default()),
            src: LINKEDIN_LOGO,
            alt: "LinkedIn Logo"
        }
    }
}
