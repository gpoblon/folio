use crate::nav::model::Navigable;
use dioxus::prelude::*;
use kernel::lang::t;

#[component]
pub(super) fn MobileNav<R: Navigable>(active: R) -> Element {
    let color = active.color();
    rsx! {
        nav { class: "nav:hidden relative flex items-center justify-between px-6 h-18 border-b-3 border-{color} backdrop-blur",
            super::Brand {}
            div { class: "flex items-center gap-2 ml-auto",
                features::lang::SelectLanguage {}
                features::theme::ToggleTheme {}
                super::mobile::BurgerMenu { active }
            }
        }
    }
}
#[component]
pub fn BurgerMenu<R: Navigable>(active: R) -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        button {
            class: "nav:hidden min-w-10 p-2 rounded-full text-muted-foreground",
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
        div {
            id: "mobile-menu",
            class: "nav:hidden fixed inset-0 top-18 z-50 bg-background flex flex-col w-full h-full space-y-6",
            div { class: "flex flex-col items-end border-b border-border py-4",
                for route in R::ITEMS.iter() {
                    Link {
                        key: "{route.slug()}",
                        to: route.clone(),
                        onclick: move |_| onclose.call(()),
                        class: "block w-full text-right",
                        MobileNavItem { route: route.clone(), is_active: &active == route }
                    }
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
        nav { class: "fixed top-0 inset-x-0 z-50 md:hidden border-t border-border bg-background backdrop-blur",
            div { class: "grid grid-cols-4 gap-2 p-2",
                for route in R::ITEMS.iter() {
                    Link {
                        key: "{route.slug()}",
                        to: route.clone(),
                        class: "flex items-center justify-center py-2",
                        div {
                            class: "size-12 rounded-full border flex items-center justify-center",
                            class: if &active == route { "bg-{color} text-foreground border-transparent" } else { "text-{color} border-border" },
                            div { class: "size-5", {route.icon()} }
                        }
                    }
                }
            }
        }
    }
}
