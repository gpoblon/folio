use dioxus::{logger::tracing, prelude::*};
use kernel::lang::t;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Used at the top level to store all articles
/// Map<Title, Article>
#[derive(Default, Debug, Clone)]
pub struct ArticleStore(pub Arc<RwLock<HashMap<String, Article>>>);

pub(super) fn use_resource_article(mut slug: String) -> dioxus::hooks::Resource<Option<Article>> {
    use_resource(move || {
        let toast = components::toast::use_toast();
        let slug = std::mem::take(&mut slug);
        async move {
            match super::api::article(slug).await {
                Ok(article) => Some(article),
                Err(err) => {
                    tracing::error!("Failed to fetch article: {}", err);
                    toast
                        .error(t!("article_error"))
                        .description(err.to_string())
                        .send();
                    None
                }
            }
        }
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub(super) content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleMetadata {
    pub title: String,
    pub description: String,
    pub lang: kernel::lang::Lang,
    pub tags: Vec<String>,
    pub state: State,
    pub expertise: Expertise,
    /// Slug is actually the path: e.g. "/IT/dev/lang/rust/intro.md" or "science/psychology/pathology/autism.md"
    pub slug: String,
    pub created_at: Option<kernel::DateTime>,
    pub modified_at: Option<kernel::DateTime>,
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
pub enum Expertise {
    #[default]
    Novice,
    Knowedgeable,
    Expert,
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
pub enum State {
    #[default]
    Draft,
    Review,
    Published,
    Private,
    Archived,
}
