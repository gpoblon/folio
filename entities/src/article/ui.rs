use dioxus::{fullstack::Loading, prelude::*};

/// Renders a single article (full page)
#[component]
pub fn Article(slug: Vec<String>) -> Element {
    let article = use_loader(move || {
        let slug = slug.join("/");
        async move { super::api::article(slug).await }
    });
    let nav = use_navigator();

    match article {
        // The request finished, but the article was not found
        Err(Loading::Failed(_)) => {
            if nav.can_go_back() {
                nav.go_back();
            } else {
                nav.push("/articles");
            }
            rsx! { div { "Redirecting..." } }
        }
        // Still pending, bubble up the suspense
        Err(Loading::Pending(pending)) => Err(Loading::Pending(pending).into()),
        // Request finished successfully
        Ok(article) => {
            let super::model::Article { metadata, content } = article();
            let (topics, _) = metadata.slug.rsplit_once("/").unwrap_or(("/", ""));
            let created_at = metadata
                .created
                .map(|date| date.format("%b %d, %Y").to_string());
            let updated_at = metadata
                .modified
                .map(|date| date.format("%b %d, %Y").to_string());

            rsx! {
                h1 { class: "text-projects", "{ metadata.title }" }
                p { class: "italic text-muted text-lg", "{ metadata.description }" }
                div {
                    class: "flex",
                    p { class: "text-muted text-lg grow", "{ topics }" }
                    if let Some(created_at) = created_at {
                        p { class: "text-muted", "{ created_at }" }
                    }
                    if let Some(updated_at) = updated_at {
                        p { class: "text-muted pl-1 italic", "• Updated { updated_at }" }
                    }
                }
                components::Separator { class: "py-4" }
                components::Markdown { content }
            }
        }
    }
}
