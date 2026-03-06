use dioxus::prelude::*;

use super::model::ArticleMetadata;

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

/// A single article preview card.
/// Pure/dumb component: accepts data as props, performs no I/O.
#[component]
pub fn ArticlePreview(meta: ArticleMetadata) -> Element {
    let (topics, _) = meta.slug.rsplit_once('/').unwrap_or(("/", ""));
    let updated_at = meta
        .modified
        .map(|date| date.format("%d.%m.%y").to_string());
    rsx! {
        a {
            class: "border border-border shadow-md px-4 py-3 flex flex-col gap-3 h-full bg-accent",
            href: "/articles/{meta.slug}",
            div {
                class: "flex justify-between",
                p { class: "text-muted-foreground", "{topics}" }
                if let Some(updated_at) = updated_at {
                    p { class: "text-muted-foreground", "{updated_at}" }
                }
            }
            h5 { class: "text-foreground text-left", "{meta.title}" }
            p { class: "italic text-left grow opacity-75", "{meta.description}" }
            div {
                class: "flex justify-between",
                p { class: "text-muted-foreground", "{meta.lang}" }
                div {
                    class: "flex gap-2",
                    for tag in meta.tags {
                        components::badge::Badge {
                            variant: components::badge::BadgeVariant::Outline,
                            "{tag}"
                        }
                    }
                }
            }
        }
    }
}
