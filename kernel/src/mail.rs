use lettre::{
    AsyncTransport, Tokio1Executor,
    message::{
        Mailbox,
        header::{HeaderName, HeaderValue},
    },
};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, bon::Builder)]
pub struct Mail {
    /// Sender address,
    #[builder(getter, with = |string: &str| -> Result<_, lettre::address::AddressError> {
        string.parse()
    })]
    pub(super) address: lettre::Address,
    #[builder(getter)]
    pub(super) name: String,
    #[builder(getter)]
    pub(super) subject: String,
    #[builder(getter)]
    pub(super) body: String,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum MailError {
    #[error("Invalid email address: {0}")]
    InvalidAddress(lettre::address::AddressError),
}

/// Send a mail to gpoblon
pub async fn send(config: &crate::config::Config, mail: Mail) -> anyhow::Result<()> {
    let receiver = Mailbox::new(
        Some(config.app.app_editor.clone()),
        config.smtp.smtp_username.expose_secret().parse()?,
    );
    let header = HeaderValue::new(
        HeaderName::new_from_ascii_str("X-Origin"),
        config.app.app_http_address.to_owned(),
    );
    let email = lettre::Message::builder()
        .from(Mailbox::new(Some(mail.name), mail.address))
        .to(receiver)
        .raw_header(header)
        .subject(mail.subject)
        .header(lettre::message::header::ContentType::TEXT_PLAIN)
        .body(mail.body)?;

    let credentials = lettre::transport::smtp::authentication::Credentials::new(
        config.smtp.smtp_username.expose_secret().to_owned(),
        config.smtp.smtp_password.expose_secret().to_owned(),
    );

    let mailer = lettre::AsyncSmtpTransport::<Tokio1Executor>::relay(
        config.smtp.smtp_relay.expose_secret(),
    )?
    .port(config.smtp.smtp_port)
    .credentials(credentials)
    .build();

    mailer.send(email).await?;
    Ok(())
}
