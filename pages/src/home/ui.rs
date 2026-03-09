use dioxus::prelude::*;

use super::cardinal::{Cardinal, CardinalCell};
use super::center::CenterCell;
use super::decorations::{Diagonals, OrbitDots};

const HOME_CSS: Asset = asset!("./home.css");

#[component]
pub fn Home() -> Element {
    let mut hovered: Signal<Option<Cardinal>> = use_signal(|| None);
    let nav = use_navigator();

    let active = hovered().unwrap_or(Cardinal::Identity);

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        section {
            id: "home",
            class: "home-grid flex-1 max-h-[calc(100dvh-9rem)]",

            for c in Cardinal::NAV {
                CardinalCell {
                    key: "{c.label()}",
                    cardinal: c,
                    is_active: hovered() == Some(c),
                    onhover: move |val| hovered.set(val),
                    onclick: move |c: Cardinal| { nav.push(c.route()); },
                }
            }

            CenterCell { active }
            Diagonals {}
            OrbitDots {}
        }
    }
}
