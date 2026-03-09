use dioxus::prelude::*;

struct OrbitalDot {
    cx: &'static str,
    cy: &'static str,
    r: &'static str,
    opacity: &'static str,
    class: &'static str,
}

const ORBITAL_DOTS: [OrbitalDot; 6] = [
    OrbitalDot { cx: "458", cy: "94",  r: "4",   opacity: "0.85", class: "orbit-a" },
    OrbitalDot { cx: "516", cy: "215", r: "2.5", opacity: "0.6",  class: "orbit-b" },
    OrbitalDot { cx: "389", cy: "484", r: "1.5", opacity: "0.45", class: "orbit-c" },
    OrbitalDot { cx: "131", cy: "484", r: "3.5", opacity: "0.75", class: "orbit-d" },
    OrbitalDot { cx: "15",  cy: "305", r: "2",   opacity: "0.55", class: "orbit-e" },
    OrbitalDot { cx: "94",  cy: "62",  r: "1.2", opacity: "0.35", class: "orbit-f" },
];

#[component]
pub fn OrbitSvg() -> Element {
    rsx! {
        svg {
            class: "absolute inset-0 size-full pointer-events-none overflow-visible",
            view_box: "0 0 520 520",
            xmlns: "http://www.w3.org/2000/svg",

            defs {
                radialGradient {
                    id: "fade-gradient",
                    cx: "50%", cy: "50%", r: "50%",
                    stop { offset: "0%", "stop-color": "var(--color-border-muted)", "stop-opacity": "1" }
                    stop { offset: "100%", "stop-color": "var(--color-border-muted)", "stop-opacity": "0" }
                }
            }

            circle { cx: "260", cy: "260", r: "258", class: "circle-outer" }
            circle { cx: "260", cy: "260", r: "245", class: "circle-inner" }
            circle { cx: "260", cy: "260", r: "225", class: "circle-outer" }
            circle { cx: "260", cy: "260", r: "227", class: "circle-fade" }

            for dot in ORBITAL_DOTS.iter() {
                circle {
                    cx: "{dot.cx}",
                    cy: "{dot.cy}",
                    r: "{dot.r}",
                    opacity: "{dot.opacity}",
                    class: "dot {dot.class}",
                }
            }
        }
    }
}
