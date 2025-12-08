# Crate `docs_md`

docs-md library interface for testing and reuse.

This module exposes the core functionality of docs-md as a library,
allowing integration tests and external tools to use the markdown
generation capabilities programmatically.

## Modules

- [`error`](error/index.md) - Error types for docs-md.
- [`generator`](generator/index.md) - Markdown documentation generator for rustdoc JSON.
- [`linker`](linker/index.md) - Cross-reference linking for markdown documentation.
- [`multi_crate`](multi_crate/index.md) - Multi-crate documentation generation.
- [`parser`](parser/index.md) - Rustdoc JSON parsing module.
- [`types`](types/index.md) - Type rendering utilities for converting rustdoc types to string representations.

## Structs

### `Generator<'a>`

```rust
struct Generator<'a> {
    ctx: GeneratorContext<'a>,
    args: &'a crate::Args,
    root_item: &'a rustdoc_types::Item,
}
```

Main documentation generator.

This struct orchestrates the entire documentation generation process,
coordinating between the context, format-specific generators, and
progress reporting.

# Example

```ignore
let generator = Generator::new(&krate, &args)?;
generator.generate()?;
```

#### Fields

- **`ctx`**: `GeneratorContext<'a>`

  Shared context containing crate data, maps, and configuration.

- **`args`**: `&'a crate::Args`

  CLI arguments containing output path and format options.

- **`root_item`**: `&'a rustdoc_types::Item`

  The root module item of the crate.

#### Implementations

