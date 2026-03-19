use dioxus::prelude::*;

const SQUARES_CSS: Asset = asset!("./style.css");

/// Two cardinal-colored rotating squares + two geometry squares.
/// Pure decorative layer — no logic, no props.
#[component]
pub fn CenterSquares() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SQUARES_CSS }

        div { class: "absolute inset-0 m-auto pointer-events-none sq-experience" }
        div { class: "absolute inset-0 m-auto pointer-events-none sq-projects" }
        div { class: "absolute inset-0 m-auto pointer-events-none sq-outer" }
        div { class: "absolute inset-0 m-auto pointer-events-none sq-shrink bg-background" }
    }
}
