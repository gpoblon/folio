use axum::extract::Extension;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

// ── Static assets ─────────────────────────────────────────────────────────────

/// Embedded OG image bytes — served at `/og-default.png` so the URL declared
/// in `kernel::seo::DEFAULT_OG_IMAGE` always resolves regardless of the asset
/// pipeline's content-hashing.
static OG_IMAGE_BYTES: &[u8] = include_bytes!("../assets/og-default.png");

/// Handler for `GET /og-default.png`
///
/// Serves the default Open Graph / social-preview image from the embedded
/// binary so that social crawlers (Twitter, LinkedIn, Slack…) and LLM agents
/// can reliably fetch it at the canonical URL `{SITE_URL}/og-default.png`.
pub async fn og_default_image() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        ],
        OG_IMAGE_BYTES,
    )
}

use entities::article::model::ArticleStore;
use entities::project::model::ProjectStore;
use kernel::seo::{
    AUTHOR_BIO, AUTHOR_EMAIL, AUTHOR_GITHUB, AUTHOR_JOB_TITLE, AUTHOR_KNOWS_ABOUT, AUTHOR_LINKEDIN,
    AUTHOR_NAME, Keywords, SITE_DESCRIPTION, SITE_NAME, SITE_URL, STATIC_SITEMAP_ROUTES,
};

// ── Sitemap templates ─────────────────────────────────────────────────────────

/// Sitemap document template — `{url_entries}` is replaced with `<url>` blocks.
const SITEMAP: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{url_entries}</urlset>
"#;

/// Formats a single `<url>` block. When `lastmod` is empty the `<lastmod>`
/// tag is omitted entirely.
fn url_entry(loc: &str, lastmod: &str, changefreq: &'static str, priority: &'static str) -> String {
    let lastmod_tag = if !lastmod.is_empty() { format!("    <lastmod>{lastmod}</lastmod>\n") } else { Default::default() };
    format!(
        "  <url>\n    <loc>{loc}</loc>\n{lastmod_tag}    <changefreq>{changefreq}</changefreq>\n    <priority>{priority}</priority>\n  </url>\n"
    )
}

// ── sitemap.xml ───────────────────────────────────────────────────────────────

/// Handler for `GET /sitemap.xml`
///
/// Emits a complete sitemap: static routes plus one `<url>` per published
/// article and project so that Googlebot and LLM crawlers have a literal map
/// of the site rather than having to discover pages by following links.
pub async fn sitemap_xml(
    Extension(article_store): Extension<ArticleStore>,
    Extension(project_store): Extension<ProjectStore>,
) -> impl IntoResponse {
    let build_date = kernel::build_info::BUILD_DATE;
    let mut entries = String::new();

    // ── Static routes ─────────────────────────────────────────────────────
    for &(path, changefreq, priority) in STATIC_SITEMAP_ROUTES {
        let loc = format!("{SITE_URL}{path}");
        entries.push_str(&url_entry(&loc, build_date, changefreq, priority));
    }

    // ── Dynamic article routes ────────────────────────────────────────────
    if let Ok(articles) = article_store.0.try_read() {
        let mut metas: Vec<_> = articles.values().map(|a| &a.metadata).collect();
        metas.sort_by(|a, b| b.created.cmp(&a.created).then(a.slug.cmp(&b.slug)));

        for meta in metas {
            let loc = format!("{SITE_URL}/articles/{}", meta.slug);
            let lastmod = meta
                .modified
                .or(meta.created)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            entries.push_str(&url_entry(&loc, &lastmod, "monthly", "0.7"));
        }
    }

    // ── Dynamic project routes ────────────────────────────────────────────
    if let Ok(projects) = project_store.0.try_read() {
        let mut metas: Vec<_> = projects.values().map(|p| &p.metadata).collect();
        metas.sort_by(|a, b| {
            b.meta
                .created
                .cmp(&a.meta.created)
                .then(a.repository.slug.cmp(&b.repository.slug))
        });

        for meta in metas {
            let loc = format!("{SITE_URL}/lab/{}", meta.repository.slug);
            let lastmod = meta
                .meta
                .modified
                .or(meta.meta.created)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            entries.push_str(&url_entry(&loc, &lastmod, "monthly", "0.6"));
        }
    }

    let xml = SITEMAP.replace("{url_entries}", &entries);

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600, s-maxage=3600"),
        ],
        xml,
    )
}

