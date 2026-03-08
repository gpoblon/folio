mod enums;

pub(crate) mod ui;
pub use enums::{Expertise, Intent, State};
pub use ui::MetadataHeader;
pub use ui::MetadataPreview;

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};

/// Fields shared by every content entry (articles and projects alike).
///
/// Concrete metadata types (`ArticleMetadata`, `ProjectMetadata`) embed this
/// struct via `#[serde(flatten)]` so the YAML / frontmatter shape stays flat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub slug: String,
    #[serde(default)]
    pub lang: kernel::lang::Lang,
    pub tags: Vec<Intent>,
    #[serde(default)]
    pub state: State,
    #[serde(default)]
    pub expertise: Expertise,
    #[serde(default, deserialize_with = "deserialize_naive_datetime")]
    pub created: Option<NaiveDateTime>,
    #[serde(default, deserialize_with = "deserialize_naive_datetime")]
    pub modified: Option<NaiveDateTime>,
}

/// `chrono::NaiveDateTime`'s default serde impl calls `deserialize_any`, which
/// causes `serde_saphyr` to choke on quoted datetime strings (e.g. "2025-01-01
/// 00:00:00"). Reading the value as a plain `String` first and then parsing it
/// avoids the issue entirely.
fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match Option::<String>::deserialize(deserializer)? {
        None => return Ok(None),
        Some(s) => s,
    };
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
        .map(Some)
        .map_err(serde::de::Error::custom)
}
