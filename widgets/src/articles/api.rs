use dioxus::prelude::*;

#[server(articles: dioxus_server::axum::Extension<entities::article::model::ArticleStore>)]
pub async fn articles() -> Result<Vec<entities::article::model::ArticleMetadata>, HttpError> {
    let Ok(articles) = (*articles).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    let meta = articles.values().map(|a| a.metadata.clone()).collect();
    dioxus::logger::tracing::debug!("articles: {:?}", meta);
    Ok(meta)
}
