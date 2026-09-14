# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-09-14

### Added
- **(comment)**add Bluesky comment style with bluesky-comments web component

### Fixed
- **(main)**handle --fix flag for single file in handle_file_command
- **(html)**handle angle bracket URLs and use rfind for closing paren in check_links
- **(html)**strip line range suffix from include paths before checking existence
- **(main)**unnecessary calls of to_string(), clippy suggestion yesterday
- **(toc)**strip HTML tags and emojis from anchor generation
- **(README)**wrong install instructions

## [0.4.0] - 2026-09-14

### Added
- **(main)**add injected fields
- **(book)**add parse_inject_fields to parse & inject custom fields from fmf.toml
- **(main)**tie in readtime
- **(readtime)**add function to inject readtime estimates into chapters
- **(readtime)**add estimate_reading_time using ceiling division at 200 wpm
- **(main)**enable --strip --toc and --strip --comment on individual files
- **(main)**tie in new strip commands
- **(comment)**add strip_comment_block to strip injected Comments blocks
- **(toc)**add strip_toc to strip toc out of chapters
- **(comment)**add parse_comment_config to read Giscus settings from fmf.toml
- **(comment)**add CommentConfig inject_comment_block_with_config with Giscus support
- **(main)**wire in --comment, add to Cli struct
- **(comment)**skip injection if comment block already exists
- **(comment)**add inject_comment_block to append collapsible comments section

### Changed
- **(main)**make struct for apply_fixes

### Fixed
- **(main)**typo
- **(readtime)**inject readtime badge after fm rather than in it
- **(main)**chain fixes on mutable current to prevent multiple writes and clobbering
- **(fm)**add newline after tags: key in fix_frontmatter
- **(main)**check --strip before --toc & --comment in handle_file_command
- **(main)**skip injecting Comments block into README.md

## [0.3.0] - 2026-09-14

### Added
- **(main)**wire in --toc for all chapters at once
- **(toc)**add support for header level 4
- **(toc)**add relative indentation
- **(toc)**add check for existing toc
- **(main)**wire in --toc
- **(toc)**add inject_toc to insert table of contents after frontmatter
- **(toc)**add nested heading support with indentation for h3

## [0.2.0] - 2026-09-14

### Added
- **(main)**add --strip flag to remove frontmatter from one or all chapters
- **(main)**wire in --strip, add flag to Cli struct
- **(main)**wire in --edit, add flag to Cli struct
- **(fm)**implement replace_frontmatter to swap edited frontmatter
- **(fm)**implement extract_frontmatter to isolate the frontmatter block

### Changed
- **(main)**extract handle_file_command and apply_fixes into separate functions

### Fixed
- **(main)**trim body content, just show what's being stripped with --strip

## [0.1.0] - 2026-09-14

### Added
- **(main)**respect exclude_fields when injecting frontmatter with --fix
- **(fm)**skip empty/excluded fields on fix_frontmatter
- **(fm)**add excluded fields support to check_frontmatter
- **(book)**add parse_excluded_fields to read fmf config from book.toml
- **(html)**add extension skip inside check_links
- **(main)**wire in check_links
- **(html)**add check_links to detect broken internal markdown links
- **(main)**wire in check_includes into main validation loop
- **(html)**add check_includes function to pass test
- **(main)**wire in --set --tag to main. When a --file is specified along with --set or --tag, apply the overrides to that file directly without running all checks
- **(overrides)**add missing field when applying override
- **(overrides)**implement apply_override to override fm values
- **(main)**add set tag and file fields to Cli struct
- **(overrides)**implement parse_set to parse key=value pairs & pass test
- **(main)**wire --dry-run flag to preview fixes without writing to disk
- **(lib)**implement run_on_chapter with dry-run support
- **(main)**wire in fix_missing_lang & fix_missing_tags
- add CHANGELOG with git-cliff
- **(fm)**implement fix_missing_lang
- **(fm)**add fix_missing_tags function to pass test
- **(fm)**add check for missing tags & test
- **(fm)**tie in tags to Frontmatter struct & fix_frontmatter function
- **(tags)**implement infer_tags function to pass test
- **(fm)**implement fix_missing_lang
- add lang check and diagnostic
- add lang field to Frontmatter struct & wire into --fix flow
- **(book)**add parse_language function to pass test
- implement --fix flag to write missing frontmatter to disk
- **(fm)**add fix_frontmatter with Frontmatter struct
- **(fm)**add check for unclosed YAML fence
- **(main)**wire check_frontmatter and check_html into main validation loop
- **(main)**read SUMMARY.md and parse it to collect chapter paths
- **(main)**exit early if book.toml isn't found in current directory
- **(summary)**strip ./ prefix from chapter paths in summary parser
- **(summary)**write parse_summary function to make test pass
- add CLI argument parsing with --fm and --html flags
- **(html)**add check for missing </summary> block to check_html
- **(html)**implement check_html function and detect missing </details> blocks
- **(fm)**add title check to check_frontmatter
- **(fm)**add author check to check_frontmatter
- **(fm)**add date check to check_frontmatter
- add check_frontmatter function to return a diagnostic when it detects missing frontmatter
- layout project structure

### Changed
- **(lib)**extract run_on_chapter for testable dry-run support
- **(main)**break down main into helper functions
- **(main)**reorganize main
- **(html)**create check_tag_balance helper to reduce replication
- create has_field helper function to reduce repetition

### Fixed
- **(book)**simplify parse_excluded_fields
- **(main)**trim src/ from file path matches
- **(summary)**use rfind in parse_summary to correctly parse chapter paths containing parentheses/backticks in title

