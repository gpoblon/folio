#[cfg(feature = "mock")]
pub mod mock;

use dioxus::prelude::*;

#[server(articles: dioxus_server::axum::Extension<super::model::ArticleStore>)]
pub async fn article(slug: String) -> Result<super::model::Article, HttpError> {
    let Ok(articles) = (*articles).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    let Some(article) = articles.get(&slug) else {
        return HttpError::not_found("Article not found".to_string());
    };
    Ok(article.clone())
}

#[server(articles: dioxus_server::axum::Extension<super::model::ArticleStore>)]
pub async fn articles() -> Result<Vec<super::model::ArticleMetadata>, HttpError> {
    let Ok(articles) = (*articles).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    let meta = articles.values().map(|a| a.metadata.clone()).collect();
    dioxus::logger::tracing::debug!("articles: {:?}", meta);
    Ok(meta)
}
