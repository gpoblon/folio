use dioxus::prelude::*;
use kernel::lang::t;

#[component]
pub fn Experiences() -> Element {
    let experiences = entities::experience::api::experiences();

    rsx! {
        div {
            class: "experiences",
            h1 { class: "text-experience text-left uppercase pb-12", { t!("experiences_introduction") } }
            components::Markdown { content: t!("experiences_introduction_content") }
            h1 { class: "text-experience text-left uppercase pt-22 pb-12", { t!("experiences_work") } }
            ul {
                class: "experiences-list",
                for experience in &experiences.jobs {
                    entities::experience::Experience { experience: experience.clone() }
                }
            }
            h1 { class: "text-experience text-left uppercase pt-22 pb-12", { t!("experiences_education") } }
            ul {
                class: "experiences-list",
                for experience in &experiences.education {
                    entities::experience::Experience { experience: experience.clone() }
                }
            }
            features::resume::DownloadResume { class: "mt-22" }
        }
    }
}
