use dioxus::prelude::*;

struct Dot {
    size: u8,
    top: &'static str,
    left: &'static str,
    opacity: f32,
    animation: &'static str,
}

const FAR_DOTS: [Dot; 10] = [
    Dot {
        size: 7,
        top: "8%",
        left: "85%",
        opacity: 0.50,
        animation: "drift-1 55s cubic-bezier(.45,.05,.55,.95) infinite",
    },
    Dot {
        size: 5,
        top: "78%",
        left: "10%",
        opacity: 0.40,
        animation: "drift-2 38s cubic-bezier(.6,0,.4,1) infinite",
    },
    Dot {
        size: 3,
        top: "15%",
        left: "12%",
        opacity: 0.30,
        animation: "drift-3 95s cubic-bezier(.3,0,.7,1) infinite",
    },
    Dot {
        size: 6,
        top: "88%",
        left: "72%",
        opacity: 0.35,
        animation: "drift-4 62s cubic-bezier(.5,0,.5,1) infinite",
    },
    Dot {
        size: 4,
        top: "45%",
        left: "92%",
        opacity: 0.45,
        animation: "drift-5 28s cubic-bezier(.4,0,.6,1) infinite",
    },
    Dot {
        size: 5,
        top: "5%",
        left: "50%",
        opacity: 0.50,
        animation: "drift-6 72s cubic-bezier(.35,.05,.65,.95) infinite",
    },
    Dot {
        size: 3,
        top: "92%",
        left: "45%",
        opacity: 0.38,
        animation: "drift-7 48s cubic-bezier(.5,.1,.5,.9) infinite",
    },
    Dot {
        size: 6,
        top: "50%",
        left: "2%",
        opacity: 0.55,
        animation: "drift-8 85s cubic-bezier(.45,0,.55,1) infinite",
    },
    Dot {
        size: 4,
        top: "22%",
        left: "95%",
        opacity: 0.42,
        animation: "drift-9 63s cubic-bezier(.4,.05,.6,.95) infinite",
    },
    Dot {
        size: 5,
        top: "78%",
        left: "88%",
        opacity: 0.48,
        animation: "drift-10 41s cubic-bezier(.55,0,.45,1) infinite",
    },
];

#[component]
pub fn Diagonals() -> Element {
    rsx! {
        svg {
            class: "absolute inset-0 size-full pointer-events-none z-0",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "none",
            xmlns: "http://www.w3.org/2000/svg",
            line { x1: "0", y1: "0", x2: "100", y2: "100", class: "diag-line" }
            line { x1: "100", y1: "0", x2: "0", y2: "100", class: "diag-line" }
        }
    }
}

#[component]
pub fn OrbitDots() -> Element {
    rsx! {
        div { class: "absolute inset-0 pointer-events-none z-0 overflow-visible",
            for (i, dot) in FAR_DOTS.iter().enumerate() {
                div {
                    key: "{i}",
                    class: "absolute rounded-full bg-border-muted",
                    style: "width:{dot.size}px;height:{dot.size}px;top:{dot.top};left:{dot.left};opacity:{dot.opacity};animation:{dot.animation}",
                }
            }
        }
    }
}
