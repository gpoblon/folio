use dioxus::server::Bytes;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use tar::Archive;

// Structured, parsed, validated repository content
pub struct RepositoryContent {
    ///Path, Markdown content decoded to UTF-9 strings
    pub markdown: HashMap<PathBuf, String>,
    /// All other files (images, configs, etc.) kept as raw bytes
    pub assets: HashMap<PathBuf, Vec<u8>>,
}

/// This will only work for tarballs
impl RepositoryContent {
    pub fn try_from_tarball(tarball_bytes: Bytes) -> anyhow::Result<Self> {
        let mut markdown = HashMap::new();
        let mut assets = HashMap::new();

        let mut archive = Archive::new(GzDecoder::new(Cursor::new(tarball_bytes)));

        for entry_result in archive.entries()? {
            let mut entry = entry_result?;

            // skip directories
            if entry.header().entry_type() != tar::EntryType::Regular {
                continue;
            }

            // Strip the top-level GitHub folder to get the true repo path
            let path = entry
                .path()?
                .components()
                .skip(1)
                .collect::<std::path::PathBuf>();

            let mut entry_content = Vec::new();
            entry.read_to_end(&mut entry_content)?;

            // Dispatch the file to either md / asset
            if path.extension().is_some_and(|ext| ext == "md") {
                // Attempt to parse as UTF-8 string
                match String::from_utf8(entry_content.clone()) {
                    Ok(text) => {
                        markdown.insert(path, text);
                    }
                    Err(e) => {
                        dioxus::logger::tracing::warn!(
                            "Failed to parse file as UTF-8: {:?}, error: {:?}. Moving to assets",
                            path,
                            e
                        );
                        assets.insert(path, entry_content);
                    }
                }
            } else {
                assets.insert(path, entry_content);
            }
        }

        dioxus::logger::tracing::info!(
            "Parsed repository content: {} md files, {} assets files",
            markdown.len(),
            assets.len()
        );

        Ok(Self { markdown, assets })
    }
}
