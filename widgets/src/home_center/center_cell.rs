use dioxus::prelude::*;

use entities::nav::Cardinal;

use components::decorations::{CenterSquares, Diagonals, OrbitDots, OrbitSvg};

use super::hub::Hub;

#[component]
pub fn CenterCell(active: Cardinal) -> Element {
    let color = active.color();

    rsx! {
        div {
            class: "relative z-1 place-self-center aspect-square
                    size-[320px] md:size-[450px] lg:size-[520px] xl:size-[650px]",
            style: "grid-area:center",

            OrbitSvg {}
            CenterSquares {}

            // Inner diamond with dynamic cardinal border color
            div {
                class: "absolute inset-0 m-auto pointer-events-none
                        size-1/2 rotate-45 border-[0.5px]
                        backdrop-blur-xl bg-background/30 dark:bg-background/70 border-{color}",
            }

            Hub { active }
        }

        Diagonals {}
        OrbitDots {}
    }
}
