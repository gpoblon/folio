#![allow(non_snake_case)]

pub mod components;
pub use components::*;

mod markdown;
mod seo;
pub mod svg;
pub use markdown::Markdown;
pub use seo::{Seo, SeoProps};

pub use dioxus_tw_components::{Icon, Icons};

pub mod decorations;
pub use decorations::*;

use dioxus::prelude::*;

const DIOXUS_THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");

#[component]
pub fn Bootstrap() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DIOXUS_THEME_CSS }
        dioxus_tw_components::Bootstrap {}
    }
}
