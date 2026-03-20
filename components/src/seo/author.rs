//! Canonical `Person` JSON-LD node for the site author.

use kernel::seo::{AUTHOR_IMAGE, AUTHOR_JOB_TITLE, AUTHOR_NAME, AUTHOR_SAME_AS, SITE_URL};
use serde_json::{Value, json};

/// The single canonical `Person` node representing the site author.
///
/// Injected into every JSON-LD object so page components never repeat
/// author information. Includes `image` for Google Knowledge Panel.
pub fn author_node() -> Value {
    json!({
        "@type": "Person",
        "@id": format!("{SITE_URL}/#person"),
        "name": AUTHOR_NAME,
        "jobTitle": AUTHOR_JOB_TITLE,
        "url": SITE_URL,
        "image": AUTHOR_IMAGE,
        "sameAs": AUTHOR_SAME_AS,
    })
}
