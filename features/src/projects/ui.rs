use dioxus::prelude::*;
use entities::project::model::ProjectMetadata;

/// A grid of project previews (metadata, no content) filtered by search query.
#[component]
pub fn FilteredProjectGrid(
    metadata: Vec<ProjectMetadata>,
    search_query: Signal<String>,
) -> Element {
    let queried_meta = use_memo(move || {
        metadata
            .iter()
            .filter(|meta| meta.meta.slug.contains(&*search_query.read()))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
            for metadata in queried_meta() {
                entities::project::ProjectPreview { metadata }
            }
        }
    }
}
