*[docs_md](../index.md) / [multi_crate](index.md)*

---

# Module `multi_crate`

Multi-crate documentation generation.

This module provides support for generating documentation from multiple
rustdoc JSON files, enabling cross-crate linking and self-contained
documentation for entire dependency trees.

# Architecture

The multi-crate system uses these components:

- [`CrateCollection`](collection/index.md): Container for parsed crates with processing order
- [`MultiCrateParser`](parser/index.md): Scans directories and parses JSON files
- [`UnifiedLinkRegistry`](registry/index.md): Cross-crate link resolution
- [`MultiCrateContext`](context/index.md): Shared state during generation
- [`MultiCrateGenerator`](generator/index.md): Orchestrates per-crate generation
- [`SummaryGenerator`](summary/index.md): Creates mdBook-compatible SUMMARY.md

# Usage

```ignore
use docs_md::multi_crate::{MultiCrateParser, MultiCrateGenerator};

let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let generator = MultiCrateGenerator::new(&crates, &args);
generator.generate()?;
```

## Structs

### `CrateCollection`

```rust
struct CrateCollection {
    crates: std::collections::HashMap<String, rustdoc_types::Crate>,
}
```

Collection of parsed crates ready for documentation generation.

Uses `HashMap` for O(1) lookups and sorts keys on-demand when iteration
is needed. This is optimal for our use case where:
- All crates are inserted first (parsing phase)
- Sorted iteration happens later (generation phase)
- Collection size is small (typically 10-50 crates)

# Example

```ignore
let mut collection = CrateCollection::new();
collection.insert("tracing".to_string(), tracing_crate);
collection.insert("tracing_core".to_string(), tracing_core_crate);

for (name, krate) in collection.iter() {
    println!("Processing {name}");
}
```

#### Fields

- **`crates`**: `std::collections::HashMap<String, rustdoc_types::Crate>`

  Map from crate name to parsed Crate data.
  HashMap provides O(1) lookups; sorting done on-demand.

#### Implementations

- `fn new() -> Self`

- `fn insert(self: &mut Self, name: String, krate: Crate) -> Option<Crate>`

- `fn get(self: &Self, name: &str) -> Option<&Crate>`

- `fn get_with_name(self: &Self, name: &str) -> Option<(&str, &Crate)>`

- `fn contains(self: &Self, name: &str) -> bool`

- `fn iter(self: &Self) -> impl Iterator<Item = (&String, &Crate)>`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn names(self: &Self) -> Vec<&String>`

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> CrateCollection` — [`CrateCollection`](../../multi_crate/collection/index.md)

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MultiCrateContext<'a>`

```rust
struct MultiCrateContext<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    registry: crate::multi_crate::UnifiedLinkRegistry,
    args: &'a crate::Args,
    cross_crate_impls: std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>,
}
```

Shared context for multi-crate documentation generation.

Holds references to all crates, the unified link registry, and
CLI configuration. Used by [`MultiCrateGenerator`](generator/index.md) to coordinate
generation across crates.


#### Fields

- **`crates`**: `&'a crate::multi_crate::CrateCollection`

  All crates being documented.

- **`registry`**: `crate::multi_crate::UnifiedLinkRegistry`

  Unified link registry for cross-crate resolution.

- **`args`**: `&'a crate::Args`

  CLI arguments.

- **`cross_crate_impls`**: `std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>`

  Pre-computed cross-crate impl blocks.
  
  Maps target crate name -> type name -> impl blocks from other crates.
  This is computed once during construction rather than per-view.

#### Implementations

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](../../multi_crate/collection/index.md), [`Args`](../../index.md)

- `fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](../../multi_crate/collection/index.md)

- `const fn crates(self: &Self) -> &CrateCollection` — [`CrateCollection`](../../multi_crate/collection/index.md)

- `const fn registry(self: &Self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../../multi_crate/registry/index.md)

- `const fn args(self: &Self) -> &Args` — [`Args`](../../index.md)

- `fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](../../multi_crate/context/index.md)

- `fn find_item(self: &Self, id: &Id) -> Option<(&str, &Item)>`

- `fn get_cross_crate_impls(self: &Self, target_crate: &str) -> Option<&HashMap<String, Vec<&'a Impl>>>`

