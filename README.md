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
- Properly renders `pub use` re-exports with their documentation
- Shows cross-crate trait implementations (impls from dependencies appear on your types)
- Filters common blanket impls (From, Into, Any, etc.) by default (`--include-blanket-impls` to show)
- Generates mdBook-compatible `SUMMARY.md` files
- Produces a `search_index.json` for client-side search (only includes rendered items)
- Respects visibility throughout—links, search index, and SUMMARY.md all honor `--include-private`

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

### Quick Start (One Command)

The easiest way to generate docs—builds rustdoc JSON and generates markdown in one step:

```bash
# Generate docs for your project and all dependencies
docs_md docs

# With options
docs_md docs --primary-crate my_crate    # Prioritize your crate for links
docs_md docs --clean                      # Full rebuild (cargo clean first)
docs_md docs --include-private            # Include private items
docs_md docs -- --all-features            # Pass args to cargo doc
```

This requires the nightly toolchain (`rustup toolchain install nightly`).

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
# Flat format (all files in one directory)
docs_md --path target/doc/my_crate.json -o docs/

# Nested format (directory per module)
docs_md --path target/doc/my_crate.json -o docs/ --format nested

# Include private items
docs_md --path target/doc/my_crate.json -o docs/ --include-private
```

_Multiple crates (workspace or with dependencies):_

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

### Development Scripts

For development, you can use either `just` or `make`:

```bash
# Using just
just          # Full rebuild: clean + build + rustdoc + generate
just quick    # Quick rebuild (skip cargo clean)
just help     # Show all available commands

# Using make
make          # Full rebuild: clean + build + rustdoc + generate
make quick    # Quick rebuild (skip cargo clean)
make help     # Show all available commands
```

Both scripts include helpful error messages if the nightly toolchain is missing.

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

1. **Collects items** - Structs, enums, traits, functions, constants, macros, type aliases, and re-exports (`pub use`)
2. **Renders documentation** - Converts the doc comments (already markdown) and adds item signatures
3. **Processes links** - Rustdoc JSON includes a `links` map that tells us what `[SomeType]` should point to. We resolve these to relative file paths.
4. **Handles impl blocks** - Gathers trait implementations and inherent methods for each type

For multi-crate mode, there's a `UnifiedLinkRegistry` that tracks items across all crates and resolves cross-crate references. When there's ambiguity (multiple crates have an item with the same name), it prefers: local crate → primary crate (if specified) → alphabetically first.

### Architecture

Single-crate and multi-crate modes share the same rendering code through a trait-based abstraction:

- **`RenderContext`** - A trait composed of `ItemAccess`, `ItemFilter`, and `LinkResolver` sub-traits
- **`GeneratorContext`** - Implements `RenderContext` for single-crate mode
- **`SingleCrateView`** - Adapter that implements `RenderContext` for multi-crate mode, allowing existing renderers to work transparently with multi-crate data

This design eliminates code duplication and ensures consistent output regardless of mode.

### Key Components

- **`Parser`** - Reads and deserializes rustdoc JSON
- **`LinkRegistry`** - Maps item IDs to file paths for single-crate linking
- **`UnifiedLinkRegistry`** - Cross-crate linking with zero-allocation lookups (hashbrown raw_entry API)
- **`ModuleRenderer`** - Generates markdown for a single module (works with any `RenderContext`)
- **`DocLinkProcessor`** - Converts rustdoc's intra-doc links to markdown links
- **`TypeRenderer`** - Formats type signatures with `Cow<str>` optimization (borrowed for simple types, owned for complex)

### Performance

Actually, I need help with this. I have tried the tricks that I know of, please let me know if you have better ideas. Especially related to improving parsing and generating the markdown content.

- **Parallel generation** - Multi-crate mode uses rayon for 2-4x speedup on multi-core systems
- **Zero-allocation lookups** - Registry queries use hashbrown's raw_entry API (~4x faster than standard HashMap)
- **ASCII fast path** - Anchor slugification skips unicode normalization for pure ASCII names (~18x faster)
- **Inline strings** - `CompactString` stores short identifiers without heap allocation

## Current Limitations

To be honest, it's currently good enough for my use cases. There are some
formatting issues here and there. I haven't tested it out on any EXTREMELY LARGE
repo yet - should probably be fine though.

- **External re-exports** - If a crate re-exports something from a dependency that isn't in your JSON files, we can't link to it. You'll see the re-export but not the full documentation. Workaround: include all dependency JSON files in `--dir`.
- **Duplicate headings** - Some crates start their docs with `# Crate Name`, which duplicates our generated heading. Basic mitigation exists for exact matches, but edge cases remain.
- **No incremental builds** - Regenerates everything every time. Fine for most crates, slow for huge workspaces. Use `just quick` to skip cargo clean.
- **Reference link conversion** - Markdown reference links like `[text][ref]` may get incorrectly processed in rare cases.
- **Cross-crate impl lookup** - When rendering re-exported types, impl blocks from the source crate might not be found in edge cases.

## What's In Development

- **Crate graph visualization** - Show dependency relationships between crates
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
- `rayon` - Parallel multi-crate generation
- `hashbrown` - High-performance hash maps with raw_entry API for zero-allocation lookups
- `unicode-normalization` - NFC normalization for anchor slugification
- `compact_str` - Inline strings (≤24 bytes without heap allocation)

## Contributing

Issues and PRs welcome. The codebase should FULLY DOCUMENTED (should not build if not, due to the `#![deny(missing_docs)]` lint in root lib.rs) each module has a `//!` header explaining what it does, and public functions have doc comments.

## License

MIT or Apache-2.0, pick whichever works for you.

_This was mostly developed for personal use. If it's useful to you too, that's great. If you find bugs or have ideas, let me know._
