use dioxus::prelude::*;
use kernel::lang::t;

use super::model;

#[component]
pub fn Experience(experience: model::Experience) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "experience text-justify space-y-3",
            div {
                class: "inline-content",
                h3 { "{experience.title}" }
                div {
                    class: "experience-details",
                    div {
                        class: "experience-details",
                        span { "{experience.start_date} " }
                        span { { t!("to") } }
                        span { " {experience.end_date}" }
                    }
                    span { " • " }
                    span { "{experience.location}" }
                    span { " • " }
                    span { "{experience.organization}" }
                }
            }
            p { class: "text-xl text-experience", "{experience.focus}" }
            if let Some(overview) = experience.overview {
                p { class: "text-muted-foreground text-lg", "{overview}" }
            }
            Achievements { achievements: experience.achievements, depth: 0 }
        }
    }
}

/// Recursively render achievements with depth tracking.
/// Depth only serves a style purpose.
#[component]
fn Achievements(achievements: Vec<model::Achievement>, depth: u8) -> Element {
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
fn Achievement(achievement: model::Achievement, depth: u8) -> Element {
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
                        class: "outline-link text-2xl link",
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
            Achievements { achievements: achievement.sub, depth: depth + 1 }
        }
    }
}
