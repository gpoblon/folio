#[derive(Default)]
pub(crate) struct State {
    pub articles: entities::article::model::ArticleStore,
    pub resources: kernel::resources::Resources,
    pub projects: entities::project::model::ProjectStore,
}
impl State {
    #[cfg(not(feature = "mock"))]
    pub async fn try_fetch_data(config: kernel::config::Config) -> anyhow::Result<Self> {
        use anyhow::Context;
        let git_client = kernel::git::GitClient::new(config.git().clone())
            .context("Failed to create GitClient")?;

        let project_store = entities::project::model::ProjectStore::try_init(&git_client)
            .await
            .unwrap_or_default();

        let repository = kernel::git::RepositoryContent::fetch(git_client)
            .await
            .context("Failed to fetch repository")?;
        let resources = kernel::resources::Resources::from(&repository);
        let article_store = entities::article::model::ArticleStore::try_from(repository.markdown)
            .unwrap_or_default();

        Ok(Self {
            articles: article_store,
            resources,
            projects: project_store,
        })
    }

    #[cfg(feature = "mock")]
    pub async fn try_fetch_data(_: kernel::config::Config) -> anyhow::Result<Self> {
        Ok(Self {
            articles: entities::article::model::ArticleStore::mock(),
            resources: kernel::resources::Resources::default(),
            projects: entities::project::model::ProjectStore::mock(),
        })
    }
}
