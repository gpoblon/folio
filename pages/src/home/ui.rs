use components::Seo;
use dioxus::prelude::*;
use entities::nav::{Cardinal, CardinalCell};
use kernel::seo::{AUTHOR_GITHUB, AUTHOR_LINKEDIN, SITE_URL};
use serde_json::json;
use widgets::home_center::CenterCell;

const HOME_CSS: Asset = asset!("./home.css");

#[component]
pub fn Home() -> Element {
    let mut hovered: Signal<Option<Cardinal>> = use_signal(|| None);
    let nav = use_navigator();

    let active = hovered().unwrap_or(Cardinal::Identity);

    rsx! {
        Seo {
            title: "Gaetan POBLON — Rust Software Engineer",
            description: "Available for hire: Rust Software Engineer based in France. Expert in Dioxus, Axum, WebAssembly, Software Architecture and cross-platform applications.",
            canonical_path: "/",
            schema_type: "Person",
            schema_keywords: vec![
                "Rust".into(),
                "Dioxus".into(),
                "WebAssembly".into(),
                "Axum".into(),
                "Tokio".into(),
                "Fullstack web development".into(),
                "Software architecture".into(),
                "AI".into(),
                "Artificial Intelligence".into(),
                "Blockchain".into(),
                "Cross-platform".into(),
            ],
            schema_data: json!({
                "@id": format!("{}/#person", SITE_URL),
                "sameAs": [
                    AUTHOR_GITHUB,
                    AUTHOR_LINKEDIN,
                ],
                "seeks": {
                    "@type": "Demand",
                    "itemOffered": {
                        "@type": "Service",
                        "name": "Software Engineering Services",
                        "description": "Available for full-time roles or freelance contracts in Rust development including backend and fullstack."
                    }
                }
            }),
        }

        document::Link { rel: "stylesheet", href: HOME_CSS }

        section {
            id: "home",
            class: "home-grid flex-1 max-h-[calc(100dvh-9rem)]",

            for c in Cardinal::NAV.iter() {
                CardinalCell {
                    key: "{c.label()}",
                    cardinal: *c,
                    is_active: hovered() == Some(*c),
                    onhover: move |val| hovered.set(val),
                    onclick: move |c: Cardinal| { nav.push(c.route()); },
                }
            }

            CenterCell { active }
        }
    }
}
