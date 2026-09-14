use std::collections::HashMap;
use std::{env, fs, path::Path, process};

use clap::Parser;
use mdbook_frontmatter_fix::fm::Frontmatter;
use mdbook_frontmatter_fix::{
    book, comment, fm, git, html, overrides, readtime, summary, tags, toc,
};

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(name = "fmf", about = "mdBook frontmatter & content validator")]
struct Cli {
    /// Check frontmatter fields only
    #[arg(long)]
    fm: bool,
    /// Check HTML structure only
    #[arg(long)]
    html: bool,
    /// Automatically fix issues where possible
    #[arg(long)]
    fix: bool,
    /// Show what would be fixed without writing to disk
    #[arg(long)]
    dry_run: bool,
    /// Override a frontmatter field (e.g. --set title="My Title")
    #[arg(long, value_name = "KEY=VALUE")]
    set: Vec<String>,
    /// Add a tag to a specific file
    #[arg(long, value_name = "TAG")]
    tag: Vec<String>,
    /// Target file for --set and --tag overrides
    #[arg(value_name = "FILE")]
    file: Option<String>,
    /// Check include paths and internal links
    #[arg(long)]
    links: bool,
    #[arg(long)]
    edit: bool,
    #[arg(long)]
    strip: bool,
    #[arg(long)]
    toc: bool,
    #[arg(long)]
    comment: bool,
    /// Inject estimated reading time into chapter frontmatter
    #[arg(long)]
    readtime: bool,
}

fn read_or_exit(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("error: could not read {path}");
        process::exit(1);
    })
}

fn write_fixed(full_path: &str, content: String) {
    match fs::write(full_path, content) {
        Ok(()) => eprintln!("fixed: {full_path}"),
        Err(e) => eprintln!("error: could not write {full_path}: {e}"),
    }
}

fn has_diag(diags: &[fm::Diagnostic], code: &str) -> bool {
    diags.iter().any(|d| d.code == code)
}

fn handle_file_command(cli: &Cli, comment_config: &comment::CommentConfig) {
    let Some(ref file) = cli.file else { return };
    let file = file.trim_start_matches("src/");
    let full_path = format!("src/{file}");
    let Ok(content) = fs::read_to_string(&full_path) else {
        eprintln!("error: could not read {full_path}");
        process::exit(1);
    };
    let mut current = content;

    if cli.readtime {
        let result = readtime::inject_reading_time(&current);
        if cli.dry_run {
            eprintln!("would inject reading time into: {full_path}");
        } else {
            write_fixed(&full_path, result);
        }
        return;
    }

    if cli.strip {
        let mut current = current;
        if !cli.toc && !cli.comment {
            current = mdbook_frontmatter_strip::strip_frontmatter(&current);
        }
        if cli.toc {
            current = toc::strip_toc(&current);
        }
        if cli.comment {
            current = comment::strip_comment_block(&current);
        }
        if cli.dry_run {
            eprintln!("would strip from: {full_path}");
        } else {
            write_fixed(&full_path, current);
        }
        return;
    }

    if cli.toc {
        let result = toc::inject_toc(&current);
        if cli.dry_run {
            eprintln!("would inject TOC into: {full_path}\n{result}");
        } else {
            write_fixed(&full_path, result);
        }
        return;
    }

    if cli.comment {
        let result = comment::inject_comment_block_with_config(&current, comment_config);
        if cli.dry_run {
            eprintln!("would inject comment block into: {full_path}");
        } else {
            write_fixed(&full_path, result);
        }
        return;
    }

    if cli.edit {
        let fm_block = fm::extract_frontmatter(&current).unwrap_or_else(|| {
            eprintln!("error: no frontmatter found in {full_path}");
            process::exit(1);
        });
        let tmp = env::temp_dir().join("fmf_edit.yaml");
        fs::write(&tmp, &fm_block).expect("failed to write temp file");
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        process::Command::new(&editor)
            .arg(&tmp)
            .status()
            .expect("failed to open editor");
        let edited = fs::read_to_string(&tmp).expect("failed to read temp file");
        current = fm::replace_frontmatter(&current, &edited);
        write_fixed(&full_path, current);
        return;
    }

    for s in &cli.set {
        if let Some((key, value)) = overrides::parse_set(s) {
            current = overrides::apply_override(&current, key, value);
        }
    }
    if !cli.tag.is_empty() {
        current = fm::fix_missing_tags(&current, &cli.tag);
    }
    if cli.dry_run {
        eprintln!("would write: {full_path}\n{current}");
    } else {
        write_fixed(&full_path, current);
    }
}

struct FixOptions<'a> {
    lang: &'a str,
    excluded: &'a [String],
    inject_fields: &'a HashMap<String, String>,
}

