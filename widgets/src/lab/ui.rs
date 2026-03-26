use dioxus::prelude::*;
use entities::project::model::ProjectMetadata;

/// A fully composed lab/project-grid widget.
///
/// FSD: widgets layer — composes entity UI + feature filtering into a
/// self-contained block.  Data-fetching is **not** the widget's concern:
/// the caller (page layer) is responsible for loading `metadata` and
/// handling the loading / error states before passing data here.
#[component]
pub fn LabGrid(metadata: Vec<ProjectMetadata>, search_query: Signal<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-6",
            LabGridHeader { search_query }
            if metadata.len() < 3 {
                components::Callout {
                    variant: components::CalloutVariant::Experiment,
                    title: kernel::lang::t!("projects_coming_soon_title"),
                    { kernel::lang::t!("projects_coming_soon_description") }
                }
            }
            features::projects::FilteredProjectGrid {
                metadata,
                search_query,
            }
        }
    }
}

/// Header row: always visible regardless of loading state.
#[component]
fn LabGridHeader(search_query: Signal<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4 w-full mb-6",
            h1 { class: "text-projects", "LAB" }
            // For now, search bar does not make sense for projects
            // components::search::SearchBar {
            //     query: search_query,
            //     suggestions: vec![],
            //     placeholder: "Search for a project".to_string()
            // }
        }
    }
}
