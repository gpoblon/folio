use dioxus::prelude::*;

use super::model::ContactFormData;

/// Result type for contact form submission
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ContactResult {
    Success,
    ValidationError(String),
    SendError(String),
}

/// Server function to handle contact form submission
///
/// This function validates the form data and sends an email.
/// The actual email sending logic should be implemented based on your email provider.
#[server]
pub async fn send_contact_email(form: ContactFormData) -> Result<ContactResult, ServerFnError> {
    use super::model::ValidatedContactForm;

    // Validate the form data
    let validated: ValidatedContactForm = match form.try_into() {
        Ok(v) => v,
        Err(e) => return Ok(ContactResult::ValidationError(e.to_string())),
    };

    // TODO send mail logic
    // TODO proper error i18n

    tracing::info!("Contact form submission (success): {validated:#?}");

    Ok(ContactResult::Success)
}
