#[cfg(all(feature = "server", feature = "mock"))]
pub mod mock;

use super::model;

use dioxus::prelude::*;

#[get("/api/articles", articles: dioxus_server::axum::Extension<model::ArticleStore>)]
pub async fn articles() -> Result<Vec<model::ArticleMetadata>, HttpError> {
    let Ok(articles) = (*articles).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    let meta = articles.values().map(|a| a.metadata.clone()).collect();
    dioxus::logger::tracing::debug!("articles: {:?}", meta);
    Ok(meta)
}

#[server(articles: dioxus_server::axum::Extension<model::ArticleStore>)]
pub async fn article(slug: String) -> Result<model::Article, HttpError> {
    let Ok(articles) = (*articles).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    let Some(article) = articles.get(&slug) else {
        return HttpError::not_found("Article not found".to_string());
    };
    Ok(article.clone())
}
