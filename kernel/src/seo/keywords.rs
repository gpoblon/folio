//! Keyword extraction and deduplication for SEO structured data.
//!
//! Centralises the slug → keyword pipeline so that `components::Seo`,
//! `app::seo_routes` (RSS categories), and any future consumer share
//! a single, tested implementation.

/// An ordered, case-insensitively deduplicated collection of SEO keywords.
///
/// # Construction
///
/// ```ignore
/// let kw = Keywords::new()
///     .with_explicit(vec!["Rust".into(), "Dioxus".into()])
///     .with_slug("it/dev/lang/rust/intro.md");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Keywords(Vec<String>);

impl Keywords {
    /// Creates an empty keyword set.
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Seeds the set with explicitly provided keywords (e.g. article tags).
    #[must_use]
    pub fn with_explicit(mut self, words: Vec<String>) -> Self {
        for w in words {
            self.insert(w);
        }
        self
    }

    /// Extracts implicit keywords from a URL slug or path by splitting on `/`
    /// and stripping the file extension from the final segment.
    ///
    /// `"it/dev/lang/rust/intro.md"` → `["it", "dev", "lang", "rust", "intro"]`
    #[must_use]
    pub fn with_slug(mut self, slug: &str) -> Self {
        for segment in Self::slug_segments(slug) {
            self.insert(segment);
        }
        self
    }

    /// Inserts a keyword if no case-insensitive duplicate already exists.
    pub fn insert(&mut self, keyword: String) {
        let normalised = keyword.trim().to_lowercase();
        if !normalised.is_empty() && !self.0.iter().any(|k| k.trim().to_lowercase() == normalised) {
            self.0.push(keyword);
        }
    }

    /// Returns a comma-separated string suitable for JSON-LD `"keywords"`.
    pub fn to_comma_string(&self) -> String {
        self.0.join(", ")
    }

    /// Returns the inner `Vec<String>`, consuming `self`.
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }

    /// Returns a reference to the inner vec.
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    /// Returns `true` if no keywords have been collected.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Splits a slug into keyword-worthy segments, stripping the file
    /// extension from the last segment.
    ///
    /// This is the single canonical implementation of what was previously
    /// `slug_keywords` (components) and `slug_categories` (app/seo_routes).
    pub fn slug_segments(slug: &str) -> Vec<String> {
        let segments: Vec<&str> = slug.split('/').filter(|s| !s.is_empty()).collect();
        let last_idx = segments.len().saturating_sub(1);
        segments
            .into_iter()
            .enumerate()
            .map(|(i, seg)| {
                if i == last_idx {
                    seg.rsplit_once('.').map(|(name, _)| name).unwrap_or(seg)
                } else {
                    seg
                }
                .to_string()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_segments_strips_extension() {
        let segments = Keywords::slug_segments("it/dev/lang/rust/intro.md");
        assert_eq!(segments, vec!["it", "dev", "lang", "rust", "intro"]);
    }

    #[test]
    fn slug_segments_no_extension() {
        let segments = Keywords::slug_segments("/blog/my-post");
        assert_eq!(segments, vec!["blog", "my-post"]);
    }

    #[test]
    fn deduplication_is_case_insensitive() {
        let kw = Keywords::new().with_explicit(vec!["Rust".into(), "rust".into(), "RUST".into()]);
        assert_eq!(kw.as_slice(), &["Rust"]);
    }

    #[test]
    fn merge_preserves_order() {
        let kw = Keywords::new()
            .with_explicit(vec!["Alpha".into(), "Beta".into()])
            .with_slug("gamma/delta");
        assert_eq!(kw.as_slice(), &["Alpha", "Beta", "gamma", "delta"]);
    }

    #[test]
    fn empty_and_whitespace_keywords_ignored() {
        let kw = Keywords::new().with_explicit(vec!["".into(), "  ".into(), "Valid".into()]);
        assert_eq!(kw.as_slice(), &["Valid"]);
    }
}
