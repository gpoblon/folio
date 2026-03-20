//! `BreadcrumbList` JSON-LD generation from a canonical URL path.

use kernel::seo::SITE_URL;
use serde_json::{Value, json};

/// Builds a `BreadcrumbList` JSON-LD node from the canonical path.
///
/// `/lab/my-project` → Home > Lab > my-project
pub fn breadcrumb_list(canonical_path: &str, page_title: &str) -> Value {
    let segments: Vec<&str> = canonical_path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let mut items = Vec::with_capacity(segments.len() + 1);
    items.push(json!({
        "@type": "ListItem",
        "position": 1,
        "name": "Home",
        "item": SITE_URL,
    }));

    let mut accumulated = String::new();
    for (i, segment) in segments.iter().enumerate() {
        accumulated.push('/');
        accumulated.push_str(segment);

        let is_last = i == segments.len() - 1;
        let name = if is_last {
            page_title.to_string()
        } else {
            capitalize(segment)
        };

        items.push(json!({
            "@type": "ListItem",
            "position": i + 2,
            "name": name,
            "item": format!("{SITE_URL}{accumulated}"),
        }));
    }

    json!({
        "@type": "BreadcrumbList",
        "itemListElement": items,
    })
}

/// Uppercase-first, leave the rest unchanged.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
