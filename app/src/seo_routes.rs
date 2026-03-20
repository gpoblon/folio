use axum::extract::Extension;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;

use entities::article::model::ArticleStore;
use entities::project::model::ProjectStore;
use kernel::seo::{
    AUTHOR_EMAIL, AUTHOR_NAME, Keywords, SITE_DESCRIPTION, SITE_NAME, SITE_URL,
    STATIC_SITEMAP_ROUTES,
};

// ── sitemap.xml ───────────────────────────────────────────────────────────────

/// Handler for `GET /sitemap.xml`
///
/// Emits a complete sitemap: static routes plus one `<url>` per published
/// article and project so that Googlebot and LLM crawlers have a literal map
/// of the site rather than having to discover pages by following links.
pub async fn sitemap_xml(
    Extension(article_store): Extension<ArticleStore>,
    Extension(project_store): Extension<ProjectStore>,
) -> impl IntoResponse {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );

    // Build-time timestamp for static routes (YYYY-MM-DD).
    let build_date = kernel::build_info::BUILD_DATE;

    // Static routes — always include lastmod so crawlers know freshness.
    for (path, changefreq, priority) in STATIC_SITEMAP_ROUTES {
        xml.push_str(&format!(
            "  <url>\n\
             \x20   <loc>{SITE_URL}{path}</loc>\n\
             \x20   <lastmod>{build_date}</lastmod>\n\
             \x20   <changefreq>{changefreq}</changefreq>\n\
             \x20   <priority>{priority}</priority>\n\
             \x20 </url>\n"
        ));
    }

    // Dynamic article routes — one entry per published article.
    if let Ok(articles) = article_store.0.try_read() {
        let mut metas: Vec<_> = articles.values().map(|a| &a.metadata).collect();
        metas.sort_by(|a, b| b.created.cmp(&a.created).then(a.slug.cmp(&b.slug)));

        for meta in metas {
            let loc = format!("{SITE_URL}/articles/{}", meta.slug);
            let lastmod = meta
                .modified
                .or(meta.created)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();

            xml.push_str("  <url>\n");
            xml.push_str(&format!("     <loc>{loc}</loc>\n"));
            if !lastmod.is_empty() {
                xml.push_str(&format!("     <lastmod>{lastmod}</lastmod>\n"));
            }
            xml.push_str("     <changefreq>monthly</changefreq>\n");
            xml.push_str("     <priority>0.7</priority>\n");
            xml.push_str("  </url>\n");
        }
    }

    // Dynamic project routes — one entry per project.
    if let Ok(projects) = project_store.0.try_read() {
        let mut metas: Vec<_> = projects.values().map(|p| &p.metadata).collect();
        metas.sort_by(|a, b| {
            b.meta
                .created
                .cmp(&a.meta.created)
                .then(a.repository.slug.cmp(&b.repository.slug))
        });

        for meta in metas {
            let loc = format!("{SITE_URL}/lab/{}", meta.repository.slug);
            let lastmod = meta
                .meta
                .modified
                .or(meta.meta.created)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();

            xml.push_str("  <url>\n");
            xml.push_str(&format!("     <loc>{loc}</loc>\n"));
            if !lastmod.is_empty() {
                xml.push_str(&format!("     <lastmod>{lastmod}</lastmod>\n"));
            }
            xml.push_str("     <changefreq>monthly</changefreq>\n");
            xml.push_str("     <priority>0.6</priority>\n");
            xml.push_str("  </url>\n");
        }
    }

    xml.push_str("</urlset>\n");

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600, s-maxage=3600"),
        ],
        xml,
    )
}

// ── rss.xml ───────────────────────────────────────────────────────────────────

/// Handler for `GET /rss.xml`
///
/// Returns a valid RSS 2.0 feed built from all published articles, ordered
/// most-recent first. Feed readers (and LLMs that support feed ingestion) can
/// subscribe to stay current with new articles automatically.
pub async fn rss_xml(Extension(article_store): Extension<ArticleStore>) -> impl IntoResponse {
    let mut items = String::new();

    if let Ok(articles) = article_store.0.try_read() {
        let mut sorted: Vec<_> = articles.values().collect();
        sorted.sort_by(|a, b| {
            b.metadata
                .created
                .cmp(&a.metadata.created)
                .then(a.metadata.slug.cmp(&b.metadata.slug))
        });

        for article in sorted {
            let meta = &article.metadata;
            let link = format!("{SITE_URL}/articles/{}", meta.slug);
            let title = escape_xml(&meta.title);
            let description = escape_xml(&meta.description);

            // RFC 822 date — required by RSS 2.0 validators.
            let pub_date = meta
                .created
                .map(|dt| dt.format("%a, %d %b %Y %H:%M:%S +0000").to_string())
                .unwrap_or_default();

            items.push_str("    <item>\n");
            items.push_str(&format!("      <title>{title}</title>\n"));
            items.push_str(&format!("      <link>{link}</link>\n"));
            items.push_str(&format!("      <guid isPermaLink=\"true\">{link}</guid>\n"));
            items.push_str(&format!("      <description>{description}</description>\n"));
            if !pub_date.is_empty() {
                items.push_str(&format!("      <pubDate>{pub_date}</pubDate>\n"));
            }
            items.push_str(&format!(
                "      <author>{AUTHOR_EMAIL} ({AUTHOR_NAME})</author>\n"
            ));

            // Article tags as <category> — meaningful topic labels for feed
            // readers and aggregators to filter on.
            for tag in &meta.tags {
                items.push_str(&format!(
                    "      <category>{}</category>\n",
                    escape_xml(tag.key())
                ));
            }

            // Supplement with slug-derived segments for broader topic coverage
            // (e.g. "rust", "dev") when tags alone are sparse.
            for seg in Keywords::slug_segments(&meta.slug) {
                items.push_str(&format!(
                    "      <category>{}</category>\n",
                    escape_xml(&seg)
                ));
            }

            items.push_str("    </item>\n");
        }
    }

    // Channel-level <lastBuildDate> is always the moment this response is built.
    let build_date = chrono::Utc::now()
        .format("%a, %d %b %Y %H:%M:%S +0000")
        .to_string();

    let xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <rss version=\"2.0\"\n\
              xmlns:atom=\"http://www.w3.org/2005/Atom\">\n\
           <channel>\n\
             <title>{SITE_NAME}</title>\n\
             <link>{SITE_URL}/blog</link>\n\
             <description>{SITE_DESCRIPTION}</description>\n\
             <language>en-us</language>\n\
             <managingEditor>{AUTHOR_EMAIL} ({AUTHOR_NAME})</managingEditor>\n\
             <webMaster>{AUTHOR_EMAIL} ({AUTHOR_NAME})</webMaster>\n\
             <lastBuildDate>{build_date}</lastBuildDate>\n\
             <atom:link href=\"{SITE_URL}/rss.xml\" rel=\"self\"\n\
                        type=\"application/rss+xml\" />\n\
         {items}\
           </channel>\n\
         </rss>\n"
    );

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/rss+xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600, s-maxage=3600"),
        ],
        xml,
    )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Minimal XML escaping for values embedded in element text.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
