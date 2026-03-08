mod repository;
#[cfg(feature = "server")]
mod store;

use crate::metadata::*;
pub use repository::RepositoryMetadata;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Used at the top level to store all projects.
/// Map<Slug, Project>
#[derive(Default, Debug, Clone)]
pub struct ProjectStore(pub Arc<RwLock<HashMap<String, Project>>>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub metadata: ProjectMetadata,
    /// The rendered README markdown content fetched from the repository.
    pub content: String,
}

/// Built from the `projects.yaml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectMetadata {
    #[serde(flatten)]
    pub meta: Metadata,
    /// Markdown introduction injected above the fetched README content.
    #[serde(default)]
    pub introduction: Option<String>,
    pub repository: RepositoryMetadata,
}

/// Intermediate YAML wrapper so we can deserialize the top-level `projects:` key.
#[cfg(feature = "server")]
#[derive(Debug, Deserialize)]
struct ProjectsYaml {
    projects: Vec<ProjectMetadata>,
}

/// Load all project metadata from the embedded YAML files (both languages).
#[cfg(feature = "server")]
pub(super) fn load_all_project_metadata() -> Vec<ProjectMetadata> {
    match serde_saphyr::from_str::<ProjectsYaml>(include_str!("../../../assets/projects.yaml")) {
        Ok(yaml) => yaml.projects.into_iter().collect(),
        Err(err) => {
            dioxus::prelude::error!("static projects.yaml is invalid: {}", err);
            Vec::new()
        }
    }
}
