use components::Seo;
use dioxus::prelude::*;
use entities::nav::{Cardinal, CardinalCell};
use kernel::seo::{AUTHOR_EMAIL, AUTHOR_GITHUB, AUTHOR_LINKEDIN, SITE_URL};
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
            description: "Hire a Rust Software Engineer based in France. Specializing in fullstack Dioxus, Axum, and WebAssembly for high-performance cross-platform applications.",
            canonical_path: "/",
            schema_type: "Person",
            schema_keywords: vec![
                "Hire Rust Developer".into(),
                "Rust Software Engineer".into(),
                "Fullstack Rust Developer".into(),
                "Dioxus Developer".into(),
                "Axum Backend Engineer".into(),
                "WebAssembly Expert".into(),
                "Cross-platform Rust".into(),
                "Software Architect".into(),
                "France".into(),
            ],
            schema_data: json!({
                "@id": format!("{}/#person", SITE_URL),
                "givenName": "Gaetan",
                "familyName": "POBLON",
                "email": AUTHOR_EMAIL,
                "nationality": "French",
                "knowsLanguage": ["English", "French"],
                "knowsAbout": [
                    "Rust Programming Language",
                    "Dioxus Framework",
                    "Axum Web Framework",
                    "WebAssembly",
                    "Software Architecture",
                    "Cross-platform Development",
                    "Artificial Intelligence",
                ],
                "sameAs": [
                    AUTHOR_GITHUB,
                    AUTHOR_LINKEDIN,
                ],
                "seeks": {
                    "@type": "Demand",
                    "itemOffered": {
                        "@type": "Service",
                        "name": "Rust Software Engineering",
                        "description": "Available for full-time positions or freelance contracts in Rust development — backend, fullstack, and cross-platform.",
                        "areaServed": "Worldwide",
                        "provider": {
                            "@type": "Person",
                            "@id": format!("{}/#person", SITE_URL),
                        }
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
