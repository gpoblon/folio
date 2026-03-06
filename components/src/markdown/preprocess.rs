// AI parsing

/// Convert Obsidian-style `![[path]]` and `![[path|alt]]` image embeds into
/// standard Markdown `![alt](url)` so that comrak can parse them as `Image`
/// nodes.
///
/// The replacement is skipped inside fenced code blocks and inline code spans
/// so that literal `![[…]]` text in code is preserved.
pub(super) fn preprocess_wikilink_images(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // ── Fenced code block (``` or ~~~) ──────────────────────────
        if i == 0 || (i > 0 && chars[i - 1] == '\n') {
            if let Some(fence_end) = skip_fenced_code_block(&chars, i) {
                let block: String = chars[i..fence_end].iter().collect();
                result.push_str(&block);
                i = fence_end;
                continue;
            }
        }

        // ── Inline code span (`…`) ─────────────────────────────────
        if chars[i] == '`' {
            if let Some(span_end) = skip_inline_code(&chars, i) {
                let span: String = chars[i..span_end].iter().collect();
                result.push_str(&span);
                i = span_end;
                continue;
            }
        }

        // ── Wikilink image embed: ![[…]] ───────────────────────────
        if i + 2 < len && chars[i] == '!' && chars[i + 1] == '[' && chars[i + 2] == '[' {
            if let Some(close) = find_closing_double_brackets(&chars, i + 3) {
                let inner: String = chars[i + 3..close].iter().collect();
                // Obsidian supports ![[path|alt text]]
                let (path, alt) = match inner.find('|') {
                    Some(pos) => (&inner[..pos], &inner[pos + 1..]),
                    None => (inner.as_str(), inner.as_str()),
                };
                // Percent-encode spaces so the URL is valid Markdown.
                let encoded_path = path.replace(' ', "%20");
                result.push_str(&format!("![{alt}]({encoded_path})"));
                i = close + 2; // skip past ]]
                continue;
            }
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Find the closing `]]` starting from `start`, returning the index of the
/// first `]`. Does not span across newlines.
fn find_closing_double_brackets(chars: &[char], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < chars.len() {
        if chars[i] == ']' && chars[i + 1] == ']' {
            return Some(i);
        }
        if chars[i] == '\n' {
            return None;
        }
        i += 1;
    }
    None
}

/// If position `i` starts a fenced code block (three or more `` ` `` or `~`
/// at the beginning of a line), return the index just past the closing fence
/// (including its trailing newline, if any). Returns `None` otherwise.
fn skip_fenced_code_block(chars: &[char], i: usize) -> Option<usize> {
    let len = chars.len();
    let fence_char = chars[i];
    if fence_char != '`' && fence_char != '~' {
        return None;
    }

    // Count opening fence length (must be >= 3).
    let mut fence_len = 0;
    while i + fence_len < len && chars[i + fence_len] == fence_char {
        fence_len += 1;
    }
    if fence_len < 3 {
        return None;
    }

    // Skip to end of the opening fence line.
    let mut j = i + fence_len;
    while j < len && chars[j] != '\n' {
        j += 1;
    }
    if j < len {
        j += 1; // skip the '\n'
    }

    // Scan for a closing fence of at least the same length.
    while j < len {
        let mut cl = 0;
        while j + cl < len && chars[j + cl] == fence_char {
            cl += 1;
        }
        if cl >= fence_len {
            // Skip rest of closing fence line.
            let mut k = j + cl;
            while k < len && chars[k] != '\n' {
                k += 1;
            }
            if k < len {
                k += 1;
            }
            return Some(k);
        }
        // Skip to next line.
        while j < len && chars[j] != '\n' {
            j += 1;
        }
        if j < len {
            j += 1;
        }
    }

    // Unclosed fence — treat everything to the end as code.
    Some(len)
}

/// If position `i` is a backtick starting an inline code span, return the
/// index just past the closing backtick(s). Returns `None` if the span isn't
/// closed on the same line.
fn skip_inline_code(chars: &[char], i: usize) -> Option<usize> {
    let len = chars.len();
    // Count opening backticks.
    let mut ticks = 0;
    while i + ticks < len && chars[i + ticks] == '`' {
        ticks += 1;
    }

    // Look for the matching sequence of backticks (same length).
    let mut j = i + ticks;
    while j + ticks <= len {
        if chars[j] == '\n' {
            return None;
        }
        if chars[j] == '`' {
            let mut cl = 0;
            while j + cl < len && chars[j + cl] == '`' {
                cl += 1;
            }
            if cl == ticks {
                return Some(j + cl);
            }
            j += cl;
        } else {
            j += 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wikilink_image_basic() {
        let result = preprocess_wikilink_images("![[resources/images/photo.png]]");
        assert_eq!(
            result,
            "![resources/images/photo.png](resources/images/photo.png)"
        );
    }

    #[test]
    fn wikilink_image_with_alt_text() {
        let result = preprocess_wikilink_images("![[resources/images/photo.png|my photo]]");
        assert_eq!(result, "![my photo](resources/images/photo.png)");
    }

    #[test]
    fn wikilink_image_with_spaces() {
        let result = preprocess_wikilink_images("![[resources/my image.png]]");
        assert_eq!(
            result,
            "![resources/my image.png](resources/my%20image.png)"
        );
    }

    #[test]
    fn wikilink_image_in_code_block_untouched() {
        let input = "```\n![[not-an-image.png]]\n```";
        let result = preprocess_wikilink_images(input);
        assert!(result.contains("![[not-an-image.png]]"));
        assert!(!result.contains("![not-an-image.png]"));
    }

    #[test]
    fn wikilink_image_in_inline_code_untouched() {
        let input = "See `![[not-an-image.png]]` here";
        let result = preprocess_wikilink_images(input);
        assert!(result.contains("![[not-an-image.png]]"));
        assert!(!result.contains("![not-an-image.png]"));
    }

    #[test]
    fn regular_wikilink_unchanged() {
        let result = preprocess_wikilink_images("[[page]]");
        assert_eq!(result, "[[page]]");
    }

    #[test]
    fn mixed_wikilinks_and_images() {
        let input = "Link: [[page]] and image: ![[img.png]]";
        let result = preprocess_wikilink_images(input);
        assert!(result.contains("[[page]]"));
        assert!(result.contains("![img.png](img.png)"));
    }
}
