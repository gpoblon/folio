use crate::metadata::*;
use dioxus::{logger::tracing, prelude::*};
#[cfg(feature = "server")]
use std::path::PathBuf;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Used at the top level to store all articles
/// Map<Slug, Article>
#[derive(Default, Debug, Clone)]
pub struct ArticleStore(pub Arc<RwLock<HashMap<String, super::Article>>>);

#[cfg(feature = "server")]
/// Parse markdown files into articles iff they have proper metadata and are published
impl TryFrom<HashMap<PathBuf, String>> for ArticleStore {
    type Error = anyhow::Error;

    fn try_from(markdown_files: HashMap<PathBuf, String>) -> Result<Self, Self::Error> {
        let mut articles = HashMap::with_capacity(markdown_files.len());
        for content in &markdown_files {
            tracing::trace!("Parsing article {:#?}", content);
        }
        for (path, content) in markdown_files {
            tracing::info!("Parsing article: {}", path.display());
            match super::Article::try_parse(path, content) {
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
