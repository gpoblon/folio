// AI Parsing

use comrak::nodes::NodeValue;
use comrak::{Arena, Options, format_html, parse_document};

use super::preprocess::preprocess_wikilink_images;
use super::rewrite::{
    rewrite_image_url, rewrite_link_url_with_anchor, rewrite_wikilink_url_with_anchor,
};

/// Parse markdown into a comrak AST, rewrite all relative links / images /
/// anchors into absolute URLs that work correctly in a web context.
///
/// Obsidian `![[image]]` embeds are preprocessed into standard Markdown images
/// before parsing, since comrak does not recognise that syntax natively.
pub fn string_to_html(content: String) -> String {
    // Preprocess wikilink images before parsing as comrak does not support them natively.
    let content = preprocess_wikilink_images(&content);
    let options = comrak_options();
    let arena = Arena::new();
    let root = parse_document(&arena, &content, &options);

    // Walk every node in the document tree.
    for node in root.descendants() {
        let mut data = node.data.borrow_mut();
        match data.value {
            // Standard markdown link: [text](url) — also used for wikilinks
            // rendered by comrak as Link nodes with `data-wikilink="true"`.
            NodeValue::Link(ref mut link) => {
                link.url = rewrite_link_url_with_anchor(&link.url);
            }
            // Standard markdown image: ![alt](url)
            NodeValue::Image(ref mut link) => {
                link.url = rewrite_image_url(&link.url);
            }
            // Wikilink that comrak keeps as a dedicated node variant.
            // Wikilinks always reference articles; Obsidian omits `.md`.
            NodeValue::WikiLink(ref mut wl) => {
                wl.url = rewrite_wikilink_url_with_anchor(&wl.url);
            }
            _ => {}
        }
    }

    let mut html = String::new();
    format_html(root, &options, &mut html).expect("comrak HTML formatting failed");
    html
}