- `fn get_impl_target_path(impl_block: &Impl) -> Option<String>`

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SingleCrateView<'a>`

```rust
struct SingleCrateView<'a> {
    crate_name: &'a str,
    krate: &'a rustdoc_types::Crate,
    registry: &'a crate::multi_crate::UnifiedLinkRegistry,
    args: &'a crate::Args,
    ctx: &'a MultiCrateContext<'a>,
    impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    cross_crate_impls: Option<&'a std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>,
    type_name_to_id: std::collections::HashMap<String, rustdoc_types::Id>,
}
```

View of a single crate within multi-crate context.

Provides an interface similar to [`GeneratorContext`](../generator/context/index.md) but uses
[`UnifiedLinkRegistry`](registry/index.md) for cross-crate link resolution. This
allows existing rendering code to work with minimal changes.


#### Fields

- **`crate_name`**: `&'a str`

  Name of this crate (borrowed from the context).

- **`krate`**: `&'a rustdoc_types::Crate`

  The crate being rendered.

- **`registry`**: `&'a crate::multi_crate::UnifiedLinkRegistry`

  Unified registry for link resolution.

- **`args`**: `&'a crate::Args`

  CLI arguments.

- **`ctx`**: `&'a MultiCrateContext<'a>`

  Reference to the parent multi-crate context for cross-crate lookups.

- **`impl_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>`

  Map from type ID to impl blocks (local crate only).

- **`cross_crate_impls`**: `Option<&'a std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>`

  Reference to pre-computed cross-crate impl blocks from context.
  Maps type name to impl blocks from other crates.

- **`type_name_to_id`**: `std::collections::HashMap<String, rustdoc_types::Id>`

  Map from type name to type ID for cross-crate impl lookup.

#### Implementations

- `fn new(crate_name: &'a str, krate: &'a Crate, registry: &'a UnifiedLinkRegistry, args: &'a Args, ctx: &'a MultiCrateContext<'a>) -> Self` — [`UnifiedLinkRegistry`](../../multi_crate/registry/index.md), [`Args`](../../index.md), [`MultiCrateContext`](../../multi_crate/context/index.md)

- `fn build_impl_map(self: &mut Self)`

- `fn build_type_name_map(self: &mut Self)`

- `const fn get_impl_target_id(impl_block: &Impl) -> Option<Id>`

- `fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- `const fn crate_name(self: &Self) -> &str`

- `const fn krate(self: &Self) -> &Crate`

- `const fn registry(self: &Self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../../multi_crate/registry/index.md)

- `const fn args(self: &Self) -> &Args` — [`Args`](../../index.md)

- `fn get_impls(self: &Self, id: Id) -> Option<&Vec<&'a Impl>>`

- `fn get_all_impls(self: &Self, id: Id) -> Vec<&'a Impl>`

- `const fn should_include_item(self: &Self, item: &rustdoc_types::Item) -> bool`

- `fn count_modules(self: &Self) -> usize`

- `fn create_link(self: &Self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`

- `fn resolve_name(self: &Self, name: &str) -> Option<(String, Id)>`

- `fn lookup_item_across_crates(self: &Self, id: &Id) -> Option<(&str, &Item)>`

- `fn resolve_external_path(self: &Self, path: &str) -> Option<(&str, &Item, Id)>`

- `fn process_backtick_links(self: &Self, docs: &str, item_links: &HashMap<String, Id>, current_file: &str) -> String`

- `fn process_plain_links(docs: &str) -> String`

- `fn resolve_link(self: &Self, link_text: &str, item_links: &HashMap<String, Id>, current_file: &str) -> Option<String>`

- `fn build_link_to_id(self: &Self, id: Id, current_file: &str, display_name: &str, anchor: Option<&str>) -> Option<String>`

- `fn resolve_crate_path(self: &Self, path_without_crate: &str, display_name: &str, current_file: &str) -> Option<String>`

- `fn split_type_and_anchor(path: &str) -> (&str, Option<&str>)`

- `fn build_markdown_link(self: &Self, current_file: &str, target_crate: &str, target_path: &str, display_name: &str, anchor: Option<&str>) -> String`

- `fn compute_cross_crate_path(current_local: &str, target_crate: &str, target_path: &str) -> String`

