mod skeleton;

use dioxus::{fullstack::Loading, prelude::*};

#[component]
pub fn Project(slug: String) -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { skeleton::ProjectSkeleton {} },
            ProjectLoader { slug }
        }
    }
}

/// Inner component that performs the actual data fetch.
/// Separated so `SuspenseBoundary` above can catch the pending state.
#[component]
fn ProjectLoader(slug: String) -> Element {
    let project = use_loader(move || {
        let slug = slug.clone();
        async move { entities::project::api::project(slug).await }
    });
    let nav = use_navigator();

    match project {
        Err(Loading::Failed(_)) => {
            if nav.can_go_back() {
                nav.go_back();
            } else {
                nav.push("/projects");
            }
            rsx! { div { "Redirecting..." } }
        }
        Err(Loading::Pending(pending)) => Err(Loading::Pending(pending).into()),
        Ok(project) => {
            rsx! {
                entities::project::Project { project: project() }
            }
        }
    }
}
