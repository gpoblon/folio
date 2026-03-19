use dioxus::{fullstack::Loading, prelude::*};

const PROXY_SCRIPT_PATH: &str = "/stats/script.js";
const PROXY_HOST_URL: &str = "/stats";

/// Injects the Umami analytics `<script>` tag.
///
/// The website-id is fetched from the server config at SSR render time
/// via a lightweight server function, keeping the secret out of source code.
///
/// The script URL points to our own backend proxy (`/stats/script.js`)
/// so ad-blockers targeting `cloud.umami.is` won't interfere.
/// `data-host-url="/stats"` redirects the tracker's API calls through
/// the same proxy.
///
/// A built-in `SuspenseBoundary` ensures the pending state is handled
/// gracefully even when this component sits above the router's own
/// boundary (e.g. directly inside `App`).
#[component]
pub fn UmamiScript() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! {},
            UmamiScriptInner {}
        }
    }
}

/// Inner component that performs the actual server-function call.
/// Separated so the outer `SuspenseBoundary` can catch the pending state
/// produced by `use_loader`.
#[component]
fn UmamiScriptInner() -> Element {
    let website_id = use_loader(move || async { get_umami_website_id().await });

    match website_id {
        Err(Loading::Pending(pending)) => Err(Loading::Pending(pending).into()),
        Err(Loading::Failed(_)) => {
            // Analytics should never break the page — silently degrade
            rsx! {}
        }
        Ok(id) => {
            let id_value = id();
            rsx! {
                document::Script {
                    src: PROXY_SCRIPT_PATH,
                    defer: true,
                    "data-website-id": "{id_value}",
                    "data-host-url": PROXY_HOST_URL,
                }
            }
        }
    }
}

/// Returns the Umami website ID from server config.
#[server(config: dioxus_server::axum::Extension<crate::config::Config>)]
async fn get_umami_website_id() -> Result<String, HttpError> {
    use secrecy::ExposeSecret;
    Ok(config.umami().website_id.expose_secret().to_string())
}
