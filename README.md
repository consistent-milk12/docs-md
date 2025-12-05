# docs-md

A CLI tool that converts Rust's rustdoc JSON output into readable, per-module markdown files.

## Why This Exists

I wanted something that mirrors how rustdoc actually organizes things: one file per module, with working cross-references between them. So that I could just have docs I can grep through, and have all docs of all of my dependencies in one place locally. Opening up a browser is a hassle and I just end up browsing other sites instead. Especially as a neovim user it's quite annoying to switch between a browser and terminal. I also forget things very quickly so I am extremely dependent on docs to remember how stuff work.

## What It Does

- Generates one markdown file per module (not one giant blob)
- Creates working links between modules and items
- Adds breadcrumb navigation at the top of each file (e.g., `crate / module / submodule`)
- Supports both flat (`module.md`) and nested (`module/index.md`) output formats
- Handles multi-crate workspaces with cross-crate linking
- Shows cross-crate trait implementations (impls from dependencies appear on your types)
- Generates mdBook-compatible `SUMMARY.md` files
- Produces a `search_index.json` for client-side search
- Respects visibility (public-only by default, `--include-private` available)

**Example output:** The [`docs/`](docs/) directory in this repository contains generated documentation for this tool's own dependencies, demonstrating multi-crate output with cross-crate linking.

## Installation

You'll need Rust (nightly toolchain too because rustdoc-types is still unstable) installed. Then:

```bash
git clone https://github.com/consistent-milk12/docs-md
cd docs-md
cargo install --path .
```

Or just run it directly with `cargo run --`.

## Usage

### Step 1: Generate rustdoc JSON

This requires nightly Rust because JSON output is still unstable:

```bash
RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc
```

This creates JSON files in `target/doc/`. Each crate gets its own `{crate_name}.json` file.

### Step 2: Generate Markdown

**Single crate:**

```bash
# Flat format (all files in one directory)
docs_md --path target/doc/my_crate.json -o docs/

# Nested format (directory per module)
docs_md --path target/doc/my_crate.json -o docs/ --format nested

# Include private items
docs_md --path target/doc/my_crate.json -o docs/ --include-private
```

**Multiple crates (workspace or with dependencies):**

Multi-crate mode always uses a nested structure (one directory per crate), regardless of `--format`.

```bash
# Basic multi-crate generation
docs_md --dir target/doc/ -o docs/

# With mdBook support (generates SUMMARY.md)
docs_md --dir target/doc/ -o docs/ --mdbook

# With search index
docs_md --dir target/doc/ -o docs/ --mdbook --search-index

# Prioritize your crate for ambiguous links
docs_md --dir target/doc/ -o docs/ --mdbook --primary-crate my_crate
```

### Output Structure

**Flat format:**

```bash
docs/
├── index.md           # Crate root
├── module_a.md
├── module_a__submodule.md   # Double underscore = nesting
└── module_b.md
```

**Nested format:**

```bash
docs/
├── index.md
├── module_a/
│   ├── index.md
│   └── submodule/
│       └── index.md
└── module_b/
    └── index.md
```

**Multi-crate:**

```bash
docs/
├── my_crate/
│   └── index.md
├── dependency_a/
│   └── index.md
├── SUMMARY.md          # mdBook table of contents
└── search_index.json   # For client-side search
```

## How It Works

The tool reads rustdoc's JSON format (defined by the `rustdoc-types` crate) and walks through the module tree. For each module, it:

1. **Collects items** - Structs, enums, traits, functions, constants, macros, type aliases
2. **Renders documentation** - Converts the doc comments (already markdown) and adds item signatures
3. **Processes links** - Rustdoc JSON includes a `links` map that tells us what `[SomeType]` should point to. We resolve these to relative file paths.
4. **Handles impl blocks** - Gathers trait implementations and inherent methods for each type

For multi-crate mode, there's a `UnifiedLinkRegistry` that tracks items across all crates and resolves cross-crate references. When there's ambiguity (multiple crates have an item with the same name), it prefers: local crate → primary crate (if specified) → alphabetically first.

### Key Components

- **`Parser`** - Reads and deserializes rustdoc JSON
- **`LinkRegistry`** - Maps item IDs to file paths for single-crate linking
- **`UnifiedLinkRegistry`** - Same thing but across multiple crates
- **`RenderContext`** - Trait that abstracts over single-crate and multi-crate contexts, enabling shared rendering code
- **`ModuleRenderer`** - Generates markdown for a single module (works with any `RenderContext`)
- **`DocLinkProcessor`** - Converts rustdoc's intra-doc links to markdown links
- **`TypeRenderer`** - Formats type signatures (generics, bounds, etc.)

## Current Limitations

Being honest about what doesn't work yet:

- **External re-exports** - If a crate re-exports something from a dependency that isn't in your JSON files, we can't link to it. You'll see the re-export but not the full documentation.
- **Duplicate headings** - Some crates start their docs with `# Crate Name`, which duplicates our generated heading. Working on stripping these.
- **No incremental builds** - Regenerates everything every time. Fine for most crates, slow for huge workspaces.

## What's In Development

Next up:

1. **Duplicate title stripping** - Detect and remove redundant `# Title` lines from crate docs
2. **Crate graph visualization** - Show dependency relationships between crates

Further out:

- Cargo subcommand (`cargo docs-md`)
- Incremental generation

## Dependencies

The heavy lifting is done by:

- `rustdoc-types` - The official rustdoc JSON schema
- `clap` - CLI argument parsing
- `serde` / `serde_json` - JSON handling
- `regex` - Processing doc links
- `miette` - Nice error messages
- `indicatif` - Progress bars

## Contributing

Issues and PRs welcome. The codebase is reasonably documented—each module has a `//!` header explaining what it does, and public functions have doc comments.

## License

MIT or Apache-2.0, pick whichever works for you.

---

_This was mostly developed for personal use. If it's useful to you too, that's great. If you find bugs or have ideas, let me know._
