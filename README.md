# cargo-docs-md

Turn rustdoc JSON into markdown you can actually navigate.

Point it at your crate and get a directory of interlinked markdown files — click through modules, jump across crate boundaries, and go straight to source code. Your code and all your dependencies, documented and searchable in one place.

The [`generated_docs/`](generated_docs/) folder in this repo has 88 crates, 1,200+ markdown files, and 24,000+ searchable items generated from this tool's own dependency tree. Checkout the added docs and source code directly in repo to see how well it currently works (or not!).

## Current Features

**Clickable everything.** Each file has breadcrumb navigation at the top. References to types, traits, and functions link to their definitions — even across crates. For example, `regex_automata` docs link directly to `regex_syntax::hir::Hir`.

```text
*[cargo_docs_md](../index.md) / [generator](index.md) / [context](context/index.md)*
```

**Source locations.** Every item shows where it's defined. Click through to the exact file and line.

```markdown
*Defined in `src/generator/context.rs:135-168`*
```

**Search index.** A `search_index.json` with every documented item — name, path, kind, and file location.

## Installation

Requires Rust nightly (rustdoc JSON is unstable):

```bash
rustup toolchain install nightly
cargo install cargo-docs-md
```

## Usage

```bash
cargo docs-md docs
```

That's it. Builds rustdoc JSON and generates markdown in one step.

```bash
cargo docs-md docs --primary-crate my_crate   # prioritize your crate for link resolution
cargo docs-md docs --exclude-private          # public items only
cargo docs-md docs --clean                    # full rebuild (cargo clean first)
```

### Source file collection

To make source links work for dependencies, collect their source files first:

```bash
cargo docs-md collect-sources   # copies sources to .source_{timestamp}/
cargo docs-md docs              # auto-detects and links to them
```

This snapshots dependency sources at their exact versions, so links stay valid even after updates.

### More options

| Flag | Effect |
|------|--------|
| `--format flat` | All markdown files in one directory |
| `--no-mdbook` | Skip `SUMMARY.md` generation |
| `--no-search-index` | Skip `search_index.json` |
| `--include-blanket-impls` | Include `From`, `Into`, `Any`, etc. |

For manual control, generate rustdoc JSON yourself:

```bash
RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc
cargo docs-md --dir target/doc/ -o docs/
```

## How it works

Reads rustdoc JSON and walks the module tree. For each module, collects all items, renders their docs, and resolves cross-reference links using rustdoc's link map.

Multi-crate workspaces get a unified link registry that tracks items across all crates. When the same name exists in multiple places, it resolves by checking: local crate first, then primary crate (if set), then any module match, then first available.

## Known limitations

- **No standard library links.** References to `Vec`, `String`, `Drop`, etc. can't link to std docs since they're not in the generated output. They may incorrectly resolve to a same-named item in your dependencies.

- **Nightly only.** Rustdoc JSON is unstable and requires the nightly toolchain. The format can change between Rust versions.

- **Ambiguous names.** When multiple crates export the same name, the resolver picks one based on priority rules. This is usually correct, but occasionally produces unexpected links.

- **Private item links.** By default, private items are documented. If you later regenerate with `--exclude-private`, links pointing to those items will break.

## Development

```bash
just help         # all commands
just test         # run tests
just lint         # clippy (pedantic + nursery)
just docs         # full rebuild
```

## License

MIT
