//! The [`Seo`] Dioxus component — renders all `<head>` SEO metadata.

use dioxus::prelude::*;
use kernel::seo::{
    AUTHOR_NAME, DEFAULT_OG_IMAGE, Keywords, OG_IMAGE_HEIGHT, OG_IMAGE_WIDTH, SITE_NAME, SITE_URL,
};
use serde_json::{Value, json};

use super::author::{author_node, author_node_compact};
use super::breadcrumb::breadcrumb_list;
use super::props::SeoProps;

#[component]
pub fn Seo(props: SeoProps) -> Element {
    // ── Destructure upfront to avoid partial-move borrow errors ───────────────

    let SeoProps {
        title,
        description,
        canonical_path,
        og_type,
        og_image,
        og_image_alt,
        locale,
        alternate_path,
        schema_type,
        schema_keywords,
        schema_data,
        date_published,
        date_modified,
        robots,
        article_tags,
    } = props;

    // Home page uses its title as-is (already contains the brand);
    // every other page gets the standard "Page Title | Brand" format.
    let full_title = format!("{title} | {SITE_NAME}");
    let canonical_url = format!("{SITE_URL}{canonical_path}");
    let og_image = og_image.as_deref().unwrap_or(DEFAULT_OG_IMAGE).to_string();
    let og_image_alt = og_image_alt.unwrap_or_else(|| title.clone());

    let og_locale = locale.locale();
    let alt_locale = locale.alternate().locale();
    let alt_lang_code = locale.alternate().code();

    // BCP-47 language tag for inLanguage (e.g. "en" from "en-US")
    let in_language = locale.code().split(['-', '_']).next().unwrap_or("en");

    // RSS feed discovery
    let rss_title = format!("{AUTHOR_NAME} — Articles");
    let rss_href = format!("{SITE_URL}/rss.xml");

    // OG image dimensions as strings for meta tags
    let og_width = OG_IMAGE_WIDTH.to_string();
    let og_height = OG_IMAGE_HEIGHT.to_string();

    let is_article = og_type == "article";
    let is_person = schema_type == "Person";

    // ── Keywords ──────────────────────────────────────────────────────────────

    let keywords = Keywords::new()
        .with_explicit(schema_keywords.unwrap_or_default())
        .with_slug(&canonical_path);

    let keywords_csv = keywords.to_comma_string();

    // ── JSON-LD construction ──────────────────────────────────────────────────

    // 1. Primary entity — no @context here, it lives on the @graph wrapper.
    //
    // For Person pages the canonical author node already carries all structured
    // career / education / contact data, so we start from it and overlay the
    // page-context fields.  For all other schema types we build a generic node.
    let mut entity = if is_person {
        let mut node = author_node();
        let obj = node.as_object_mut().unwrap();
        obj.insert("mainEntityOfPage".into(), json!(canonical_url));
        obj.insert("inLanguage".into(), json!(in_language));
        node
    } else {
        json!({
            "@type": schema_type,
            "@id": canonical_url,
            "mainEntityOfPage": canonical_url,
            "name": title,
            "description": description,
            "url": canonical_url,
            "inLanguage": in_language,
        })
    };

    let base = entity.as_object_mut().unwrap();

    // Attach compact author/publisher to creative works — the full Person
    // node lives in the @graph; compact refs link to it via @id.
    if schema_type != "Person" && schema_type != "Organization" {
        base.insert("author".into(), author_node_compact());
        base.insert("publisher".into(), author_node_compact());
    }

    // 2. Article-specific: headline + image (required by Google for Article rich results).
    if is_article {
        base.insert("headline".into(), Value::String(title.clone()));
        base.insert("image".into(), Value::String(og_image.clone()));
    }

    // 3. Dates — search engines use these for freshness signals.
    if let Some(ref dp) = date_published {
        base.insert("datePublished".into(), Value::String(dp.clone()));
    }
    if let Some(ref dm) = date_modified {
        base.insert("dateModified".into(), Value::String(dm.clone()));
    }

    // 4. Keywords & GEO Entities.
    // See `SeoProps::schema_keywords` for the full list of available `@type`
    // annotations and GEO disambiguation guidance.
    if !keywords_csv.is_empty() {
        base.insert("keywords".into(), Value::String(keywords_csv.clone()));

        let about_nodes: Vec<Value> = keywords
            .as_slice()
            .iter()
            .map(|k| json!({ "@type": "Thing", "name": k }))
            .collect();

        base.insert("about".into(), json!(about_nodes));
    }

    // 4b. Speakable — tells voice assistants and AI overviews which parts of
    // the page are most suitable for text-to-speech or featured snippet extraction.
    base.insert(
        "speakable".into(),
        json!({
            "@type": "SpeakableSpecification",
            "cssSelector": ["[role=main]", "article", "#experience", "#blog", "#lab", ".prose"]
        }),
    );

    // 5. WebSite — add SearchAction for sitelinks searchbox.
    if schema_type == "WebSite" {
        base.insert(
            "potentialAction".into(),
            json!({
                "@type": "SearchAction",
                "target": format!("{SITE_URL}/blog?q={{search_term_string}}"),
                "query-input": "required name=search_term_string",
            }),
        );
    }

    // 6. Merge caller-supplied schema_data last — caller wins on conflict.
    if let Some(extra_map) = schema_data.as_ref().and_then(Value::as_object) {
        for (k, v) in extra_map {
            base.insert(k.clone(), v.clone());
        }
    }

    // 7. Breadcrumb as a separate JSON-LD node.
    let breadcrumb = breadcrumb_list(&canonical_path, &title);

    // 8. ProfessionalService node — gives crawlers a Local-Business-like
    //    signal without requiring a physical storefront address.
    let professional_service = json!({
        "@type": "ProfessionalService",
        "@id": format!("{SITE_URL}/#service"),
        "name": format!("{AUTHOR_NAME} — Rust Software Engineering"),
        "url": SITE_URL,
        "description": "Freelance Rust Software Engineering — fullstack cross-platform applications, compiler tooling, developer tools, and technical product delivery. Specialized in Axum, Dioxus, WebAssembly, and Domain-Driven Design.",
        "provider": author_node_compact(),
        "areaServed": [
            { "@type": "Country", "name": "France" },
            { "@type": "AdministrativeArea", "name": "Worldwide" }
        ],
        "serviceType": [
            "Software Engineering",
            "Rust Development",
            "Fullstack Development",
            "Software Architecture Consulting"
        ],
        "knowsAbout": [
            "Rust", "Dioxus", "Axum", "WebAssembly",
            "Domain-Driven Design", "Cross-platform Development"
        ],
        "availableLanguage": ["English", "French"],
        "priceRange": "$$",
        "location": {
            "@type": "PostalAddress",
            "addressLocality": "Niort",
            "addressRegion": "Nouvelle-Aquitaine",
            "addressCountry": "FR"
        },
    });

    // 9. Wrap all nodes in a single @graph with one @context.
    //    The full Person node is always included so that compact @id
    //    references from author/publisher fields resolve within the graph.
    let mut graph_nodes = vec![entity, breadcrumb, professional_service];

    // Only append the full Person node when the primary entity is NOT
    // already a Person (avoids duplicate Person nodes on the home page).
    if !is_person {
        graph_nodes.push(author_node());
    }

    let json_ld_graph = json!({
        "@context": "https://schema.org",
        "@graph": graph_nodes,
    });

    // 10. Serialize.
    let json_ld_string = serde_json::to_string(&json_ld_graph).unwrap_or_else(|e| {
        dioxus::prelude::warn!("JSON-LD serialisation failed: {e}");
        "{}".to_string()
    });

    // ── Derived OG article meta ───────────────────────────────────────────────

    let article_tags = article_tags.unwrap_or_default();

    rsx! {
        document::Title { "{full_title}" }

        // ── Core meta ─────────────────────────────────────────────────────────
        document::Meta { name: "description", content: "{description}" }
        // The max-* directives are crucial for Generative AI and search engines to
        // allow them to use your content in rich snippets and AI Overviews.
        document::Meta { name: "robots", content: "{robots}, max-snippet:-1, max-image-preview:large, max-video-preview:-1" }
        document::Meta { name: "author", content: AUTHOR_NAME }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        if !keywords_csv.is_empty() {
            document::Meta { name: "keywords", content: "{keywords_csv}" }
        }

        document::Link { rel: "canonical", href: "{canonical_url}" }

        // ── RSS feed discovery ────────────────────────────────────────────────
        document::Link {
            rel: "alternate",
            r#type: "application/rss+xml",
            title: "{rss_title}",
            href: "{rss_href}",
        }

        // ── Open Graph ────────────────────────────────────────────────────────
        document::Meta { property: "og:title", content: "{title}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:type", content: "{og_type}" }
        document::Meta { property: "og:image", content: "{og_image}" }
        document::Meta { property: "og:image:width", content: "{og_width}" }
        document::Meta { property: "og:image:height", content: "{og_height}" }
        document::Meta { property: "og:image:alt", content: "{og_image_alt}" }
        document::Meta { property: "og:site_name", content: SITE_NAME }
        document::Meta { property: "og:locale", content: og_locale }
        document::Meta { property: "og:locale:alternate", content: alt_locale }

        // ── OG Article meta (only for article pages) ──────────────────────────
        if is_article {
            if let Some(ref dp) = date_published {
                document::Meta { property: "article:published_time", content: "{dp}" }
            }
            if let Some(ref dm) = date_modified {
                document::Meta { property: "article:modified_time", content: "{dm}" }
            }
            document::Meta { property: "article:author", content: SITE_URL }
            for tag in &article_tags {
                document::Meta { property: "article:tag", content: "{tag}" }
            }
        }

        // ── OG Profile meta (Person pages) ────────────────────────────────────
        if is_person {
            document::Meta { property: "profile:first_name", content: "Gaëtan" }
            document::Meta { property: "profile:last_name", content: "POBLON" }
        }

        // ── Twitter Card ──────────────────────────────────────────────────────
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "{title}" }
        document::Meta { name: "twitter:description", content: "{description}" }
        document::Meta { name: "twitter:image", content: "{og_image}" }
        document::Meta { name: "twitter:image:alt", content: "{og_image_alt}" }

        // ── hreflang alternates ───────────────────────────────────────────────
        document::Link {
            rel: "alternate",
            hreflang: locale.code(),
            href: "{canonical_url}",
        }
        if let Some(alt_path) = &alternate_path {
            document::Link {
                rel: "alternate",
                hreflang: alt_lang_code,
                href: "{SITE_URL}{alt_path}",
            }
        }
        document::Link {
            rel: "alternate",
            hreflang: "x-default",
            href: "{canonical_url}",
        }

        // ── JSON-LD ───────────────────────────────────────────────────────────
        document::Script {
            r#type: "application/ld+json",
            "{json_ld_string}"
        }
    }
}
