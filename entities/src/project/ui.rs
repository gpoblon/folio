use dioxus::prelude::*;

/// Renders the full content of a single project.
/// The content is the repository's README rendered as markdown.
#[component]
pub fn Project(project: super::model::Project) -> Element {
    let super::model::Project {
        metadata: meta,
        content,
    } = project;

    let repo_url = format!(
        "https://github.com/{}/{}",
        meta.repository.owner, meta.repository.slug
    );

    rsx! {
        div {
            class: "space-y-4",
            crate::metadata::MetadataHeader {
                meta: meta.meta,
                title_color: "text-projects",
                content_len: content.len(),
            }
            div {
                class: "flex gap-2 items-center justify-end",
                components::svg::Github { class: "h-6" }
                span { { kernel::lang::t!("projects_source_code") } }
                a {
                    class: "link",
                    href: "{repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    aria_label: "{repo_url}",
                    p { "{repo_url}" }
                }
            }
            components::Separator { class: "py-4" }
            if let Some(content) = meta.introduction {
                components::Markdown { content }
            }
            components::Markdown { content }
        }
    }
}

/// A single project preview card.
/// Currently used in the project list view.
#[component]
pub fn ProjectPreview(metadata: super::model::ProjectMetadata) -> Element {
    let href = format!("/projects/{}", metadata.repository.slug);

    rsx! {
        crate::metadata::MetadataPreview {
            meta: metadata.meta,
            href,
        }
    }
}
