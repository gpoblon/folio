mod skeleton;

use dioxus::{fullstack::Loading, prelude::*};

/// Smart article widget that owns its own loading & error states.
///
/// FSD: widgets layer — composes entity UI with data-fetching and
/// provides a self-contained loading skeleton so the page layer
/// doesn't need to know about article internals.
#[component]
pub fn Article(slug: Vec<String>) -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { skeleton::ArticleSkeleton {} },
            ArticleLoader { slug }
        }
    }
}

/// Inner component that performs the actual data fetch.
/// Separated so `SuspenseBoundary` above can catch the pending state.
#[component]
fn ArticleLoader(slug: Vec<String>) -> Element {
    let article = use_loader(move || {
        let slug = slug.join("/");
        async move { entities::article::api::article(slug).await }
    });
    let nav = use_navigator();

    match article {
        Err(Loading::Failed(_)) => {
            if nav.can_go_back() {
                nav.go_back();
            } else {
                nav.push("/knowledge");
            }
            rsx! { div { "Redirecting..." } }
        }
        Err(Loading::Pending(pending)) => Err(Loading::Pending(pending).into()),
        Ok(article) => {
            rsx! {
                entities::article::Article { article: article() }
            }
        }
    }
}
