use dioxus::server::Bytes;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use tar::Archive;

use crate::git::GitClient;

/// Top-level directory in the Obsidian vault that holds binary resources (images, etc.).
const RESOURCES_DIR: &str = "resources";

/// Structured, parsed, validated repository content.
pub struct RepositoryContent {
    /// Path → Markdown content decoded to UTF-8 strings.
    ///
    /// Keys are repo-relative paths (e.g. `IT/architecture.md`).
    /// Articles now live at the repo root — there is no `articles/` prefix to strip.
    /// These paths double as URL slugs under the `/articles/` Dioxus route.
    pub markdown: HashMap<PathBuf, String>,
    /// All other files (images, configs, etc.) kept as raw bytes.
    ///
    /// Keys keep their original repo-relative path (e.g. `resources/images/photo.png`)
    /// because they are served through Axum under `/resources/<path>`.
    pub assets: HashMap<PathBuf, Vec<u8>>,
}

/// This will only work for tarballs
impl RepositoryContent {
    pub async fn fetch(client: GitClient) -> anyhow::Result<RepositoryContent> {
        client
            .fetch_repository_tarball()
            .await
            .and_then(RepositoryContent::try_from_tarball)
    }

    fn try_from_tarball(tarball_bytes: Bytes) -> anyhow::Result<Self> {
        let mut articles = HashMap::new();
        let mut resources = HashMap::new();

        let mut archive = Archive::new(GzDecoder::new(Cursor::new(tarball_bytes)));

        for entry_result in archive.entries()? {
            let mut entry = entry_result?;

            // skip directories
            if entry.header().entry_type() != tar::EntryType::Regular {
                continue;
            }

            // Strip the top-level GitHub folder to get the true repo path
            let repo_path: PathBuf = entry.path()?.components().skip(1).collect::<PathBuf>();

            if repo_path.starts_with(RESOURCES_DIR) {
                // Binary resource (image, PDF, …) — keep the full repo-relative path.
                let mut entry_content = Vec::new();
                entry.read_to_end(&mut entry_content)?;
                resources.insert(repo_path, entry_content);
            } else if repo_path.extension().is_some_and(|ext| ext == "md") {
                // Markdown article — the repo path *is* the slug (no prefix to strip).
                let mut entry_content = Vec::new();
                entry.read_to_end(&mut entry_content)?;
                articles.insert(
                    repo_path,
                    String::from_utf8_lossy(&entry_content).to_string(),
                );
            }
            // Any other file at the repo root (e.g. .obsidian/, README) is ignored.
        }

        dioxus::logger::tracing::info!(
            "Parsed repository content: {} md files, {} assets files",
            articles.len(),
            resources.len()
        );

        Ok(Self {
            markdown: articles,
            assets: resources,
        })
    }
}
