use super::model;
use dioxus::prelude::*;
use kernel::lang::t;

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
    let form_controller = model::use_contact_form_submission();
    let is_submitting = form_controller.is_submitting();

    rsx! {
        div {
            class: "flex-1 flex flex-col justify-center items-center",
            form {
                class: "grid flex-1 gap-2 grid-cols-1 md:grid-cols-[2fr_1fr] md:grid-rows-[auto_auto_1fr]
                        w-full max-w-[1480px] h-full max-h-[690px]
                        outline-3 outline-[var(--color-connect)] shadow-2xl shadow-black",
                onsubmit: move |evt: FormEvent| form_controller.handle_submit(evt),
                { header } // Row 1, Col 1: Header section
                IdentityInputs {} // Row 2, Col 1: Email and Name inputs
                SubjectInput {} // Row 1-2, Col 2: Subject input
                div { // Row 3: Message and actions
                    class: "flex flex-col flex-1 sm:flex-row md:col-span-2 md:row-start-3",
                    MessageInput {}
                    div {
                        class: "flex flex-col w-full sm:w-2/5",
                        SendButton { is_submitting }
                        div {
                            class: "flex-1 block w-full pr-2",
                            { ad_slot }
                        }
                    }
                }
            }
        }
    }
}

/// Identity section with email and name inputs
#[component]
fn IdentityInputs() -> Element {
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
fn SubjectInput() -> Element {
    rsx! {
        div {
            class: "flex flex-col rounded-bl-4xl bg-[var(--color-connect)]/25 md:col-start-2 md:row-start-1 md:row-span-2",
            input {
                class: "mt-auto p-6 rounded-bl-4xl",
                r#type: "text",
                id: "subject",
                name: "subject",
                required: true,
                placeholder: t!("connect_subject"),
            }
        }
    }
}

/// Message textarea and action buttons section
#[component]
fn MessageInput() -> Element {
    rsx! {
        textarea {
            class: "w-full sm:w-3/5 p-6 rounded-tr-4xl bg-[var(--color-projects)]/25",
            id: "message",
            name: "message",
            required: true,
            placeholder: t!("connect_message"),
        }
    }
}

#[component]
fn SendButton(is_submitting: bool) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full",
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
                        components::Icon {
                            class: "animate-spin text-xl sm:text-3xl",
                            icon: components::Icons::Sync
                        }
                    } else {
                        {t!("connect_send")}
                    }
                }
            }
        }
    }
}
