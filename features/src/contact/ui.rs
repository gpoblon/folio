use std::time::Duration;

use super::{api, model};
use dioxus::prelude::*;
use kernel::lang::t;

/// State for the contact form submission
#[derive(Debug, Clone, PartialEq, Default)]
pub enum FormState {
    #[default]
    Idle,
    Submitting,
    Success,
    Error(String),
}

/// Contact form widget with email, name, subject, message fields and social links
#[component]
pub fn ContactForm(
    /// Top Left location to put some encouraging message, or a title.
    #[props(optional)]
    header: Option<Element>,
    /// Bottom Right location to put some advertising / links.
    #[props(optional)]
    ad_slot: Option<Element>,
) -> Element {
    let mut form_state = use_signal(|| FormState::default());

    let onsubmit = move |evt: FormEvent| async move {
        evt.prevent_default();
        form_state.set(FormState::Submitting);
        // let toast = components::toast::consume_toast();

        if let Ok(form_data) = evt.parsed_values::<model::ContactFormData>() {
            match api::send_contact_email(form_data).await {
                Ok(api::ContactResult::Success) => {
                    form_state.set(FormState::Success);
                }
                Ok(api::ContactResult::ValidationError(msg)) => {
                    form_state.set(FormState::Error(msg));
                }
                Ok(api::ContactResult::SendError(msg)) => {
                    form_state.set(FormState::Error(msg));
                }
                Err(e) => {
                    form_state.set(FormState::Error(e.to_string()));
                }
            }
        } else {
            form_state.set(FormState::Error(t!("connect_send_error_parser")));
        }
        // match *form_state.peek() {
        //     FormState::Success => toast.success(
        //         t!("connect_send_success"),
        //         components::toast::ToastOptions::new()
        //             .duration(Duration::from_secs(5))
        //             .permanent(false),
        //     ),
        //     FormState::Error(ref msg) => toast.error(
        //         msg.to_string(),
        //         components::toast::ToastOptions::new()
        //             .duration(Duration::from_secs(5))
        //             .permanent(false),
        //     ),
        //     _ => (),
        // };
    };

    let is_submitting = matches!(form_state(), FormState::Submitting);

    rsx! {
        components::toast::ToastProvider {
            form {
                class: "grid flex-1 gap-2 grid-cols-1 md:grid-cols-[2fr_1fr] md:grid-rows-[auto_auto_1fr]",
                onsubmit,

                // Row 1, Col 1: Header section
                { header }

                // Row 2, Col 1: Email and Name inputs
                ContactFormIdentity {}

                // Row 1-2, Col 2: Subject input
                ContactFormSubject {}

                // Row 3: Message and actions
                ContactFormMessageSection { is_submitting, ad_slot }
            }
        }
    }
}

/// Status message component for form feedback
#[component]
fn FormStatusMessage(message: String, is_error: bool) -> Element {
    rsx! {
        div {
            class: "md:col-span-2 p-4 rounded-lg bg-green-500/25 data-[error=true]:bg-ref-500/25",
            "data-error": is_error,
            p { class: "text-center", {message} }
        }
    }
}

/// Identity section with email and name inputs
#[component]
fn ContactFormIdentity() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 sm:flex-row md:col-start-1 md:row-start-2",

            input {
                class: "flex-1 p-6 rounded-full bg-[var(--color-experience)]/25 md:rounded-l-none",
                r#type: "email",
                id: "email",
                name: "email",
                required: true,
                placeholder: t!("connect_email"),
            }
            input {
                class: "flex-1 p-6 rounded-full bg-[var(--color-experience)]/25",
                r#type: "text",
                id: "name",
                name: "name",
                required: true,
                placeholder: t!("connect_name"),
            }
        }
    }
}

/// Subject input section
#[component]
fn ContactFormSubject() -> Element {
    rsx! {
        div {
            class: "flex flex-col rounded-bl-4xl bg-[var(--color-connect)]/25 md:col-start-2 md:row-start-1 md:row-span-2",
            input {
                class: "mt-auto p-6 rounded-bl-4xl",
                r#type: "text",
                id: "subject",
                name: "subject",
                required: true,
                minlength: 12,
                placeholder: t!("connect_subject"),
            }
        }
    }
}

/// Message textarea and action buttons section
#[component]
fn ContactFormMessageSection(
    is_submitting: bool,
    #[props(optional)] ad_slot: Option<Element>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col flex-1 sm:flex-row md:col-span-2 md:row-start-3",

            textarea {
                class: "w-full sm:w-3/5 p-6 rounded-tr-4xl bg-[var(--color-projects)]/25",
                id: "message",
                name: "message",
                required: true,
                minlength: 24,
                placeholder: t!("connect_message"),
            }

            div {
                class: "flex flex-col w-full sm:w-2/5",

                // Send button
                button {
                    class: "h-20 rounded-l-4xl bg-[var(--color-knowledge)]/50 ml-2 disabled:opacity-50",
                    r#type: "submit",
                    disabled: is_submitting,
                    div {
                        class: "flex flex-row items-center justify-center gap-2 h-full",
                        components::Icon {
                            class: "rotate-325 text-xl sm:text-3xl",
                            icon: components::Icons::Send
                        }
                        if is_submitting {
                            "Sending..."
                        } else {
                            {t!("connect_send")}
                        }
                    }
                }

                div {
                    class: "flex-1 block w-full",
                    { ad_slot }
                }
            }
        }
    }
}
