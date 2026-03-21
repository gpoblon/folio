use http_body_util::BodyExt;
use secrecy::{ExposeSecret, SecretString};

pub use octocrab::models::repos::Content;

pub struct GitClient {
    client: octocrab::Octocrab,
    owner: SecretString,
    repository: SecretString,
    branch: String,
}

impl GitClient {
    pub fn new(config: crate::config::GitConfig) -> anyhow::Result<Self> {
        let client = octocrab::Octocrab::builder()
            .personal_token(config.git_token)
            .build()?;
        Ok(Self {
            client,
            owner: config.git_owner,
            repository: config.git_repository,
            branch: config.git_branch,
        })
    }

    pub async fn fetch_repository(&self) -> anyhow::Result<Vec<Content>> {
        // fetches repository content from GitHub
        let repository = self
            .client
            .repos(
                self.owner.expose_secret().to_owned(),
                self.repository.expose_secret().to_owned(),
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
                self.owner.expose_secret().to_owned(),
                self.repository.expose_secret().to_owned(),
            )
            .download_tarball(octocrab::params::repos::Reference::Branch(
                self.branch.clone(),
            ))
            .await?
            .into_body()
            .collect()
            .await?
            .to_bytes())
    }

    /// Fetch the default-branch README (as raw Markdown) from a GitHub repository
    pub async fn fetch_readme(
        &self,
        // repository name (no url)
        repository: String,
        // In case the repository belongs to a different workspace
        owner: String,
    ) -> anyhow::Result<String> {
        let readme_content = self
            .client
            .repos(owner, repository)
            .get_readme()
            .r#ref("main")
            .send()
            .await?;

        match readme_content.decoded_content() {
            Some(readme) => Ok(readme),
            None => Err(anyhow::anyhow!("Failed to decode README content")),
        }
    }
}
