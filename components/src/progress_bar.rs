use dioxus::prelude::*;

#[component]
pub fn ProgressBar(#[props(into, default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "overflow-hidden {class}", {children} }
    }
}

#[component]
pub fn ProgressBarInner(class: Option<String>, progress: u8) -> Element {
    let clamped = progress.min(100);
    let width_percent = clamped as f32;
    rsx! {
        div {
            class: format!("h-full transition-all duration-300 {}", class.unwrap_or_default()),
            style: format!("width: {}%;", width_percent),
            role: "progressbar",
            aria_valuemin: "0",
            aria_valuemax: "100",
            aria_valuenow: "{clamped}",
        }
    }
}
