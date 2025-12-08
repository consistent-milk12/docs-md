*[cargo_docs_md](../index.md) / [multi_crate](index.md)*

---

# Module `multi_crate`

Multi-crate documentation generation.

This module provides support for generating documentation from multiple
rustdoc JSON files, enabling cross-crate linking and self-contained
documentation for entire dependency trees.

# Architecture

The multi-crate system uses these components:

- [`CrateCollection`](../index.md): Container for parsed crates with processing order
- [`MultiCrateParser`](../index.md): Scans directories and parses JSON files
- [`UnifiedLinkRegistry`](../index.md): Cross-crate link resolution
- [`MultiCrateContext`](../index.md): Shared state during generation
- [`MultiCrateGenerator`](../index.md): Orchestrates per-crate generation
- [`SummaryGenerator`](#summarygenerator): Creates mdBook-compatible SUMMARY.md

# Usage

```ignore
use docs_md::multi_crate::{MultiCrateParser, MultiCrateGenerator};

let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let generator = MultiCrateGenerator::new(&crates, &args);
generator.generate()?;
```

## Modules

- [`collection`](collection/index.md) - Crate collection for multi-crate documentation.
- [`context`](context/index.md) - Multi-crate generation context.
- [`generator`](generator/index.md) - Multi-crate documentation generator.
- [`parser`](parser/index.md) - Multi-crate JSON parser.
- [`registry`](registry/index.md) - Unified link registry for cross-crate documentation.
- [`search`](search/index.md) - Search index generation for multi-crate documentation.
- [`summary`](summary/index.md) - mdBook SUMMARY.md generator.

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

- `fn default() -> CrateCollection` — [`CrateCollection`](../index.md)

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
CLI configuration. Used by [`MultiCrateGenerator`](../index.md) to coordinate
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

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](../index.md), [`Args`](../index.md)

- `fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](../index.md)

- `const fn crates(self: &Self) -> &CrateCollection` — [`CrateCollection`](../index.md)

- `const fn registry(self: &Self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../index.md)

- `const fn args(self: &Self) -> &Args` — [`Args`](../index.md)

- `fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](#singlecrateview)

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

Provides an interface similar to [`GeneratorContext`](../generator/index.md) but uses
[`UnifiedLinkRegistry`](../index.md) for cross-crate link resolution. This
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

- `fn new(crate_name: &'a str, krate: &'a Crate, registry: &'a UnifiedLinkRegistry, args: &'a Args, ctx: &'a MultiCrateContext<'a>) -> Self` — [`UnifiedLinkRegistry`](../index.md), [`Args`](../index.md), [`MultiCrateContext`](../index.md)

- `fn build_impl_map(self: &mut Self)`

- `fn build_type_name_map(self: &mut Self)`

- `const fn get_impl_target_id(impl_block: &Impl) -> Option<Id>`

- `fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- `const fn crate_name(self: &Self) -> &str`

- `const fn krate(self: &Self) -> &Crate`

- `const fn registry(self: &Self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../index.md)

- `const fn args(self: &Self) -> &Args` — [`Args`](../index.md)

- `fn get_impls(self: &Self, id: Id) -> Option<&Vec<&'a Impl>>`

- `fn get_all_impls(self: &Self, id: Id) -> Vec<&'a Impl>`

- `fn get_impls_from_crate(self: &Self, id: Id, source_krate: &'a Crate) -> Vec<&'a Impl>`

- `const fn get_impl_target_id_from_type(ty: &rustdoc_types::Type) -> Option<Id>`

- `const fn should_include_item(self: &Self, item: &rustdoc_types::Item) -> bool`

- `fn count_modules(self: &Self) -> usize`

- `fn create_link(self: &Self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`

- `fn resolve_name(self: &Self, name: &str) -> Option<(String, Id)>`

- `fn lookup_item_across_crates(self: &Self, id: &Id) -> Option<(&str, &Item)>`

- `fn get_crate(self: &Self, name: &str) -> Option<&Crate>`

- `fn resolve_external_path(self: &Self, path: &str) -> Option<(&str, &Item, Id)>`

- `fn process_backtick_links(self: &Self, docs: &str, item_links: &HashMap<String, Id>, current_file: &str) -> String`

- `fn process_plain_links(self: &Self, docs: &str, current_file: &str) -> String`

- `fn resolve_plain_link(self: &Self, link_text: &str, current_file: &str) -> Option<String>`

- `fn resolve_link(self: &Self, link_text: &str, item_links: &HashMap<String, Id>, current_file: &str) -> Option<String>`

- `fn build_link_to_id(self: &Self, id: Id, current_file: &str, display_name: &str, anchor: Option<&str>) -> Option<String>`

