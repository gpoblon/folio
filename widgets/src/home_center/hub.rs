use dioxus::prelude::*;

use entities::nav::Cardinal;

#[component]
pub fn Hub(active: Cardinal) -> Element {
    let color = active.color();

    rsx! {
        div {
            key: "{active.label()}",
            class: "absolute inset-0 m-auto pointer-events-none z-10 p-2
                    flex flex-col items-center justify-center text-center
                    w-[321px] h-[148px]
                    outline-[0.5px] outline-{color}
                    bg-accent",

            h2 {
                class: "text-lg md:text-xl font-light tracking-[0.3em] uppercase text-{color}",
                "{active.label()}"
            }

            hr {
                class: "w-12 min-h-0.5 h-0.5 mt-3 border-none opacity-60 bg-{color}",
            }

            p {
                class: "mt-3 text-[0.7rem] md:text-sm font-thin whitespace-pre-line tracking-[0.06em]",
                "{active.description()}"
            }
        }
    }
}
