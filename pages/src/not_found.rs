use components::decorations::{CenterSquares, Diagonals, OrbitDots, OrbitSvg};
use components::{Button, ButtonVariant};
use dioxus::prelude::*;
use kernel::lang::t;

/// 404 — Page not found.
///
/// Displayed for every URL that does not match a known route.
/// Design mirrors the home page aesthetics (orbital decorations, hub card).
///
/// `t!()` is evaluated at component-render level (not inside the effect
/// closure) because it calls `consume_context()` internally — a Dioxus
/// hook that must run in the component scope.
#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    let _ = &segments;

    let nav = use_navigator();
    let toast = components::toast::use_toast();
    let toast_msg = t!("route_not_found");

    use_effect(move || {
        toast.error(toast_msg.clone()).send();
    });

    rsx! {
        components::Seo {
            title: "404 — Page Not Found",
            description: "The page you are looking for does not exist.",
            canonical_path: "/404",
            robots: "noindex, nofollow",
        }

        section {
            id: "not-found",
            class: "relative flex-1 flex items-center justify-center overflow-hidden select-none",

            div {
                class: "relative z-1 place-self-center aspect-square
                        size-[320px] md:size-[450px] lg:size-[520px] xl:size-[650px]",

                OrbitSvg {}
                CenterSquares {}

                div {
                    class: "absolute inset-0 m-auto pointer-events-none
                            size-1/2 rotate-45 border-[0.5px]
                            backdrop-blur-xl bg-background/30 dark:bg-background/70
                            border-border",
                }

                div {
                    class: "absolute inset-0 m-auto z-10 p-4
                            flex flex-col items-center justify-center gap-3 text-center
                            w-[321px] h-[220px]
                            outline-[0.5px] outline-border
                            bg-accent",

                    p { class: "text-4xl text-foreground", "404" }

                    div { class: "w-12 h-px bg-border" }

                    p { class: "text-sm text-muted-foreground", { t!("route_not_found") } }

                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            if nav.can_go_back() {
                                nav.go_back();
                            } else {
                                nav.replace("/");
                            }
                        },
                        span {
                            class: "uppercase p-2",
                            "←  " { t!("go_back") }
                        }
                    }
                }
            }

            Diagonals {}
            OrbitDots {}
        }
    }
}
