use dioxus::prelude::*;
use features::social::SocialLink;
use kernel::lang::t;
use kernel::seo::{AUTHOR_EMAIL, AUTHOR_GITHUB, AUTHOR_LINKEDIN, SITE_URL};
use serde_json::json;

/// Connect page component
///
/// Displays a contact form allowing visitors to reach out.
/// Uses the contact feature for validation and email sending.
#[component]
pub fn Connect() -> Element {
    rsx! {
        components::Seo {
            title: "Contact",
            description: "Get in touch with Gaetan POBLON. Whether for professional collaboration, freelance projects, or a casual chat between enthusiasts.",
            canonical_path: "/contact",
            schema_type: "ContactPage",
            schema_keywords: vec![
                "Contact Gaetan Poblon".into(),
                "Hire Rust Developer".into(),
                "Freelance Software Engineer".into(),
                "Fullstack Web Development".into(),
                "Software Architecture Consulting".into(),
                "France".into(),
            ],
            schema_data: json!({
                "mainEntity": {
                    "@type": "Person",
                    "@id": format!("{}/#person", SITE_URL),
                    "email": AUTHOR_EMAIL,
                    "url": SITE_URL,
                    "sameAs": [
                        AUTHOR_GITHUB,
                        AUTHOR_LINKEDIN,
                    ],
                    "contactPoint": {
                        "@type": "ContactPoint",
                        "email": AUTHOR_EMAIL,
                        "contactType": "Professional Inquiries",
                        "availableLanguage": ["English", "French"]
                    }
                }
            }),
        }
        features::contact::ContactForm {
            header: rsx! {
                div {
                    class: "p-6",
                    h4 {
                        class: "md:col-start-1 md:row-start-1 text-connect pb-4",
                        {t!("connect_title")}
                    }
                    p {
                        {t!("connect_description")}
                    }
                }
            },
            ad_slot: rsx! {
                div {
                    class: "flex flex-col gap-4 pt-8",
                    p {
                        class: "pl-8 text-muted-foreground",
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
}
