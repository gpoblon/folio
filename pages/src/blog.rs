use dioxus::{fullstack::Loading, prelude::*};
use serde_json::json;

#[component]
pub fn Blog() -> Element {
    let search_query = use_signal(String::new);

    let metadata = use_loader(move || async { entities::article::api::articles().await });

    rsx! {
        components::Seo {
            title: "Blog — Rust & Software Engineering Articles",
            description: "Technical deep-dive articles by Gaëtan POBLON on Rust, Dioxus, Axum, WebAssembly, software architecture, Domain-Driven Design, compiler internals, and modern engineering practices. Practical insights from building production Rust systems.",
            canonical_path: "/blog",
            og_type: "website",
            schema_type: "Blog",
            schema_keywords: vec![
                "Rust Programming Blog".into(),
                "Rust lang Articles".into(),
                "Idiomatic Rust Programming".into(),
                "Dioxus GUI Development".into(),
                "Axum Backend API".into(),
                "Software Architecture".into(),
                "Design Patterns".into(),
                "Domain-Driven Design".into(),
                "Compiler Internals".into(),
                "Web Development".into(),
                "Cross-platform Native Development".into(),
                "WebAssembly".into(),
                "Asynchronous Programming".into(),
                "Infrastructure as Code".into(),
                "DevOps".into(),
                "Systems Programming".into(),
                "Artificial Intelligence".into(),
                "Technical Writing".into(),
            ],
            schema_data: json!({
                "mainEntity": {
                    "@type": "ItemList",
                    "name": "Blog — Rust & Software Engineering Articles",
                    "description": "A curated knowledge base of technical articles on Rust, Dioxus, Axum, software architecture, and modern engineering practices.",
                    "itemListOrder": "https://schema.org/ItemListOrderDescending"
                },
                "about": [
                    {"@type": "ComputerLanguage", "name": "Rust Programming Language"},
                    {"@type": "SoftwareApplication", "name": "Dioxus Framework"},
                    {"@type": "SoftwareApplication", "name": "Axum Web Framework"},
                    {"@type": "DefinedTerm", "name": "Software Architecture"},
                    {"@type": "DefinedTerm", "name": "Domain-Driven Design"},
                    {"@type": "DefinedTerm", "name": "WebAssembly"}
                ]
            }),
        }
        section {
            class: "max-w-5xl mx-auto px-4 py-32 center-content",
            id: "blog",
            match metadata {
                Err(Loading::Pending(_)) => {
                    rsx! { components::GridSkeleton {} }
                }
                Err(Loading::Failed(_)) => {
                    components::toast::use_toast()
                        .error(kernel::lang::t!("article_list_metadata_error"))
                        .send();
                    rsx! {
                        components::Callout {
                            variant: components::CalloutVariant::Caution,
                            title: kernel::lang::t!("article_list_metadata_error"),
                            p { { kernel::lang::t!("article_list_metadata_error") } }
                        }
                    }
                }
                Ok(metadata) => {
                    rsx! {
                        widgets::blog::BlogGrid {
                            metadata: metadata(),
                            search_query,
                        }
                    }
                }
            }
        }
    }
}
