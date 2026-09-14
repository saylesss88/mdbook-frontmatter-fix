use std::path::Path;

use crate::fm::Diagnostic;

#[must_use]
pub fn check_html(content: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    diags.extend(check_tag_balance(
        content,
        "details",
        "html::unclosed-details",
    ));
    diags.extend(check_tag_balance(
        content,
        "summary",
        "html::unclosed-summary",
    ));
    diags
}

fn check_tag_balance(content: &str, tag: &str, code: &'static str) -> Option<Diagnostic> {
    let opens = content.matches(&format!("<{tag}>")).count();
    let closes = content.matches(&format!("</{tag}>")).count();

    if opens == closes {
        None
    } else {
        Some(Diagnostic {
            code,
            message: format!("unclosed <{tag}> block"),
        })
    }
}

#[must_use]
pub fn check_includes(content: &str, file_dir: &Path) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    for line in content.lines() {
        if let Some(inner) = line
            .strip_prefix("{{#include ")
            .and_then(|s| s.strip_suffix("}}"))
        {
            // Strip line range suffix: path:N or path:N:M
            let path = inner.split(':').next().unwrap_or(inner).trim();
            let include_path = file_dir.join(path);
            if !include_path.exists() {
                diags.push(Diagnostic {
                    code: "html::broken-include",
                    message: format!("include path does not exist: {inner}"),
                });
            }
        }
    }
    diags
}

#[must_use]
pub fn check_links(content: &str, file_dir: &Path) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    for line in content.lines() {
        // Match markdown links: [text](path)
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            rest = &rest[start + 2..];
            let Some(end) = rest.find(')') else { break };
            let target = &rest[..end];
            rest = &rest[end + 1..];

            // Skip external links and anchors
            if target.starts_with("http") || target.starts_with('#') || target.is_empty() {
                continue;
            }

            // Strip anchor from path
            let path = target.split('#').next().unwrap_or(target);

            // Skip non-markdown links (images, PDFs, etc.)
            let ext = std::path::Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            if !matches!(ext, "md" | "markdown" | "") {
                continue;
            }

            let resolved = file_dir.join(path);
            if !resolved.exists() {
                diags.push(Diagnostic {
                    code: "html::broken-link",
                    message: format!("broken internal link: {target}"),
                });
            }
        }
    }

    diags
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn unclosed_details_block_produces_diagnostic() {
        let content = "<details>\n<summary>Click me</summary>\n\nSome content.\n";
        let diags = check_html(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::unclosed-details");
    }
    #[test]
    fn unclosed_summary_block_produces_diagnostic() {
        let content = "<details>\n<summary>Click me\n\nSome content.\n</details>\n";
        let diags = check_html(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::unclosed-summary");
    }

    #[test]
    fn broken_include_path_produces_diagnostic() {
        let dir = tempfile::tempdir().unwrap();
        let file_dir = dir.path();
        let content = "{{#include ../nonexistent.rs}}\n";
        let diags = check_includes(content, file_dir);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::broken-include");
    }

    #[test]
    fn valid_include_path_produces_no_diagnostics() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("example.rs");
        fs::write(&file, "fn main() {}").unwrap();
        let content = "{{#include example.rs}}\n";
        let diags = check_includes(content, dir.path());
        assert!(diags.is_empty());
    }

    #[test]
    fn broken_internal_link_produces_diagnostic() {
        let dir = tempfile::tempdir().unwrap();
        let content = "[see this](./nonexistent.md)\n";
        let diags = check_links(content, dir.path());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::broken-link");
    }

    #[test]
    fn valid_internal_link_produces_no_diagnostics() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("existing.md");
        std::fs::write(&target, "# Existing").unwrap();
        let content = "[see this](existing.md)\n";
        let diags = check_links(content, dir.path());
        assert!(diags.is_empty());
    }

    #[test]
    fn image_links_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let content = "![logo](img/rust-gaps.png)\n";
        let diags = check_links(content, dir.path());
        assert!(diags.is_empty());
    }

    #[test]
    fn include_with_line_range_is_valid() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("style.css");
        std::fs::write(&file, "body { color: red; }").unwrap();
        let content = "{{#include style.css:25:29}}\n";
        let diags = check_includes(content, dir.path());
        assert!(diags.is_empty());
    }
}
