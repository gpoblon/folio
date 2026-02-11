use secrecy::SecretString;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    #[serde(rename = "app")]
    pub app: AppConfig,
    #[serde(rename = "smtp")]
    pub smtp: SmtpConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    pub http_address: String,
    pub editor: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SmtpConfig {
    pub username: SecretString,
    pub password: SecretString,
    pub relay: SecretString,
    pub port: u16,
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
"#
    }
}
