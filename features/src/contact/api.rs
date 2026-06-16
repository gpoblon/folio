use super::model::ContactForm;
use dioxus::{fullstack::Form, prelude::*};

#[server(
    config: dioxus_server::axum::Extension<kernel::config::Config>,
    limiter: dioxus_server::axum::Extension<kernel::rate_limit::RateLimiter>,
    client_ip: kernel::rate_limit::ClientIp,
)]
pub async fn send_contact_email(Form(form): Form<ContactForm>) -> Result<(), HttpError> {
    use dioxus::logger::tracing;
    use garde::Validate;

    // --- Honeypot: reject if the hidden field was filled (bots only) ---
    if !form.phone.is_empty() {
        tracing::warn!("Honeypot triggered — likely bot submission.");
        // Return Ok so bots think it succeeded (don't reveal the trap)
        return Ok(());
    }

    // --- Rate limiting by IP + email composite key ---
    let key = kernel::rate_limit::fingerprint_key(&client_ip, &form.email);
    if limiter.check(&key).is_err() {
        tracing::warn!("Rate limit exceeded for key={key}");
        return HttpError::bad_request(
            "Too many messages sent. Please try again later.".to_string(),
        );
    }

    // --- Validate form fields ---
    if let Err(e) = form.validate() {
        return HttpError::bad_request(format!("Validation error: {}", e));
    }

    let mail = kernel::mail::Mail::builder()
        .address(&form.email)
        .or_else(|e| HttpError::bad_request(format!("Invalid Address: {}", e)))?
        .name(form.name)
        .subject(form.subject)
        .body(form.message)
        .build();

    kernel::mail::send(&config, mail).await.or_else(|e| {
        tracing::error!("Failed to send contact email: {e}");
        HttpError::internal_server_error(
            "An internal error occurred. Please try again later or contact me directly."
                .to_string(),
        )
    })?;

    tracing::info!("Contact form submitted.");

    Ok(())
}
