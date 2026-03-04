#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod router;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    kernel::lang::init_i18n();
    let theme = use_signal(kernel::theme::ThemeMode::default);
    provide_context(theme);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Inter:wght@100..900&display=swap" }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@100..800&display=swap" }
        document::Stylesheet { href: TAILWIND_CSS }
        components::Bootstrap {}
        div {
            id: "root",
            class: "min-h-screen flex flex-col bg-background text-foreground border-border font-light font-sans overflow-x-hidden",
            lang: "fr",
            "data-theme": theme().as_str(),
            components::toast::ToastProvider {
                Router::<router::Route> {}
            }
        }
    }
}

/// Initialization of the article store, loading complete git repository content
#[cfg(all(feature = "server", not(feature = "mock")))]
async fn article_store_init(
    config: &kernel::config::GitConfig,
) -> anyhow::Result<entities::article::model::ArticleStore> {
    let git_client =
        kernel::git::GitClient::new(config.clone()).context("Failed to create GitClient")?;
    let Ok(repository) = git_client
        .fetch_repository_tarball()
        .await
        .and_then(kernel::git::RepositoryContent::try_from_tarball)
    else {
        return Err(anyhow::anyhow!("Failed to fetch repository"));
    };
    let Ok(article_store) = entities::article::model::ArticleStore::try_from(repository) else {
        return Err(anyhow::anyhow!(
            "Failed to parse ArticleStore based on fetched repository"
        ));
    };
    Ok(article_store)
}

/// Same method, but with a mock implementation.
#[cfg(all(feature = "server", feature = "mock"))]
async fn article_store_init(
    _: &kernel::config::GitConfig,
) -> anyhow::Result<entities::article::model::ArticleStore> {
    Ok(entities::article::model::ArticleStore::mock())
}

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::logger::tracing;
        use dioxus::server::axum::Extension;

        let config = kernel::config::Config::init();
        // If unable to fetch article repository, don't crash: init empty.
        let article_store = article_store_init(config.git())
            .await
            .unwrap_or_else(|err| {
                tracing::error!("Failed to initialize ArticleStore: {}", err);
                Default::default()
            });

        let router = dioxus::server::router(App)
            .layer(Extension(config))
            .layer(Extension(article_store));
        tracing::info!("Server initialized and running");
        Ok(router)
    });
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}
