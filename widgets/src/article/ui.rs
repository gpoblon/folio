use dioxus::{fullstack::Loading, prelude::*};

#[component]
pub fn Article(slug: Vec<String>) -> Element {
    let article = use_loader(move || {
        // explicitly add back the leading slash as the url loses it
        let slug = format!("/{}", slug.join("/"));
        async move { entities::article::api::article(slug).await }
    });
    let nav = use_navigator();

    match article {
        // Request finished but article was not found: navigate away gracefully.
        Err(Loading::Failed(_)) => {
            if nav.can_go_back() {
                nav.go_back();
            } else {
                nav.push("/knowledge");
            }
            rsx! { div { "Redirecting..." } }
        }
        // Still in-flight: bubble the pending suspension up to the nearest SuspenseBoundary.
        Err(Loading::Pending(pending)) => Err(Loading::Pending(pending).into()),
        // Data ready: hand off to the dumb ArticleView.
        Ok(article) => {
            rsx! {
                entities::article::Article { article: article() }
            }
        }
    }
}
