use dioxus::prelude::*;

/// A responsive three-column grid layout.
///
/// Delegates the grid CSS to a single place so every consumer (articles,
/// projects, …) stays in sync automatically.
#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn Grid(props: GridProps) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
            ..props.attributes,
            {props.children}
        }
    }
}

/// A full-grid skeleton placeholder.
///
/// Renders [`CARD_COUNT`] [`PreviewCardSkeleton`] cards inside a [`Grid`],
/// matching the visual weight of a fully-loaded preview grid.
const CARD_COUNT: usize = 6;

#[component]
pub fn GridSkeleton() -> Element {
    rsx! {
        Grid {
            for _ in 0..CARD_COUNT {
                PreviewCardSkeleton {}
            }
        }
    }
}

/// A single animated placeholder card.
///
/// Mirrors the shared structure of both `ArticlePreview` and `ProjectPreview`:
///
/// - Topic path + date row
/// - Title bar
/// - Description block
/// - Language + tag badges row
#[component]
pub fn PreviewCardSkeleton() -> Element {
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
                    for _ in 0..3 {
                        div { class: "h-7 bg-muted rounded px-2 w-16" }
                    }
                }
            }
        }
    }
}
