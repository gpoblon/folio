use dioxus::prelude::*;
use kernel::seo::SITE_URL;
use serde_json::json;

#[component]
pub fn Experience() -> Element {
    rsx! {
        components::Seo {
            title: "Experience — About me, Career & Resume",
            description: "Career path of Gaëtan POBLON, Rust Software Engineer since 2019. From scaling a 592-student campus as Head of Education & IT at 42 Angoulême to building a Rust compiler toolchain at Normation — spanning education technology, Infrastructure-as-Code, product management, and fullstack development with Dioxus, Axum, and WebAssembly.",
            canonical_path: "/experience",
            schema_type: "ProfilePage",
            schema_keywords: vec![
                "Gaëtan POBLON Resume".into(),
                "Gaetan Poblon Experience".into(),
                "Rust Software Engineer Career".into(),
                "Fullstack Rust Developer".into(),
                "42 School".into(),
                "Head of Education IT".into(),
                "Compiler Engineer Rust".into(),
                "Infrastructure as Code Developer".into(),
                "Dioxus Developer".into(),
                "Axum Backend Engineer".into(),
                "Software Architect France".into(),
                "Product Manager Engineer".into(),
                "Cross-platform Development".into(),
                "Education Technology".into(),
            ],
            // The full Person node (occupation, alumni, credentials, awards, …)
            // is already in the JSON-LD graph via author_node(). A compact @id
            // reference is sufficient to link this ProfilePage to it.
            schema_data: json!({
                "mainEntity": {
                    "@type": "Person",
                    "@id": format!("{SITE_URL}/#person"),
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
