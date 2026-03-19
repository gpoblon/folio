use dioxus::prelude::*;
use kernel::lang::Lang;
use kernel::seo::{
    AUTHOR_GITHUB, AUTHOR_JOB_TITLE, AUTHOR_LINKEDIN, AUTHOR_NAME, DEFAULT_OG_IMAGE, SITE_NAME,
    SITE_URL,
};
use serde_json::{Value, json};

// ── Author / publisher ────────────────────────────────────────────────────────

/// The single canonical `Person` node that represents the site author.
/// Injected automatically into every JSON-LD object so pages never repeat it.
fn author_node() -> Value {
    json!({
        "@type": "Person",
        "@id": format!("{}/#person", SITE_URL),
        "name": AUTHOR_NAME,
        "jobTitle": AUTHOR_JOB_TITLE,
        "url": SITE_URL,
        "sameAs": [
            AUTHOR_GITHUB,
            AUTHOR_LINKEDIN,
        ]
    })
}

// ── Slug keyword extraction ───────────────────────────────────────────────────

/// Extracts implicit keywords from a slug by splitting on `/` and stripping
/// the file extension from the last segment.
///
/// `"it/dev/lang/rust/intro.md"` → `["it", "dev", "lang", "rust", "intro"]`
fn slug_keywords(slug: &str) -> Vec<String> {
    let segments: Vec<&str> = slug.split('/').filter(|s| !s.is_empty()).collect();
    let last = segments.len().saturating_sub(1);
    segments
        .into_iter()
        .enumerate()
        .map(|(i, seg)| {
            if i == last {
                seg.rsplit_once('.').map(|(name, _)| name).unwrap_or(seg)
            } else {
                seg
            }
            .to_string()
        })
        .collect()
}

/// Merges `extra` keywords into `base`, deduplicating case-insensitively while
/// preserving insertion order.
fn merge_keywords(base: &mut Vec<String>, extra: &[String]) {
    for keyword in extra {
        let normalised = keyword.trim().to_lowercase();
        if !base.iter().any(|k| k.trim().to_lowercase() == normalised) {
            base.push(keyword.clone());
        }
    }
}

// ── Breadcrumb ────────────────────────────────────────────────────────────────

/// Builds a `BreadcrumbList` JSON-LD node from the canonical path.
///
/// `/lab/my-project` → Home > Lab > my-project
fn breadcrumb_list(canonical_path: &str, page_title: &str) -> Value {
    let mut items = vec![json!({
        "@type": "ListItem",
        "position": 1,
        "name": "Home",
        "item": SITE_URL,
    })];

    let segments: Vec<&str> = canonical_path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let mut accumulated = String::new();
    for (i, segment) in segments.iter().enumerate() {
        accumulated.push('/');
        accumulated.push_str(segment);
        let position = (i + 2) as u32;

        // Last segment uses the page title; intermediate segments use the
        // capitalised slug segment as a readable label.
        let name = if i == segments.len() - 1 {
            page_title.to_string()
        } else {
            let mut chars = segment.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => segment.to_string(),
            }
        };

        items.push(json!({
            "@type": "ListItem",
            "position": position,
            "name": name,
            "item": format!("{}{}", SITE_URL, accumulated),
        }));
    }

    json!({
        "@type": "BreadcrumbList",
        "itemListElement": items,
    })
}

// ── Props ─────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Props)]
pub struct SeoProps {
    /// Page title — will be suffixed with " | {SITE_NAME}"
    pub title: String,
    /// Meta description for SERP snippet
    pub description: String,
    /// Canonical path, e.g. "/lab" (will be prefixed with SITE_URL)
    pub canonical_path: String,

    /// Open Graph type — defaults to "website"
    #[props(default = "website".to_string())]
    pub og_type: String,
    /// Override the default OG image URL
    #[props(optional)]
    pub og_image: Option<String>,
    /// Alt text for the OG image (used by `og:image:alt`)
    #[props(optional)]
    pub og_image_alt: Option<String>,

    /// Primary locale, derived from `kernel::lang::Lang`.
    #[props(default = Lang::default())]
    pub locale: Lang,

    /// Alternate path (same page, other language) for hreflang link
    #[props(optional)]
    pub alternate_path: Option<String>,

    /// Schema.org `@type` as a static string, e.g. "WebSite", "Person",
    /// "TechArticle", "SoftwareSourceCode". Defaults to "WebSite".
    #[props(default = "WebSite")]
    pub schema_type: &'static str,

    /// Plain keyword strings for structured data.
    ///
    /// The component uses these to:
    /// - emit a `"keywords"` meta tag (comma-joined)
    /// - set the `"keywords"` field in JSON-LD
    ///
    /// Slug segments are always merged in automatically as implicit keywords,
    /// so even passing an empty `Vec` yields useful entries for well-known
    /// path segments like `"rust"` or `"dioxus"`.
    #[props(optional)]
    pub schema_keywords: Option<Vec<String>>,

