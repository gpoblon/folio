mod desktop;
mod mobile;
use crate::nav::model::Navigable;
use dioxus::prelude::*;
use dioxus::router::use_route;
/// Generic entry point — the **only** function that knows about `R`.
///
/// Erases `R` into a concrete [`NavCtx`] provided via context so that every
/// component below can remain non-generic.  Usage mirrors Dioxus's own
/// `Router::<Route> {}` pattern.
#[allow(non_snake_case)]
pub fn NavBar<R: Navigable>() -> Element {
    let active = use_route::<R>();
    rsx! {
        desktop::DesktopNav { active }
        mobile::MobileNav { active }
    }
}
#[component]
pub(super) fn Brand() -> Element {
    rsx! {
        Link { class: "flex gap-3 items-center w-1/6", to: "/",
            div { class: "flex min-w-10 size-10 rounded-full items-center justify-center text-black logo-gradient",
                div { class: "size-4", components::svg::Knowledge {} }
            }
            span { class: "text-base font-light whitespace-nowrap", "· GPOBLON ·" }
        }
    }
}
