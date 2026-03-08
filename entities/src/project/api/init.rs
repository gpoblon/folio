impl super::model::ProjectStore {
    pub async fn try_init(git_client: &kernel::git::GitClient) -> anyhow::Result<Self> {
        use dioxus::logger::tracing;
        use std::collections::HashMap;

        let metadata = super::model::load_all_project_metadata();
        let mut readmes = HashMap::with_capacity(metadata.len());

        for meta in &metadata {
            match git_client
                .fetch_readme(meta.repository.slug.clone(), meta.repository.owner.clone())
                .await
            {
                Ok(content) => {
                    tracing::info!(
                        "Fetched README for project '{}' from {}",
                        meta.repository.slug,
                        meta.repository.owner
                    );
                    readmes.insert(meta.repository.slug.clone(), content);
                }
                Err(err) => {
                    tracing::warn!(
                        "Failed to fetch README for project '{}': {}",
                        meta.repository.slug,
                        err
                    );
                }
            }
        }

        let store = Self::from_metadata_and_readmes(metadata, readmes)?;
        Ok(store)
    }
}