/// Build the comrak [`Options`] shared by both parsing and rendering.
fn comrak_options<'c>() -> Options<'c> {
    let mut options = Options::default();
    options.extension.autolink = true;
    options.extension.wikilinks_title_after_pipe = true;
    options.extension.alerts = true;
    options.extension.footnotes = true;
    options.extension.multiline_block_quotes = true;
    options.extension.highlight = true;
    options.extension.inline_footnotes = true;
    options.extension.spoiler = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.header_ids = Some(String::new());
    options.extension.shortcodes = true;
    options.extension.math_code = true;
    options.extension.tasklist = true;
    options.extension.underline = true;
    options.extension.strikethrough = true;
    options.parse.tasklist_in_table = true;
    options.render.figure_with_caption = true;
    options.render.r#unsafe = true;
    options
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(md: &str) -> String {
        string_to_html(md.to_string())
    }

    // ── Link rewriting ──────────────────────────────────────────────

    #[test]
    fn absolute_url_untouched() {
        let html = render("[x](https://example.com)");
        assert!(html.contains("href=\"https://example.com\""), "got: {html}");
    }

    #[test]
    fn anchor_rewritten_to_comrak_style() {
        let html = render("[s](#My Cool Heading)");
        assert!(html.contains("href=\"#my-cool-heading\""), "got: {html}");
    }

    #[test]
    fn already_absolute_path_untouched() {
        let html = render("![i](/resources/images/photo.png)");
        assert!(
            html.contains("src=\"/resources/images/photo.png\""),
            "got: {html}"
        );
    }

    #[test]
    fn relative_md_link_prefixed() {
        let html = render("[arch](IT/architecture.md)");
        assert!(
            html.contains("href=\"/articles/IT/architecture.md\""),
            "got: {html}"
        );
    }

    #[test]
    fn relative_image_prefixed() {
        let html = render("![p](resources/images/photo.png)");
        assert!(
            html.contains("src=\"/resources/images/photo.png\""),
            "got: {html}"
        );
    }

    #[test]
    fn code_block_links_untouched() {
        let html = render("```\n[link](relative/path.md)\n```");
        // Inside a code block the text must appear as-is, not as a hyperlink.
        assert!(!html.contains("href="), "got: {html}");
        assert!(html.contains("relative/path.md"), "got: {html}");
    }

    #[test]
    fn inline_code_links_untouched() {
        let html = render("See `[link](relative/path.md)` for details");
        assert!(!html.contains("href=\"/articles/"), "got: {html}");
    }

    // ── Anchor normalisation ────────────────────────────────────────

    #[test]
    fn percent_encoded_anchor() {
        let html = render("[go](#My%20Cool%20Heading)");
        assert!(html.contains("href=\"#my-cool-heading\""), "got: {html}");
    }

    #[test]
    fn anchor_with_apostrophe() {
        let html = render("[go](#Ticks aren't in)");
        assert!(html.contains("href=\"#ticks-arent-in\""), "got: {html}");
    }

    #[test]
    fn link_with_path_and_anchor() {
        let html = render("[go](IT/architecture.md#Design Principles)");
        assert!(
            html.contains("href=\"/articles/IT/architecture.md#design-principles\""),
            "got: {html}"
        );
    }

    // ── Wikilinks ───────────────────────────────────────────────────

    #[test]
    fn wikilink_bare_prefixed_with_articles_and_md() {
        let html = render("See [[IT/Published]] here");
        assert!(
            html.contains("href=\"/articles/IT/Published.md\""),
            "got: {html}"
        );
    }

    #[test]
    fn wikilink_with_display_text() {
        let html = render("[[IT/architecture|the arch article]]");
        assert!(
            html.contains("href=\"/articles/IT/architecture.md\""),
            "got: {html}"
        );
    }

    #[test]
    fn wikilink_already_has_md_extension() {
        let html = render("[[IT/architecture.md]]");
        assert!(
            html.contains("href=\"/articles/IT/architecture.md\""),
            "got: {html}"
        );
    }

    #[test]
    fn wikilink_with_anchor() {
        let html = render("[[IT/architecture#Design Principles]]");
        assert!(
            html.contains("href=\"/articles/IT/architecture.md#design-principles\""),
            "got: {html}"
        );
    }

    #[test]
    fn wikilink_absolute_url_untouched() {
        let html = render("[[https://example.com]]");
        assert!(html.contains("href=\"https://example.com\""), "got: {html}");
    }

    // ── Wikilink image embeds (![[…]]) ──────────────────────────────

    #[test]
    fn wikilink_image_basic() {
        let html = render("![[resources/images/photo.png]]");
        assert!(
            html.contains("src=\"/resources/images/photo.png\""),
            "got: {html}"
        );
    }

    #[test]
    fn wikilink_image_with_alt_text() {
        let html = render("![[resources/images/photo.png|my photo]]");
        assert!(
            html.contains("src=\"/resources/images/photo.png\""),
            "got: {html}"
        );
        assert!(html.contains("alt=\"my photo\""), "got: {html}");
    }

    #[test]
    fn wikilink_image_absolute_path() {
        let html = render("![[/assets/test.png]]");
        assert!(html.contains("src=\"/assets/test.png\""), "got: {html}");
    }

    #[test]
    fn wikilink_image_with_spaces() {
        let html = render("![[resources/my image.png]]");
        assert!(
            html.contains("src=\"/resources/my%20image.png\""),
            "got: {html}"
        );
    }

    #[test]
    fn wikilink_image_in_code_block_untouched() {
        let html = render("```\n![[not-an-image.png]]\n```");
        assert!(!html.contains("<img"), "got: {html}");
        assert!(html.contains("![[not-an-image.png]]"), "got: {html}");
    }

    #[test]
    fn wikilink_image_in_inline_code_untouched() {
        let html = render("See `![[not-an-image.png]]` here");
        assert!(!html.contains("<img"), "got: {html}");
        assert!(html.contains("![[not-an-image.png]]"), "got: {html}");
    }

    #[test]
    fn wikilink_image_mixed_with_regular_wikilink() {
        let html = render("Link: [[page]] and image: ![[img.png]]");
        assert!(html.contains("href=\"/articles/page.md\""), "got: {html}");
        assert!(html.contains("<img"), "got: {html}");
    }

    // ── Image edge cases ────────────────────────────────────────────

    #[test]
    fn external_image_untouched() {
        let html = render("![x](https://img.example.com/pic.png)");
        assert!(
            html.contains("src=\"https://img.example.com/pic.png\""),
            "got: {html}"
        );
    }

    // ── Heading IDs are generated ───────────────────────────────────

    #[test]
    fn heading_gets_id() {
        let html = render("## My Cool Heading");
        assert!(html.contains("id=\"my-cool-heading\""), "got: {html}");
    }
}
