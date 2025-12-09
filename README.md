A CLI tool that converts Rust's rustdoc JSON output into readable, per-module markdown files.

I wanted something that mirrors how rustdoc actually organizes things: one file per module, with working cross-references between them. So that I could just have docs I can grep through, and have all docs of all of my dependencies in one place locally. Opening up a browser is a hassle and I just end up browsing other sites instead. Especially as a neovim user it's quite annoying to switch between a browser and terminal. I also forget things very quickly so I am extremely dependent on docs to remember how stuff work.

By default, all items (including private ones) are documented. Use `--exclude-private` to only include public items. This ensures maximum detail from rustdoc JSON files, though it may cause broken links if private items are later excluded.

## What It Does

- Generates one markdown file per module (not one giant blob)
- Creates working links between modules and items
- Adds breadcrumb navigation at the top of each file (e.g., `crate / module / submodule`)
- Supports both flat (`module.md`) and nested (`module/index.md`) output formats
- Handles multi-crate workspaces with cross-crate linking
- Properly renders `pub use` re-exports with their documentation
- Shows cross-crate trait implementations (impls from dependencies appear on your types)
- Filters common blanket impls (From, Into, Any, etc.) by default (`--include-blanket-impls` to show)
- Generates mdBook-compatible `SUMMARY.md` files
- Produces a `search_index.json` for client-side search (only includes rendered items)
- Includes all items by default—use `--exclude-private` to limit to public items only (affects links, search index, and SUMMARY.md)
- Source location links: Shows where each item is defined with clickable links to source files (requires `source-parsing` feature)

**Example output:** The [`generated_docs/`](generated_docs/) directory in this repository contains generated documentation for this tool's own dependencies, demonstrating multi-crate output with cross-crate linking.

## Installation

You'll need Rust (nightly toolchain too because rustdoc-types is still unstable) installed. Then:

```bash
# Install from crates.io
cargo install cargo-docs-md

# Or direct install from git
cargo install --git https://github.com/consistent-milk12/docs-md

# Or clone and install locally
git clone https://github.com/consistent-milk12/docs-md
cd docs-md
cargo install --path .
```

Or just run it directly with `cargo run --`.

## Usage

### Quick Start (One Command)

The easiest way to generate docs—builds rustdoc JSON and generates markdown in one step:

```bash
# Generate docs for your project and all dependencies
cargo docs-md docs

# With options
cargo docs-md docs --primary-crate my_crate    # Prioritize your crate for links
cargo docs-md docs --clean                      # Full rebuild (cargo clean first)
cargo docs-md docs --exclude-private            # Only include public items
cargo docs-md docs -- --all-features            # Pass args to cargo doc
```

This requires the nightly toolchain (`rustup toolchain install nightly`).

**Defaults:** Nested format is used by default. The `docs` subcommand also generates `SUMMARY.md` + `search_index.json` by default. Use `--format flat`, `--no-mdbook`, or `--no-search-index` to change this.

### Manual Two-Step Process

If you need more control, you can run the steps separately:

**Step 1: Generate rustdoc JSON**

```bash
RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc
```

This creates JSON files in `target/doc/`. Each crate gets its own `{crate_name}.json` file.

**Step 2: Generate Markdown**

_Single crate:_

```bash
# Nested format (directory per module) - default
cargo docs-md --path target/doc/my_crate.json -o generated_docs/

# Flat format (all files in one directory)
cargo docs-md --path target/doc/my_crate.json -o generated_docs/ --format flat

# Exclude private items (public only)
cargo docs-md --path target/doc/my_crate.json -o generated_docs/ --exclude-private
```

_Multiple crates (workspace or with dependencies):_

Multi-crate mode always uses a nested structure (one directory per crate), regardless of `--format`.

```bash
# Basic multi-crate generation
cargo docs-md --dir target/doc/ -o generated_docs/

# With mdBook support (generates SUMMARY.md)
cargo docs-md --dir target/doc/ -o generated_docs/ --mdbook

# With search index
cargo docs-md --dir target/doc/ -o generated_docs/ --mdbook --search-index

# Prioritize your crate for ambiguous links
cargo docs-md --dir target/doc/ -o generated_docs/ --mdbook --primary-crate my_crate
```

### Development Scripts

For development, use `just` (install with `cargo install just`):

