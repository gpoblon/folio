use http_body_util::BodyExt;
use secrecy::{ExposeSecret, SecretString};

pub use octocrab::models::repos::Content;

pub struct GitClient {
    client: octocrab::Octocrab,
    owner: SecretString,
    repository: SecretString,
}

impl GitClient {
    pub fn new(config: crate::config::GitConfig) -> anyhow::Result<Self> {
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
            .await?
            .take_items();

        Ok(repository)
    }

    pub async fn fetch_repository_tarball(&self) -> anyhow::Result<dioxus::server::Bytes> {
        Ok(self
            .client
            .repos(
                self.owner.expose_secret().parse::<String>()?,
                self.repository.expose_secret().parse::<String>()?,
            )
            .download_tarball(octocrab::params::repos::Reference::Branch(
                "main".to_owned(),
            ))
            .await?
            .into_body()
            .collect()
            .await?
            .to_bytes())
    }
}
