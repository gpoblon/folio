use dioxus::prelude::*;

#[component]
pub fn Experience() -> Element {
    rsx! {
        components::Seo {
            title: "Experience — Professional Journey",
            description: "Professional experience and education of Gaetan POBLON, Software Engineer. Rustacean since 2019, specializing in FullStack Rust cross-platform applications.",
            canonical_path: "/experience",
            schema_type: "ProfilePage",
            schema_keywords: vec![
                "Rust".into(),
                "Dioxus".into(),
                "Axum".into(),
                "WebAssembly".into(),
                "Tokio".into(),
                "Software architecture".into(),
                "Cross-platform".into(),
                "Fullstack web development".into(),
            ],
        }
        section {
            class: "max-w-5xl mx-auto py-32 center-content",
            id: "experience",
            widgets::experiences::Experiences {}
        }
    }
}
