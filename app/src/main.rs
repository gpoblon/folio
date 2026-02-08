#![allow(non_snake_case)]
use dioxus::prelude::*;

pub mod router;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const THEME_COMPONENTS_CSS: Asset = asset!("/assets/components.css");
const THEME_BASE_CSS: Asset = asset!("/assets/main.css");
const TYPOGRAPHY_CSS: Asset = asset!("/assets/typography.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    kernel::lang::init_i18n();
    let theme = use_signal(|| kernel::theme::ThemeMode::default());
    provide_context(theme);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap" }
        document::Stylesheet { href: THEME_COMPONENTS_CSS }
        components::Bootstrap {}
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: THEME_BASE_CSS }
        document::Stylesheet { href: TYPOGRAPHY_CSS }
        div {
            id: "root",
            class: "min-h-screen flex flex-col bg-primary text-primary border-primary font-light font-sans overflow-x-hidden",
            lang: "fr",
            "data-theme": theme().as_str(),
            Router::<router::Route> {}
        }
    }
}

fn main() {
    dioxus::launch(App)
}
