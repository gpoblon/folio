use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::metadata::*;

impl super::ProjectStore {
    /// Build a [`ProjectStore`] from a parsed YAML list of project metadata
    /// and a function that resolves each repository URL to its README content.
    ///
    /// Projects whose `state` is not `Published` are silently skipped.
    pub fn from_metadata_and_readmes(
        metadata_list: Vec<super::ProjectMetadata>,
        readmes: HashMap<String, String>,
    ) -> anyhow::Result<Self> {
        use dioxus::logger::tracing;

        let mut projects = HashMap::with_capacity(metadata_list.len());

        for meta in metadata_list {
            if meta.meta.state != State::Published {
                tracing::warn!("Project {} is not published, skipping", meta.meta.title);
                continue;
            }

            let slug = &meta.repository.slug;

            let content = readmes.get(slug).cloned().unwrap_or_else(|| {
                tracing::warn!(
                    "No README content found for project '{}' (slug: {})",
                    meta.meta.title,
                    slug
                );
                String::new()
            });

            tracing::info!("Loaded project: {}", meta.meta.title);
            projects.insert(
                slug.clone(),
                super::Project {
                    metadata: meta,
                    content,
                },
            );
        }

        Ok(Self(Arc::new(RwLock::new(projects))))
    }
}
