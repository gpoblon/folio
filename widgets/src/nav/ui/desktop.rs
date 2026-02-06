use crate::nav::model::Navigable;
use dioxus::prelude::*;
use kernel::lang::t;
#[component]
pub(super) fn DesktopNav<R: Navigable>(active: R) -> Element {
    let color = active.color();
    rsx! {
        nav { class: "hidden nav:flex relative items-center justify-between px-6 h-18 border-b-3 border-{color} bg-primary backdrop-blur",
            super::Brand {}
            PlainMenu { active }
            Profile {}
        }
    }
}
#[component]
fn PlainMenu<R: Navigable>(active: R) -> Element {
    rsx! {
        div { class: "flex gap-3 justify-center w-fit z-1",
            for route in R::ITEMS.iter() {
                Link { key: "{route.slug()}", to: *route,
                    PlainNavItem { route: *route, is_active: &active == route }
                }
            }
        }
    }
}
#[component]
fn PlainNavItem<R: Navigable>(route: R, is_active: bool) -> Element {
    let color = route.color();
    let slug = route.slug();
    rsx! {
        div {
            class: "border p-1 border-{color} data-[active=false]:border-transparent",
            "data-active": is_active,
            button {
                class: "inline-flex p-1 min-w-36 justify-center items-center gap-3 whitespace-nowrap",
                class: if is_active { "bg-accent border border-primary" } else { "border border-primary text-primary bg-primary" },
                key: "{slug}",
                div { class: "min-w-5 min-h-5", {route.icon()} }
                label { class: "text-md", {t!(slug)} }
            }
        }
    }
}
#[component]
fn Profile() -> Element {
    rsx! {
        div { class: "flex w-1/6 items-center justify-end gap-3",
            features::lang::SelectLanguage {}
            features::theme::ToggleTheme {}
        }
    }
}