- `fn resolve_crate_path(self: &Self, path_without_crate: &str, display_name: &str, current_file: &str) -> Option<String>`

- `fn split_type_and_anchor(path: &str) -> (&str, Option<&str>)`

- `fn build_markdown_link(self: &Self, current_file: &str, target_crate: &str, target_path: &str, display_name: &str, anchor: Option<&str>) -> String`

- `fn compute_cross_crate_path(current_local: &str, target_crate: &str, target_path: &str) -> String`

- `fn strip_crate_prefix(path: &str) -> &str`

- `fn looks_like_external_reference(link_text: &str) -> bool`

#### Trait Implementations

##### `impl<T> Instrument for SingleCrateView<'a>`

##### `impl<T> IntoEither for SingleCrateView<'a>`

##### `impl ItemAccess for SingleCrateView<'_>`

- `fn krate(self: &Self) -> &Crate`

- `fn crate_name(self: &Self) -> &str`

- `fn get_item(self: &Self, id: &Id) -> Option<&Item>`

- `fn get_impls(self: &Self, id: &Id) -> Option<&[&Impl]>`

- `fn crate_version(self: &Self) -> Option<&str>`

##### `impl ItemFilter for SingleCrateView<'_>`

- `fn should_include_item(self: &Self, item: &Item) -> bool`

- `fn include_private(self: &Self) -> bool`

- `fn include_blanket_impls(self: &Self) -> bool`

##### `impl LinkResolver for SingleCrateView<'_>`

- `fn link_registry(self: &Self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../index.md)

- `fn process_docs(self: &Self, item: &Item, current_file: &str) -> Option<String>`

- `fn create_link(self: &Self, id: Id, current_file: &str) -> Option<String>`

##### `impl<D> OwoColorize for SingleCrateView<'a>`

##### `impl<T> Pointable for SingleCrateView<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> RenderContext for SingleCrateView<'a>`

##### `impl<T> WithSubscriber for SingleCrateView<'a>`

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

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self` — [`CrateCollection`](../index.md), [`Args`](../index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](../error/index.md)

- `fn collect_rendered_items(self: &Self) -> HashMap<String, HashSet<Id>>`

- `fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](#singlecrateview)

- `fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](#singlecrateview)

- `fn generate_crate(self: &Self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](#singlecrateview), [`Error`](../error/index.md)

- `fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](#singlecrateview), [`Error`](../error/index.md)

- `fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../error/index.md)

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

- `fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](../index.md), [`Error`](../error/index.md)

- `fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](../error/index.md)

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

Unlike [`LinkRegistry`](../index.md) which handles a single crate, this registry
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

- `fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](../index.md)

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

- `fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../index.md)

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

Traverses all crates in a [`CrateCollection`](../index.md) and builds a comprehensive
search index of all public items (or all items if `include_private` is set).

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let rendered_items = generator.generate();  // Returns HashMap<String, HashSet<Id>>
let generator = SearchIndexGenerator::new(&crates, false, rendered_items);
generator.write(Path::new("generated_docs/"))?;
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

- `const fn new(crates: &'a CrateCollection, include_private: bool, rendered_items: HashMap<String, HashSet<Id>>) -> Self` — [`CrateCollection`](../index.md)

- `fn generate(self: &Self) -> SearchIndex` — [`SearchIndex`](../index.md)

- `fn write(self: &Self, output_dir: &Path) -> std::io::Result<()>`

- `fn index_crate(self: &Self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](search/index.md)

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

### `SummaryGenerator<'a>`

```rust
struct SummaryGenerator<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    output_dir: &'a std::path::Path,
    include_private: bool,
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

- **`include_private`**: `bool`

  Whether to include private items.

#### Implementations

- `const fn new(crates: &'a CrateCollection, output_dir: &'a Path, include_private: bool) -> Self` — [`CrateCollection`](../index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](../error/index.md)

- `fn add_modules(self: &Self, content: &mut String, krate: &rustdoc_types::Crate, items: &[rustdoc_types::Id], path_prefix: &str, indent: usize)`

#### Trait Implementations

##### `impl<T> Instrument for SummaryGenerator<'a>`

##### `impl<T> IntoEither for SummaryGenerator<'a>`

##### `impl<D> OwoColorize for SummaryGenerator<'a>`

##### `impl<T> Pointable for SummaryGenerator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for SummaryGenerator<'a>`

## Constants

### `RUST_PATH_SEP`

```rust
const RUST_PATH_SEP: &str;
```

Rust module path separator (e.g., `serde_json::de::from_str`).

### `FILE_PATH_SEP`

```rust
const FILE_PATH_SEP: char = '/';
```

File system path separator for generated documentation.

