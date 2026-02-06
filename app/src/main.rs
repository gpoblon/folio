#![allow(non_snake_case)]
pub mod router;
use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const THEME_BASE_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
#[component]
fn App() -> Element {
    kernel::lang::init_i18n();
    let theme = use_signal(|| kernel::theme::ThemeMode::default());
    provide_context(theme);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap" }
        if cfg!(debug_assertions) {
            document::Script {
                src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4",
                r#type: "module",
            }
        }
        document::Stylesheet { href: THEME_BASE_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        div {
            id: "root",
            class: "min-h-screen flex flex-col bg-primary text-primary border-primary font-light font-sans overflow-x-hidden",
            "data-theme": theme().as_str(),
            lang: "fr",
            Router::<router::Route> {}
        }
    }
}
fn main() {
    dioxus::launch(App)
}
