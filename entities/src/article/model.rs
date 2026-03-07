#[cfg(feature = "server")]
use anyhow::anyhow;
#[cfg(feature = "server")]
use dioxus::{logger::tracing, prelude::*};
#[cfg(feature = "server")]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Used at the top level to store all articles
/// Map<Slug, Article>
#[derive(Default, Debug, Clone)]
pub struct ArticleStore(pub Arc<RwLock<HashMap<String, Article>>>);

#[cfg(feature = "server")]
/// Parse markdown files into articles iff they have proper metadata and are published
impl TryFrom<kernel::git::RepositoryContent> for ArticleStore {
    type Error = anyhow::Error;

    fn try_from(contents: kernel::git::RepositoryContent) -> Result<Self, Self::Error> {
        let mut articles = HashMap::with_capacity(contents.markdown.len());
        for content in &contents.markdown {
            tracing::trace!("Parsing article {:#?}", content);
        }
        for (path, content) in contents.markdown {
            tracing::info!("Parsing article: {}", path.display());
            match Article::try_parse(path, content) {
                Ok(article) => {
                    if article.metadata.state != State::Published {
                        // Skip unpublished articles
                        tracing::warn!("Article {} is not published", article.metadata.title);
                        continue;
                    }
                    tracing::info!("Parsing successful: {}", article.metadata.title);
                    articles.insert(article.metadata.slug.clone(), article);
                }
                // TODO cumulative error
                Err(err) => {
                    tracing::error!("Failed to parse article: {}", err);
                    continue;
                }
            };
        }
        Ok(ArticleStore(Arc::new(RwLock::new(articles))))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub(super) content: String,
}

#[cfg(feature = "server")]
impl Article {
    pub fn try_parse(path: std::path::PathBuf, content: String) -> anyhow::Result<Self> {
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
            let md_trimmed = md_block.trim_start().trim_end();
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleMetadata {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub lang: kernel::lang::Lang,
    pub tags: Vec<Intent>,
    #[serde(default)]
    pub state: State,
    #[serde(default)]
    pub expertise: Expertise,
    /// Slug is the repo-relative path used as URL segment under `/articles/`,
    /// e.g. `IT/architecture.md` or `science/psychology/autism.md`.
    #[serde(default)]
    pub slug: String,
    pub created: Option<kernel::DateTime>,
    pub modified: Option<kernel::DateTime>,
}

#[cfg(feature = "server")]
impl ArticleMetadata {
    fn new(path: PathBuf, yaml: &str) -> anyhow::Result<Self> {
        match serde_saphyr::from_str::<ArticleMetadata>(yaml.trim()) {
            Ok(mut metadata) => {
                metadata.slug = path.to_string_lossy().to_string();
                Ok(metadata)
            }
            Err(e) => Err(anyhow!(
                r#"Failed to parse YAML frontmatter into metadata.
                Error: {}
                Input: {}"#,
                e,
                yaml
            )),
        }
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::EnumIter,
    strum::EnumString,
)]
#[serde(rename_all = "lowercase")]
pub enum Expertise {
    Novice,
    Knowledgeable,
    Expert,
    #[default]
    #[serde(other)]
    Undefined,
}

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::EnumIter,
    strum::EnumString,
)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Draft,
    Review,
    Published,
    Private,
    Archived,
    #[default]
    #[serde(other)]
    Undefined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, strum::Display)]
#[serde(rename_all = "kebab-case")]
pub enum Intent {
    HandsOn,
    Concept,
    DeepDive,
    Review,
    CaseStudy,
    Reference,
    Essay,
    #[serde(other)]
    Undefined,
}
impl From<&str> for Intent {
    fn from(value: &str) -> Self {
        match value {
            "hands-on" => Intent::HandsOn,
            "concept" => Intent::Concept,
            "deep-dive" => Intent::DeepDive,
            "review" => Intent::Review,
            "case-study" => Intent::CaseStudy,
            "reference" => Intent::Reference,
            "essay" => Intent::Essay,
            _ => Intent::Undefined,
        }
    }
}
