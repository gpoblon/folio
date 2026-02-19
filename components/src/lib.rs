#![allow(non_snake_case)]

pub mod components;

pub use components::badge;
pub use components::button::*;
pub use components::dropdown_menu::*;
pub use components::separator::*;
pub use components::toast;

pub mod progress_bar;
pub mod svg;

pub use dioxus_tw_components::{Icon, Icons, Markdown};

use dioxus::prelude::*;

const DIOXUS_THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");

#[component]
pub fn Bootstrap() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DIOXUS_THEME_CSS }
        dioxus_tw_components::Bootstrap {}
    }
}