// ── RSS templates ────────────────────────────────────────────────────────────

/// Formats a single `<item>` block. `pub_date` may be empty (tag omitted);
/// `categories` is a pre-formatted string of `<category>` lines.
fn rss_item(
    link: &str,
    title: &str,
    description: &str,
    pub_date: &str,
    categories: &str,
) -> String {
    let pub_date_tag = if !pub_date.is_empty() { format!("    <pubDate>{pub_date}</pubDate>\n") } else { Default::default() };
    format!(
        r#"  <item>
    <title>{title}</title>
    <link>{link}</link>
    <guid isPermaLink="true">{link}</guid>
    <description>{description}</description>
{pub_date_tag}    <author>{AUTHOR_EMAIL} ({AUTHOR_NAME})</author>
{categories}  </item>
"#
    )
}

// ── rss.xml ───────────────────────────────────────────────────────────────────

/// Handler for `GET /rss.xml`
///
/// Returns a valid RSS 2.0 feed built from all published articles, ordered
/// most-recent first. Feed readers (and LLMs that support feed ingestion) can
/// subscribe to stay current with new articles automatically.
pub async fn rss_xml(Extension(article_store): Extension<ArticleStore>) -> impl IntoResponse {
    let mut items = String::new();

    if let Ok(articles) = article_store.0.try_read() {
        let mut sorted: Vec<_> = articles.values().collect();
        sorted.sort_by(|a, b| {
            b.metadata
                .created
                .cmp(&a.metadata.created)
                .then(a.metadata.slug.cmp(&b.metadata.slug))
        });

        for article in sorted {
            let meta = &article.metadata;
            let link = format!("{SITE_URL}/articles/{}", meta.slug);
            let title = escape_xml(&meta.title);
            let description = escape_xml(&meta.description);

            // RFC 822 date — required by RSS 2.0 validators.
            let pub_date = meta
                .created
                .map(|dt| dt.format("%a, %d %b %Y %H:%M:%S +0000").to_string())
                .unwrap_or_default();

            // Article tags + slug-derived segments as <category> elements.
            let categories: String = meta
                .tags
                .iter()
                .map(|tag| format!("    <category>{}</category>\n", escape_xml(tag.key())))
                .chain(
                    Keywords::slug_segments(&meta.slug)
                        .into_iter()
                        .map(|seg| format!("    <category>{}</category>\n", escape_xml(&seg))),
                )
                .collect();

            items.push_str(&rss_item(
                &link,
                &title,
                &description,
                &pub_date,
                &categories,
            ));
        }
    }

    // Channel-level <lastBuildDate> is always the moment this response is built.
    let build_date = chrono::Utc::now()
        .format("%a, %d %b %Y %H:%M:%S +0000")
        .to_string();

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{SITE_NAME}</title>
    <link>{SITE_URL}/blog</link>
    <description>{SITE_DESCRIPTION}</description>
    <language>en-us</language>
    <managingEditor>{AUTHOR_EMAIL} ({AUTHOR_NAME})</managingEditor>
    <webMaster>{AUTHOR_EMAIL} ({AUTHOR_NAME})</webMaster>
    <lastBuildDate>{build_date}</lastBuildDate>
    <atom:link href="{SITE_URL}/rss.xml" rel="self" type="application/rss+xml" />
{items}  </channel>
</rss>
"#
    );

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/rss+xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600, s-maxage=3600"),
        ],
        xml,
    )
}

// ── llms.txt ──────────────────────────────────────────────────────────────────

