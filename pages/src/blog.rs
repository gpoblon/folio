use dioxus::{fullstack::Loading, prelude::*};
use serde_json::json;

#[component]
pub fn Blog() -> Element {
    let search_query = use_signal(String::new);

    let metadata = use_loader(move || async { entities::article::api::articles().await });

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