fn apply_fixes(
    cli: &Cli,
    path: &str,
    full_path: &str,
    content: &str,
    diags: &[fm::Diagnostic],
    opts: &FixOptions<'_>,
) {
    let mut current = if has_diag(diags, "fm::missing-frontmatter") {
        let abs_path = Path::new(full_path)
            .canonicalize()
            .unwrap_or_else(|_| Path::new(full_path).to_path_buf());
        let commit = git::file_commit_info(&abs_path, "%Y-%m-%d", false)
            .ok()
            .flatten();
        let title = path
            .trim_end_matches(".md")
            .split('/')
            .next_back()
            .unwrap_or("untitled");
        fm::fix_frontmatter(
            content,
            &Frontmatter {
                title,
                author: commit.as_ref().map_or("Unknown", |c| c.author.as_str()),
                date: commit.as_ref().map_or("Unknown", |c| c.date.as_str()),
                lang: if opts.excluded.contains(&"lang".to_string()) {
                    ""
                } else {
                    opts.lang
                },
                tags: if opts.excluded.contains(&"tags".to_string()) {
                    vec![]
                } else {
                    tags::infer_tags(path)
                },
            },
        )
    } else {
        content.to_string()
    };
    if has_diag(diags, "fm::missing-lang") {
        current = fm::fix_missing_lang(&current, opts.lang);
    }

    if has_diag(diags, "fm::missing-tags") {
        current = fm::fix_missing_tags(&current, &tags::infer_tags(path));
    }

    for (key, value) in opts.inject_fields {
        if !current.contains(&format!("{key}:")) {
            current = overrides::apply_override(&current, key, value);
        }
    }

    if current != content {
        if cli.dry_run {
            eprintln!("would fix: {full_path}");
        } else {
            write_fixed(full_path, current);
        }
    }
}

fn try_execute_transformations(
    cli: &Cli,
    paths: &[String],
    comment_config: &comment::CommentConfig,
) -> bool {
    if cli.strip {
        process_paths(paths, cli.dry_run, "strip from", &[], |content| {
            let mut curr = content.to_string();
            if !cli.toc && !cli.comment {
                curr = mdbook_frontmatter_strip::strip_frontmatter(&curr);
            }
            if cli.toc {
                curr = toc::strip_toc(&curr);
            }
            if cli.comment {
                curr = comment::strip_comment_block(&curr);
            }
            curr
        });
        return true;
    }

    if cli.toc {
        process_paths(paths, cli.dry_run, "inject TOC into", &[], |c| {
            toc::inject_toc(c)
        });
        return true;
    }

    if cli.comment {
        process_paths(paths, cli.dry_run, "inject comment block into", &[], |c| {
            comment::inject_comment_block_with_config(c, comment_config)
        });
        return true;
    }

    if cli.readtime {
        process_paths(paths, cli.dry_run, "inject reading time into", &[], |c| {
            readtime::inject_reading_time(c)
        });
        return true;
    }

    false
}

fn process_paths<F>(
    paths: &[String],
    dry_run: bool,
    action_msg: &str,
    skip: &[&str],
    mut transform: F,
) where
    F: FnMut(&str) -> String,
{
    for path in paths {
        if skip.contains(&path.as_str()) {
            continue;
        }
        let full_path = format!("src/{path}");
        let Ok(content) = fs::read_to_string(&full_path) else {
            eprintln!("error: could not read {full_path}");
            continue;
        };

        let result = transform(&content);
        if dry_run {
            eprintln!("would {action_msg}: {full_path}");
        } else {
            write_fixed(&full_path, result);
        }
    }
}

fn run_diagnostics_and_lints(
    cli: &Cli,
    paths: &[String],
    lang: &str,
    excluded: &[String],
    injected_fields: &HashMap<String, String>,
) {
    let run_fm = cli.fm || !cli.html && !cli.links;
    let run_html = cli.html || !cli.fm && !cli.links;
    let run_links = cli.links || !cli.fm && !cli.html;
    let mut total = 0;

    for path in paths {
        let full_path = format!("src/{path}");
        let Ok(content) = fs::read_to_string(&full_path) else {
            eprintln!("error: could not read {full_path}");
            continue;
        };

        let mut diags = Vec::new();
        if run_fm {
            diags.extend(fm::check_frontmatter(&content, excluded));
        }
        if run_html {
            diags.extend(html::check_html(&content));
        }
        if run_links {
            let file_dir = Path::new(&full_path)
                .parent()
                .unwrap_or_else(|| Path::new("src"));
            diags.extend(html::check_includes(&content, file_dir));
            diags.extend(html::check_links(&content, file_dir));
        }

        for diag in &diags {
            eprintln!(
                "warning[{}]: {}\n  --> src/{}",
                diag.code, diag.message, path
            );
            total += 1;
        }

        if cli.fix || cli.dry_run {
            let fix_opts = FixOptions {
                lang,
                excluded,
                inject_fields: injected_fields,
            };
            apply_fixes(cli, path, &full_path, &content, &diags, &fix_opts);
        }
    }

    if total == 0 {
        println!("fmf: no issues found");
    } else {
        eprintln!("\nfmf: {total} issue(s) found");
        process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();

    if !Path::new("book.toml").exists() {
        eprintln!("error: no book.toml found. Run fmf from your book root");
        process::exit(1);
    }

    let lang = book::parse_language(&read_or_exit("book.toml"));

    let fmf_content = if Path::new("fmf.toml").exists() {
        read_or_exit("fmf.toml")
    } else {
        String::new()
    };
    let injected_fields = book::parse_inject_fields(&fmf_content);
    let excluded = book::parse_excluded_fields(&fmf_content);
    let comment_config = comment::parse_comment_config(&fmf_content);

    if cli.file.is_some() {
        handle_file_command(&cli, &comment_config);
        return;
    }

    let paths = summary::parse_summary(&read_or_exit("src/SUMMARY.md"));

    if try_execute_transformations(&cli, &paths, &comment_config) {
        return;
    }

    run_diagnostics_and_lints(&cli, &paths, &lang, &excluded, &injected_fields);
}