/// Handler for `GET /llms.txt`
///
/// Returns a machine-readable summary of the site for LLM crawlers,
/// following the proposed llms.txt standard (<https://llmstxt.org/>).
/// Dynamic sections (articles, projects) are built from the live stores
/// so the file stays current without manual maintenance.
pub async fn llms_txt(
    Extension(article_store): Extension<ArticleStore>,
    Extension(project_store): Extension<ProjectStore>,
) -> impl IntoResponse {
    // ── Pre-build variable sections ───────────────────────────────────────

    let skills: String = AUTHOR_KNOWS_ABOUT
        .iter()
        .map(|t| format!("- {t}\n"))
        .collect();

    let articles_section = if let Ok(articles) = article_store.0.try_read()
        && !articles.is_empty()
    {
        let mut metas: Vec<_> = articles.values().map(|a| &a.metadata).collect();
        metas.sort_by(|a, b| b.created.cmp(&a.created).then(a.slug.cmp(&b.slug)));
        let list: String = metas
            .iter()
            .map(|m| {
                format!(
                    "- [{}]({SITE_URL}/articles/{}): {}\n",
                    m.title, m.slug, m.description
                )
            })
            .collect();
        format!("## Blog Articles\n\n{list}\n")
    } else {
        String::new()
    };

    let projects_section = if let Ok(projects) = project_store.0.try_read()
        && !projects.is_empty()
    {
        let mut metas: Vec<_> = projects.values().map(|p| &p.metadata).collect();
        metas.sort_by(|a, b| {
            b.meta
                .created
                .cmp(&a.meta.created)
                .then(a.repository.slug.cmp(&b.repository.slug))
        });
        let list: String = metas
            .iter()
            .map(|m| {
                format!(
                    "- [{}]({SITE_URL}/lab/{}): {}\n",
                    m.meta.title, m.repository.slug, m.meta.description
                )
            })
            .collect();
        format!("## Lab Projects\n\n{list}\n")
    } else {
        String::new()
    };

    // ── Assemble from template ────────────────────────────────────────────

    let body = format!(
        r#"# {SITE_NAME}

> {SITE_DESCRIPTION}

## About

- Author: {AUTHOR_NAME}
- Role: {AUTHOR_JOB_TITLE}
- Location: Niort, Nouvelle-Aquitaine, France
- Languages: French (native), English (professional)
- Email: {AUTHOR_EMAIL}
- Site: {SITE_URL}

{AUTHOR_BIO}

## Skills & Expertise

{skills}
## Career

### Head of Education & IT — 42 Angoulême (Aug 2021 – Feb 2026)

Spearheaded the campus launch from day one, scaling to 592 students and fostering 30+ corporate partnerships. End-to-end Product Ownership of a 70k LoC fullstack Rust educational platform (Axum / Dioxus / Tailwind) leveraging DDD layered architecture. Designed a Rust open-source UI Component Library (Dioxus). Recruited, mentored, and managed a cross-functional team of 4.

### Software Engineer — Normation / Rudder (Dec 2019 – Mar 2021)

Core developer of rudder-lang, a declarative Infrastructure-as-Code language written in Rust. Contributed to the open-source compiler toolchain (lexer, parser, AST, semantic analysis, transpiler) from conception to production. Wrote comprehensive tests following DDD principles and authored technical documentation.

### Fullstack Web Developer — uRehab (Sep 2017 – Jan 2018)

Designed and built frontend interfaces (React), wrote API and conversational chatbot logic (Node.js), and modeled data (MongoDB) for a healthcare prevention platform.

## Education

- **42 Paris** — Digital Technologies Architect (MSc equivalent), 2016–2020. Project-based curriculum: Unix/C/C++/Rust, Algorithms, Rendering, Security. Hackathon awards: 42Startup, SexTechLab, Société Générale.
- **University of Burgundy** — Bachelor of Private Law, 2012–2016.

## Sections

- [Blog]({SITE_URL}/blog): Technical deep-dive articles on Rust, Dioxus, Axum, WebAssembly, software architecture, and Domain-Driven Design
- [Lab]({SITE_URL}/lab): Open-source Rust projects — applications, developer tools, UI component libraries, and cross-platform experiments
- [Experience]({SITE_URL}/experience): Full professional career path, education, and resume
- [Connect]({SITE_URL}/contact): Contact form for professional collaboration, freelance, or full-time opportunities

{articles_section}{projects_section}## Links

- GitHub: {AUTHOR_GITHUB}
- LinkedIn: {AUTHOR_LINKEDIN}
- RSS: {SITE_URL}/rss.xml
- Sitemap: {SITE_URL}/sitemap.xml
"#
    );

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/plain; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600, s-maxage=3600"),
        ],
        body,
    )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Minimal XML escaping for values embedded in element text.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
