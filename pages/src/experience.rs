use dioxus::prelude::*;
use serde_json::json;

#[component]
pub fn Experience() -> Element {
    rsx! {
        components::Seo {
            title: "Experience",
            description: "Career path of Gaetan POBLON — Rust Software Engineer since 2019. Fullstack development with Dioxus, Axum, and WebAssembly. Background in fintech, music-tech, and SaaS platforms.",
            canonical_path: "/experience",
            schema_type: "ProfilePage",
            schema_keywords: vec![
                "Gaetan Poblon Resume".into(),
                "Rust Developer Experience".into(),
                "Fullstack Software Engineer".into(),
                "Dioxus Developer".into(),
                "Axum Backend Engineer".into(),
                "WebAssembly Engineer".into(),
                "Software Architect".into(),
                "France".into(),
            ],
            schema_data: json!({
                "mainEntity": {
                    "@type": "Person",
                    "@id": "https://gpoblon.net/#person",
                    "hasOccupation": {
                        "@type": "Occupation",
                        "name": "Rust Software Engineer",
                        "occupationLocation": {
                            "@type": "Country",
                            "name": "France"
                        },
                        "skills": "Rust, Dioxus, Axum, WebAssembly, Software Architecture, Cross-platform Development"
                    },
                    "knowsAbout": [
                        "Rust Programming Language",
                        "Fullstack Web Development",
                        "Cross-platform Applications",
                        "Software Architecture",
                        "WebAssembly",
                        "Distributed Systems"
                    ]
                }
            }),
        }
        section {
            class: "max-w-5xl mx-auto py-32 center-content",
            id: "experience",
            widgets::experiences::Experiences {}
        }
    }
}
