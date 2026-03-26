use dioxus::{fullstack::Loading, prelude::*};
use serde_json::json;

#[component]
pub fn Lab() -> Element {
    let search_query = use_signal(String::new);

    let metadata = use_loader(move || async { entities::project::api::projects().await });

    rsx! {
        components::Seo {
            title: "Lab — Open-Source Rust Projects",
            description: "Open-source Rust projects by Gaëtan POBLON — production-grade applications, developer tools, UI component libraries, and cross-platform experiments built with Dioxus, Axum, WebAssembly, and Domain-Driven Design.",
            canonical_path: "/lab",
            schema_type: "CollectionPage",
            schema_keywords: vec![
                "Open Source Rust Projects".into(),
                "Dioxus Applications".into(),
                "Axum Web Services".into(),
                "WebAssembly Projects".into(),
                "Rust Developer Portfolio".into(),
                "Cross-Platform Software".into(),
                "Fullstack Rust".into(),
                "Rust UI Component Library".into(),
                "Rust Developer Tools".into(),
                "SurrealDB Projects".into(),
            ],
            schema_data: json!({
                "mainEntity": {
                    "@type": "ItemList",
                    "name": "Lab — Open-Source Rust Projects by Gaëtan POBLON",
                    "description": "A curated collection of production-grade Rust projects spanning fullstack web applications, cross-platform developer tools, UI component libraries, and software experiments built with Dioxus, Axum, and WebAssembly.",
                    "itemListOrder": "https://schema.org/ItemListUnordered"
                }
            }),
        }
        section {
            class: "max-w-5xl mx-auto px-4 py-32 center-content",
            id: "lab",
            match metadata {
                Err(Loading::Pending(_)) => {
                    rsx! { components::GridSkeleton {} }
                }
                Err(Loading::Failed(_)) => {
                    components::toast::use_toast()
                        .error(kernel::lang::t!("project_list_metadata_error"))
                        .send();
                    rsx! {
                        components::Callout {
                            variant: components::CalloutVariant::Caution,
                            title: kernel::lang::t!("project_list_metadata_error"),
                            p { { kernel::lang::t!("project_list_metadata_error") } }
                        }
                    }
                }
                Ok(metadata) => {
                    rsx! {
                        widgets::lab::LabGrid {
                            metadata: metadata(),
                            search_query,
                        }
                    }
                }
            }
        }
    }
}
