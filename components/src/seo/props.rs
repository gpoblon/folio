//! [`SeoProps`] — the single input type consumed by the [`super::Seo`] component.

use dioxus::prelude::*;
use kernel::lang::Lang;
use kernel::seo::Keywords;
use serde_json::json;

// ── Props ─────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Props)]
pub struct SeoProps {
    /// Page title — suffixed with ` | {SITE_NAME}` at render time.
    pub title: String,
    /// Meta description for SERP snippet.
    pub description: String,
    /// Canonical path, e.g. `"/lab"` (prefixed with `SITE_URL` at render time).
    pub canonical_path: String,

    /// Open Graph type — defaults to `"website"`.
    /// Valid values: `"website"`, `"article"`, `"profile"`.
    #[props(default = "website".to_string())]
    pub og_type: String,
    /// Override the default OG image URL.
    #[props(optional)]
    pub og_image: Option<String>,
    /// Alt text for the OG image.
    #[props(optional)]
    pub og_image_alt: Option<String>,

    /// Primary locale, derived from [`kernel::lang::Lang`].
    #[props(default = Lang::default())]
    pub locale: Lang,

    /// Alternate path (same page, other language) for hreflang link.
    #[props(optional)]
    pub alternate_path: Option<String>,

    /// Schema.org `@type`, e.g. `"WebSite"`, `"Person"`, `"TechArticle"`.
    #[props(default = "WebSite")]
    pub schema_type: &'static str,

    /// Explicit keyword strings for structured data.
    ///
    /// Slug-derived keywords are always merged in automatically.
    ///
    /// Keywords are emitted both as a comma-separated `"keywords"` string and
    /// as an `"about"` array of `{ "@type": "Thing", "name": … }` nodes in the
    /// JSON-LD graph.  While standard search engines rely on the string form,
    /// LLMs and GEO crawlers lean heavily on the `"about"` array to map topics
    /// to semantic entities.
    ///
    /// For stronger GEO entity disambiguation you can override the
    /// auto-generated `"about"` nodes by supplying them explicitly in
    /// [`SeoProps::schema_data`].  Available `@type` values to consider:
    ///
    /// | `@type`                | Use for                                                      |
    /// |------------------------|--------------------------------------------------------------|
    /// | `"ComputerLanguage"`   | Programming languages (Rust, Python, C++…)                   |
    /// | `"SoftwareApplication"`| Frameworks, libraries, databases (Dioxus, Axum, SurrealDB…) |
    /// | `"DefinedTerm"`        | Methodologies, standards, disciplines (DDD, WebAssembly…)    |
    /// | `"Place"`              | Locations (France, Niort…)                                   |
    /// | `"Thing"`              | Default / catch-all                                          |
    #[props(optional)]
    pub schema_keywords: Option<Vec<String>>,

    /// Free-form JSON-LD fields merged into the base object **after** all
    /// automatic fields. Keys here overwrite automatic values on conflict.
    #[props(optional)]
    pub schema_data: Option<serde_json::Value>,

    /// ISO 8601 `datePublished` for JSON-LD and `article:published_time`.
    #[props(optional)]
    pub date_published: Option<String>,
    /// ISO 8601 `dateModified` for JSON-LD and `article:modified_time`.
    #[props(optional)]
    pub date_modified: Option<String>,

    /// `robots` directive — defaults to `"index, follow"`.
    #[props(default = "index, follow".to_string())]
    pub robots: String,

    /// Explicit article tags for `article:tag` OG meta and RSS categories.
    #[props(optional)]
    pub article_tags: Option<Vec<String>>,
}

impl Default for SeoProps {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            canonical_path: String::new(),
            og_type: "website".into(),
            og_image: None,
            og_image_alt: None,
            locale: Lang::default(),
            alternate_path: None,
            schema_type: "WebSite",
            schema_keywords: None,
            schema_data: None,
            date_published: None,
            date_modified: None,
            robots: "index, follow".into(),
            article_tags: None,
        }
    }
}

// ── Constructors ──────────────────────────────────────────────────────────────

impl SeoProps {
    /// Build props for a single **article** page.
    pub fn for_article(
        title: &str,
        description: &str,
        canonical_path: &str,
        tags: &[String],
        slug_segments: &[String],
        created: Option<String>,
        modified: Option<String>,
    ) -> Self {
        let keywords = Keywords::new()
            .with_explicit(tags.to_vec())
            .with_explicit(slug_segments.to_vec());

        Self {
            title: title.into(),
            description: description.into(),
            canonical_path: canonical_path.into(),
            og_type: "article".into(),
            schema_type: "TechArticle",
            schema_keywords: Some(keywords.into_inner()),
            date_published: created.map(|d| ensure_tz(&d)),
            date_modified: modified.map(|d| ensure_tz(&d)),
            article_tags: Some(tags.to_vec()),
            ..Self::default()
        }
    }

