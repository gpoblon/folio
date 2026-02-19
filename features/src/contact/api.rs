use super::model::ContactForm;
use dioxus::{fullstack::Form, prelude::*};

#[post("/contact", config: dioxus_server::axum::Extension<kernel::config::Config>)]
pub async fn send_contact_email(Form(form): Form<ContactForm>) -> Result<(), HttpError> {
    use dioxus::logger::tracing;
    // Validate the form data and retrieve its content
    use garde::Validate;
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

    kernel::mail::send(&config, mail)
        .await
        .or_else(|e| HttpError::internal_server_error(format!("Failed to send: {}", e)))?;

    tracing::info!("Contact form submitted.");

    Ok(())
}