- `fn strip_crate_prefix(path: &str) -> &str`

- `fn looks_like_external_reference(link_text: &str) -> bool`

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl RenderContext`

- `fn krate(self: &Self) -> &Crate`

- `fn crate_name(self: &Self) -> &str`

- `fn get_item(self: &Self, id: &Id) -> Option<&Item>`

- `fn get_impls(self: &Self, id: &Id) -> Option<&[&Impl]>`

- `fn should_include_item(self: &Self, item: &Item) -> bool`

- `fn include_private(self: &Self) -> bool`

- `fn include_blanket_impls(self: &Self) -> bool`

- `fn crate_version(self: &Self) -> Option<&str>`

- `fn link_registry(self: &Self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../../linker/index.md)

- `fn process_docs(self: &Self, item: &Item, current_file: &str) -> Option<String>`

- `fn create_link(self: &Self, id: Id, current_file: &str) -> Option<String>`

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    ctx: crate::multi_crate::MultiCrateContext<'a>,
    args: &'a crate::Args,
}
```

Generator for multi-crate documentation.

Produces a directory structure with one subdirectory per crate,
each containing nested markdown files with cross-crate linking.

# Output Structure

```text
output/
├── tracing/
│   ├── index.md
│   └── span/
│       └── index.md
├── tracing_core/
│   ├── index.md
│   └── subscriber/
│       └── index.md
└── SUMMARY.md        # If --mdbook enabled
```

#### Fields

- **`ctx`**: `crate::multi_crate::MultiCrateContext<'a>`

  Multi-crate context with unified registry.

- **`args`**: `&'a crate::Args`

  CLI arguments.

#### Implementations

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](../../multi_crate/collection/index.md), [`Args`](../../index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn generate_crate(self: &Self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../../multi_crate/context/index.md), [`Error`](../../error/index.md)

- `fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](../../multi_crate/context/index.md), [`Error`](../../error/index.md)

- `fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../../error/index.md)

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MultiCrateParser`

```rust
struct MultiCrateParser;
```

Parser for multiple rustdoc JSON files in a directory.

Discovers JSON files and parses each one, extracting the crate name
from the root module item.

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
println!("Found {} crates", crates.len());
```

#### Implementations

- `fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](../../multi_crate/collection/index.md), [`Error`](../../error/index.md)

- `fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](../../error/index.md)

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `UnifiedLinkRegistry`

```rust
struct UnifiedLinkRegistry {
    item_paths: hashbrown::HashMap<(String, rustdoc_types::Id), String>,
    item_names: hashbrown::HashMap<(String, rustdoc_types::Id), String>,
    name_index: std::collections::HashMap<String, Vec<(String, rustdoc_types::Id)>>,
    primary_crate: Option<String>,
}
```

Registry mapping item IDs to documentation paths across multiple crates.

Unlike [`LinkRegistry`](../linker/index.md) which handles a single crate, this registry
spans multiple crates and supports cross-crate link resolution with
disambiguation based on local/primary crate preference.

# Path Format

All paths use the nested format: `{crate_name}/{module_path}/index.md`

Examples:
- `tracing/index.md` (crate root)
- `tracing/span/index.md` (module)
- `tracing_core/subscriber/index.md` (cross-crate reference)

# Link Resolution Priority

When resolving ambiguous names:
1. Items in the current crate (where the link appears)
2. Items in the primary crate (if specified via `--primary-crate`)
3. Items with the shortest qualified path

# Performance

Uses `hashbrown` with raw entry API for zero-allocation lookups.
This avoids allocating a `String` for the crate name on every lookup.

#### Fields

- **`item_paths`**: `hashbrown::HashMap<(String, rustdoc_types::Id), String>`

  Maps `(crate_name, item_id)` to the file path within output.
  Uses hashbrown for raw_entry API (zero-alloc lookups).

- **`item_names`**: `hashbrown::HashMap<(String, rustdoc_types::Id), String>`

  Maps `(crate_name, item_id)` to the item's display name.
  Uses hashbrown for raw_entry API (zero-alloc lookups).

- **`name_index`**: `std::collections::HashMap<String, Vec<(String, rustdoc_types::Id)>>`

  Maps short names to all `(crate_name, item_id)` pairs.
  Used for disambiguating links like `Span` that exist in multiple crates.

