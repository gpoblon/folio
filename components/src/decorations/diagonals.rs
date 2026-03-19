use dioxus::prelude::*;

#[component]
pub fn Diagonals() -> Element {
    rsx! {
        svg {
            class: "absolute inset-0 size-full pointer-events-none z-0",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "none",
            xmlns: "http://www.w3.org/2000/svg",
            line {
                x1: "0", y1: "0", x2: "100", y2: "100",
                stroke: "var(--color-border-muted)",
                opacity: "0.35",
                stroke_width: "1",
                vector_effect: "non-scaling-stroke",
            }
            line {
                x1: "100", y1: "0", x2: "0", y2: "100",
                stroke: "var(--color-border-muted)",
                opacity: "0.35",
                stroke_width: "1",
                vector_effect: "non-scaling-stroke",
            }
        }
    }
}
