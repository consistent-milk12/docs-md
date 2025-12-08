This document proposes a hybrid approach to Rust documentation generation that combines the existing rustdoc JSON parser with direct source code parsing from `~/.cargo`. The goal is to generate richer documentation that includes implementation details, function bodies, private items, and code examples that rustdoc JSON doesn't expose. Things we can get:

- Access to private items and internal implementation details
- Function body extraction for implementation examples
- Inline code snippets from actual implementations
- Better support for understanding library internals
- Complementary to rustdoc JSON (not a replacement)

The idea is to get full exposure to source code of the dependencies directly in the workspace.

**Current Ideas:** Use `syn` for syntactic parsing, optionally augmented with `ra_ap_*` crates for semantic analysis when cross-crate type resolution is needed.

---

## Source Collection Feature

Before parsing, collect dependency sources into a local `.source_{timestamp}/` directory.

### Why?

- Decoupled workflow: collect once, parse many times
- Portable: project becomes self-contained
- No ~/.cargo path resolution at parse time
- Explicit: user sees exactly what's documented

### Flow

```
1. Parse Cargo.toml (and workspace members via cargo_metadata)
2. Collect external crate info: name, version, features
3. Generate .source_{unix_timestamp}
   - Check exists → retry up to 3x (increment timestamp) → error if all collide
4. Copy src/ + Crate.toml (renamed from Cargo.toml to avoid rust-analyzer)
5. Write manifest.json with collected metadata
6. Append ".source_*" to .gitignore if not present
7. Return created directory path
```

### Output Structure

```
.source_1733660400/
├── manifest.json
├── serde-1.0.228/
│   ├── Crate.toml
│   └── src/
│       ├── lib.rs
│       └── de/
├── tokio-1.48.0/
│   ├── Crate.toml
│   └── src/
└── ...
```

### manifest.json

Stores all metadata from cargo_metadata—no need to parse individual Crate.toml files:

```json
{
  "collected_at": "2025-12-08T12:00:00Z",
  "workspace_root": "/path/to/project",
  "crates": {
    "serde-1.0.228": {
      "name": "serde",
      "version": "1.0.228",
      "edition": "2021",
      "features": ["derive", "std"],
      "description": "A serialization framework",
      "source_path": "serde-1.0.228/"
    }
  }
}
```

### CLI

```bash
cargo docs-md collect-sources
    --output .source/           # override default random suffix
    --include-dev               # include dev-dependencies
    --dry-run                   # list what would be copied
```

### rust-analyzer Exclusion

Renaming `Cargo.toml` → `Crate.toml` prevents rust-analyzer from indexing copied sources. Zero overhead—just a different target filename during copy.

---

## Implementation Modules

```
src/source/
├── mod.rs          # exports
├── types.rs        # CrateSource, FunctionInfo, etc.
├── collector.rs    # collect sources to .source_*/
├── locator.rs      # find sources (now just reads .source_*/)
└── parser.rs       # syn-based parsing
```
