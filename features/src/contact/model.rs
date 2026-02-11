use super::api;
use dioxus::{fullstack::Form, prelude::*};
use garde::Validate;
use kernel::lang::t;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Raw contact form data as submitted by the user
#[derive(Debug, Clone, Deserialize, Serialize, garde::Validate)]
pub(super) struct ContactForm {
    #[garde(ascii, length(min = 4, max = 64))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 4, max = 64))]
    pub subject: String,
    #[garde(ascii, length(min = 4, max = 4092))]
    pub message: String,
}

/// State for the contact form submission
#[derive(Debug, Clone, PartialEq, Default)]
pub(super) enum FormState {
    #[default]
    Idle,
    Submitting,
    Success,
    Error(String),
}

/// Controller for contact form submission
///
/// This encapsulates all client business logic related to form submission:
/// - Form state management
/// - Validation
/// - API communication orchestration
/// - Toast notifications
#[derive(Clone, Copy)]
pub(super) struct FormController {
    form_state: Signal<FormState>,
}

impl FormController {
    /// Check if form is currently submitting
    pub(super) fn is_submitting(&self) -> bool {
        matches!(self.form_state.read().clone(), FormState::Submitting)
    }

    /// Handle form submission event
    pub(super) fn handle_submit(&self, evt: FormEvent) {
        let mut form_state = self.form_state;

        spawn(async move {
            let toast = components::toast::consume_toast();
            evt.prevent_default();
            form_state.set(FormState::Submitting);

            match evt.parsed_values::<ContactForm>() {
                Ok(form_data) => match form_data.validate() {
                    Ok(_) => match api::send_contact_email(Form(form_data)).await {
                        Ok(_) => form_state.set(FormState::Success),
                        Err(e) => form_state.set(FormState::Error(e.to_string())),
                    },
                    Err(e) => form_state.set(FormState::Error(e.to_string())),
                },
                Err(_) => {
                    form_state.set(FormState::Error(t!("connect_send_error_parser")));
                }
            };

            match *form_state.peek() {
                FormState::Success => toast.success(
                    t!("connect_send_success"),
                    components::toast::ToastOptions::new()
                        .duration(Duration::from_secs(5))
                        .permanent(false),
                ),
                FormState::Error(ref msg) => toast.error(
                    msg.to_string(),
                    components::toast::ToastOptions::new()
                        .duration(Duration::from_secs(5))
                        .permanent(false),
                ),
                _ => (),
            };
        });
    }
}

/// Custom hook for contact form submission logic
///
/// Returns a FormController that encapsulates form state and submission logic
pub(super) fn use_contact_form_submission() -> FormController {
    let form_state = use_signal(FormState::default);
    FormController { form_state }
}
