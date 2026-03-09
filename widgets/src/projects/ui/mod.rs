mod skeleton;

use dioxus::{fullstack::Loading, prelude::*};

#[component]
pub fn ProjectGrid() -> Element {
    // Keep it in case we want to add a search bar back
    let search_query = use_signal(|| String::new());

    let project_list_metadata =
        use_loader(move || async { entities::project::api::projects().await });

    rsx! {
        div {
            ProjectGridHeader { search_query }
            match project_list_metadata {
                Err(Loading::Pending(_)) => rsx! {},
                Err(Loading::Failed(_)) => {
                    rsx! {
                        div {
                            class: "text-red-500",
                            "Failed to load projects"
                        }
                    }
                }
                Ok(metadata) => {
                    rsx! {
                        features::projects::FilteredProjectGrid {
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
#[component]
fn ProjectGridHeader(search_query: Signal<String>) -> Element {
    rsx! {
        div {
            class: "flex justify-between w-5xl mb-12",
            h1 { class: "text-projects", { kernel::lang::t!("projects") } }
            // For now, search bar does not make sense for projects
            // components::search::SearchBar {
            //     query: search_query,
            //     suggestions: vec![],
            //     placeholder: "Search for a project".to_string()
            // }
        }
    }
}
