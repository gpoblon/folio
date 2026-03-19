#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod router;

#[cfg(feature = "server")]
mod seo_routes;

#[cfg(feature = "server")]
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const APPLE_TOUCH_ICON: Asset = asset!("/assets/apple-touch-icon.png");
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
        document::Meta { name: "theme-color", content: "#0a0a0a" }
        kernel::umami::UmamiScript {}
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap" }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@100..800&display=swap" }
        document::Stylesheet { href: TAILWIND_CSS }
        components::Bootstrap {}
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
        let state = state::State::try_fetch_data(config.clone())
            .await
            .unwrap_or_default();

        let router = dioxus::server::router(App)
            .route(
                "/resources/{*path}",
                get(kernel::resources::serve_vault_resource),
            )
            .route("/robots.txt", get(kernel::seo_routes::robots_txt))
            .route("/sitemap.xml", get(seo_routes::sitemap_xml))
            .route("/rss.xml", get(seo_routes::rss_xml))
            .route(
                "/stats/script.js",
                get(kernel::seo_routes::umami_script_proxy),
            )
            .route("/stats/api/send", post(kernel::seo_routes::umami_api_proxy))
            .layer(Extension(config))
            .layer(Extension(state.articles))
            .layer(Extension(state.resources))
            .layer(Extension(state.projects));
        tracing::info!("Server initialized and running");
        Ok(router)
    });
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}
