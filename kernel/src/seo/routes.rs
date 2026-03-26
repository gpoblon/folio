//! Server-only SEO route handlers.
//!
//! Pure-data constants live in `kernel::seo` (always available on every target).
//! This module adds the Axum handlers that can only run on the server.

use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

use super::SITE_URL;

// ── robots.txt ────────────────────────────────────────────────────────────────

/// Handler for `GET /robots.txt`
///
/// Tells crawlers which paths to index and points them at both the sitemap
/// and the RSS feed.
pub async fn robots_txt() -> impl IntoResponse {
    let body = concat!(
        "User-agent: *\n",
        "Allow: /\n",
        "Disallow: /resources/\n",
        "Disallow: /stats/\n",
        "\n",
    );

    // Sitemap and RSS lines need runtime interpolation of SITE_URL.
    let body = format!(
        "{body}\
         Sitemap: {SITE_URL}/sitemap.xml\n\
         \n\
         # RSS feed discovery (also via <link rel=alternate> in HTML head)\n\
         Sitemap: {SITE_URL}/rss.xml\n\
         \n\
         # AI / LLM agent discovery — https://llmstxt.org\n\
         # Agents: see {SITE_URL}/llms.txt for a structured site summary.\n"
    );

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/plain; charset=utf-8"),
            (
                header::CACHE_CONTROL,
                "public, max-age=86400, s-maxage=86400",
            ),
        ],
        body,
    )
}

// ── Static routes shared between kernel and app ───────────────────────────────

/// The fixed, domain-independent routes that are always part of the sitemap.
/// Returned as a slice of `(path, changefreq, priority)`.
pub const STATIC_SITEMAP_ROUTES: &[(&str, &str, &str)] = &[
    ("/", "weekly", "1.0"),
    ("/fr", "weekly", "1.0"),
    ("/en", "weekly", "1.0"),
    ("/lab", "weekly", "0.9"),
    ("/blog", "weekly", "0.9"),
    ("/experience", "monthly", "0.8"),
    ("/contact", "monthly", "0.7"),
    ("/terms-of-use", "yearly", "0.2"),
];
