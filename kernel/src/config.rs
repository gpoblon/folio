use secrecy::SecretString;
use serde::Deserialize;

/// `podman --env-file` passes values as raw strings without stripping quotes,
/// so a value like `SMTP_PORT=465` may arrive as the string `"465"` with
/// literal quote characters. This deserializer trims surrounding ASCII quotes
/// and whitespace before parsing into a `u16`.
fn deserialize_u16_from_str<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    let trimmed = raw.trim().trim_matches('"').trim_matches('\'');
    trimmed.parse::<u16>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub(crate) app: AppConfig,
    #[serde(flatten)]
    pub(crate) smtp: SmtpConfig,
    #[serde(flatten)]
    pub(crate) git: GitConfig,
    #[serde(flatten)]
    pub(crate) umami: UmamiConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    pub(crate) app_http_address: String,
    pub(crate) app_editor: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SmtpConfig {
    pub(crate) smtp_username: SecretString,
    pub(crate) smtp_password: SecretString,
    pub(crate) smtp_relay: SecretString,
    #[serde(deserialize_with = "deserialize_u16_from_str")]
    pub(crate) smtp_port: u16,
}

/// Git configuration for the application, allowing to:
/// - Interact with a repository (read only)
/// - Interact with issues (read & write)
/// - Receive webhooks
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GitConfig {
    /// Token has the following permissions: read contents, write issues, read webhooks
    /// PAT token: account level but restricted to the knowledge base repository.
    pub(crate) git_token: SecretString,
    /// Owner of the repository
    pub(crate) git_owner: SecretString,
    /// Name of the repository
    pub(crate) git_repository: SecretString,
    /// Branch to use for the repository
    pub(crate) git_branch: String,
    // TODO Webhook secret
    // git_webhook_secret: SecretString,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UmamiConfig {
    pub(crate) umami_website_id: SecretString,
}

impl Config {
    fn load_from_env() -> anyhow::Result<Self> {
        // Load .env file into the process environment, ignoring errors if the
        // file is absent (e.g. when env vars are injected directly at runtime).
        let _ = dotenvy::dotenv();
        let config = envy::from_env::<Config>()?;
        Ok(config)
    }

    pub fn init() -> Self {
        match Self::load_from_env() {
            Ok(config) => config,
            Err(err) => panic!(
                "Failed to load config from environment: {err}.\n\
                (Please set the required environment variables or create a .env file.\n\
                See .env_example for reference.)"
            ),
        }
    }

    pub fn git(&self) -> &GitConfig {
        &self.git
    }

    pub fn umami(&self) -> &UmamiConfig {
        &self.umami
    }
}
