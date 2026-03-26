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
            title: "Connect — Hire a Rust Software Engineer",
            description: "Get in touch with Gaëtan POBLON for Rust software engineering, fullstack development, or technical consulting. Available for full-time positions, freelance contracts, and professional collaboration. Based in France, working worldwide.",
            canonical_path: "/contact",
            schema_type: "ContactPage",
            schema_keywords: vec![
                "Contact Gaëtan POBLON".into(),
                "Hire Rust Developer France".into(),
                "Freelance Rust Software Engineer".into(),
                "Fullstack Rust Developer".into(),
                "Software Architecture Consulting".into(),
                "Rust Development Services".into(),
                "Cross-platform Developer".into(),
                "Technical Consulting France".into(),
            ],
            schema_data: json!({
                "mainEntity": {
                    "@type": "Person",
                    "@id": format!("{}/#person", SITE_URL),
                    "email": AUTHOR_EMAIL,
                    "url": SITE_URL,
                    "address": {
                        "@type": "PostalAddress",
                        "addressLocality": "Niort",
                        "addressRegion": "Nouvelle-Aquitaine",
                        "addressCountry": "FR"
                    },
                    "sameAs": [
                        AUTHOR_GITHUB,
                        AUTHOR_LINKEDIN,
                    ],
                    "contactPoint": {
                        "@type": "ContactPoint",
                        "email": AUTHOR_EMAIL,
                        "contactType": "Professional Inquiries",
                        "availableLanguage": ["English", "French"],
                        "areaServed": "Worldwide"
                    }
                }
            }),
        }
        features::contact::ContactForm {
            header: rsx! {
                div {
                    class: "p-6",
                    h1 {
                        class: "md:col-start-1 md:row-start-1 text-connect pb-4 text-base font-normal",
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
