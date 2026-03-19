// AI parsing

/// Returns `true` when the URL is already absolute or points to an external
/// resource (http(s), mailto, …) or is a pure fragment (`#…`).
fn is_absolute_or_external(url: &str) -> bool {
    url.starts_with('/')
        || url.starts_with("http")
        || url.starts_with("mailto:")
        || url.starts_with('#')
}

/// Rewrite a relative **link** URL so the browser resolves it correctly.
///
/// - Markdown article paths (ending in `.md`) → `/articles/<path>`
/// - Everything else (resources, etc.)        → `/<path>`
fn rewrite_link_url(url: &str) -> String {
    if is_absolute_or_external(url) {
        return url.to_string();
    }
    if url.ends_with(".md") {
        format!("/articles/{url}")
    } else {
        format!("/{url}")
    }
}

/// Rewrite a **wikilink** URL. Wikilinks always reference articles in the
/// knowledge-base. Obsidian omits the `.md` extension, so we append it when
/// missing and always prefix with `/articles/`.
fn rewrite_wikilink_url(url: &str) -> String {
    if is_absolute_or_external(url) {
        return url.to_string();
    }
    if url.ends_with(".md") {
        format!("/articles/{url}")
    } else {
        format!("/articles/{url}.md")
    }
}

/// Rewrite a relative **image** URL. Images always live under the
/// `resources/` tree in the knowledge-base, so we just ensure a leading `/`.
pub(super) fn rewrite_image_url(url: &str) -> String {
    if is_absolute_or_external(url) {
        return url.to_string();
    }
    format!("/{url}")
}

/// Convert an Obsidian-style anchor (the raw heading text, possibly
/// percent-encoded) into the anchor comrak will generate for the
/// corresponding heading.
///
/// Obsidian anchors look like `#My Cool Heading` or `#My%20Cool%20Heading`
/// while comrak produces `my-cool-heading` (GFM-style).
fn anchorize_fragment(fragment: &str) -> String {
    // Percent-decode first so `%20` becomes a space, etc.
    let decoded = percent_decode(fragment);
    // Re-use comrak's own algorithm so we are guaranteed to match.
    let mut anchorizer = comrak::Anchorizer::new();
    format!("#{}", anchorizer.anchorize(&decoded))
}

/// Minimal percent-decoding (covers the common `%XX` sequences Obsidian emits).
fn percent_decode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hi = chars.next();
            let lo = chars.next();
            if let (Some(h), Some(l)) = (hi, lo) {
                let mut buf = [0u8; 2];
                buf[0] = h as u8;
                buf[1] = l as u8;
                if let Ok(s) = std::str::from_utf8(&buf)
                    && let Ok(byte) = u8::from_str_radix(s, 16)
                {
                    out.push(byte as char);
                    continue;
                }
                // Not a valid hex pair — emit verbatim.
                out.push('%');
                out.push(h);
                out.push(l);
            } else {
                out.push('%');
                if let Some(h) = hi {
                    out.push(h);
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Rewrite a standard link URL that may contain both a path and a fragment.
///
/// Examples:
///   `#My Heading`              → `#my-heading`
///   `IT/arch.md#Some Section`  → `/articles/IT/arch.md#some-section`
///   `https://example.com`      → unchanged
pub(super) fn rewrite_link_url_with_anchor(url: &str) -> String {
    rewrite_url_with_anchor(url, rewrite_link_url)
}

/// Rewrite a wikilink URL that may contain both a path and a fragment.
///
/// Wikilinks always target articles and Obsidian omits `.md`, so the path
/// component is always treated as an article reference.
///
/// Examples:
///   `IT/Published`              → `/articles/IT/Published.md`
///   `IT/Published#Some Section` → `/articles/IT/Published.md#some-section`
pub(super) fn rewrite_wikilink_url_with_anchor(url: &str) -> String {
    rewrite_url_with_anchor(url, rewrite_wikilink_url)
}

/// Shared implementation for link/wikilink URL rewriting with fragment
/// normalisation. The `rewrite_path` closure decides how the path portion
/// is rewritten.
fn rewrite_url_with_anchor(url: &str, rewrite_path: fn(&str) -> String) -> String {
    // Pure fragment
    if let Some(fragment) = url.strip_prefix('#') {
        return anchorize_fragment(fragment);
    }

    // Path + optional fragment
    if let Some((path, fragment)) = url.split_once('#') {
        let rewritten_path = rewrite_path(path);
        let rewritten_fragment = anchorize_fragment(fragment);
        // anchorize_fragment already includes the leading '#'
        return format!("{rewritten_path}{rewritten_fragment}");
    }

    rewrite_path(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_url_untouched() {
        assert_eq!(
            rewrite_link_url("https://example.com"),
            "https://example.com"
        );
    }

    #[test]
    fn root_relative_path_untouched() {
        assert_eq!(rewrite_link_url("/already/absolute"), "/already/absolute");
    }

    #[test]
    fn relative_md_link_prefixed() {
        assert_eq!(
            rewrite_link_url("IT/architecture.md"),
            "/articles/IT/architecture.md"
        );
    }

    #[test]
    fn relative_non_md_link_prefixed() {
        assert_eq!(
            rewrite_link_url("resources/file.txt"),
            "/resources/file.txt"
        );
    }

    #[test]
    fn wikilink_bare_prefixed_with_articles_and_md() {
        assert_eq!(
            rewrite_wikilink_url("IT/Published"),
            "/articles/IT/Published.md"
        );
    }

    #[test]
    fn wikilink_with_md_extension() {
        assert_eq!(
            rewrite_wikilink_url("IT/architecture.md"),
            "/articles/IT/architecture.md"
        );
    }

    #[test]
    fn image_url_prefixed() {
        assert_eq!(
            rewrite_image_url("resources/images/photo.png"),
            "/resources/images/photo.png"
        );
    }

    #[test]
    fn absolute_image_untouched() {
        assert_eq!(
            rewrite_image_url("/already/absolute.png"),
            "/already/absolute.png"
        );
    }

    #[test]
    fn external_image_untouched() {
        assert_eq!(
            rewrite_image_url("https://example.com/pic.png"),
            "https://example.com/pic.png"
        );
    }

    #[test]
    fn anchor_rewritten_to_comrak_style() {
        assert_eq!(anchorize_fragment("My Cool Heading"), "#my-cool-heading");
    }

    #[test]
    fn percent_encoded_anchor() {
        assert_eq!(
            anchorize_fragment("My%20Cool%20Heading"),
            "#my-cool-heading"
        );
    }

    #[test]
    fn anchor_with_special_chars() {
        assert_eq!(anchorize_fragment("Ticks aren't in"), "#ticks-arent-in");
    }

    #[test]
    fn link_with_path_and_anchor() {
        assert_eq!(
            rewrite_link_url_with_anchor("IT/architecture.md#Design Principles"),
            "/articles/IT/architecture.md#design-principles"
        );
    }

    #[test]
    fn wikilink_with_anchor() {
        assert_eq!(
            rewrite_wikilink_url_with_anchor("IT/architecture#Design Principles"),
            "/articles/IT/architecture.md#design-principles"
        );
    }

    #[test]
    fn pure_fragment() {
        assert_eq!(rewrite_link_url_with_anchor("#My Heading"), "#my-heading");
    }
}
