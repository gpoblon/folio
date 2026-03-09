use dioxus::prelude::*;

use super::cardinal::Cardinal;
use super::hub::Hub;
use super::orbit::OrbitSvg;

#[component]
pub fn CenterCell(active: Cardinal) -> Element {
    let color = active.color();

    rsx! {
        div {
            class: "relative z-1 place-self-center aspect-square
                    size-[320px] md:size-[450px] lg:size-[520px] xl:size-[650px]",
            style: "grid-area:center",

            OrbitSvg {}

            // Cardinal-colored rotating squares
            div { class: "sq sq-experience" }
            div { class: "sq sq-projects" }

            // Geometry squares
            div { class: "sq sq-outer" }
            div { class: "sq sq-shrink bg-background" }
            div { class: "sq sq-inner backdrop-blur-xl bg-background/30 dark:bg-background/80 border border-{color}" }

            Hub { active }
        }
    }
}
