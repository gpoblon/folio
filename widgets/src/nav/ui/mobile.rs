use crate::nav::model::Navigable;
use dioxus::prelude::*;
use kernel::lang::t;
#[component]
pub(super) fn MobileNav<R: Navigable>(active: R) -> Element {
    let color = active.color();
    rsx! {
        nav { class: "nav:hidden relative flex items-center justify-between px-6 h-18 border-b-3 border-{color} backdrop-blur",
            super::Brand {}
            super::mobile::BurgerMenu { active }
        }
    }
}
#[component]
pub fn BurgerMenu<R: Navigable>(active: R) -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        button {
            class: "nav:hidden ml-auto min-w-10 p-2 rounded-full text-muted",
            aria_label: if is_open() { "Close mobile menu" } else { "Open mobile menu" },
            aria_expanded: is_open(),
            aria_controls: "mobile-menu",
            onclick: move |_| is_open.set(!is_open()),
            {components::svg::Burger()}
        }
        if is_open() {
            MobileNavMenu { active, onclose: move |_| is_open.set(false) }
        }
    }
}
#[component]
fn MobileNavMenu<R: Navigable>(active: R, onclose: EventHandler<()>) -> Element {
    rsx! {
        div { id: "mobile-menu", class: "nav:hidden fixed inset-0 top-18 z-50",
            div { class: "flex flex-col w-full h-full space-y-6",
                div { class: "flex flex-col items-end border-b border-primary py-4 bg-primary",
                    for route in R::ITEMS.iter() {
                        Link {
                            key: "{route.slug()}",
                            to: *route,
                            onclick: move |_| onclose.call(()),
                            class: "block w-full text-right",
                            MobileNavItem { route: *route, is_active: &active == route }
                        }
                    }
                }
                div { class: "flex items-center space-x-8 justify-end pr-8",
                    features::lang::SelectLanguage {}
                    features::theme::ToggleTheme {}
                }
            }
        }
    }
}
#[component]
fn MobileNavItem<R: Navigable>(route: R, is_active: bool) -> Element {
    let color = route.color();
    rsx! {
        button {
            class: "my-2 py-4 inline-flex items-center justify-end gap-3 w-full pr-8",
            class: if is_active { "text-black bg-{color}" } else { "text-{color}" },
            label { class: "text-md cursor-pointer", {t!(route.slug())} }
            div { class: "size-8", {route.icon()} }
        }
    }
}
/// Alternate mobile navigation: bottom fixed bar with icons only.
#[component]
pub fn MobileBottomIcons<R: Navigable>(active: R) -> Element {
    let color = active.color();
    rsx! {
        nav { class: "fixed top-0 inset-x-0 z-50 md:hidden border-t border-primary bg-primary backdrop-blur",
            div { class: "grid grid-cols-4 gap-2 p-2",
                for route in R::ITEMS.iter() {
                    Link {
                        key: "{route.slug()}",
                        to: *route,
                        class: "flex items-center justify-center py-2",
                        div {
                            class: "size-12 rounded-full border flex items-center justify-center",
                            class: if &active == route { "bg-{color} text-primary border-transparent" } else { "text-{color} border-primary" },
                            div { class: "size-5", {route.icon()} }
                        }
                    }
                }
            }
        }
    }
}
