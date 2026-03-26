use components::Seo;
use dioxus::prelude::*;
use entities::nav::{Cardinal, CardinalCell};
use features::lang::detect_preferred_language;
use kernel::lang::Lang;
use kernel::seo::SITE_URL;
use serde_json::json;
use widgets::home_center::CenterCell;

const HOME_CSS: Asset = asset!("./home.css");

// ── Route wrappers ────────────────────────────────────────────────────────────

/// `/` — main user-facing home page.
///
/// The preferred language is resolved server-side from the `Accept-Language`
/// request header during SSR and embedded in the HTML payload.  After
/// hydration the client reads the pre-resolved value — no JS eval and no
/// extra network round-trip needed.
///
/// The visitor always stays on `/`.  Language can be switched at any time
/// via the dropdown, exactly like every other page.
#[component]
pub fn Home() -> Element {
    let lang_resource = use_server_future(detect_preferred_language)?;
    let mut initialized = use_signal(|| false);

    // Set the detected locale exactly once (SSR + first hydration render).
    // On later re-renders the guard is true, so the user's dropdown choice
    // is never overwritten.
    if !*initialized.peek() {
        let detected_lang = match &*lang_resource.read() {
            Some(Ok(code)) if code.starts_with("en") => Lang::English,
            _ => Lang::French,
        };
        set_locale(detected_lang);
        initialized.set(true);
    }

    rsx! { HomeContent { is_default: true } }
}

/// `/fr` — French home page exposed for SEO crawlers.
#[component]
pub fn HomeFr() -> Element {
    set_locale(Lang::French);
    rsx! { HomeContent { is_default: false } }
}

/// `/en` — English home page exposed for SEO crawlers.
#[component]
pub fn HomeEn() -> Element {
    set_locale(Lang::English);
    rsx! { HomeContent { is_default: false } }
}

// ── Locale helper ─────────────────────────────────────────────────────────────

/// Sets the global i18n language if it differs from the target.
fn set_locale(lang: Lang) {
    let mut i18n = kernel::lang::use_i18n();
    let target =
        kernel::lang::LanguageIdentifier::from_bytes(lang.code().as_bytes()).expect("valid lang");
    if i18n.language() != target {
        i18n.set_language(target);
    }
}

// ── Content + SEO ─────────────────────────────────────────────────────────────

#[component]
fn HomeContent(is_default: bool) -> Element {
    let lang = kernel::lang::use_lang();
    let seo = home_seo(lang, is_default);

    let mut hovered: Signal<Option<Cardinal>> = use_signal(|| None);
    let nav = use_navigator();
    let active = hovered().unwrap_or(Cardinal::Identity);

    let sr_h1 = match lang {
        Lang::French => "Gaëtan POBLON — Développeur Logiciel Rust",
        Lang::English => "Gaetan POBLON — Rust Software Engineer",
    };

    rsx! {
        Seo { ..seo }

        document::Link { rel: "stylesheet", href: HOME_CSS }

        section {
            id: "home",
            class: "home-grid flex-1 max-h-[calc(100dvh-9rem)]",

            h1 { class: "sr-only", "{sr_h1}" }

            for c in Cardinal::NAV.iter() {
                CardinalCell {
                    key: "{c.label()}",
                    cardinal: *c,
                    is_active: hovered() == Some(*c),
                    onhover: move |val| hovered.set(val),
                    onclick: move |c: Cardinal| { nav.push(c.route()); },
                }
            }

            CenterCell { active }
        }
    }
}

// ── Per-locale SEO builder ────────────────────────────────────────────────────

fn home_seo(lang: Lang, is_default: bool) -> components::SeoProps {
    let (title, description, canonical_path, locale, alternate_path, robots) = match lang {
        Lang::French => (
            "Développeur Logiciel Rust — Fullstack & Cross-Platform",
            "Gaëtan POBLON — Développeur Logiciel Rust installé à Niort.
            Profil transverse : architecture, produit & lead. J'aligne acteurs et technique pour servir l'UX et les besoins business.",
            if is_default { "/" } else { "/fr" },
            Lang::French,
            Some("/en"),
            "index, follow",
        ),
        Lang::English => (
            "Rust Software Engineer — Fullstack & Cross-Platform",
            "Gaëtan POBLON — Rust Software Developer based in France.
            Cross-functional: architecture, product & lead. Aligning stakeholders and tech to serve UX and business needs.",
            if is_default { "/" } else { "/en" },
            Lang::English,
            Some("/fr"),
            "index, follow",
        ),
    };

    let schema_keywords = match lang {
        Lang::French => vec![
            // ID
            "Gaëtan POBLON",
            "Gaetan Poblon",
            "Développeur Niort",
            "Développeur France",
            // Tech
            "Développeur Rust",
            "Développeur Logiciel",
            "Développeur Fullstack",
            "Développeur Cross-platform",
            "Développeur API Axum",
            "Développeur GUI Dioxus",
            "WebAssembly",
            // Role
            "Lead Tech",
            "Software Engineering",
            "Architecte Logiciel",
            "Freelance Rust",
            // Specifics
            "Developer Experience (DX)",
            "User Centric Development (UX)",
            "Product-oriented Engineering",
            "Automation",
            "École 42",
        ],
        Lang::English => vec![
            // ID
            "Gaëtan POBLON",
            "Gaetan Poblon",
            "Software Developer Niort",
            "Software Engineer France",
            // Tech
            "Rust Developer",
            "Software Developer",
            "Fullstack Developer",
            "Cross-platform Developer",
            "Axum API Development",
            "Dioxus GUI Development",
            "WebAssembly",
            // Role
            "Tech Lead",
            "Software Engineering",
            "Software Architect",
            "Rust Freelancer",
            // Specifics
            "Developer Experience (DX)",
            "User-Centric Development (UX)",
            "Product-oriented Engineering",
            "Automation",
            "42 School",
        ],
    };

    // All structured author data (name, bio, career, education, address, sameAs, …)
    // is already embedded in the Person entity by component::Seo via author_node().
    // Only the home-page-specific availability signal belongs here.
    let schema_data = json!({
        "seeks": {
            "@type": "Demand",
            "itemOffered": {
                "@type": "Service",
                "name": "Rust Software Engineering",
                "description": "Available for full-time positions or freelance contracts in Rust development — backend, frontend (Dioxus), fullstack, cross-platform, compiler tooling, and product engineering.",
                "areaServed": "Worldwide",
                "provider": {
                    "@type": "Person",
                    "@id": format!("{SITE_URL}/#person"),
                }
            }
        }
    });

    components::SeoProps {
        title: title.into(),
        description: description.into(),
        canonical_path: canonical_path.into(),
        schema_type: "Person",
        locale,
        alternate_path: alternate_path.map(Into::into),
        schema_keywords: Some(
            schema_keywords
                .into_iter()
                .map(ToString::to_string)
                .collect(),
        ),
        schema_data: Some(schema_data),
        robots: robots.into(),
        ..Default::default()
    }
}
