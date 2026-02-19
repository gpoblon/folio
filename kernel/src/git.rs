use secrecy::{ExposeSecret, SecretString};

pub use octocrab::models::repos::Content;

pub struct GitClient {
    client: octocrab::Octocrab,
    owner: SecretString,
    repository: SecretString,
}

impl GitClient {
    fn new(config: crate::config::GitConfig) -> anyhow::Result<Self> {
        let client = octocrab::Octocrab::builder()
            .personal_token(config.token)
            .build()?;
        Ok(Self {
            client,
            owner: config.owner,
            repository: config.repository,
        })
    }

    pub async fn fetch_repository(&self) -> anyhow::Result<Vec<Content>> {
        // fetches repository content from GitHub
        let repository = self
            .client
            .repos(
                self.owner.expose_secret().parse::<String>()?,
                self.repository.expose_secret().parse::<String>()?,
            )
            .get_content()
            .send()
            .await?;

        Ok(repository.items)
    }
}
