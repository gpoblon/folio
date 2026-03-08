use dioxus::prelude::*;
use entities::{article::model::ArticleMetadata, metadata::Metadata};
use std::collections::BTreeSet;

pub fn category_tree(metadata: &[ArticleMetadata]) -> Vec<String> {
    let mut categories = BTreeSet::new();
    for meta in metadata.iter() {
        let mut current_slug = meta.slug.as_str();
        while let Some((parent, _)) = current_slug.rsplit_once('/')
            && !parent.is_empty()
        {
            categories.insert(parent.to_owned());
            current_slug = parent;
        }
    }
    categories.into_iter().collect()
}

/// A grid of previews (metadata, no content) filtered by search query.
#[component]
pub fn FilteredArticleGrid(metadata: Vec<Metadata>, search_query: Signal<String>) -> Element {
    let queried_meta = use_memo(move || {
        metadata
            .iter()
            .filter(|meta| meta.slug.contains(&*search_query.read()))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
            for meta in queried_meta() {
                entities::article::ArticlePreview { meta }
            }
        }
    }
}
