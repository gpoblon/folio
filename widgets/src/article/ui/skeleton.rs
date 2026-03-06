use dioxus::prelude::*;

/// A placeholder that mirrors the real article layout:
///
/// - Title bar
/// - Description line
/// - Metadata row (topics · reading time · dates)
/// - Separator
/// - Body paragraphs (varied widths for a natural look)
#[component]
pub(super) fn ArticleSkeleton() -> Element {
    rsx! {
        div {
            class: "space-y-8 animate-pulse min-h-[60vh]",
            // Title
            div { class: "h-9 w-3/5 bg-muted rounded" }
            // Description
            div { class: "h-5 w-4/5 bg-muted rounded" }
            // Metadata row
            div {
                class: "flex items-baseline justify-between",
                div { class: "h-4 w-1/6 bg-muted rounded" }
                div { class: "h-4 w-20 bg-muted rounded" }
                div {
                    class: "flex gap-1",
                    div { class: "h-4 w-24 bg-muted rounded" }
                    div { class: "h-4 w-28 bg-muted rounded" }
                }
            }
            // Separator
            div { class: "py-4",
                div { class: "h-px w-full bg-border" }
            }
            // Body paragraphs – varied widths for a natural feel
            div {
                class: "space-y-4",
                div { class: "h-4 w-full bg-muted rounded" }
                div { class: "h-4 w-11/12 bg-muted rounded" }
                div { class: "h-4 w-4/5 bg-muted rounded" }
                div { class: "h-4 w-full bg-muted rounded" }
                div { class: "h-4 w-3/4 bg-muted rounded" }
            }
            // Simulated second paragraph block
            div {
                class: "space-y-4 pt-2",
                div { class: "h-4 w-full bg-muted rounded" }
                div { class: "h-4 w-5/6 bg-muted rounded" }
                div { class: "h-4 w-full bg-muted rounded" }
                div { class: "h-4 w-2/3 bg-muted rounded" }
            }
            // Simulated heading
            div { class: "h-7 w-2/5 bg-muted rounded pt-2" }
            // Third paragraph block
            div {
                class: "space-y-4",
                div { class: "h-4 w-full bg-muted rounded" }
                div { class: "h-4 w-11/12 bg-muted rounded" }
                div { class: "h-4 w-3/5 bg-muted rounded" }
            }
        }
    }
}
