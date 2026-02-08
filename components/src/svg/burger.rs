use dioxus::prelude::*;

#[component]
pub fn Burger() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line {
                x1: "3",
                y1: "6",
                x2: "21",
                y2: "6",
            }
            line {
                x1: "3",
                y1: "12",
                x2: "21",
                y2: "12",
            }
            line {
                x1: "3",
                y1: "18",
                x2: "21",
                y2: "18",
            }
        }
    }
}
