#[cfg(feature = "server")]
mod store;
#[cfg(feature = "server")]
pub use store::ArticleStore;

#[cfg(feature = "server")]
use anyhow::anyhow;
#[cfg(feature = "server")]
use std::path::PathBuf;

use crate::metadata::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub(super) content: String,
}

#[cfg(feature = "server")]
impl Article {
    pub fn try_parse(path: PathBuf, content: String) -> anyhow::Result<Self> {
        // [
        //     0 -> trash (before first ---),
        //     1 -> yaml (in-between ---),
        //     2 -> markdown (after second ---)
        // ]
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        let Some(s_yaml) = parts.get(1) else {
            return Err(anyhow!("No yaml metadata"));
        };
        let md = {
            let Some(md_block) = parts.get(2) else {
                return Err(anyhow!("No markdown content"));
            };
            let md_trimmed = md_block.trim();
            if md_trimmed.is_empty() {
                return Err(anyhow!("Empty markdown content"));
            }
            md_trimmed.to_string()
        };
        let metadata = ArticleMetadata::new(path, s_yaml)?;

        Ok(Self {
            metadata,
            content: md,
        })
    }
}

pub type ArticleMetadata = Metadata;

#[cfg(feature = "server")]
impl ArticleMetadata {
    fn new(path: PathBuf, yaml: &str) -> anyhow::Result<Self> {
        match serde_saphyr::from_str::<Self>(yaml.trim()) {
            Ok(mut metadata) => {
                metadata.slug = path.to_string_lossy().to_string();
                Ok(metadata)
            }
            Err(e) => Err(anyhow!(
                "Failed to parse YAML frontmatter into metadata.\nError: {}\nInput: {}",
                e,
                yaml
            )),
        }
    }
}
