use super::api;
use dioxus::{fullstack::Form, prelude::*};
use garde::Validate;
use kernel::lang::t;
use serde::{Deserialize, Serialize};

/// Raw contact form data as submitted by the user
///
/// Note: Store is necessary to reset the form upon success.
/// If `FormEvent::reset(self)` is implemented by Dioxus, the store won't be necessary and uncontrolled inputs would work perfectly.
#[derive(Debug, Clone, Default, Deserialize, Serialize, garde::Validate, dioxus_stores::Store)]
pub(super) struct ContactForm {
    #[garde(ascii, length(min = 4, max = 64))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 4, max = 64))]
    pub subject: String,
    #[garde(ascii, length(min = 4, max = 4092))]
    pub message: String,
    /// Honeypot field — hidden from real users, filled by bots.
    #[garde(skip)]
    pub phone: String,
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
    form: Store<ContactForm>,
}

impl FormController {
    /// Custom hook for contact form submission logic
    ///
    /// Returns a FormController that encapsulates form state and submission logic
    pub(super) fn use_contact_form_submission() -> Self {
        let form_state = use_signal(FormState::default);
        let form = use_store(ContactForm::default);
        Self { form_state, form }
    }

    /// Returns the current submission state
    pub(super) fn state(&self) -> ReadSignal<FormState> {
        self.form_state.boxed()
    }

    /// Returns the form store, giving access to individual field lenses
    pub(super) fn form(&self) -> Store<ContactForm> {
        self.form
    }

    /// Clear all form fields and reset submission state back to Idle
    pub(super) fn reset(&self) {
        self.form.name().take();
        self.form.email().take();
        self.form.subject().take();
        self.form.message().take();
        self.form.phone().take();
        let mut form_state = self.form_state;
        form_state.set(FormState::default());
    }

    /// Handle form submission event
    pub(super) async fn submit(&self, evt: FormEvent) {
        evt.prevent_default();
        // keep form state updated
        let mut form_state = self.form_state;
        form_state.set(FormState::Submitting);

        // Retrieve form, validate, and send email
        let form = self.form.peek().cloned();
        if let Err(e) = form.validate() {
            form_state.set(FormState::ClientError(e.to_string()));
            return;
        }
        match api::send_contact_email(Form(form)).await {
            Ok(_) => form_state.set(FormState::Success),
            Err(e) => form_state.set(FormState::ServerError(e.to_string())),
        };
    }
}

impl components::toast::ToastDispatcher for FormController {
    /// Send a toast notification based on internal state (usually [`Signal`])
    fn notify(&self) {
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
