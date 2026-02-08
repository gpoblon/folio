use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, PartialEq)]
pub struct TerminalOption {
    pub text: String,
    pub value: String,
}
#[derive(Clone, PartialEq)]
pub enum TerminalMode {
    Visual { text: String },
    Selection { options: Vec<TerminalOption> },
    Edit,
}
#[component]
fn Edit() -> Element {
    let mut input = use_signal(|| String::new());
    rsx! {
        span { "$> " }
        input {
            class: "bg-transparent border-0",
            value: "{input}",
            oninput: move |evt| {
                input.set(evt.value());
            },
        }
    }
}
#[component]
fn Visual(text: String) -> Element {
    rsx! {
        span { "$> " }
        TypewriterText { text: "{text}" }
    }
}
#[component]
fn Selection(options: Vec<TerminalOption>, mut selected: Signal<Option<String>>) -> Element {
    let mut focused_option_i = use_signal(|| 0usize);
    let opts = options;
    let opts_for_keys = opts.clone();
    let render_options = opts.iter().enumerate().map(|(idx, option)| {
        let option_value = option.value.clone();
        let option_text = option.text.clone();
        rsx! {
            li {
                key: "{idx}-{option_value}",
                "data-focused": focused_option_i() == idx,
                class: "cursor-pointer data-[focused=true]:font-bold",
                tabindex: 0,
                onclick: move |_| {
                    focused_option_i.set(idx);
                    selected.set(Some(option_value.clone()));
                },
                "{option_text}"
            }
        }
    });
    rsx! {
        div {
            tabindex: 0,
            autofocus: true,
            onkeydown: move |evt| {
                let key = evt.key();
                match key {
                    Key::ArrowDown => {
                        let i = focused_option_i();
                        if i + 1 < opts_for_keys.len() {
                            focused_option_i.set(i + 1);
                        }
                        evt.prevent_default();
                    }
                    Key::ArrowUp => {
                        let i = focused_option_i();
                        if i > 0 {
                            focused_option_i.set(i - 1);
                        }
                        evt.prevent_default();
                    }
                    Key::Enter => {
                        let i = focused_option_i();
                        if let Some(opt) = opts_for_keys.get(i) {
                            selected.set(Some(opt.value.clone()));
                        }
                        evt.prevent_default();
                    }
                    _ => {}
                }
            },
            span { "$> " }
            TypewriterText { text: "Please select an option:" }
            ul { {render_options} }
            if let Some(v) = selected().as_ref() {
                p { "{v}" }
            }
        }
    }
}
#[component]
pub fn Terminal(mode: TerminalMode) -> Element {
    let selected = use_signal(|| None);
    match mode {
        TerminalMode::Visual { text } => {
            rsx! {
                Visual { text }
            }
        }
        TerminalMode::Selection { options } => {
            rsx! {
                Selection { options, selected }
            }
        }
        TerminalMode::Edit => {
            rsx! {
                Edit {}
            }
        }
    }
}
static TYPEWRITER_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);
#[component]
pub fn TypewriterText(
    class: Option<String>,
    text: String,
    #[props(default = 84_u64)] speed_ms: u64,
) -> Element {
    let id = use_signal(|| TYPEWRITER_INSTANCE_ID.fetch_add(1, Ordering::Relaxed));
    let n = text.chars().count();
    let duration = n as u64 * speed_ms;
    let key_value = format!("{}-{}", id(), &text);
    rsx! {
        span {
            key: "{key_value}",
            class: format!(
                "font-mono inline-block whitespace-nowrap overflow-hidden [animation:typing_var(--dur)_steps(var(--steps))_forwards] {}",
                class.unwrap_or_default(),
            ),
            style: format!("--dur: {}ms; --steps: {};", duration, n),
            {text}
        }
    }
}
