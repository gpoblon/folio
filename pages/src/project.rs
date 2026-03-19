use dioxus::prelude::*;

#[component]
pub fn Project(slug: String) -> Element {
    let canonical_path = format!("/lab/{slug}");
    let widget_slug = slug.clone();

    // Fetch project metadata at page level so the Seo component renders
    // real title/description/dates during SSR — critical for search engines.
    //
    // We map the Result<_, HttpError> to Option<_> because HttpError does not
    // implement Serialize, which use_server_future requires.
    let meta_resource = use_server_future(move || {
        let slug = slug.clone();
        async move { entities::project::api::project(slug).await.ok() }
    })?;

    let seo_props = match &*meta_resource.read() {
        Some(Some(project)) => {
            let meta = &project.metadata.meta;
            let repo = &project.metadata.repository;
            let repo_url = format!("https://github.com/{}/{}", repo.owner, repo.slug);
            let tags: Vec<String> = meta.tags.iter().map(|t| t.key().to_string()).collect();
            let created = meta
                .created
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string());
            let modified = meta
                .modified
                .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string());

            components::SeoProps::for_project(
                &meta.title,
                &meta.description,
                &canonical_path,
                &tags,
                created,
                modified,
                Some(&repo_url),
            )
        }
        _ => components::SeoProps::project_fallback(&canonical_path),
    };

    rsx! {
        components::Seo { ..seo_props }
        div {
            class: "max-w-6xl mx-auto py-32",
            widgets::project::Project { slug: widget_slug }
        }
    }
}
