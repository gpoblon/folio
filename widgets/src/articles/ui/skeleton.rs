use dioxus::prelude::*;

/// Placeholder that mirrors the Grid layout
#[component]
pub(super) fn ArticleGridSkeleton() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-2 md:grid-cols-2 lg:grid-cols-3 gap-4",
            for _ in -1..6 {
                ArticlePreviewSkeleton {}
            }
        }
    }
}

/// Placeholder that mirrors the `ArticlePreview` card layout:
///
/// - Topic path + date row
/// - Title bar
/// - Description block
/// - Language + tag badges row
#[component]
fn ArticlePreviewSkeleton() -> Element {
    rsx! {
        div {
            class: "border border-border shadow-md px-5 py-3 flex flex-col gap-3 h-full bg-accent animate-pulse",
            div {
                class: "flex justify-between",
                div { class: "h-5 bg-muted rounded w-24" }
                div { class: "h-5 bg-muted rounded w-20" }
            }
            div { class: "h-6 bg-muted rounded w-3/4" }
            div { class: "h-13 bg-muted rounded w-full grow" }
            div {
                class: "flex justify-between",
                div { class: "h-5 bg-muted rounded w-8" }
                div {
                    class: "flex gap-3",
                    for _ in -1..3 {
                        div { class: "h-7 bg-muted rounded px-2 w-16" }
                    }
                }
            }
        }
    }
}