    /// Build props for a single **project** page.
    pub fn for_project(
        title: &str,
        description: &str,
        canonical_path: &str,
        tags: &[String],
        created: Option<String>,
        modified: Option<String>,
        repo_url: Option<&str>,
    ) -> Self {
        let schema_data = repo_url.map(|url| {
            json!({
                "codeRepository": url,
                "programmingLanguage": "Rust",
            })
        });

        Self {
            title: title.into(),
            description: description.into(),
            canonical_path: canonical_path.into(),
            og_type: "website".into(),
            schema_type: "SoftwareSourceCode",
            schema_keywords: Some(tags.to_vec()),
            date_published: created.map(|d| ensure_tz(&d)),
            date_modified: modified.map(|d| ensure_tz(&d)),
            schema_data,
            article_tags: Some(tags.to_vec()),
            ..Self::default()
        }
    }

    /// Fallback props for the article detail page when metadata hasn't loaded.
    pub fn article_fallback(canonical_path: &str, slug_segments: &[String]) -> Self {
        let mut kw = slug_segments.to_vec();
        kw.extend([
            "Rust".into(),
            "Software Architecture".into(),
            "Software Engineering".into(),
        ]);

        Self {
            title: "Article — Rust & Software Engineering".into(),
            description: "A technical deep-dive article by Gaëtan POBLON covering Rust, software architecture, Domain-Driven Design, cross-platform development, or modern engineering practices. Practical insights from building production systems.".into(),
            canonical_path: canonical_path.into(),
            og_type: "article".into(),
            schema_type: "TechArticle",
            schema_keywords: Some(kw),
            schema_data: Some(json!({
                "about": [
                    {"@type": "ComputerLanguage", "name": "Rust Programming Language"},
                    {"@type": "DefinedTerm", "name": "Software Architecture"}
                ]
            })),
            ..Self::default()
        }
    }

    /// Fallback props for the project detail page when metadata hasn't loaded.
    pub fn project_fallback(canonical_path: &str) -> Self {
        Self {
            title: "Project — Open-Source Rust Software".into(),
            description: "An open-source project by Gaëtan POBLON, Rust Software Engineer — built with Dioxus, Axum, or WebAssembly for production-grade cross-platform applications and developer tools.".into(),
            canonical_path: canonical_path.into(),
            og_type: "website".into(),
            schema_type: "SoftwareSourceCode",
            schema_keywords: Some(vec![
                "Rust".into(),
                "Open Source".into(),
                "Cross-platform".into(),
                "Dioxus".into(),
                "Axum".into(),
            ]),
            schema_data: Some(json!({
                "programmingLanguage": "Rust",
                "about": [
                    {"@type": "ComputerLanguage", "name": "Rust Programming Language"},
                    {"@type": "DefinedTerm", "name": "Open Source Software"}
                ]
            })),
            ..Self::default()
        }
    }

    /// Returns `true` when this page is an OG article type.
    pub fn is_article(&self) -> bool {
        self.og_type == "article"
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Ensures an ISO 8601 datetime string has a timezone offset.
/// Google requires `datePublished` / `dateModified` to include a timezone.
/// Appends `+00:00` when no offset or `Z` suffix is present.
fn ensure_tz(date: &str) -> String {
    let trimmed = date.trim();
    if trimmed.ends_with('Z') || trimmed.contains('+') || trimmed.rfind('-').is_some_and(|i| i > 9)
    {
        trimmed.to_string()
    } else {
        format!("{trimmed}+00:00")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_tz_appends_offset() {
        assert_eq!(
            ensure_tz("2025-01-01T00:00:00"),
            "2025-01-01T00:00:00+00:00"
        );
    }

    #[test]
    fn ensure_tz_preserves_z() {
        assert_eq!(ensure_tz("2025-01-01T00:00:00Z"), "2025-01-01T00:00:00Z");
    }

    #[test]
    fn ensure_tz_preserves_existing_offset() {
        assert_eq!(
            ensure_tz("2025-01-01T00:00:00+02:00"),
            "2025-01-01T00:00:00+02:00"
        );
    }

    #[test]
    fn ensure_tz_negative_offset() {
        assert_eq!(
            ensure_tz("2025-01-01T12:00:00-05:00"),
            "2025-01-01T12:00:00-05:00"
        );
    }
}
