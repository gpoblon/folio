use dioxus::prelude::*;
use entities::article::model::ArticleMetadata;

/// A fully composed article-grid widget.
///
/// FSD: widgets layer — composes entity UI + feature filtering into a
/// self-contained block.  Data-fetching is **not** the widget's concern:
/// the caller (page layer) is responsible for loading `metadata` and
/// handling the loading / error states before passing data here.
#[component]
pub fn BlogGrid(metadata: Vec<ArticleMetadata>, search_query: Signal<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6",
            BlogGridHeader {
                search_query,
                suggestions: features::articles::category_tree(&metadata),
            }
            if metadata.len() < 8 {
                components::Callout {
                    variant: components::CalloutVariant::Experiment,
                    title: kernel::lang::t!("articles_coming_soon_title"),
                    p {{ kernel::lang::t!("articles_coming_soon_description") }}
                }
            }
            features::articles::FilteredArticleGrid {
                metadata,
                search_query,
            }
        }
    }
}

/// Header row: always visible regardless of loading state.
/// Owns the search bar so its input is never destroyed on re-render.
#[component]
fn BlogGridHeader(search_query: Signal<String>, suggestions: Vec<String>) -> Element {
    let lang = kernel::lang::use_lang();

    rsx! {
        div {
            class: "flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4 w-full mb-6",
            div {
                class: "flex items-center gap-3",
                h1 { class: "text-knowledge", "ARTICLES" }
                entities::metadata::IntentLegendIcon { lang }
            }
            components::search::SearchBar {
                query: search_query,
                suggestions,
                placeholder: "Search for a category".to_string()
            }
        }
    }
}
