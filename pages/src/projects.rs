use dioxus::prelude::*;
use serde_json::json;

#[component]
pub fn Projects() -> Element {
    rsx! {
        components::Seo {
            title: "Open-Source Projects",
            description: "Open-source Rust projects by Gaetan POBLON — production-grade prototypes, developer tools, and experiments in Dioxus, Axum, and WebAssembly.",
            canonical_path: "/lab",
            schema_type: "CollectionPage",
            schema_keywords: vec![
                "Open Source Rust Projects".into(),
                "Dioxus Applications".into(),
                "Axum Web Services".into(),
                "WebAssembly".into(),
                "Rust Developer Portfolio".into(),
                "Cross-Platform Software".into(),
                "Fullstack Rust".into(),
            ],
            schema_data: json!({
                "mainEntity": {
                    "@type": "ItemList",
                    "name": "Open-Source Projects by Gaetan POBLON",
                    "description": "A curated collection of Rust-based open-source projects spanning web applications, developer tools, and cross-platform software.",
                    "itemListOrder": "https://schema.org/ItemListUnordered"
                }
            }),
        }
        section {
            class: "max-w-5xl mx-auto px-4 py-32 center-content",
            id: "projects",
            widgets::projects::ProjectGrid {}
        }
    }
}
