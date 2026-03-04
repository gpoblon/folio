use std::collections::BTreeSet;

use dioxus::prelude::*;
use entities::article::model::ArticleMetadata;

fn category_tree(metadata: &[ArticleMetadata]) -> Vec<String> {
    let mut categories = BTreeSet::new();
    categories.insert(String::from("/"));

    for meta in metadata.iter() {
        let mut current_slug = meta.slug.as_str();
        // Reassign `current_slug` until  it's empty
        while let Some((parent, _)) = current_slug.rsplit_once('/')
            && !parent.is_empty()
        {
            categories.insert(parent.to_owned());
            current_slug = parent;
        }
    }
    categories.into_iter().collect()
}

#[component]
pub fn ArticleGrid() -> Element {
    let article_list_metadata = use_loader(async move || entities::article::api::articles().await)?;

    // selected category
    let search_query = use_signal(|| String::from("/"));
    // category tree. e.g. [ / /rust /rust/borrow_checker /zig ... ]
    // Calculate categories in a Memo to avoid mutating state during the render pass
    let suggestions = use_memo(move || category_tree(&*article_list_metadata.read()));

    let queried_meta = use_memo(move || {
        article_list_metadata
            .read()
            .iter()
            .filter(|meta| meta.slug.contains(&*search_query.read()))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div {
            div {
                class: "flex justify-between w-5xl mb-12",
                h1 { class: "text-knowledge", "ARTICLES" }
                components::search::SearchBar {
                    query: search_query,
                    suggestions: suggestions(),
                    placeholder: "Search for a category".to_string()
                }
            }
            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                for meta in queried_meta() {
                    ArticlePreview { meta }
                }
            }
        }
    }
}

#[component]
fn ArticlePreview(meta: ArticleMetadata) -> Element {
    let (topics, _) = meta.slug.rsplit_once("/").unwrap_or(("/", ""));
    let updated_at = meta
        .modified
        .map(|date| date.format("%d.%m.%y").to_string());
    rsx! {
        a {
            class: "border border-border shadow-md px-4 py-3 flex flex-col gap-3 h-full bg-accent",
            // slug starts with `/`.
            // ! Do not add one to avoid double slashes in url which breaks routing
            href: "articles{meta.slug}",
            div {
                class: "flex justify-between",
                p { class: "text-muted-foreground", "{ topics }" }
                if let Some(updated_at) = updated_at {
                    p { class: "text-muted-foreground", "{ updated_at }" }
                }
            }
            h5 { class: "text-foreground text-left", "{ meta.title }" }
            p { class: "italic text-left grow opacity-75", "{ meta.description }" }
            div {
                class: "flex justify-between",
                p { class: "text-muted-foreground", "{ meta.lang }" }
                div {
                    class: "flex gap-2",
                    for tag in meta.tags {
                        components::badge::Badge {
                            variant: components::badge::BadgeVariant::Outline,
                            "{ tag }"
                        }
                    }
                }
            }
        }
    }
}
