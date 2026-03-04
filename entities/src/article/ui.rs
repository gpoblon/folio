use dioxus::prelude::*;

/// Renders the full content of a single article.
/// Pure/dumb component: accepts data as props, performs no I/O or navigation.
#[component]
pub fn Article(article: super::model::Article) -> Element {
    let super::model::Article { metadata, content } = article;
    let super::model::ArticleMetadata {
        title,
        description,
        slug,
        created,
        modified,
        ..
    } = metadata;

    let (topics, _) = slug.rsplit_once('/').unwrap_or(("/", ""));
    let created_at = created.map(|date| date.format("%b %d, %Y").to_string());
    let updated_at = modified.map(|date| date.format("%b %d, %Y").to_string());

    // Assume an average silent reading speed of 200 WPM.
    // A word is avg 5 chars + 1 space.
    // 200 * 6 = 1200 chars per minute
    let average_reading_time_as_minutes = content.len() / 1200;

    rsx! {
        div {
            class: "space-y-8",
            h1 { class: "text-knowledge", "{title}" }
            p { class: "italic text-lg", "{description}" }
            div {
                class: "flex items-baseline justify-between text-muted-foreground",
                p { class: "text-lg", "{topics}" }
                p { "{average_reading_time_as_minutes} min read" }
                div {
                    class: "flex gap-1",
                    if let Some(created_at) = created_at {
                        p { class: "", "{created_at}" }
                    }
                    if let Some(updated_at) = updated_at {
                        p { "• Updated {updated_at}" }
                    }
                }
            }
            components::Separator { class: "py-4" }
            components::Markdown { content }
        }
    }
}
