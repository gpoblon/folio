use dioxus::prelude::*;
use features::social::SocialLink;
use kernel::lang::t;
use serde_json::json;

/// Connect page component
///
/// Displays a contact form allowing visitors to reach out.
/// Uses the contact feature for validation and email sending.
#[component]
pub fn Connect() -> Element {
    rsx! {
        components::Seo {
            title: "Connect — Contact",
            description: "Get in touch with Gaetan POBLON. Whether for professional collaboration, project ideas, or a casual chat between enthusiasts.",
            canonical_path: "/contact",
            schema_type: "ContactPage",
            schema_keywords: vec![
                "Contact".into(),
                "Software Engineering".into(),
                "Fullstack web development".into(),
                "Software architecture".into(),
                "Open source".into(),
                "Artificial Intelligence".into(),
                "Rust".into(),
                "Axum".into(),
                "Dioxus".into(),
                "SurrealDB".into()
            ],
            schema_data: json!({
                "contactPoint": {
                    "@type": "ContactPoint",
                    "email": "hello@gpoblon.net",
                    "contactType": "professional inquiries",
                    "availableLanguage": ["English", "French"]
                }
            }),
        }
        features::contact::ContactForm {
            header: rsx! {
                div {
                    class: "p-6",
                    h4 {
                        class: "md:col-start-1 md:row-start-1 text-connect",
                        {t!("connect_title")}
                    }
                    p {
                        {t!("connect_description")}
                    }
                }
            },
            ad_slot: rsx! {
                p {
                    class: "pt-4 pl-8 text-muted-foreground",
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
