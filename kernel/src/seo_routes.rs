// ── Server-only SEO route handlers ────────────────────────────────────────────
//
// Pure-data constants live in `kernel::seo` (always available on every target).
// This module re-exports them for backward compatibility and adds the Axum
// handlers that can only run on the server.

use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

// Re-export shared constants so existing `use kernel::seo_routes::*` keeps working.
pub use crate::seo::{
    AUTHOR_EMAIL, AUTHOR_NAME, DEFAULT_OG_IMAGE, OG_IMAGE_HEIGHT, OG_IMAGE_WIDTH, SITE_DESCRIPTION,
    SITE_NAME, SITE_URL,
};

// ── robots.txt ────────────────────────────────────────────────────────────────

/// Handler for `GET /robots.txt`
///
/// Tells crawlers which paths to index and points them at both the sitemap
/// and the RSS feed.
pub async fn robots_txt() -> impl IntoResponse {
    let body = format!(
        "User-agent: *\n\
         Allow: /\n\
         Disallow: /resources/\n\
         Disallow: /stats/\n\
         \n\
         Sitemap: {SITE_URL}/sitemap.xml\n\
         \n\
         # RSS feed: {SITE_URL}/rss.xml\n"
    );

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body,
    )
}

// ── Static routes shared between kernel and app ───────────────────────────────

/// The fixed, domain-independent routes that are always part of the sitemap.
/// Returned as a slice of `(path, changefreq, priority)`.
pub const STATIC_SITEMAP_ROUTES: &[(&str, &str, &str)] = &[
    ("/", "weekly", "1.0"),
    ("/lab", "weekly", "0.9"),
    ("/blog", "weekly", "0.9"),
    ("/experience", "monthly", "0.8"),
    ("/contact", "monthly", "0.7"),
    ("/terms-of-use", "yearly", "0.2"),
];

// ── Umami proxies ─────────────────────────────────────────────────────────────

/// Proxy handler for Umami script: `GET /stats/script.js`
///
/// Fetches the real tracker script from Umami Cloud and serves it
/// from our own domain, making it invisible to ad-blocker domain lists.
pub async fn umami_script_proxy() -> impl IntoResponse {
    let client = reqwest::Client::new();
    match client.get("https://cloud.umami.is/script.js").send().await {
        Ok(resp) if resp.status().is_success() => match resp.text().await {
            Ok(body) => (
                StatusCode::OK,
                [
                    (
                        header::CONTENT_TYPE,
                        "application/javascript; charset=utf-8",
                    ),
                    (header::CACHE_CONTROL, "public, max-age=86400"),
                ],
                body,
            )
                .into_response(),
            Err(_) => StatusCode::BAD_GATEWAY.into_response(),
        },
        _ => StatusCode::BAD_GATEWAY.into_response(),
    }
}

/// Proxy handler for Umami API calls: `POST /stats/api/send`
///
/// The tracker script, configured with `data-host-url="/stats"`, will
/// POST pageview & event payloads to `/stats/api/send`.  We relay them
/// to Umami Cloud so the browser never contacts `cloud.umami.is` directly.
pub async fn umami_api_proxy(
    req_headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let client = reqwest::Client::new();
    let mut upstream = client.post("https://cloud.umami.is/api/send");

    // Forward relevant headers
    if let Some(ct) = req_headers.get(header::CONTENT_TYPE) {
        upstream = upstream.header(header::CONTENT_TYPE, ct);
    }
    if let Some(ua) = req_headers.get(header::USER_AGENT) {
        upstream = upstream.header(header::USER_AGENT, ua);
    }

    match upstream.body(body.to_vec()).send().await {
        Ok(resp) => {
            let status =
                StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            let resp_body = resp.text().await.unwrap_or_default();
            (status, resp_body).into_response()
        }
        Err(_) => StatusCode::BAD_GATEWAY.into_response(),
    }
}
