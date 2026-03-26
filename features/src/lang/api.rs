//! Server-side language detection from the `Accept-Language` request header.

use dioxus::prelude::*;

/// Detects the visitor's preferred language by inspecting the HTTP
/// `Accept-Language` request header.
///
/// Returns the two-letter BCP-47 primary subtag of the highest-priority
/// language (e.g. `"en"`, `"fr"`).  Falls back to `"fr"` — the site's
/// default locale — when the header is absent or cannot be parsed.
///
/// The function is called during SSR so the result is embedded in the
/// initial HTML payload; no extra network round-trip is needed on the
/// client after hydration.
#[server(
    headers: dioxus_fullstack::HeaderMap,
)]
pub async fn detect_preferred_language() -> Result<String, ServerFnError> {
    // Accept-Language format: "en-US,en;q=0.9,fr;q=0.8"
    // Take the first comma-separated token (highest priority), then strip
    // any region subtag (after '-') or quality parameter (after ';') to
    // obtain the bare two-letter primary subtag.
    let primary = headers
        .get("accept-language")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(',')
                .next()
                .and_then(|tag| tag.split(['-', ';']).next())
                .map(|s| s.trim().to_lowercase())
        })
        .unwrap_or_else(|| "fr".to_string());

    Ok(primary)
}
