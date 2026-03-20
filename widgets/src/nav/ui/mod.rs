mod desktop;
mod mobile;

use crate::nav::model::Navigable;
use dioxus::prelude::*;
use dioxus::router::use_route;

/// Generic navigation bar entry point.
///
/// Renders both desktop and mobile variants; only one is visible at a time
/// based on the `nav` breakpoint (950 px).
#[allow(non_snake_case)]
pub fn NavBar<R: Navigable>() -> Element {
    let active = use_route::<R>();
    rsx! {
        desktop::DesktopNav { active: active.clone() }
        mobile::MobileNav { active: active.clone() }
    }
}

#[component]
pub(super) fn Brand() -> Element {
    rsx! {
        Link { class: "flex gap-3 items-center w-1/6", to: "/",
            div {
                class: "flex min-w-10 size-10 rounded-full items-center
                        justify-center text-black logo-gradient",
                div { class: "size-4", components::svg::Knowledge {} }
            }
            span { class: "text-base font-light whitespace-nowrap", "· GPOBLON ·" }
        }
    }
}
