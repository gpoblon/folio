mod skeleton;

use dioxus::{fullstack::Loading, prelude::*};

#[component]
pub fn ArticleGrid() -> Element {
    let search_query = use_signal(|| String::new());

    let article_list_metadata =
        use_loader(move || async { entities::article::api::articles().await });

    rsx! {
        div {
            match article_list_metadata {
                Err(Loading::Pending(_)) => {
                    rsx! {
                        ArticleGridHeader { search_query, suggestions: vec![] }
                        skeleton::ArticleGridSkeleton {}
                    }
                }
                Err(Loading::Failed(_)) => {
                    rsx! {
                        ArticleGridHeader { search_query, suggestions: vec![] }
                        div {
                            class: "text-red-500",
                            "Failed to load articles"
                        }
                    }
                }
                Ok(metadata) => {
                    rsx! {
                        ArticleGridHeader {
                            search_query,
                            suggestions: features::articles::category_tree(&metadata()),
                        }
                        features::articles::FilteredArticleGrid {
                            metadata: metadata(),
                            search_query,
                        }
                    }
                }
            }
        }
    }
}

/// Header row: always visible regardless of loading state.
/// Owns the search bar so its input is never destroyed on re-render.
#[component]
fn ArticleGridHeader(search_query: Signal<String>, suggestions: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex justify-between w-5xl mb-12",
            h1 { class: "text-knowledge", "ARTICLES" }
            components::search::SearchBar {
                query: search_query,
                suggestions,
                placeholder: "Search for a category".to_string()
            }
        }
    }
}
