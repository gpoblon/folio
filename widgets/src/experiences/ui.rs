use dioxus::prelude::*;
use kernel::lang::t;

#[component]
pub fn Experiences() -> Element {
    let experiences = entities::experience::api::experiences();

    rsx! {
        div {
            class: "experiences",
            h1 { class: "text-experience text-left pb-12", { t!("experiences_work") } }
            ul {
                class: "experiences-list",
                for experience in &experiences.jobs {
                    features::experience::Experience { experience: experience.clone() }
                }
            }
            h1 { class: "text-experience text-left pt-22 pb-12", { t!("experiences_education") } }
            ul {
                class: "experiences-list",
                for experience in &experiences.education {
                    features::experience::Experience { experience: experience.clone() }
                }
            }
            features::resume::DownloadResume { class: "mt-22" }
        }
    }
}
