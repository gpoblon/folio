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

/// Initialization of the article store, loading complete git repository content.
/// Returns both the ArticleStore and the Resources built from the same repository fetch.
#[cfg(all(feature = "server", not(feature = "mock")))]
async fn article_store_init(
    config: &kernel::config::GitConfig,
) -> anyhow::Result<(
    entities::article::model::ArticleStore,
    kernel::resources::Resources,
)> {
    let git_client =
        kernel::git::GitClient::new(config.clone()).context("Failed to create GitClient")?;
    let Ok(repository) = git_client
        .fetch_repository_tarball()
        .await
        .and_then(kernel::git::RepositoryContent::try_from_tarball)
    else {
        return Err(anyhow::anyhow!("Failed to fetch repository"));
    };

    // Build Resources from the repository assets *before* we consume the repository.
    let resources = kernel::resources::Resources::from(&repository);

    let Ok(article_store) = entities::article::model::ArticleStore::try_from(repository) else {
        return Err(anyhow::anyhow!(
            "Failed to parse ArticleStore based on fetched repository"
        ));
    };
    Ok((article_store, resources))
}

/// Same method, but with a mock implementation.
#[cfg(all(feature = "server", feature = "mock"))]
async fn article_store_init(
    _: &kernel::config::GitConfig,
) -> anyhow::Result<(
    entities::article::model::ArticleStore,
    kernel::resources::Resources,
)> {
    Ok((
        entities::article::model::ArticleStore::mock(),
        kernel::resources::Resources::default(),
    ))
}

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::logger::tracing;
        use dioxus::server::axum::Extension;
        use dioxus::server::axum::routing::get;

        let config = kernel::config::Config::init();
        // If unable to fetch article repository, don't crash: init empty.
        let (article_store, resources) =
            article_store_init(config.git())
                .await
                .unwrap_or_else(|err| {
                    tracing::error!("Failed to initialize ArticleStore: {}", err);
                    (Default::default(), Default::default())
                });

        tracing::info!(
            "VaultResources loaded: {} assets available",
            resources.0.len()
        );

        let router = dioxus::server::router(App)
            .route(
                "/resources/{*path}",
                get(kernel::resources::serve_vault_resource),
            )
            .layer(Extension(config))
            .layer(Extension(article_store))
            .layer(Extension(resources));
        tracing::info!("Server initialized and running");
        Ok(router)
    });
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}
