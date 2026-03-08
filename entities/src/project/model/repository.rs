use serde::{Deserialize, Serialize};

/// Project-specific metadata describing the source repository.
///
/// Only projects carry this information; articles do not.
/// Embedded inside `ProjectMetadata` via `#[serde(flatten)]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryMetadata {
    /// Repository owner (GitHub username / organisation).
    #[serde(default)]
    pub owner: String,
    /// Repository slug (the repo name without the full URL prefix).
    pub slug: String,
}
