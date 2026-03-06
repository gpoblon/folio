use secrecy::SecretString;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub(crate) app: AppConfig,
    pub(crate) smtp: SmtpConfig,
    pub(crate) git: GitConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    pub(crate) http_address: String,
    pub(crate) editor: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SmtpConfig {
    pub(crate) username: SecretString,
    pub(crate) password: SecretString,
    pub(crate) relay: SecretString,
    pub(crate) port: u16,
}

/// Git configuration for the application, allowing to:
/// - Interact with a repository (read only)
/// - Interact with issues (read & write)
/// - Receive webhooks
///
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GitConfig {
    /// Token has the following permissions: read contents, write issues, read webhooks
    /// PAT token: account level but restricted to gpoblon/knowledge_base repository.
    pub(crate) token: secrecy::SecretString,
    /// Owner of the repository
    pub(crate) owner: secrecy::SecretString,
    /// Name of the repository
    pub(crate) repository: secrecy::SecretString,
    /// Branch to use for the repository
    pub(crate) branch: String,
    // TODO Webhook secret
    // webhook_secret: secrecy::SecretString,
}

impl Config {
    fn load_from_env_file() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(".env")?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn init() -> Self {
        match Self::load_from_env_file() {
            Ok(config) => config,
            Err(err) => panic!(
                "Failed to read config file: {err}.\n(Please create a .env file with the following example content:\n{})",
                Self::env_example()
            ),
        }
    }

    fn env_example() -> &'static str {
        r#"
[app]
http_address = "127.0.0.1:8080"
editor = "John Doe"

[smtp]
username = "jdoe@example.tld"
password = "password"
relay = "smtp.example.tld"
port = 587

[git]
token = "pat_token_example"
owner = "jdoe"
repository = "kb"
"#
    }

    pub fn git(&self) -> &GitConfig {
        &self.git
    }
}
