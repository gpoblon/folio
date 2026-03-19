#[cfg(all(feature = "server", feature = "mock"))]
pub mod mock;

#[cfg(feature = "server")]
mod init;

#[cfg(feature = "server")]
use super::model;

use dioxus::prelude::*;

#[server(projects: dioxus_server::axum::Extension<super::model::ProjectStore>)]
pub async fn project(slug: String) -> Result<super::model::Project, HttpError> {
    let Ok(projects) = (*projects).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    let Some(project) = projects.get(&slug) else {
        return HttpError::not_found("Project not found".to_string());
    };
    Ok(project.clone())
}

#[server(projects: dioxus_server::axum::Extension<super::model::ProjectStore>)]
pub async fn projects() -> Result<Vec<super::model::ProjectMetadata>, HttpError> {
    let Ok(projects) = (*projects).0.try_read() else {
        return HttpError::internal_server_error("Failed to acquire read lock".to_string());
    };
    let meta = projects.values().map(|p| p.metadata.clone()).collect();
    dioxus::logger::tracing::debug!("projects: {:?}", meta);
    Ok(meta)
}
