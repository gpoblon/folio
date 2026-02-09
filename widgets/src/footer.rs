use dioxus::prelude::*;
use kernel::lang::t;

static DIOXUS_LOGO: Asset = asset!("/assets/dioxus.png");

#[component]
pub fn Footer(tos_route: NavigationTarget) -> Element {
    rsx! {
        footer { class: "w-full border-t border-primary py-2",
            div { class: "container center-content text-xs flex flex-wrap items-center justify-center gap-2",
                span { class: "inline-flex items-center gap-1",
                    span { "© 2026 Gaëtan Poblon" }
                    div { class: "mx-1 w-4 h-4 text-knowledge", components::svg::Knowledge {} }
                    span { {t!("footer_made_by")} }
                    a {
                        class: "underline hover:opacity-80",
                        href: "https://github.com/gpoblon",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "gpoblon"
                    }
                    span { {t!("footer_using")} }
                    img { class: "inline h-4", src: DIOXUS_LOGO }
                    a {
                        class: "underline hover:opacity-80",
                        href: "https://dioxus.dev",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Dioxus"
                    }
                    span { "·" }
                    Link {
                        class: "underline hover:opacity-80",
                        to: tos_route,
                        rel: "noopener noreferrer",
                        {t!("tos_route")}
                    }
                }
            }
        }
    }
}
