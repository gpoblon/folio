use dioxus::prelude::*;

#[component]
pub fn Article(slug: Vec<String>) -> Element {
    let joined_slug = slug.join("/");
    let canonical_path = format!("/articles/{joined_slug}");

    // Fetch article metadata at page level so the Seo component renders
    // real title/description/dates during SSR — critical for search engines.
    //
    // We map the Result<_, HttpError> to Option<_> because HttpError does not
    // implement Serialize, which use_server_future requires.
    let meta_resource = use_server_future(move || {
        let slug = joined_slug.clone();
        async move { entities::article::api::article(slug).await.ok() }
    })?;

    let seo_props = match &*meta_resource.read() {
        Some(Some(article)) => {
            let meta = &article.metadata;
            let tags: Vec<String> = meta.tags.iter().map(|t| t.key().to_string()).collect();
            let created = meta
                .created
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string());
            let modified = meta
                .modified
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string());

            components::SeoProps::for_article(
                &meta.title,
                &meta.description,
                &canonical_path,
                &tags,
                &slug,
                created,
                modified,
            )
        }
        _ => components::SeoProps::article_fallback(&canonical_path, &slug),
    };

    rsx! {
        components::Seo { ..seo_props }
        div {
            class: "max-w-6xl mx-auto py-32",
            widgets::article::Article { slug }
        }
    }
}
