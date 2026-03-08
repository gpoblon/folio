use dioxus::prelude::*;

/// Shared heading section rendered at the top of both article and project pages.
///
/// Displays title, description, and a row of metadata.
///
/// Caller can insert
#[component]
pub fn MetadataHeader(
    meta: super::Metadata,
    title_color: &'static str,
    content_len: usize,
) -> Element {
    let (topics, _) = meta.slug.rsplit_once('/').unwrap_or(("/", ""));
    let topics = topics.to_string();

    // Assume an average silent reading speed of 200 WPM.
    // A word is avg 5 chars + 1 space.
    // 200 * 6 = 1200 chars per minute
    let average_reading_time_as_minutes = content_len / 1200;

    let created_at = meta
        .created
        .map(|date| date.format("%b %d, %Y").to_string());

    rsx! {
        h1 { class: "{title_color}", "{meta.title}" }
        p { class: "italic text-lg", "{meta.description}" }
        div {
            class: "flex items-baseline justify-between text-muted-foreground",
            p { class: "text-lg", "{topics}" }
            p { "{average_reading_time_as_minutes} min read" }
            div {
                class: "flex gap-1",
                if let Some(created_at) = created_at {
                    p { "{created_at}" }
                }
            }
        }
    }
}

/// Preview card used in both article and project grids.
///
/// Renders a uniform card with:
/// - A top row (left slot for topic/date, right slot for date).
/// - Title and description.
/// - A bottom row with language badge and tag badges.
///
/// The caller controls the link `href` and any extra top-row content via
/// `top_left` / `top_right`.
#[component]
pub fn MetadataPreview(meta: super::Metadata, href: String) -> Element {
    let updated_at = meta
        .modified
        .as_ref()
        .map(|date| date.format("%d.%m.%y").to_string());

    rsx! {
        a {
            class: "border border-border shadow-md px-4 py-3 flex flex-col gap-3 h-full bg-accent",
            href,
            div {
                class: "flex justify-between text-muted-foreground",
                if let Some((category, _)) = meta.slug.rsplit_once("/") {
                    p { class: "truncate", "{category}" }
                }
                if let Some(updated_at) = updated_at {
                    p { "{updated_at}" }
                }
            }
            h5 { class: "text-foreground text-left", "{meta.title}" }
            p { class: "italic text-left grow opacity-75", "{meta.description}" }
            div {
                class: "flex justify-between",
                p { class: "text-muted-foreground", "{meta.lang}" }
                div {
                    class: "flex gap-2",
                    for tag in meta.tags.iter().filter(|tag| !matches!(tag, crate::metadata::Intent::Other(_))) {
                        components::badge::Badge {
                            variant: components::badge::BadgeVariant::Outline,
                            "{tag}"
                        }
                    }
                    components::badge::Badge {
                        class: "border-blue",
                        variant: components::badge::BadgeVariant::Outline,
                        "{meta.expertise}"
                    }
                }
            }
        }
    }
}
