use dioxus::prelude::*;
#[component]
pub fn Projects() -> Element {
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "15 15 18 18",
            circle {
                cx: "24",
                cy: "24",
                r: "1",
                fill: "currentColor",
            }
            circle {
                cx: "24",
                cy: "24",
                r: "5",
                stroke: "currentColor",
                stroke_width: "1.2",
                fill: "none",
            }
            circle {
                cx: "24",
                cy: "24",
                r: "8.5",
                stroke: "currentColor",
                stroke_width: "1",
                opacity: "0.5",
                fill: "none",
            }
        }
    }
}
