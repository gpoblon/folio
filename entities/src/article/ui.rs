use dioxus::prelude::*;

use super::model::ArticleMetadata;

/// Renders the full content of a single article.
/// Pure/dumb component: accepts data as props, performs no I/O or navigation.
#[component]
pub fn Article(article: super::model::Article) -> Element {
    let super::model::Article { metadata, content } = article;
    rsx! {
        div {
            class: "space-y-8",
            crate::metadata::MetadataHeader {
                meta: metadata,
                title_color: "text-knowledge",
                content_len: content.len(),
            }
            components::Separator { class: "py-4" }
            components::Markdown { content }
            components::Separator { class: "py-4" }
            p {
                class: "text-left",
                { kernel::lang::t!("article_feedback_contact") }
                " "
                Link {
                    class: "link underline",
                    new_tab: true,
                    to: "/contact",
                    { kernel::lang::t!("article_feedback_contact_link_alt") }
                }
                "."
            }
        }
    }
}

/// A single article preview card.
/// Pure/dumb component: accepts data as props, performs no I/O.
#[component]
pub fn ArticlePreview(meta: ArticleMetadata) -> Element {
    let href = format!("/articles/{}", meta.slug);

    rsx! {
        crate::metadata::MetadataPreview {
            meta,
            href,
        }
    }
}
