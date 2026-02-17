use super::api;
use dioxus::{fullstack::Form, prelude::*};
use garde::Validate;
use kernel::lang::t;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, PartialEq, Default, strum::EnumString)]
pub(super) enum FormState {
    #[default]
    Idle,
    Submitting,
    Success,
    ClientError(String),
    ServerError(String),
}

impl FormState {
    pub fn is_submitting(&self) -> bool {
        self == &FormState::Submitting
    }

    pub fn is_success(&self) -> bool {
        self == &FormState::Success
    }
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
    /// Custom hook for contact form submission logic
    ///
    /// Returns a FormController that encapsulates form state and submission logic
    pub(super) fn use_contact_form_submission() -> Self {
        let form_state = use_signal(FormState::default);
        Self { form_state }
    }

    /// Check if form is currently submitting
    pub(super) fn state(&self) -> ReadSignal<FormState> {
        self.form_state.boxed()
    }

    /// If form has been successfully submitted, reset form state
    pub(super) fn is_submittable(&self) {
        let mut form_state = self.form_state;
        if form_state.peek().is_success() {
            form_state.set(FormState::default());
        }
    }

    /// Handle form submission event
    pub(super) async fn handle_submit(&self, evt: FormEvent) {
        evt.prevent_default();
        let mut form_state = self.form_state;
        form_state.set(FormState::Submitting);

        let validated_form = {
            let parsed_form = match evt.parsed_values::<ContactForm>() {
                Ok(form_data) => form_data,
                Err(e) => {
                    form_state.set(FormState::ClientError(e.to_string()));
                    return;
                }
            };
            if let Err(e) = parsed_form.validate() {
                form_state.set(FormState::ClientError(e.to_string()));
                return;
            }
            parsed_form
        };

        match api::send_contact_email(Form(validated_form)).await {
            Ok(_) => form_state.set(FormState::Success),
            Err(e) => form_state.set(FormState::ServerError(e.to_string())),
        };
    }
}

impl components::toast::ToastDispatcher for FormController {
    /// Send a toast notification based on internal state (usually [`Signal`])
    fn notify(&self) {
        // Notify of success or error
        tracing::info!("{:?}", *self.form_state.peek());

        let toast = components::toast::use_toast();
        match *self.form_state.peek() {
            FormState::Success => toast
                .success(t!("connect_send_success"))
                .description(t!("connect_send_success_desc"))
                .send(),
            FormState::ClientError(ref msg) => toast
                .error(t!("connect_send_client_error"))
                .description(msg.to_string())
                .send(),
            FormState::ServerError(ref msg) => toast
                .error(t!("connect_send_server_error"))
                .description(msg.to_string())
                .send(),
            _ => (),
        };
    }
}