- **`primary_crate`**: `Option<String>`

  The primary crate name for preferential resolution.

#### Implementations

- `fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](../../multi_crate/collection/index.md)

- `fn register_crate(self: &mut Self, crate_name: &str, krate: &Crate)`

- `fn register_from_paths(self: &mut Self, crate_name: &str, krate: &Crate)`

- `fn register_item_recursive(self: &mut Self, krate: &Crate, crate_name: &str, item_id: Id, item: &rustdoc_types::Item, parent_path: &str)`

- `fn register_item(self: &mut Self, crate_name: &str, id: Id, name: &str, path: &str)`

- `fn get_path(self: &Self, crate_name: &str, id: Id) -> Option<&String>`

- `fn get_name(self: &Self, crate_name: &str, id: Id) -> Option<&String>`

- `fn resolve_name(self: &Self, name: &str, current_crate: &str) -> Option<(String, Id)>`

- `fn resolve_path(self: &Self, path: &str) -> Option<(String, Id)>`

- `fn create_link(self: &Self, from_crate: &str, from_path: &str, to_crate: &str, to_id: Id) -> Option<String>`

- `fn compute_cross_crate_path(from: &str, to: &str) -> String`

- `fn get_anchor(self: &Self, crate_name: &str, id: Id) -> Option<String>`

- `fn contains(self: &Self, crate_name: &str, id: Id) -> bool`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../../multi_crate/registry/index.md)

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SearchIndex`

```rust
struct SearchIndex {
    pub items: Vec<SearchEntry>,
}
```

The complete search index containing all searchable items.

Serialized to `search_index.json` for client-side consumption.

#### Fields

- **`items`**: `Vec<SearchEntry>`

  All searchable items across all crates.

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

### `SearchIndexGenerator<'a>`

```rust
struct SearchIndexGenerator<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    include_private: bool,
}
```

Generator for multi-crate search indices.

Traverses all crates in a [`CrateCollection`](collection/index.md) and builds a comprehensive
search index of all public items (or all items if `include_private` is set).

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let generator = SearchIndexGenerator::new(&crates, false);
generator.write(Path::new("docs/"))?;
```

#### Fields

- **`crates`**: `&'a crate::multi_crate::CrateCollection`

  Collection of crates to index.

- **`include_private`**: `bool`

  Whether to include private items in the search index.
  
  When false (default), only public items are indexed.
  When true, all items regardless of visibility are indexed.

#### Implementations

- `const fn new(crates: &'a CrateCollection, include_private: bool) -> Self` — [`CrateCollection`](../../multi_crate/collection/index.md)

- `fn generate(self: &Self) -> SearchIndex` — [`SearchIndex`](../../multi_crate/search/index.md)

- `fn write(self: &Self, output_dir: &Path) -> std::io::Result<()>`

- `fn index_crate(self: &Self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](../../multi_crate/search/index.md)

- `fn build_path_map(krate: &Crate) -> HashMap<Id, String>`

- `fn compute_file_path(crate_name: &str, module_path: &str, kind: &str) -> String`

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `SummaryGenerator<'a>`

```rust
struct SummaryGenerator<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    output_dir: &'a std::path::Path,
}
```

Generates mdBook-compatible SUMMARY.md file.

Creates a table of contents linking all crates and their modules,
allowing the documentation to be built as an mdBook site.

# Output Format

```markdown
Summary

- [tracing](tracing/index.md)
  - [span](tracing/span/index.md)
  - [field](tracing/field/index.md)
- [tracing_core](tracing_core/index.md)
  - [subscriber](tracing_core/subscriber/index.md)
```

#### Fields

- **`crates`**: `&'a crate::multi_crate::CrateCollection`

  Collection of crates to document.

- **`output_dir`**: `&'a std::path::Path`

  Output directory for SUMMARY.md.

#### Implementations

- `const fn new(crates: &'a CrateCollection, output_dir: &'a Path) -> Self` — [`CrateCollection`](../../multi_crate/collection/index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn add_modules(content: &mut String, krate: &rustdoc_types::Crate, items: &[rustdoc_types::Id], path_prefix: &str, indent: usize)`

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

