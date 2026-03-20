use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        components::Seo {
            title: "Projects — Lab",
            description: "Curated selection of open-source projects by Gaetan POBLON: functional prototypes, ongoing experiments, and archived work in Rust, Dioxus, and WebAssembly.",
            canonical_path: "/lab",
            schema_type: "CollectionPage",
            schema_keywords: vec![
                "Rust".into(),
                "Dioxus".into(),
                "WebAssembly".into(),
                "Axum".into(),
                "Open source".into(),
                "Cross-platform".into(),
                "Fullstack web development".into(),
            ],
        }
        section {
            class: "max-w-5xl mx-auto px-4 py-32 center-content",
            id: "projects",
            widgets::projects::ProjectGrid {}
        }
    }
}
