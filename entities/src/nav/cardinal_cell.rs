use dioxus::prelude::*;

use super::cardinal::Cardinal;

#[component]
pub fn CardinalCell(
    cardinal: Cardinal,
    is_active: bool,
    onhover: EventHandler<Option<Cardinal>>,
    onclick: EventHandler<Cardinal>,
) -> Element {
    let color = cardinal.color();
    let area = cardinal.area();
    let label = cardinal.label();
    let subtitle = cardinal.subtitle();
    let align_start = matches!(cardinal, Cardinal::Knowledge | Cardinal::Experience);

    rsx! {
        div {
            class: "cursor-pointer z-20 justify-self-stretch",
            style: "grid-area:{area}",
            onmouseenter: move |_| onhover.call(Some(cardinal)),
            onmouseleave: move |_| onhover.call(None),
            onclick: move |_| onclick.call(cardinal),

            // ── Desktop (md+): centered icon + label + optional subtitle ──
            div {
                class: "hidden md:flex items-center justify-center
                        p-12 justify-self-stretch self-stretch
                        transition-all duration-300 ease-in-out",
                class: if is_active { "opacity-100 scale-120" } else { "opacity-85" },

                div { class: "flex flex-col items-center gap-1",

                    div {
                        class: "flex items-center gap-3 px-3 py-2 border-2 border-{color}
                                transition-all duration-300 ease-in-out",
                        class: if is_active { "font-semibold" } else { "border-transparent" },

                        div { class: "text-{color} text-2xl", {cardinal.icon()} }

                        span {
                            class: "whitespace-nowrap font-mono font-medium uppercase
                                    text-2xl text-{color}",
                            "{label}"
                        }
                    }

                    if let Some(sub) = &subtitle {
                        span {
                            class: "text-xs text-{color}/70 font-light text-center max-w-48
                                    transition-opacity duration-300",
                            class: if is_active { "opacity-100" } else { "opacity-60" },
                            "{sub}"
                        }
                    }
                }
            }

            // ── Mobile (<md): burger-style button, alternating alignment ──
            button {
                class: "flex md:hidden my-1 py-4 items-center gap-3 w-full
                        border-2 border-{color} px-4
                        transition-all duration-200 ease-in-out",
                class: if align_start { "justify-start" } else { "justify-end" },
                class: if is_active { "bg-transparent text-{color}" } else { "bg-{color} text-black" },

                div {
                    class: "flex flex-col",
                    class: if align_start { "items-start" } else { "items-end" },

                    label {
                        class: "text-md font-mono font-medium uppercase cursor-pointer",
                        "{label}"
                    }

                    if let Some(sub) = &subtitle {
                        span {
                            class: "text-xs font-light mt-0.5",
                            class: if is_active { "text-{color}/70" } else { "text-black/70" },
                            "{sub}"
                        }
                    }
                }

                {cardinal.icon()}
            }
        }
    }
}