```bash
# Documentation
just docs         # Full docs (clean + build + rustdoc + generate)
just regen        # Regenerate generated_docs/ (quick, no cargo clean)
just quick        # Quick rebuild docs/ for mdbook

# Testing
just test         # Run all tests
just test-lib     # Run unit tests only
just test-int     # Run integration tests only
just test-filter PATTERN  # Run tests matching pattern

# Development
just build        # Build debug binary (with source-parsing feature)
just release      # Build release binary
just check        # Quick cargo check
just lint         # Run clippy (pedantic + nursery)
just errors       # Build and show only errors/warnings
just bench        # Run benchmarks
just clean        # Remove docs/ and cargo clean

just help         # Show all available commands
```

The scripts include helpful error messages if the nightly toolchain is missing.

### Python Utility Scripts

The `scripts/` directory contains Python utilities for debugging and inspecting rustdoc JSON and generated output:

```bash
# List all available scripts with descriptions
python3 scripts/index.py

# Get help for a specific script
python3 scripts/index.py <script_name>

# Run a script directly
python3 scripts/<script_name>.py --help
```

**Available scripts:**

| Script                    | Description                                                           |
| ------------------------- | --------------------------------------------------------------------- |
| `inspect_impl.py`         | Inspect impl blocks for a specific type in rustdoc JSON               |
| `find_blanket_impls.py`   | Find blanket impls (`impl<T> Trait for T`) in rustdoc JSON            |
| `dump_type_info.py`       | Dump detailed type information from rustdoc JSON                      |
| `check_generated_docs.py` | Check generated markdown for issues (duplicate anchors, broken links) |

**Examples:**

```bash
# Inspect all impls for a type
python3 scripts/inspect_impl.py target/doc/my_crate.json MyStruct

# Filter by trait name
python3 scripts/inspect_impl.py target/doc/my_crate.json MyStruct --trait Clone -v

# Check generated docs for problems
python3 scripts/check_generated_docs.py generated_docs/
python3 scripts/check_generated_docs.py generated_docs/ --file my_crate/module/index.md

# Find blanket impls in a crate
python3 scripts/find_blanket_impls.py target/doc/my_crate.json --limit 50
```

### Source Location Links

When built with the `source-parsing` feature, each item shows a "Defined in" link pointing to the source file and line numbers:

```markdown
_Defined in [`serde-1.0.228/src/lib.rs:10-25`](../../.source_xxx/serde-1.0.228/src/lib.rs#L10-L25)_
```

**Setup:**

1. First, collect dependency sources to a local directory:

   ```bash
   cargo docs-md collect-sources
   ```

   This creates a `.source_{timestamp}/` directory with copies of all dependency source files.

2. Generate docs (the tool auto-detects the `.source_*` directory):
   ```bash
   cargo docs-md docs
   ```

The links are relative to the generated markdown files and use GitHub-compatible line number fragments (`#L10-L25`). This keeps your home directory path private while providing clickable source navigation.

## How It Works

The tool reads rustdoc's JSON format (defined by the `rustdoc-types` crate) and walks through the module tree. For each module, it:

1. **Collects items** - Structs, enums, traits, functions, constants, macros, type aliases, and re-exports (`pub use`)
2. **Renders documentation** - Converts the doc comments (already markdown) and adds item signatures
3. **Processes links** - Rustdoc JSON includes a `links` map that tells us what `[SomeType]` should point to. We resolve these to relative file paths.
4. **Handles impl blocks** - Gathers trait implementations and inherent methods for each type

For multi-crate mode, there's a `UnifiedLinkRegistry` that tracks items across all crates and resolves cross-crate references. When there's ambiguity (multiple crates have an item with the same name), it prefers: local crate → primary crate (if specified) → modules over other items → first match.

## What's In Development

Lots of boilerplate AI generated code was added in the last two commits. This will probably take a lot of time to clean up than manually writing on my own, but I don't really have enough time on hand to work on so many of these types on my own.

The heavy lifting is done by:

- `rustdoc-types` - The official rustdoc JSON schema
- `clap` - CLI argument parsing
- `serde` / `serde_json` - JSON handling
- `regex` - Processing doc links
- `miette` - Nice error messages
- `indicatif` - Progress bars
- `rayon` - Parallel multi-crate generation
- `hashbrown` - High-performance hash maps with raw_entry API for zero-allocation lookups
- `unicode-normalization` - NFC normalization for anchor slugification
- `compact_str` - Inline strings (≤24 bytes without heap allocation)
- `tracing` - Structured logging (compiled out in release builds via `release_max_level_info`)

## Contributing

Issues and PRs welcome. The codebase should be FULLY DOCUMENTED (should not build if not, due to the `#![deny(missing_docs)]` lint in root lib.rs). Each module has a `//!` header explaining what it does, and public functions have doc comments.

## License

MIT

This was mostly developed for personal use. If it's useful to you too, that's great. If you find bugs or have ideas, let me know.
