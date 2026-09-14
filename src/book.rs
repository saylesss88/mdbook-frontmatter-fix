use std::collections::HashMap;

#[must_use]
pub fn parse_language(content: &str) -> String {
    for line in content.lines() {
        if line.starts_with("language")
            && let Some(val) = line.split('=').nth(1)
        {
            return val.trim().trim_matches('"').to_string();
        }
    }
    "en".to_string()
}

#[must_use]
pub fn parse_excluded_fields(content: &str) -> Vec<String> {
    for line in content.lines() {
        if line.trim().starts_with("exclude_fields")
            && let Some(val) = line.split('=').nth(1)
        {
            return val
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    Vec::new()
}

#[must_use]
pub fn parse_inject_fields(content: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    let mut in_inject = false;

    for line in content.lines() {
        if line.trim() == "[inject]" {
            in_inject = true;
            continue;
        }
        if line.starts_with('[') {
            in_inject = false;
        }
        if in_inject && let Some((key, value)) = line.split_once('=') {
            fields.insert(
                key.trim().to_string(),
                value.trim().trim_matches('"').to_string(),
            );
        }
    }

    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::fm;

    #[test]
    fn parses_language_from_book_toml() {
        let content = "[book]\ntitle = \"My Book\"\nlanguage = \"en\"\n";
        let lang = parse_language(content);
        assert_eq!(lang, "en");
    }

    #[test]
    fn missing_language_defaults_to_en() {
        let content = "[book]\ntitle = \"My Book\"\nauthors = [\"Tom\"]\n";
        let lang = parse_language(content);
        assert_eq!(lang, "en");
    }

    #[test]
    fn parses_exclude_fields_from_fmf_toml() {
        let content = "exclude_fields = [\"author\", \"lang\"]\n";
        let excluded = parse_excluded_fields(content);
        assert_eq!(excluded, vec!["author", "lang"]);
    }

    #[test]
    fn missing_fmf_section_returns_empty_excluded_fields() {
        let content = "[book]\ntitle = \"My Book\"\n";
        let excluded = parse_excluded_fields(content);
        assert_eq!(excluded, [] as [std::string::String; 0]);
    }

    #[test]
    fn excluded_fields_are_not_checked() {
        let content = "---\ntitle: Hello\nauthor: Jr\ndate: 2026-09-03\n---\n\nContent.\n";
        let excluded = vec![
            "lang".to_string(),
            "tags".to_string(),
            "author-email".to_string(),
        ];
        let diags = fm::check_frontmatter(content, &excluded);
        assert!(diags.is_empty());
    }

    #[test]
    fn parses_custom_inject_fields() {
        let content = "[inject]\nversion = \"1.0\"\nstatus = \"draft\"\n";
        let fields = parse_inject_fields(content);
        assert_eq!(fields.get("version"), Some(&"1.0".to_string()));
        assert_eq!(fields.get("status"), Some(&"draft".to_string()));
    }
}
