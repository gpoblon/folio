use dioxus::prelude::*;
use kernel::lang::{Lang, t, use_lang};

#[component]
pub fn DownloadResume(#[props(into, default)] class: String) -> Element {
    let lang = use_lang();
    let cv_asset = match lang {
        Lang::English => asset!("assets/cv-gpoblon-engineer-en.pdf"),
        Lang::French => asset!("assets/cv-gpoblon-engineer-fr.pdf"),
    };

    rsx! {
        a {
            href: cv_asset,
            download: "resume_gpoblon_engineer.pdf",
            class: "inline-flex items-center gap-4 px-6 py-4 border border-border bg-accent hover:var(--color-bg) transition {class}",
            components::Icon {
                icon: components::Icons::Download,
                class: "text-4xl"
            }
            span { class: "text-xl", { t!("download_cv") } }
        }
    }
}
