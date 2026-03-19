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

    rsx! {
        div {
            class: "flex items-center justify-center cursor-pointer z-20
                    py-2 px-20 sm:justify-self-stretch sm:self-stretch",
            style: "grid-area:{area}",
            onmouseenter: move |_| onhover.call(Some(cardinal)),
            onmouseleave: move |_| onhover.call(None),
            onclick: move |_| onclick.call(cardinal),

            span {
                class: "whitespace-nowrap font-mono font-medium uppercase
                        text-{color} text-lg sm:text-2xl
                        border-2 border-transparent py-1.5 px-2
                        transition-all duration-300 ease-in-out",
                class: if is_active { "opacity-100 font-semibold scale-120 !border-current" } else { "opacity-85" },
                "{cardinal.label()}"
            }
        }
    }
}