    /// Free-form JSON-LD fields merged into the base object **after** all
    /// automatic fields are set.
    ///
    /// Use this for page-specific fields that the component cannot derive
    /// on its own (`"seeks"`, `"contactPoint"`, `"codeRepository"`, …).
    /// Keys provided here overwrite any automatic value if they conflict.
    #[props(optional)]
    pub schema_data: Option<serde_json::Value>,

    /// ISO 8601 date string for `datePublished` in JSON-LD.
    /// Typically comes from article/project metadata.
    #[props(optional)]
    pub date_published: Option<String>,
    /// ISO 8601 date string for `dateModified` in JSON-LD.
    #[props(optional)]
    pub date_modified: Option<String>,

    /// robots directive — defaults to "index, follow"
    #[props(default = "index, follow".to_string())]
    pub robots: String,
}

// ── SeoProps constructors for dynamic pages ───────────────────────────────────
//
// These associated functions centralise the Metadata → SeoProps conversion so
// that page components stay thin.  They accept only primitive / standard types
// (no entity imports) to respect FSD layering: components (L6) must not depend
// on entities (L5).

impl SeoProps {
    /// Build `SeoProps` for a single **article** page.
    ///
    /// `tags` are the raw tag/intent key strings from the article metadata.
    /// `slug_segments` are the URL path segments (used as fallback keywords).
    pub fn for_article(
        title: &str,
        description: &str,
        canonical_path: &str,
        tags: &[String],
        slug_segments: &[String],
        created: Option<String>,
        modified: Option<String>,
    ) -> Self {
        let mut keywords: Vec<String> = tags.to_vec();
        merge_keywords(&mut keywords, slug_segments);

        Self {
            title: title.to_string(),
            description: description.to_string(),
            canonical_path: canonical_path.to_string(),
            og_type: "article".to_string(),
            schema_type: "TechArticle",
            schema_keywords: Some(keywords),
            date_published: created,
            date_modified: modified,
            // defaults
            og_image: None,
            og_image_alt: None,
            locale: Lang::default(),
            alternate_path: None,
            schema_data: None,
            robots: "index, follow".to_string(),
        }
    }

    /// Build `SeoProps` for a single **project** page.
    ///
    /// `tags` are the raw tag/intent key strings from the project metadata.
    /// `repo_url` is the full GitHub (or other) repository URL, if available.
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
            title: title.to_string(),
            description: description.to_string(),
            canonical_path: canonical_path.to_string(),
            og_type: "website".to_string(),
            schema_type: "SoftwareSourceCode",
            schema_keywords: Some(tags.to_vec()),
            date_published: created,
            date_modified: modified,
            schema_data,
            // defaults
            og_image: None,
            og_image_alt: None,
            locale: Lang::default(),
            alternate_path: None,
            robots: "index, follow".to_string(),
        }
    }

    /// Fallback `SeoProps` for the article list page when no metadata has
    /// loaded yet, or the server function failed.
    pub fn article_fallback(canonical_path: &str, slug_segments: &[String]) -> Self {
        Self {
            title: "Article — Knowledge Base".to_string(),
            description: "A technical article by Gaetan POBLON on Rust, Dioxus, software architecture, or engineering practices.".to_string(),
            canonical_path: canonical_path.to_string(),
            og_type: "article".to_string(),
            schema_type: "TechArticle",
            schema_keywords: Some(slug_segments.to_vec()),
            // defaults
            date_published: None,
            date_modified: None,
            og_image: None,
            og_image_alt: None,
            locale: Lang::default(),
            alternate_path: None,
            schema_data: None,
            robots: "index, follow".to_string(),
        }
    }

    /// Fallback `SeoProps` for a single project page when no metadata has
    /// loaded yet, or the server function failed.
    pub fn project_fallback(canonical_path: &str) -> Self {
        Self {
            title: "Project — Lab".to_string(),
            description:
                "An open-source project by Gaetan POBLON built with Rust, Dioxus, or WebAssembly."
                    .to_string(),
            canonical_path: canonical_path.to_string(),
            og_type: "website".to_string(),
            schema_type: "SoftwareSourceCode",
            // defaults
            schema_keywords: None,
            date_published: None,
            date_modified: None,
            og_image: None,
            og_image_alt: None,
            locale: Lang::default(),
            alternate_path: None,
            schema_data: None,
            robots: "index, follow".to_string(),
        }
    }
}

// ── Component ─────────────────────────────────────────────────────────────────

