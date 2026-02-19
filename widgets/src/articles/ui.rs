use dioxus::prelude::*;
use entities::article::model::ArticleMetadata;

#[component]
pub fn ArticleGrid() -> Element {
    let metadatas = super::model::mock_resource_article_list_metadata();

    let categories = vec![
        "All",
        "Technology",
        "Design",
        "Development",
        "Business",
        "Tutorials",
        "News",
        "News",
        "News",
        "Opinion",
        "science/psychology/pathology",
        "Opinion",
        "science/psychology/pathology",
        "Opinion",
        "science/psychology/pathology",
        "Guides",
    ];

    rsx! {
        div {
            class: "p-32 flex flex-row md:flow-col gap-8",
            CategoriesAside {
                categories: categories.iter().map(|s| s.to_string()).collect()
            }
            main { class: "w-full md:w-4/5",
                h1 { class: "text-projects", "Articles" }
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    for meta in metadatas {
                        ArticlePreview { meta }
                    }
                }
            }
        }
    }
}

#[component]
fn CategoriesAside(categories: Vec<String>) -> Element {
    rsx! {
        aside {
            class: "w-full md:max-w-54 hover:md:max-w-xs transition-[max-width] duration-300 ease-in-out border border-primary py-2 mt-21",
            for category in categories.iter() {
                components::Button {
                    class: "block px-2 py-1 text-left w-full hover:bg-muted text-nowrap text-ellipsis overflow-hidden",
                    variant: components::ButtonVariant::Ghost,
                    "{category}"
                }
            }
        }
    }
}

#[component]
fn ArticlePreview(meta: ArticleMetadata) -> Element {
    let (topics, _) = meta.slug.rsplit_once("/").unwrap_or(("root", ""));
    rsx! {
        a {
            class: "border border-primary shadow-md p-4 flex flex-col h-full bg-accent",
            href: "articles/{meta.slug}",
            target: "_blank",
            rel: "noopener noreferrer",
            h5 { "{ meta.title }" }
            p { class: "italic grow", "{ meta.description }" }
            div {
                class: "mt-4 flex items-baseline justify-between",
                p { class: "text-muted grow", "{ topics }" }
                div {
                    class: "flex gap-2 flex-nowrap",
                    for tag in meta.tags {
                        components::badge::Badge {
                            variant: components::badge::BadgeVariant::Outline,
                            "{ tag }"
                        }
                    }
                    p { class: "text-muted text-right", "{ meta.lang }" }
                }
            }
        }
    }
}
