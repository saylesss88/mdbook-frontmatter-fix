# mdbook-frontmatter-fix

A Swiss Army knife for [mdBook](https://rust-lang.github.io/mdBook/) projects.
Validates frontmatter, HTML structure, and broken links with clippy-style
diagnostics. Injects, fixes, or strips content across every chapter
automatically.

```
warning[fm::missing-date]: frontmatter has no 'date' field
  --> src/io/input_output.md
warning[html::unclosed-details]: unclosed <details> block
  --> src/err/write_own_error_type.md

fmv: 2 issue(s) found
```

## Features

- **Validate**: clippy-style diagnostics for frontmatter fields, HTML structure, and broken links
- **Fix**: auto-inject frontmatter from git history and `book.toml` with a single command
- **Inject**: tables of contents, reading time badges, and collapsible Giscus comment widgets
- **Override**: set or edit individual frontmatter fields per chapter
- **Strip**: remove frontmatter, TOC blocks, or comment blocks from any chapter
- **Configure**: exclude fields, inject custom fields, and set Giscus config via `fmf.toml`

---

## Installation

```sh
cargo install mdbook-frontmatter-fix
```

Run from your book root (where `book.toml` lives).

## Usage

```sh
fmf                                          # run all checks
fmf --fm                                     # frontmatter checks only
fmf --html                                   # HTML structure checks only
fmf --fix                                    # auto-fix what can be fixed
fmf --dry-run                                # preview fixes without writing
fmf --set "title=My Title" io/my-file.md     # override a field in one file
fmf --tag rust --tag cli io/my-file.md       # set tags on one file
fmf --strip                                  # strip fm from all chapters
fmf --strip io/input_output.md               # strip fm from single file
fmf --strip --dry-run                        # preview without writing
fmf --edit io/input_output.md                # opens just the fm in $EDITOR
fmf --toc io/input_output.md                 # generate table of contents for chapter
fmf --toc                                    # generate toc for all chapters
```

Exit code is `0` when clean, `1` when issues are found, CI friendly.

---

## Checks

### Frontmatter (`--fm`)

| Code                       | Description                        |
| -------------------------- | ---------------------------------- |
| `fm::missing-frontmatter`  | Chapter has no `---` block         |
| `fm::unclosed-frontmatter` | Opening `---` has no closing `---` |
| `fm::missing-title`        | No `title:` field                  |
| `fm::missing-author`       | No `author:` field                 |
| `fm::missing-date`         | No `date:` field                   |
| `fm::missing-lang`         | No `lang:` field                   |
| `fm::missing-tags`         | No `tags:` field                   |

### HTML (`--html`)

| Code                     | Description                      |
| ------------------------ | -------------------------------- |
| `html::unclosed-details` | `<details>` without `</details>` |
| `html::unclosed-summary` | `<summary>` without `</summary>` |

### Links (`--links`)

| Code                   | Description                             |
| ---------------------- | --------------------------------------- |
| `html::broken-include` | `{{#include path}}` file doesn't exist  |
| `html::broken-link`    | Internal markdown link target not found |

---

## Auto-fix

`fmf --fix` writes missing frontmatter to disk. For each chapter without a `---`
block it injects:

```yaml
---
title: Chapter Name # from SUMMARY.md
author: Jr # from git log
date: 2026-09-03 # from git log
lang: en # from book.toml language field
tags:
  - io # inferred from directory structure
---
```

Chapters that already have frontmatter are left untouched. HTML issues are
reported but not auto-fixed, the correct insertion point is ambiguous.

---

## Dry run

Preview what `--fix` would write without touching any files:

```sh
fmf --dry-run
```

```text
would fix: src/io/input_output.md
---
title: input_output
author: Jr
date: 2026-09-03
lang: en
tags:
  - io
---
...
```

---

## Field overrides

```sh
fmf --set "title=Input and Output" io/input_output.md
fmf --set "author=Tom" --set "date=2026-01-01" io/input_output.md
```

You can pass the path with or withour the `src/` prefix, both work:

```sh
fmf --set "title=My Title" src/io/input_output.md
fmf --set "title=My Title" io/input_output.md
```

Add or replace tags on a single file:

```sh
fmf --tag rust --tag cli io/input_output.md
```

Combine with `--dry-run` to preview before writing:

```sh
fmf --set "title=My Title" io/input_output.md --dry-run
```

---

## Edit frontmatter

Open a chapter's frontmatter in `$EDITOR` for quick editing:

```sh
fmf --edit io/input_output.md
```

Only the frontmatter block is shown, the chapter content stays out of view. Uses
`$EDITOR` with a fallback to `vi`.

---

## Table of contents

Inject a table of contents into chapters based on their headings:

```sh
fmf --toc                            # inject TOC into all chapters
fmf --toc io/input_output.md        # inject TOC into one chapter
fmf --toc --dry-run                  # preview without writing
```

The TOC is inserted after the frontmatter block and before the chapter body.
Headings `##`, `###`, and `####` are included; `#` (the chapter title) is
skipped. Nesting is relative, if your chapter only uses `###`, entries appear
flat with no indent:

```markdown
## Table of Contents

- [The Basics](#the-basics)
- [Accessing Arguments](#accessing-arguments)
  - [Where This Leads](#where-this-leads)
- [Further Reading](#further-reading)
```

Running `--toc` on a chapter that already has a `## Table of Contents` block is
a no-op, it won't add a second one.

---

## Strip frontmatter

Remove frontmatter from all chapters or a single file:

```sh
fmf --strip                          # strip frontmatter from all chapters
fmf --strip io/input_output.md       # strip frontmatter from one file
fmf --strip --toc                    # strip TOC blocks from all chapters
fmf --strip --comment                # strip comment blocks from all chapters
fmf --strip --toc --comment          # strip both TOC and comment blocks
fmf --strip --dry-run                # preview without writing
```

`--strip` alone removes frontmatter. Combined with `--toc` or `--comment` it
targets those blocks instead and the frontmatter is left untouched.

Useful for cleaning up before publishing or removing frontmatter, TOC's, and
comment blocks you no longer need. Uses
[mdbook-frontmatter-strip](https://crates.io/crates/mdbook-frontmatter-strip)
under the hood.

---

## Comments

Inject a collapsible Q&A/comments block at the bottom of chapters:

```sh
fmf --comment                        # inject into all chapters
fmf --comment io/input_output.md     # inject into one chapter
fmf --comment --dry-run              # preview without writing
```

By default injects a plain `<details>` block:

```html
<details>
  <summary>Comments</summary>

  <!-- Add your questions or comments below -->
</details>
```

### Giscus Integration

For a live comment system backed by GitHub Discussions, configure Giscus in
`fmf.toml`:

```toml
comment_style = "giscus"
giscus_repo = "yourname/your-repo"
giscus_repo_id = "R_kgDO..."        # from giscus.app
giscus_category = "Q&A"
giscus_category_id = "DIC_kwDO..."  # from giscus.app
```

Get your `giscus_repo_id` and `giscus_category_id` from
[giscus.app](https://giscus.app) — enter your repo, enable GitHub Discussions,
and copy the generated values. The README is skipped automatically since it
serves as a landing page rather than a chapter.

To remove comment blocks:

```sh
fmf --strip --comment                # all chapters
fmf --strip --comment io/input_output.md  # one file
```

---

### Bluesky Integration

For comments backed by Bluesky replies, configure in `fmf.toml`:

```toml
comment_style = "bluesky"
bluesky_handle = "yourhandle.bsky.social"
```

Uses the [bluesky-comments](https://github.com/nicholasstephan/bluesky-comments)
web component to display replies to your Bluesky posts as chapter comments.

---

## Reading time

Inject an estimated reading time into chapter frontmatter:

```sh
fmf --readtime                       # inject into all chapters
fmf --readtime io/input_output.md   # inject into one chapter
fmf --readtime --dry-run             # preview without writing
```

Injects a visible badge after the frontmatter block:

```md
> ⏱ ~7 min read
```

The badge appears above the table of contents if one is present, and remains
visible after `mdbook-frontmatter-strip` removes the frontmatter block. Chapters
that already have a ⏱ badge are left untouched.

---

## Configuration

Create a `fmf.toml` in your book root to configure which fields `fmf` checks and
injects:

```toml
# fmf.toml

# Skip validation and injection of specific frontmatter fields
exclude_fields = ["lang", "tags"]

# Inject custom fields into every chapter's frontmatter via --fix
[inject]
status = "draft"
feed = "exclude"
license = "MIT"

# Comment style: "plain" (default), "giscus", or "bluesky"
comment_style = "giscus"
giscus_repo = "yourname/your-repo"
giscus_repo_id = "R_kgDO..."
giscus_category = "Q&A"
giscus_category_id = "DIC_kwDO..."

# Or for Bluesky:
# comment_style = "bluesky"
# bluesky_handle = "yourhandle.bsky.social"
```

Excluded fields are skipped during validation and not injected by `--fix`.
Useful when your book doesn't need RSS metadata (skip date, author) or
multilingual support (skip `lang`).

All fields are required by default when no `fmf.toml` is present.

### Custom Field Injection

Any fields under `[inject]` in `fmf.toml` are automatically added to every
chapter's frontmatter when running `--fix`:

```toml
[inject]
status = "draft"
version = "1.0"
license = "Apache-2.0"
```

Produces the following frontmatter:

```yaml
---
status: draft
license: Apache-2.0
---
```


Fields that already exist in a chapter's frontmatter are left untouched.

## How tags are inferred

Tags come from the directory segments between `src/` and the filename:

```
src/io/input_output.md          → tags: [io]
src/blog/rust/my-post.md        → tags: [blog, rust]
src/README.md                   → tags: []
```

## Works well with

- [mdbook-frontmatter-strip](https://crates.io/crates/mdbook-frontmatter-strip):
  strips frontmatter before the HTML renderer sees it
- [mdbook-rss-feed](https://crates.io/crates/mdbook-rss-feed): uses `date:` and
  `author:` from frontmatter to generate RSS/Atom/JSON feeds

## CI

`fmf` exits with code `1` when issues are found, making it easy to enforce clean
frontmatter in CI:

```yaml
- name: Validate book
  run: fmf
```

Or as a pre-commit hook:

```sh
#!/usr/bin/env sh
fmf || exit 1
```

## License

[Apache-2.0](https://github.com/saylesss88/mdbook-frontmatter-fix/blob/main/LICENSE)