- `fn new(krate: &'a Crate, args: &'a Args) -> Result<Self, Error>` — [`Args`](#args), [`Error`](error/index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](error/index.md)

- `fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](error/index.md)

- `fn generate_to_capture(krate: &Crate, format: CliOutputFormat, include_private: bool) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](#clioutputformat), [`MarkdownCapture`](#markdowncapture), [`Error`](error/index.md)

- `fn generate_flat_to_capture(ctx: &GeneratorContext<'_>, root: &Item, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/index.md), [`MarkdownCapture`](#markdowncapture), [`Error`](error/index.md)

- `fn generate_flat_recursive_capture(ctx: &GeneratorContext<'_>, item: &Item, prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/index.md), [`MarkdownCapture`](#markdowncapture), [`Error`](error/index.md)

- `fn generate_nested_to_capture(ctx: &GeneratorContext<'_>, root: &Item, path_prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/index.md), [`MarkdownCapture`](#markdowncapture), [`Error`](error/index.md)

- `fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error>` — [`Args`](#args), [`Error`](error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for Generator<'a>`

##### `impl<T> IntoEither for Generator<'a>`

##### `impl<D> OwoColorize for Generator<'a>`

##### `impl<T> Pointable for Generator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for Generator<'a>`

### `MarkdownCapture`

```rust
struct MarkdownCapture {
    files: std::collections::HashMap<String, String>,
}
```

Captures generated markdown in memory for testing.

Instead of writing files to disk, this struct stores all generated
markdown content in a `HashMap`, keyed by relative file path. This
enables snapshot testing and verification of output without filesystem
side effects.

#### Fields

- **`files`**: `std::collections::HashMap<String, String>`

  Maps file paths (relative to output directory) to their generated content.

#### Implementations

- `fn new() -> Self`

- `fn insert(self: &mut Self, path: String, content: String)`

- `fn get(self: &Self, path: &str) -> Option<&String>`

- `fn paths(self: &Self) -> Vec<&String>`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn to_snapshot_string(self: &Self) -> String`

- `fn into_inner(self: Self) -> HashMap<String, String>`

#### Trait Implementations

##### `impl Debug for MarkdownCapture`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MarkdownCapture`

- `fn default() -> MarkdownCapture` — [`MarkdownCapture`](#markdowncapture)

##### `impl<T> Instrument for MarkdownCapture`

##### `impl<T> IntoEither for MarkdownCapture`

##### `impl<D> OwoColorize for MarkdownCapture`

##### `impl<T> Pointable for MarkdownCapture`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MarkdownCapture`

### `LinkRegistry`

```rust
struct LinkRegistry {
    item_paths: std::collections::HashMap<rustdoc_types::Id, String>,
    item_names: std::collections::HashMap<rustdoc_types::Id, String>,
}
```

Registry mapping item IDs to their documentation file paths.

This is the central data structure for cross-reference resolution.
It's built once during generation and queried whenever we need to
create links between items.

#### Fields

- **`item_paths`**: `std::collections::HashMap<rustdoc_types::Id, String>`

  Maps each item's ID to the markdown file path where it's documented.
  
  Paths are relative to the output directory root.
  Examples: `"index.md"`, `"span.md"`, `"span/index.md"`

- **`item_names`**: `std::collections::HashMap<rustdoc_types::Id, String>`

  Maps each item's ID to its display name.
  
  Used to generate the link text (e.g., `[`name`](path)`).
  This is typically the item's identifier without the full path.

#### Implementations

- `fn build(krate: &Crate, flat_format: bool, include_private: bool) -> Self`

- `fn register_module_items(self: &mut Self, krate: &Crate, module_id: Id, module_item: &rustdoc_types::Item, path: &str, module_prefix: &str, flat_format: bool, include_private: bool)`

- `fn register_glob_items(self: &mut Self, krate: &Crate, use_item: &rustdoc_types::Use, path: &str, include_private: bool)`

- `fn get_path(self: &Self, id: Id) -> Option<&String>`

- `fn get_name(self: &Self, id: Id) -> Option<&String>`

- `fn create_link(self: &Self, id: Id, from_path: &str) -> Option<String>`

- `fn compute_relative_path(from: &str, to: &str) -> String`

#### Trait Implementations

##### `impl Debug for LinkRegistry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for LinkRegistry`

- `fn default() -> LinkRegistry` — [`LinkRegistry`](#linkregistry)

##### `impl<T> Instrument for LinkRegistry`

##### `impl<T> IntoEither for LinkRegistry`

##### `impl<D> OwoColorize for LinkRegistry`

##### `impl<T> Pointable for LinkRegistry`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for LinkRegistry`

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
  `HashMap` provides O(1) lookups; sorting done on-demand.

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

##### `impl Debug for CrateCollection`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for CrateCollection`

- `fn default() -> CrateCollection` — [`CrateCollection`](#cratecollection)

##### `impl<T> Instrument for CrateCollection`

##### `impl<T> IntoEither for CrateCollection`

##### `impl<D> OwoColorize for CrateCollection`

##### `impl<T> Pointable for CrateCollection`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for CrateCollection`

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
CLI configuration. Used by [`MultiCrateGenerator`](#multicrategenerator) to coordinate
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

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](#cratecollection), [`Args`](#args)

- `fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](#cratecollection)

- `const fn crates(self: &Self) -> &CrateCollection` — [`CrateCollection`](#cratecollection)

- `const fn registry(self: &Self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](#unifiedlinkregistry)

- `const fn args(self: &Self) -> &Args` — [`Args`](#args)

- `fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](multi_crate/index.md)

- `fn find_item(self: &Self, id: &Id) -> Option<(&str, &Item)>`

- `fn get_cross_crate_impls(self: &Self, target_crate: &str) -> Option<&HashMap<String, Vec<&'a Impl>>>`

- `fn get_impl_target_path(impl_block: &Impl) -> Option<String>`

#### Trait Implementations

##### `impl<T> Instrument for MultiCrateContext<'a>`

##### `impl<T> IntoEither for MultiCrateContext<'a>`

##### `impl<D> OwoColorize for MultiCrateContext<'a>`

##### `impl<T> Pointable for MultiCrateContext<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MultiCrateContext<'a>`

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

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](#cratecollection), [`Args`](#args)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](error/index.md)

- `fn collect_rendered_items(self: &Self) -> HashMap<String, HashSet<Id>>`

- `fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](multi_crate/index.md)

- `fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](multi_crate/index.md)

- `fn generate_crate(self: &Self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](multi_crate/index.md), [`Error`](error/index.md)

- `fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](multi_crate/index.md), [`Error`](error/index.md)

- `fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for MultiCrateGenerator<'a>`

##### `impl<T> IntoEither for MultiCrateGenerator<'a>`

##### `impl<D> OwoColorize for MultiCrateGenerator<'a>`

##### `impl<T> Pointable for MultiCrateGenerator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MultiCrateGenerator<'a>`

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

- `fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](#cratecollection), [`Error`](error/index.md)

- `fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for MultiCrateParser`

##### `impl<T> IntoEither for MultiCrateParser`

##### `impl<D> OwoColorize for MultiCrateParser`

##### `impl<T> Pointable for MultiCrateParser`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MultiCrateParser`

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

##### `impl Debug for SearchIndex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for SearchIndex`

##### `impl<T> IntoEither for SearchIndex`

##### `impl<D> OwoColorize for SearchIndex`

##### `impl<T> Pointable for SearchIndex`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Serialize for SearchIndex`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl<T> WithSubscriber for SearchIndex`

### `SearchIndexGenerator<'a>`

```rust
struct SearchIndexGenerator<'a> {
    crates: &'a super::CrateCollection,
    include_private: bool,
    rendered_items: std::collections::HashMap<String, std::collections::HashSet<rustdoc_types::Id>>,
}
```

Generator for multi-crate search indices.

Traverses all crates in a [`CrateCollection`](#cratecollection) and builds a comprehensive
search index of all public items (or all items if `include_private` is set).

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let rendered_items = generator.generate();  // Returns HashMap<String, HashSet<Id>>
let generator = SearchIndexGenerator::new(&crates, false, rendered_items);
generator.write(Path::new("docs/"))?;
```

#### Fields

- **`crates`**: `&'a super::CrateCollection`

  Collection of crates to index.

- **`include_private`**: `bool`

  Whether to include private items in the search index.
  
  When false (default), only public items are indexed.
  When true, all items regardless of visibility are indexed.

- **`rendered_items`**: `std::collections::HashMap<String, std::collections::HashSet<rustdoc_types::Id>>`

  Set of item IDs that were actually rendered per crate.
  
  Only items in this set will appear in the search index.
  This ensures the search index matches the generated documentation.

#### Implementations

- `const fn new(crates: &'a CrateCollection, include_private: bool, rendered_items: HashMap<String, HashSet<Id>>) -> Self` — [`CrateCollection`](#cratecollection)

- `fn generate(self: &Self) -> SearchIndex` — [`SearchIndex`](#searchindex)

- `fn write(self: &Self, output_dir: &Path) -> std::io::Result<()>`

- `fn index_crate(self: &Self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](multi_crate/search/index.md)

- `fn build_path_map(krate: &Crate) -> HashMap<Id, String>`

- `fn compute_file_path(crate_name: &str, module_path: &str, kind: &str) -> String`

#### Trait Implementations

##### `impl<T> Instrument for SearchIndexGenerator<'a>`

##### `impl<T> IntoEither for SearchIndexGenerator<'a>`

##### `impl<D> OwoColorize for SearchIndexGenerator<'a>`

##### `impl<T> Pointable for SearchIndexGenerator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for SearchIndexGenerator<'a>`

### `UnifiedLinkRegistry`

```rust
struct UnifiedLinkRegistry {
    item_paths: hashbrown::HashMap<(compact_str::CompactString, rustdoc_types::Id), compact_str::CompactString>,
    item_names: hashbrown::HashMap<(compact_str::CompactString, rustdoc_types::Id), compact_str::CompactString>,
    name_index: std::collections::HashMap<compact_str::CompactString, Vec<(compact_str::CompactString, rustdoc_types::Id, rustdoc_types::ItemKind)>>,
    re_export_sources: hashbrown::HashMap<(compact_str::CompactString, rustdoc_types::Id), compact_str::CompactString>,
    primary_crate: Option<compact_str::CompactString>,
}
```

Registry mapping item IDs to documentation paths across multiple crates.

Unlike [`LinkRegistry`](#linkregistry) which handles a single crate, this registry
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

- **`item_paths`**: `hashbrown::HashMap<(compact_str::CompactString, rustdoc_types::Id), compact_str::CompactString>`

  Maps `(crate_name, item_id)` to the file path within output.
  Uses hashbrown for `raw_entry` API (zero-alloc lookups).

- **`item_names`**: `hashbrown::HashMap<(compact_str::CompactString, rustdoc_types::Id), compact_str::CompactString>`

  Maps `(crate_name, item_id)` to the item's display name.
  Uses hashbrown for `raw_entry` API (zero-alloc lookups).

- **`name_index`**: `std::collections::HashMap<compact_str::CompactString, Vec<(compact_str::CompactString, rustdoc_types::Id, rustdoc_types::ItemKind)>>`

  Maps short names to all `(crate_name, item_id, item_kind)` tuples.
  Used for disambiguating links like `Span` that exist in multiple crates.
  The `ItemKind` enables preferring modules over macros with the same name.

- **`re_export_sources`**: `hashbrown::HashMap<(compact_str::CompactString, rustdoc_types::Id), compact_str::CompactString>`

  Maps `(crate_name, reexport_id)` to the original source path.
  Used for resolving external re-exports where `use_item.id` is `None`
  but `use_item.source` provides the canonical path.
  Example: `("tracing", id_123)` -> `"tracing_core::field::Visit"`

- **`primary_crate`**: `Option<compact_str::CompactString>`

  The primary crate name for preferential resolution.

#### Implementations

- `fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](#cratecollection)

- `fn register_crate(self: &mut Self, crate_name: &str, krate: &Crate)`

- `fn register_from_paths(self: &mut Self, crate_name: &str, krate: &Crate)`

- `fn item_enum_to_kind(inner: &ItemEnum) -> ItemKind`

- `fn register_item_recursive(self: &mut Self, krate: &Crate, crate_name: &str, item_id: Id, item: &rustdoc_types::Item, parent_path: &str)`

- `fn register_item(self: &mut Self, crate_name: &str, id: Id, name: &str, path: &str, kind: ItemKind)`

- `fn get_path(self: &Self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- `fn get_name(self: &Self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- `fn get_re_export_source(self: &Self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- `fn resolve_reexport(self: &Self, crate_name: &str, id: Id) -> Option<(compact_str::CompactString, Id)>`

- `fn resolve_name(self: &Self, name: &str, current_crate: &str) -> Option<(compact_str::CompactString, Id)>`

- `fn resolve_path(self: &Self, path: &str) -> Option<(compact_str::CompactString, Id)>`

- `fn create_link(self: &Self, from_crate: &str, from_path: &str, to_crate: &str, to_id: Id) -> Option<String>`

- `fn compute_cross_crate_path(from: &str, to: &str) -> String`

- `fn get_anchor(self: &Self, crate_name: &str, id: Id) -> Option<String>`

- `fn contains(self: &Self, crate_name: &str, id: Id) -> bool`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for UnifiedLinkRegistry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for UnifiedLinkRegistry`

- `fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](#unifiedlinkregistry)

##### `impl<T> Instrument for UnifiedLinkRegistry`

##### `impl<T> IntoEither for UnifiedLinkRegistry`

##### `impl<D> OwoColorize for UnifiedLinkRegistry`

##### `impl<T> Pointable for UnifiedLinkRegistry`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for UnifiedLinkRegistry`

### `Cli`

```rust
struct Cli {
    pub command: Option<Command>,
    pub args: GenerateArgs,
}
```

Top-level CLI for docs-md.

#### Fields

- **`command`**: `Option<Command>`

  Subcommand to run

- **`args`**: `GenerateArgs`

  Generation options (used when no subcommand is specified)

#### Trait Implementations

##### `impl Args for Cli`

- `fn group_id() -> Option<clap::Id>`

- `fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- `fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl CommandFactory for Cli`

- `fn command<'b>() -> clap::Command`

- `fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for Cli`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FromArgMatches for Cli`

- `fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- `fn update_from_arg_matches_mut(self: &mut Self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl<T> Instrument for Cli`

##### `impl<T> IntoEither for Cli`

##### `impl<D> OwoColorize for Cli`

##### `impl Parser for Cli`

##### `impl<T> Pointable for Cli`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for Cli`

### `DocsArgs`

```rust
struct DocsArgs {
    pub output: std::path::PathBuf,
    pub format: CliOutputFormat,
    pub primary_crate: Option<String>,
    pub exclude_private: bool,
    pub include_blanket_impls: bool,
    pub no_mdbook: bool,
    pub no_search_index: bool,
    pub clean: bool,
    pub cargo_args: Vec<String>,
}
```

Arguments for the `docs` subcommand (build + generate).

#### Fields

- **`output`**: `std::path::PathBuf`

  Output directory for generated markdown files.
  
  Defaults to `docs/` in the current directory.

- **`format`**: `CliOutputFormat`

  Output format (flat or nested).

- **`primary_crate`**: `Option<String>`

  Primary crate name for preferential link resolution.
  
  If not specified, attempts to detect from Cargo.toml.

- **`exclude_private`**: `bool`

  Exclude private (non-public) items from the output.
  
  By default, all items are documented including private ones.
  Enable this to only include public items.

- **`include_blanket_impls`**: `bool`

  Include blanket trait implementations in the output.

- **`no_mdbook`**: `bool`

  Skip generating mdBook SUMMARY.md file.

- **`no_search_index`**: `bool`

  Skip generating `search_index.json` file.

- **`clean`**: `bool`

  Run cargo clean before building (full rebuild).

- **`cargo_args`**: `Vec<String>`

  Additional arguments to pass to cargo doc.
  
  Example: `docs-md docs -- --all-features`

#### Trait Implementations

##### `impl Args for DocsArgs`

- `fn group_id() -> Option<clap::Id>`

- `fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- `fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl CommandFactory for DocsArgs`

- `fn command<'b>() -> clap::Command`

- `fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for DocsArgs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FromArgMatches for DocsArgs`

- `fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- `fn update_from_arg_matches_mut(self: &mut Self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl<T> Instrument for DocsArgs`

##### `impl<T> IntoEither for DocsArgs`

##### `impl<D> OwoColorize for DocsArgs`

##### `impl Parser for DocsArgs`

##### `impl<T> Pointable for DocsArgs`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for DocsArgs`

### `GenerateArgs`

```rust
struct GenerateArgs {
    pub path: Option<std::path::PathBuf>,
    pub dir: Option<std::path::PathBuf>,
    pub mdbook: bool,
    pub search_index: bool,
    pub primary_crate: Option<String>,
    pub output: std::path::PathBuf,
    pub format: CliOutputFormat,
    pub exclude_private: bool,
    pub include_blanket_impls: bool,
}
```

Command-line arguments for direct generation (no subcommand).

The tool accepts input from two mutually exclusive sources:
1. A local rustdoc JSON file (`--path`)
2. A directory of rustdoc JSON files (`--dir`)

#### Fields

- **`path`**: `Option<std::path::PathBuf>`

  Path to a local rustdoc JSON file.
  
  Generate this file with: `cargo doc --output-format json`
  The JSON file will be in `target/doc/{crate_name}.json`
  
  Mutually exclusive with `--dir`.

- **`dir`**: `Option<std::path::PathBuf>`

  Directory containing multiple rustdoc JSON files.
  
  Use this for multi-crate documentation generation. The tool will
  scan the directory for all `*.json` files (rustdoc format) and
  generate documentation for each crate with cross-crate linking.
  
  Generate JSON files with:
  `RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc`
  
  Mutually exclusive with `--path`.

- **`mdbook`**: `bool`

  Generate mdBook-compatible SUMMARY.md file.
  
  Only valid with `--dir` for multi-crate documentation.
  Creates a `SUMMARY.md` file in the output directory that can be
  used as the entry point for an mdBook documentation site.

- **`search_index`**: `bool`

  Generate `search_index.json` for client-side search.
  
  Only valid with `--dir` for multi-crate documentation.
  Creates a `search_index.json` file containing all documented items,
  which can be used with client-side search libraries like Fuse.js,
  Lunr.js, or `FlexSearch`.

- **`primary_crate`**: `Option<String>`

  Primary crate name for preferential link resolution.
  
  When specified with `--dir`, links to items in this crate take
  precedence over items with the same name in dependencies.
  This helps resolve ambiguous links like `exit` to the intended
  crate rather than `std::process::exit`.

- **`output`**: `std::path::PathBuf`

  Output directory for generated markdown files.
  
  The directory will be created if it doesn't exist.
  Defaults to `docs/` in the current directory.

- **`format`**: `CliOutputFormat`

  Output format (flat or nested).
  
  - `flat`: All files in one directory (default)
  - `nested`: Directory hierarchy mirroring modules

- **`exclude_private`**: `bool`

  Exclude private (non-public) items from the output.
  
  By default, all items are documented including `pub(crate)`,
  `pub(super)`, and private items. Enable this to only include
  public items.

- **`include_blanket_impls`**: `bool`

  Include blanket trait implementations in the output.
  
  By default, blanket impls like `From`, `Into`, `TryFrom`, `TryInto`,
  `Any`, `Borrow`, `BorrowMut`, and `ToOwned` are filtered out to reduce
  noise. Enable this to include them in the documentation.

#### Trait Implementations

##### `impl Args for GenerateArgs`

- `fn group_id() -> Option<clap::Id>`

- `fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- `fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl CommandFactory for GenerateArgs`

- `fn command<'b>() -> clap::Command`

- `fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for GenerateArgs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for GenerateArgs`

- `fn default() -> GenerateArgs` — [`GenerateArgs`](#generateargs)

##### `impl FromArgMatches for GenerateArgs`

- `fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- `fn update_from_arg_matches_mut(self: &mut Self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl<T> Instrument for GenerateArgs`

##### `impl<T> IntoEither for GenerateArgs`

##### `impl<D> OwoColorize for GenerateArgs`

##### `impl Parser for GenerateArgs`

##### `impl<T> Pointable for GenerateArgs`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for GenerateArgs`

## Enums

### `OutputFormat`

```rust
enum OutputFormat {
    Flat,
    Nested,
}
```

Output format for the generated markdown documentation.

Controls how module files are organized in the output directory.

#### Variants

- **`Flat`**

  Flat structure: all files in one directory.
  
  Module hierarchy is encoded in filenames using double underscores.
  Example: `parent__child__grandchild.md`

- **`Nested`**

  Nested structure: directories mirror module hierarchy.
  
  Each module gets its own directory with an `index.md` file.
  Example: `parent/child/grandchild/index.md`

#### Trait Implementations

##### `impl Clone for OutputFormat`

- `fn clone(self: &Self) -> OutputFormat` — [`OutputFormat`](#outputformat)

##### `impl Copy for OutputFormat`

##### `impl Debug for OutputFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for OutputFormat`

- `fn default() -> OutputFormat` — [`OutputFormat`](#outputformat)

##### `impl<T> Instrument for OutputFormat`

##### `impl<T> IntoEither for OutputFormat`

##### `impl<D> OwoColorize for OutputFormat`

##### `impl<T> Pointable for OutputFormat`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl ValueEnum for OutputFormat`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value<'a>(self: &Self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl<T> WithSubscriber for OutputFormat`

### `Command`

```rust
enum Command {
    Docs(DocsArgs),
}
```

Available subcommands

#### Variants

- **`Docs`**

  Build rustdoc JSON and generate markdown in one step.
  
  This runs `cargo +nightly doc` with JSON output, then generates
  markdown documentation from the result. Requires nightly toolchain.
  
  Example: `docs-md docs --primary-crate my_crate`

#### Trait Implementations

##### `impl Debug for Command`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl FromArgMatches for Command`

- `fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- `fn update_from_arg_matches_mut<'b>(self: &mut Self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl<T> Instrument for Command`

##### `impl<T> IntoEither for Command`

##### `impl<D> OwoColorize for Command`

##### `impl<T> Pointable for Command`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Subcommand for Command`

- `fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command`

- `fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

- `fn has_subcommand(__clap_name: &str) -> bool`

##### `impl<T> WithSubscriber for Command`

### `CliOutputFormat`

```rust
enum CliOutputFormat {
    Flat,
    Nested,
}
```

CLI-compatible output format enum (for clap `ValueEnum` derive).

#### Variants

- **`Flat`**

  Flat structure with double-underscore separators in filenames.

- **`Nested`**

  Nested directory structure mirroring the module hierarchy.

#### Trait Implementations

##### `impl Clone for CliOutputFormat`

- `fn clone(self: &Self) -> CliOutputFormat` — [`CliOutputFormat`](#clioutputformat)

##### `impl Copy for CliOutputFormat`

##### `impl Debug for CliOutputFormat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for CliOutputFormat`

- `fn default() -> CliOutputFormat` — [`CliOutputFormat`](#clioutputformat)

##### `impl<T> Instrument for CliOutputFormat`

##### `impl<T> IntoEither for CliOutputFormat`

##### `impl<D> OwoColorize for CliOutputFormat`

##### `impl<T> Pointable for CliOutputFormat`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl ValueEnum for CliOutputFormat`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value<'a>(self: &Self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl<T> WithSubscriber for CliOutputFormat`

## Functions

## Type Aliases

### `Args`

```rust
type Args = GenerateArgs;
```

Backwards-compatible type alias for existing code.

