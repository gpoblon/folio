use dioxus::prelude::*;

#[component]
pub fn Experience() -> Element {
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "15 15 18 18",
            circle {
                cx: "16.5",
                cy: "24",
                r: "1",
                fill: "currentColor",
            }
            circle {
                cx: "31.5",
                cy: "24",
                r: "1",
                fill: "currentColor",
            }
            circle {
                cx: "24",
                cy: "16.5",
                r: "1",
                fill: "currentColor",
            }
            circle {
                cx: "24",
                cy: "31.5",
                r: "1",
                fill: "currentColor",
            }
            path {
                d: "M24 24L16.5 24M24 24L31.5 24M24 24L24 16.5M24 24L24 31.5",
                stroke: "currentColor",
                stroke_width: "1",
            }
        }
    }
}
