use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Experiences() -> Element {
    let experiences = super::model::Experiences::use_resource_experiences();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "experiences",
            h1 { class: "text-projects", "WORK EXPERIENCE" }
            ul {
                class: "experiences-list",
                for experience in experiences.jobs {
                    Experience { experience }
                }
            }
            h1 { class: "text-projects pt-15", "EDUCATION" }
            ul {
                class: "experiences-list",
                for experience in experiences.education {
                    Experience { experience }
                }
            }
        }
    }
}

#[component]
fn Experience(experience: super::model::Experience) -> Element {
    rsx! {
        div {
            class: "experience",
            div {
                class: "inline-content",
                h3 { "{experience.title}" }
                div {
                    class: "experience-details text-experience",
                    div {
                        class: "experience-details",
                        span { "{experience.start_date} " }
                        span { { t!("to") } }
                        span { " {experience.end_date}" }
                    }
                    span { class: "separator", " • " }
                    span { "{experience.location}" }
                    span { class: "separator", " • " }
                    span { "{experience.organization}" }
                }
            }
            div {
                class: "inline-content",
                p { class: "text-xl text-experience", "{experience.focus}" }
                if let Some(overview) = experience.overview {
                    p { class: "text-lg text-muted", "{overview}" }
                }
            }
            Achievements { achievements: experience.achievements, depth: 0 }
        }
    }
}

/// Recursively render achievements with depth tracking
/// Depth only serves style purpose
#[component]
fn Achievements(achievements: Vec<super::model::Achievement>, depth: u8) -> Element {
    let is_nested = depth > 0;
    rsx! {
        ul {
            class: if is_nested { "nested" } else { "achievements" },
            for achievement in achievements {
                Achievement { achievement, depth }
            }
        }
    }
}

#[component]
fn Achievement(achievement: super::model::Achievement, depth: u8) -> Element {
    rsx! {
        li {
            class: "achievement",
            if let Some(label) = achievement.label {
                p {
                    class: "achievement-label",
                    "{label}"
                }
            }
            div {
                class: "flex items-start gap-4",
                div {
                    class: "flex-1",
                    components::Markdown { content: achievement.description }
                }
                if let Some(link) = achievement.link {
                    a {
                        class: "outline-link text-2xl",
                        href: "{link}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        components::Icon {
                            class: "mb-0! -mt-2",
                            alt: "{link}",
                            icon: components::Icons::ArrowOutward
                        }
                    }
                }
            }

            // Recursively render sub-achievements
            Achievements { achievements: achievement.sub, depth: depth + 1 }
        }
    }
}
