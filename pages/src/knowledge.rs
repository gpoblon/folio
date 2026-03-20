use dioxus::prelude::*;
use serde_json::json;

#[component]
pub fn Knowledge() -> Element {
    rsx! {
        components::Seo {
            title: "Blog — Technical Articles on Rust & Software Engineering",
            description: "Deep-dive technical articles by Gaetan POBLON. Rust, Dioxus, Axum, WebAssembly, software architecture, and systems programming — no fluff, pure signal.",
            canonical_path: "/blog",
            og_type: "website",
            schema_type: "Blog",
            schema_keywords: vec![
                "Rust Blog".into(),
                "Dioxus Tutorial".into(),
                "Axum Web Framework".into(),
                "WebAssembly Guide".into(),
                "Software Architecture".into(),
                "Systems Programming".into(),
                "Fullstack Rust".into(),
                "Technical Writing".into(),
            ],
            schema_data: json!({
                "mainEntity": {
                    "@type": "ItemList",
                    "name": "Technical Articles",
                    "description": "A curated knowledge base of technical articles on Rust, Dioxus, and modern software engineering.",
                    "itemListOrder": "https://schema.org/ItemListOrderDescending"
                },
                "about": [
                    {"@type": "Thing", "name": "Rust Programming Language"},
                    {"@type": "Thing", "name": "Software Architecture"},
                    {"@type": "Thing", "name": "Web Development"}
                ]
            }),
        }
        section {
            class: "max-w-5xl mx-auto px-4 py-32 center-content",
            id: "blog",
            widgets::articles::ArticleGrid {}
        }
    }
}
