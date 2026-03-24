use dioxus::prelude::*;
use kernel::build_info;
use kernel::lang::{t, use_lang};

static DIOXUS_LOGO: Asset = asset!("/assets/dioxus.png");

#[component]
pub fn Footer(tos_route: NavigationTarget) -> Element {
    let lang = use_lang();
    let date = build_info::build_date(lang);
    rsx! {
        footer { class: "w-full border-t border-border py-4 bg-background",
            div { class: "container center-content text-xs",
                div { class: "flex flex-col sm:flex-row sm:flex-wrap sm:items-center sm:justify-center gap-1",
                    // Line 1 on mobile: Copyright + logo + legal + source code link
                    div { class: "flex items-center justify-center gap-1",
                        span { class: "hidden sm:inline", "© 2026" }
                        span { class: "sm:hidden", "©" }
                        span { { t!("me") } }

                        // Separator
                        span { class: "inline sm:hidden", "·" }
                        div { class: "hidden sm:inline w-4 h-4 text-knowledge flex-shrink-0", components::svg::Knowledge {} }

                        // Legal
                        Link {
                            class: "underline hover:opacity-80 whitespace-nowrap",
                            to: tos_route,
                            rel: "noopener noreferrer",
                            {t!("tos_route")}
                        }

                        span { class: "inline", "·" }

                        // Source code
                        a {
                            class: "underline hover:opacity-80 link whitespace-nowrap",
                            href: "https://github.com/gpoblon/folio",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "data-umami-event": "footer-source-code",
                            "data-umami-event-url": "https://github.com/gpoblon/folio",
                            span {
                                components::svg::Github { class: "h-4 inline pr-1" }
                                {t!("footer_source_code")}
                            }
                        }
                    }

                    // Separator (hidden on mobile)
                    span { class: "hidden sm:inline", "·" }

                    // Line 2 on mobile: Tech stack
                    div { class: "flex items-center justify-center gap-1",
                        span { {t!("footer_made_by")} }
                        a {
                            class: "underline hover:opacity-80 link whitespace-nowrap",
                            href: "https://github.com/gpoblon",
                            target: "_blank",
                            rel: "me noopener noreferrer",
                            "data-umami-event": "footer-github",
                            "data-umami-event-url": "https://github.com/gpoblon",
                            "gpoblon"
                        }
                        span { {t!("footer_using")} }
                        span { "🦀 Rust &" }
                        img { class: "inline h-4 flex-shrink-0", src: DIOXUS_LOGO }
                        a {
                            class: "underline hover:opacity-80 link whitespace-nowrap",
                            href: "https://dioxus.dev",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "data-umami-event": "footer-dioxus",
                            "data-umami-event-url": "https://dioxus.dev",
                            "Dioxus"
                        }
                    }

                    // Separator (hidden on mobile)
                    span { class: "hidden sm:inline", "·" }

                    // Last update date (desktop only)
                    div { class: "hidden sm:flex items-center justify-center gap-1",
                        span { {t!("home_last_update")} }
                        span { "{date}" }
                    }
                }
            }
        }
    }
}
