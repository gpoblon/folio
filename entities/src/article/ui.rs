use dioxus::prelude::*;

/// Renders a single article (full page)
#[component]
pub fn Article(slug: String) -> Element {
    let nav = use_navigator();

    match super::model::use_resource_article(slug)() {
        // Request is pending
        None => rsx! {
            div { class: "p-4 text-gray-500", "Loading article..." }
        },
        // The request finished, but the article was not found
        Some(None) => {
            if nav.can_go_back() {
                nav.go_back();
            } else {
                nav.push("/articles");
            }
            rsx! { div { "Redirecting..." } }
        }
        // Request finished successfully
        Some(Some(article)) => rsx! {
            div {
                class: "bg-white rounded-lg shadow-md p-4",

                // TODO: Render article metadata here (article.meta.title, etc.)

                components::Markdown {
                    content: article.content
                }
            }
        },
    }
}
