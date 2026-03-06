use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Extension;
use axum::extract::Path;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

use crate::git::RepositoryContent;

/// Shared, read-only store of every non-markdown file pulled from the
/// Obsidian vault Git repository.
///
/// Keys are repo-relative paths (e.g. `resources/images/photo.png`).
/// Values are the raw file bytes.
#[derive(Debug, Clone, Default)]
pub struct Resources(pub Arc<HashMap<PathBuf, Vec<u8>>>);

impl From<&RepositoryContent> for Resources {
    fn from(repo: &RepositoryContent) -> Self {
        Self(Arc::new(repo.assets.clone()))
    }
}

/// Axum handler that serves an in-memory vault asset by its path.
///
/// Mounted as  `GET /resources/*path`  so a request to
/// `/resources/images/photo.png` will look up the key
/// `resources/images/photo.png` in the shared map.
///
/// This means the resources must be stored in the `resources/` vault root directory,
/// And md files must reference images using vault-relative paths (e.g. `resources/images/photo.png`).
pub async fn serve_vault_resource(
    Path(path): Path<String>,
    Extension(resources): Extension<Resources>,
) -> impl IntoResponse {
    dioxus::prelude::info!("Serving vault resource: {}", path);
    let key = PathBuf::from(format!("resources/{path}"));

    let Some(bytes) = resources.0.get(&key) else {
        dioxus::prelude::warn!("Asset not found: {:?}", resources.0.keys());
        return (StatusCode::NOT_FOUND, "Asset not found").into_response();
    };

    // Derive Content-Type from the file extension; fall back to
    // application/octet-stream when the extension is unknown.
    let content_type = mime_guess::from_path(&key)
        .first_or_octet_stream()
        .to_string();

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, content_type)],
        bytes.clone(),
    )
        .into_response()
}
