use dioxus::prelude::*;

#[get("/articles", articles: dioxus_server::axum::Extension<entities::article::model::ArticleStore>)]
pub async fn articles() -> Result<Vec<entities::article::model::ArticleMetadata>, HttpError> {
    let Ok(articles) = (*articles).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    Ok(articles
        .values()
        .map(|article| article.metadata.clone())
        .collect())
}
