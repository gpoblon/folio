use dioxus::prelude::*;
#[component]
pub fn ToggleTheme() -> Element {
    let mut theme = use_context::<Signal<kernel::theme::ThemeMode>>();
    let is_dark = theme() == kernel::theme::ThemeMode::Dark;
    let label = if is_dark {
        "Switch to light"
    } else {
        "Switch to dark"
    };
    rsx! {
        button {
            class: "size-10 text-primary border border-primary bg-primary p-2 flex items-center justify-center",
            aria_label: "{label}",
            onclick: move |_| theme.set(theme().toggle()),
            div { class: "size-5",
                if is_dark {
                    components::svg::Dark {}
                } else {
                    components::svg::Light {}
                }
            }
        }
    }
}
