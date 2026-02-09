use dioxus::prelude::*;
use features::social::SocialLink;
use kernel::lang::t;

/// Connect page component
///
/// Displays a contact form allowing visitors to reach out.
/// Uses the contact feature for validation and email sending.
#[component]
pub fn Connect() -> Element {
    rsx! {
        features::contact::ContactForm {
            header: rsx! {
                div {
                    class: "p-6",
                    h4 {
                        class: "dark:text-connect md:col-start-1 md:row-start-1",
                        {t!("connect_title")}
                    }
                    p {
                        {t!("connect_description")}
                    }
                }
            },
            ad_slot: rsx! {
                p {
                    class: "pt-4 pl-8 text-muted",
                    {t!("connect_alternative")}
                }
                SocialLink {
                    prefix: "00",
                    label: "EMAIL",
                    href: "mailto:hello@gpoblon.net",
                    alt: "hello@gpoblon.net"
                }
                SocialLink {
                    prefix: "01",
                    label: "GITHUB",
                    href: "https://github.com/gpoblon"
                }
                SocialLink {
                    prefix: "02",
                    label: "LINKEDIN",
                    href: "https://linkedin.com/in/gpoblon"
                }
            }
        }
    }
}