#[component]
pub fn Seo(props: SeoProps) -> Element {
    let full_title = format!("{} | {}", props.title, SITE_NAME);
    let canonical_url = format!("{}{}", SITE_URL, props.canonical_path);
    let og_image = props
        .og_image
        .unwrap_or_else(|| DEFAULT_OG_IMAGE.to_string());
    let og_image_alt = props.og_image_alt.unwrap_or_else(|| props.title.clone());

    let og_locale = props.locale.locale();
    let alt_locale = props.locale.alternate().locale();
    let alt_lang_code = props.locale.alternate().code();

    // BCP-47 language tag for inLanguage (e.g. "en" from "en-US")
    let in_language = props
        .locale
        .code()
        .split(['-', '_'])
        .next()
        .unwrap_or("en")
        .to_string();

    // RSS feed discovery
    let rss_title = format!("{AUTHOR_NAME} — Articles");
    let rss_href = format!("{SITE_URL}/rss.xml");

    // ── JSON-LD construction ──────────────────────────────────────────────────

    // 1. Primary entity — always present.
    let mut json_ld_obj = json!({
        "@context": "https://schema.org",
        "@type": props.schema_type,
        "name": props.title,
        "description": props.description,
        "url": canonical_url,
        "inLanguage": in_language,
    });

    // 2. Author / publisher — always the site owner, never repeated by callers.
    let base = json_ld_obj.as_object_mut().unwrap();
    base.insert("author".into(), author_node());
    base.insert("publisher".into(), author_node());

    // 3. Dates — search engines use these for freshness signals.
    if let Some(ref dp) = props.date_published {
        base.insert("datePublished".into(), Value::String(dp.clone()));
    }
    if let Some(ref dm) = props.date_modified {
        base.insert("dateModified".into(), Value::String(dm.clone()));
    }

    // 4. Keywords — merge explicit keywords with slug-derived implicit ones,
    //    deduplicate while preserving order.
    let slug_kw = slug_keywords(&props.canonical_path);
    let mut all_keywords: Vec<String> = props.schema_keywords.unwrap_or_default();
    merge_keywords(&mut all_keywords, &slug_kw);

    if !all_keywords.is_empty() {
        base.insert("keywords".into(), Value::String(all_keywords.join(", ")));
    }

    // 5. Merge caller-supplied fields last — they win over everything above.
    if let Some(extra) = props.schema_data
        && let (Some(base_map), Some(extra_map)) = (json_ld_obj.as_object_mut(), extra.as_object())
    {
        for (k, v) in extra_map {
            base_map.insert(k.clone(), v.clone());
        }
    }

    // 6. Build the BreadcrumbList as a separate JSON-LD node.
    let breadcrumb = breadcrumb_list(&props.canonical_path, &props.title);

    // 7. Wrap both nodes in a `@graph` so we emit a single <script> tag with
    //    two structured-data entities.
    let json_ld_graph = json!({
        "@context": "https://schema.org",
        "@graph": [
            json_ld_obj,
            breadcrumb,
        ]
    });

    // 8. Serialize — input is always developer-authored, never user data.
    //    Panic in debug so mistakes surface immediately; fall back to a valid
    //    empty object in release so a bug never breaks the page for a visitor.
    let json_ld_string = {
        #[cfg(debug_assertions)]
        {
            serde_json::to_string(&json_ld_graph).expect("JSON-LD serialisation failed")
        }
        #[cfg(not(debug_assertions))]
        {
            serde_json::to_string(&json_ld_graph).unwrap_or_else(|e| {
                dioxus::prelude::warning!("JSON-LD serialisation failed: {}", e);
                "{}".to_string()
            })
        }
    };

    rsx! {
        document::Title { "{full_title}" }

        // ── Core meta ─────────────────────────────────────────────────────────
        document::Meta { name: "description", content: "{props.description}" }
        document::Meta { name: "robots", content: "{props.robots}" }
        document::Meta { name: "author", content: AUTHOR_NAME }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        document::Link { rel: "canonical", href: "{canonical_url}" }

        // ── RSS feed discovery ────────────────────────────────────────────────
        document::Link {
            rel: "alternate",
            r#type: "application/rss+xml",
            title: "{rss_title}",
            href: "{rss_href}",
        }

        // ── Open Graph ────────────────────────────────────────────────────────
        document::Meta { property: "og:title", content: "{props.title}" }
        document::Meta { property: "og:description", content: "{props.description}" }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:type", content: "{props.og_type}" }
        document::Meta { property: "og:image", content: "{og_image}" }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }
        document::Meta { property: "og:image:alt", content: "{og_image_alt}" }
        document::Meta { property: "og:site_name", content: SITE_NAME }
        document::Meta { property: "og:locale", content: og_locale }
        document::Meta { property: "og:locale:alternate", content: alt_locale }

        // ── Twitter Card ──────────────────────────────────────────────────────
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{props.title}" }
        document::Meta { name: "twitter:description", content: "{props.description}" }
        document::Meta { name: "twitter:image", content: "{og_image}" }
        document::Meta { name: "twitter:image:alt", content: "{og_image_alt}" }

        // ── hreflang alternates ───────────────────────────────────────────────
        // Self-referencing hreflang (required by spec)
        document::Link {
            rel: "alternate",
            hreflang: props.locale.code(),
            href: "{canonical_url}",
        }
        // Alternate language version
        if let Some(alt_path) = &props.alternate_path {
            document::Link {
                rel: "alternate",
                hreflang: alt_lang_code,
                href: "{SITE_URL}{alt_path}",
            }
        }
        // x-default: tells search engines which URL to use when no locale matches
        document::Link {
            rel: "alternate",
            hreflang: "x-default",
            href: "{canonical_url}",
        }

        // ── JSON-LD ───────────────────────────────────────────────────────────
        // Text child avoids dangerous_inner_html; serde_json guarantees correct
        // escaping.
        document::Script {
            r#type: "application/ld+json",
            "{json_ld_string}"
        }
    }
}
