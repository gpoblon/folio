#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod router;

#[cfg(feature = "server")]
mod seo_routes;

#[cfg(feature = "server")]
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon.png");
#[allow(dead_code)] // kept to register og-default.png with the asset pipeline (web bundle)
const OG_DEFAULT_IMAGE: Asset = asset!("/assets/og-default.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    kernel::lang::init_i18n();
    let lang = kernel::lang::use_lang();
    let theme = use_signal(kernel::theme::ThemeMode::default);
    provide_context(theme);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "apple-touch-icon", href: APPLE_TOUCH_ICON }
        document::Meta { property: "og:image", content: kernel::seo::DEFAULT_OG_IMAGE }
        document::Meta { name: "twitter:image", content: kernel::seo::DEFAULT_OG_IMAGE }
        document::Meta { name: "theme-color", content: "#0a0a0a" }

        // ── Critical CSS first — ensures styled SSR paint before anything else ──
        document::Stylesheet { href: TAILWIND_CSS }
        components::Bootstrap {}

        // ── Preconnect hints — let the browser open connections early ──
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }

        // ── Google Fonts (non-blocking, after local CSS) ──
        // Inter was previously duplicated via a CSS @import in dx-components-theme.css
        // which triggered a referrer-policy console warning. Loading both families
        // through <link> tags avoids that and lets preconnect hints take effect.
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&family=JetBrains+Mono:wght@100..800&display=swap" }

        // ── Analytics (deferred, non-critical) ──
        kernel::umami::UmamiScript {}
        div {
            id: "root",
            class: "min-h-screen flex flex-col bg-background text-foreground border-border font-light font-sans overflow-x-hidden",
            lang: lang.code(),
            "data-theme": theme().as_str(),
            components::toast::ToastProvider {
                Router::<router::Route> {}
            }
        }
    }
}

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::logger::tracing;
        use dioxus::server::axum::Extension;
        use dioxus::server::axum::routing::{get, post};

        let config = kernel::config::Config::init();
        let state = match state::State::try_fetch_data(config.clone()).await {
            Ok(state) => state,
            Err(err) => {
                tracing::warn!("Failed to fetch data: {err}.\n Using default (empty) state.");
                state::State::default()
            }
        };

        let contact_limiter =
            kernel::rate_limit::RateLimiter::new(2, std::time::Duration::from_secs(600));

        // XRealIp reads the `X-Real-IP` header
        // Switch to ClientIpSource::ConnectInfo for proxy-free deployments.
        let ip_source = kernel::rate_limit::ClientIpSource::XRealIp;

        let router = dioxus::server::router(App)
            .route(
                "/resources/{*path}",
                get(kernel::resources::serve_vault_resource),
            )
            .route("/robots.txt", get(kernel::seo::robots_txt))
            .route("/sitemap.xml", get(seo_routes::sitemap_xml))
            .route("/rss.xml", get(seo_routes::rss_xml))
            .route("/llms.txt", get(seo_routes::llms_txt))
            .route("/og-default.png", get(seo_routes::og_default_image))
            .route(
                "/stats/script.js",
                get(kernel::umami::routes::umami_script_proxy),
            )
            .route(
                "/stats/api/send",
                post(kernel::umami::routes::umami_api_proxy),
            )
            .layer(Extension(config))
            .layer(Extension(contact_limiter))
            .layer(ip_source.into_extension())
            .layer(Extension(state.articles))
            .layer(Extension(state.resources))
            .layer(Extension(state.projects));
        tracing::info!("Server initialized and running");
        Ok(router)
    });
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}
