use dioxus::prelude::*;

#[component]
pub fn Knowledge() -> Element {
    rsx! {
        components::Seo {
            title: "Blog — Knowledge Base",
            description: "Technical articles by Gaetan POBLON. Deep-dives into Rust, Dioxus, software architecture, AI, and engineering practices. No AI, no fluff — pure signal.",
            canonical_path: "/blog",
            og_type: "blog",
            schema_type: "Blog",
            schema_keywords: vec![
                "Rust".into(),
                "Dioxus".into(),
                "WebAssembly".into(),
                "Software architecture".into(),
                "Artificial intelligence".into(),
                "Systems programming".into(),
                "Fullstack web development".into(),
            ],
        }
        section {
            class: "max-w-5xl mx-auto px-4 py-32 center-content",
            id: "blog",
            widgets::articles::ArticleGrid {}
        }
    }
}
