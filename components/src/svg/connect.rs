use dioxus::prelude::*;
#[component]
pub fn Connect() -> Element {
    rsx! {
        svg { xmlns: "http://www.w3.org/2000/svg", view_box: "15 15 18 18",
            circle {
                cx: "21",
                cy: "24",
                r: "5.5",
                stroke: "currentColor",
                stroke_width: "1",
                fill: "none",
            }
            circle {
                cx: "27",
                cy: "24",
                r: "5.5",
                stroke: "currentColor",
                stroke_width: "1",
                fill: "none",
            }
        }
    }
}
