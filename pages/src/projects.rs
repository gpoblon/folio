use components::progress_bar::{ProgressBar, ProgressBarInner};
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let progress = use_signal(|| 22);
    rsx! {
        section { id: "projects", class: "space-y-8",
            h1 { class: "text-3xl", "My projects" }
            div { class: "space-y-4",
                ProgressBar { class: "w-full bg-muted h-3",
                    ProgressBarInner { class: "bg-projects", progress: progress() }
                }
                p { class: "text-sm text-muted", "{progress()}% complete" }
            }
        }
    }
}
