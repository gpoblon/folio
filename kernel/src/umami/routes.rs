//! Server-only Umami analytics proxy handlers.
//!
//! These handlers relay analytics traffic through our own domain so that
//! ad-blockers targeting `cloud.umami.is` don't interfere with tracking.

use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

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

    // Forward relevant headers.
    if let Some(ct) = req_headers.get(header::CONTENT_TYPE) {
        upstream = upstream.header(header::CONTENT_TYPE, ct);
    }
    if let Some(ua) = req_headers.get(header::USER_AGENT) {
        upstream = upstream.header(header::USER_AGENT, ua);
    }

    match upstream.body(body).send().await {
        Ok(resp) => {
            let status =
                StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            let resp_body = resp.text().await.unwrap_or_default();
            (status, resp_body).into_response()
        }
        Err(_) => StatusCode::BAD_GATEWAY.into_response(),
    }
}
