*[cargo_docs_md](../index.md) / [multi_crate](index.md)*

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

## Contents

- [Modules](#modules)
  - [`collection`](#collection)
  - [`context`](#context)
  - [`generator`](#generator)
  - [`parser`](#parser)
  - [`registry`](#registry)
  - [`search`](#search)
  - [`summary`](#summary)
- [Structs](#structs)
  - [`CrateCollection`](#cratecollection)
  - [`MultiCrateContext`](#multicratecontext)
  - [`SingleCrateView`](#singlecrateview)
  - [`MultiCrateGenerator`](#multicrategenerator)
  - [`MultiCrateParser`](#multicrateparser)
  - [`UnifiedLinkRegistry`](#unifiedlinkregistry)
  - [`SearchIndex`](#searchindex)
  - [`SearchIndexGenerator`](#searchindexgenerator)
  - [`SummaryGenerator`](#summarygenerator)
- [Constants](#constants)
  - [`RUST_PATH_SEP`](#rust-path-sep)
  - [`FILE_PATH_SEP`](#file-path-sep)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`collection`](#collection) | mod | Crate collection for multi-crate documentation. |
| [`context`](#context) | mod | Multi-crate generation context. |
| [`generator`](#generator) | mod | Multi-crate documentation generator. |
| [`parser`](#parser) | mod | Multi-crate JSON parser. |
| [`registry`](#registry) | mod | Unified link registry for cross-crate documentation. |
| [`search`](#search) | mod | Search index generation for multi-crate documentation. |
| [`summary`](#summary) | mod | mdBook SUMMARY.md generator. |
| [`CrateCollection`](#cratecollection) | struct |  |
| [`MultiCrateContext`](#multicratecontext) | struct |  |
| [`SingleCrateView`](#singlecrateview) | struct |  |
| [`MultiCrateGenerator`](#multicrategenerator) | struct |  |
| [`MultiCrateParser`](#multicrateparser) | struct |  |
| [`UnifiedLinkRegistry`](#unifiedlinkregistry) | struct |  |
| [`SearchIndex`](#searchindex) | struct |  |
| [`SearchIndexGenerator`](#searchindexgenerator) | struct |  |
| [`SummaryGenerator`](#summarygenerator) | struct |  |
| [`RUST_PATH_SEP`](#rust-path-sep) | const | Rust module path separator (e.g., `serde_json::de::from_str`). |
| [`FILE_PATH_SEP`](#file-path-sep) | const | File system path separator for generated documentation. |

## Modules

- [`collection`](collection/index.md) — Crate collection for multi-crate documentation.
- [`context`](context/index.md) — Multi-crate generation context.
- [`generator`](generator/index.md) — Multi-crate documentation generator.
- [`parser`](parser/index.md) — Multi-crate JSON parser.
- [`registry`](registry/index.md) — Unified link registry for cross-crate documentation.
- [`search`](search/index.md) — Search index generation for multi-crate documentation.
- [`summary`](summary/index.md) — mdBook SUMMARY.md generator.

## Structs

### `CrateCollection`

```rust
struct CrateCollection {
    crates: std::collections::HashMap<String, rustdoc_types::Crate>,
}
```

*Defined in `src/multi_crate/collection.rs:30-34`*

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

- <span id="cratecollection-new"></span>`fn new() -> Self`

  Create an empty crate collection.

- <span id="cratecollection-insert"></span>`fn insert(&mut self, name: String, krate: Crate) -> Option<Crate>`

  Insert a crate into the collection.

  

  If a crate with the same name already exists, it is replaced

  and `Some(old_crate)` is returned.

- <span id="cratecollection-get"></span>`fn get(&self, name: &str) -> Option<&Crate>`

  Get a crate by name.

- <span id="cratecollection-get-with-name"></span>`fn get_with_name(&self, name: &str) -> Option<(&str, &Crate)>`

  Get a crate by name, returning the stored key as well.

  

  This is useful when you need a reference to the crate name that

  has the same lifetime as the collection.

- <span id="cratecollection-contains"></span>`fn contains(&self, name: &str) -> bool`

  Check if a crate exists in the collection.

- <span id="cratecollection-iter"></span>`fn iter(&self) -> impl Iterator<Item = (&String, &Crate)>`

  Iterate over crates in alphabetical order.

  

  Returns tuples of `(&crate_name, &Crate)` sorted alphabetically

  by crate name for deterministic output.

  

  Sorting is done on-demand since collection size is small (10-50 crates).

- <span id="cratecollection-len"></span>`fn len(&self) -> usize`

  Get the number of crates in the collection.

- <span id="cratecollection-is-empty"></span>`fn is_empty(&self) -> bool`

  Check if the collection is empty.

- <span id="cratecollection-names"></span>`fn names(&self) -> Vec<&String>`

  Get crate names in alphabetical order.

  

  Returns a sorted `Vec` of crate names for deterministic processing.

  Sorting is done on-demand since collection size is small.

#### Trait Implementations

##### `impl Any for CrateCollection`

- <span id="cratecollection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrateCollection`

- <span id="cratecollection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrateCollection`

- <span id="cratecollection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CrateCollection`

- <span id="cratecollection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateCollection`

- <span id="cratecollection-default"></span>`fn default() -> CrateCollection` — [`CrateCollection`](collection/index.md#cratecollection)

##### `impl<T> From for CrateCollection`

- <span id="cratecollection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CrateCollection`

##### `impl<U> Into for CrateCollection`

- <span id="cratecollection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CrateCollection`

##### `impl OwoColorize for CrateCollection`

##### `impl Pointable for CrateCollection`

- <span id="cratecollection-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cratecollection-pointable-type-init"></span>`type Init = T`

- <span id="cratecollection-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratecollection-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratecollection-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratecollection-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CrateCollection`

- <span id="cratecollection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cratecollection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrateCollection`

- <span id="cratecollection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cratecollection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for CrateCollection`

### `MultiCrateContext<'a>`

```rust
struct MultiCrateContext<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    registry: crate::multi_crate::UnifiedLinkRegistry,
    args: &'a crate::Args,
    config: crate::generator::config::RenderConfig,
    cross_crate_impls: std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>,
    source_path_config: Option<crate::generator::render_shared::SourcePathConfig>,
}
```

*Defined in `src/multi_crate/context.rs:39-62`*

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

- **`config`**: `crate::generator::config::RenderConfig`

  Rendering configuration options.

- **`cross_crate_impls`**: `std::collections::HashMap<String, std::collections::HashMap<String, Vec<&'a rustdoc_types::Impl>>>`

  Pre-computed cross-crate impl blocks.
  
  Maps target crate name -> type name -> impl blocks from other crates.
  This is computed once during construction rather than per-view.

- **`source_path_config`**: `Option<crate::generator::render_shared::SourcePathConfig>`

  Base source path configuration for transforming cargo registry paths.
  
  `None` if source locations are disabled or no `.source_*` dir detected.

#### Implementations

- <span id="multicratecontext-new"></span>`fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self` — [`CrateCollection`](collection/index.md#cratecollection), [`Args`](../index.md#args), [`RenderConfig`](../generator/config/index.md#renderconfig)

  Create a new multi-crate context.

  

  Builds the unified link registry and pre-computes cross-crate impls.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `args` - CLI arguments

  * `config` - Rendering configuration options

- <span id="multicratecontext-set-source-dir"></span>`fn set_source_dir(&mut self, source_dir: &Path)`

  Set the source directory for path transformation.

  

  This can be called after construction if a `.source_*` directory

  is detected or specified via CLI. Only has effect if `source_locations`

  is enabled in the config.

- <span id="multicratecontext-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../generator/render_shared/index.md#sourcepathconfig)

  Get source path config for a specific file.

  

  Returns `None` if source locations are disabled or no source dir configured.

- <span id="multicratecontext-build-cross-crate-impls"></span>`fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](collection/index.md#cratecollection)

  Build the cross-crate impl map for all crates.

  

  Scans all crates once and groups impl blocks by their target crate

  and type name. This avoids O(n*m) scanning per view creation.

- <span id="multicratecontext-crates"></span>`const fn crates(&self) -> &CrateCollection` — [`CrateCollection`](collection/index.md#cratecollection)

  Get the crate collection.

- <span id="multicratecontext-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](registry/index.md#unifiedlinkregistry)

  Get the unified link registry.

- <span id="multicratecontext-args"></span>`const fn args(&self) -> &Args` — [`Args`](../index.md#args)

  Get CLI arguments.

- <span id="multicratecontext-single-crate-view"></span>`fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](context/index.md#singlecrateview)

  Create a single-crate view for rendering one crate.

  

  This bridges multi-crate mode to existing single-crate rendering

  code by providing a compatible interface that uses the unified

  registry for cross-crate link resolution.

- <span id="multicratecontext-find-item"></span>`fn find_item(&self, id: &Id) -> Option<(&str, &Item)>`

  Find an item across all crates by ID.

  

  Searches through all crates in the collection to find an item with

  the given ID. This is useful for resolving re-exports that point to

  items in external crates.

  

  # Returns

  

  A tuple of `(crate_name, item)` if found, or `None` if the item

  doesn't exist in any crate.

- <span id="multicratecontext-get-cross-crate-impls"></span>`fn get_cross_crate_impls(&self, target_crate: &str) -> Option<&HashMap<String, Vec<&'a Impl>>>`

  Get pre-computed cross-crate impl blocks for a target crate.

  

  Returns a map from type name to impl blocks from other crates.

  This data is pre-computed during context construction for efficiency.

  

  # Returns

  

  Reference to the type-name -> impl-blocks map, or `None` if the

  crate is not in the collection.

- <span id="multicratecontext-get-impl-target-path"></span>`fn get_impl_target_path(impl_block: &Impl) -> Option<String>`

  Get the target type path for an impl block.

#### Trait Implementations

##### `impl Any for MultiCrateContext<'a>`

- <span id="multicratecontext-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiCrateContext<'a>`

- <span id="multicratecontext-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiCrateContext<'a>`

- <span id="multicratecontext-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiCrateContext<'a>`

- <span id="multicratecontext-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MultiCrateContext<'a>`

##### `impl<U> Into for MultiCrateContext<'a>`

- <span id="multicratecontext-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiCrateContext<'a>`

##### `impl OwoColorize for MultiCrateContext<'a>`

##### `impl Pointable for MultiCrateContext<'a>`

- <span id="multicratecontext-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicratecontext-pointable-type-init"></span>`type Init = T`

- <span id="multicratecontext-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicratecontext-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicratecontext-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicratecontext-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MultiCrateContext<'a>`

- <span id="multicratecontext-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multicratecontext-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiCrateContext<'a>`

- <span id="multicratecontext-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multicratecontext-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MultiCrateContext<'a>`

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

*Defined in `src/multi_crate/context.rs:276-301`*

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

- <span id="singlecrateview-new"></span>`fn new(crate_name: &'a str, krate: &'a Crate, registry: &'a UnifiedLinkRegistry, args: &'a Args, ctx: &'a MultiCrateContext<'a>) -> Self` — [`UnifiedLinkRegistry`](registry/index.md#unifiedlinkregistry), [`Args`](../index.md#args), [`MultiCrateContext`](context/index.md#multicratecontext)

  Create a new single-crate view.

- <span id="singlecrateview-build-impl-map"></span>`fn build_impl_map(&mut self)`

  Build the impl map for all types.

  

  Uses the `impls` field on Struct/Enum/Union items directly rather than

  scanning all items and checking the `for_` field. This provides clearer

  semantics and leverages `rustdoc_types` structured data.

- <span id="singlecrateview-build-type-name-map"></span>`fn build_type_name_map(&mut self)`

  Build a map from type name to type ID.

  

  This is used to look up cross-crate impls by type name.

- <span id="singlecrateview-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

  Generate a sort key for impl blocks.

- <span id="singlecrateview-crate-name"></span>`const fn crate_name(&self) -> &str`

  Get the crate name.

- <span id="singlecrateview-krate"></span>`const fn krate(&self) -> &Crate`

  Get the crate being rendered.

- <span id="singlecrateview-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](registry/index.md#unifiedlinkregistry)

  Get the unified registry.

- <span id="singlecrateview-args"></span>`const fn args(&self) -> &Args` — [`Args`](../index.md#args)

  Get CLI arguments.

- <span id="singlecrateview-get-impls"></span>`fn get_impls(&self, id: Id) -> Option<&Vec<&'a Impl>>`

  Get impl blocks for a type (local crate only).

- <span id="singlecrateview-get-all-impls"></span>`fn get_all_impls(&self, id: Id) -> Vec<&'a Impl>`

  Get all impl blocks for a type, including cross-crate impls.

  

  This method merges local impls (from this crate) with impls from

  other crates that implement traits for this type.

- <span id="singlecrateview-get-impls-from-crate"></span>`fn get_impls_from_crate(&self, id: Id, source_krate: &'a Crate) -> Vec<&'a Impl>`

  Get impl blocks for a type from a specific crate.

  

  This is used for cross-crate re-exports where we need to look up

  impl blocks from the source crate rather than the current crate.

  

  # Arguments

  

  * `id` - The ID of the type to get impls for

  * `source_krate` - The crate to look up impls from

  

  # Returns

  

  A vector of impl blocks found in the source crate for the given type ID.

- <span id="singlecrateview-get-impl-target-id-from-type"></span>`const fn get_impl_target_id_from_type(ty: &rustdoc_types::Type) -> Option<Id>`

  Extract the target ID from a Type (for impl block matching).

- <span id="singlecrateview-should-include-item"></span>`const fn should_include_item(&self, item: &rustdoc_types::Item) -> bool`

  Check if an item should be included based on visibility.

- <span id="singlecrateview-count-modules"></span>`fn count_modules(&self) -> usize`

  Count modules for progress reporting.

- <span id="singlecrateview-create-link"></span>`fn create_link(&self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`

  Create a markdown link using the unified registry.

- <span id="singlecrateview-resolve-name"></span>`fn resolve_name(&self, name: &str) -> Option<(String, Id)>`

  Resolve a name to a crate and ID.

- <span id="singlecrateview-lookup-item-across-crates"></span>`fn lookup_item_across_crates(&self, id: &Id) -> Option<(&str, &Item)>`

  Look up an item across all crates by ID.

  

  This is useful for resolving re-exports that point to items in

  external crates. First checks the local crate, then searches

  all other crates in the collection.

  

  # Returns

  

  A tuple of `(crate_name, item)` if found, or `None` if the item

  doesn't exist in any crate.

- <span id="singlecrateview-get-crate"></span>`fn get_crate(&self, name: &str) -> Option<&Crate>`

  Get a crate by name from the collection.

  

  This is useful for getting the source crate context when rendering

  re-exported items from other crates.

  

  # Returns

  

  The crate if found, or `None` if no crate with that name exists.

- <span id="singlecrateview-resolve-external-path"></span>`fn resolve_external_path(&self, path: &str) -> Option<(&str, &Item, Id)>`

  Resolve a path like `regex_automata::Regex` to an item.

  

  This is used for external re-exports where `use_item.id` is `None`

  but the source path is available.

  

  # Returns

  

  A tuple of `(source_crate, item, item_id)` if found.

- <span id="singlecrateview-process-backtick-links"></span>`fn process_backtick_links(&self, docs: &str, item_links: &HashMap<String, Id>, current_file: &str) -> String`

  Process backtick links like ``Span`` to markdown links.

- <span id="singlecrateview-process-plain-links"></span>`fn process_plain_links(&self, docs: &str, current_file: &str) -> String`

  Process plain links like `[enter]` to markdown links.

  

  Uses the registry to resolve links to proper paths. If the item exists

  in the registry, creates a link to its location. If on the current page

  and has a heading anchor, uses an anchor link.

  

  Skips matches that are:

  - Inside inline code (backticks)

  - Already markdown links (followed by `(` or `[`)

- <span id="singlecrateview-resolve-plain-link"></span>`fn resolve_plain_link(&self, link_text: &str, current_file: &str) -> Option<String>`

  Resolve a plain link `[name]` to a markdown link.

  

  Returns `Some(markdown_link)` if the item can be resolved,

  `None` if it should remain as plain text.

- <span id="singlecrateview-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>, current_file: &str) -> Option<String>`

  Resolve a link text to a markdown link using the registry.

  

  This function attempts to convert rustdoc link syntax into valid markdown

  links that work in the generated documentation.

  

  # Arguments

  * `link_text` - The raw link target from rustdoc (e.g., "`crate::config::ConfigBuilder::method`")

  * `item_links` - Map of link texts to Item IDs from rustdoc's `links` field

  * `current_file` - The markdown file being generated (e.g., "ureq/index.md")

  

  # Returns

  * `Some(markdown_link)` - A formatted markdown link like `[`text`](path.md#anchor)`

  * `None` - If the link cannot be resolved (will be rendered as inline code)

  

  # Examples

  

  ```text

  Input:  link_text = "crate::config::ConfigBuilder::http_status_as_error"

          current_file = "ureq/index.md"

  Output: Some("[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)")

  

  Input:  link_text = "ConfigBuilder"

          current_file = "ureq/agent/index.md"

  Output: Some("[`ConfigBuilder`](../config/index.md#configbuilder)")

  

  Input:  link_text = "std::io::Error"  (external crate, not in registry)

          current_file = "ureq/index.md"

  Output: None  (rendered as `std::io::Error` inline code)

  ```

- <span id="singlecrateview-build-link-to-id"></span>`fn build_link_to_id(&self, id: Id, current_file: &str, display_name: &str, anchor: Option<&str>) -> Option<String>`

  Build a link to an item by ID.

  

  This is the simplest path when we already have a resolved Item ID from

  rustdoc's links map. We just need to look up the file path in our registry.

  

  # Arguments

  * `id` - The rustdoc Item ID to link to

  * `current_file` - Source file for relative path computation

  * `display_name` - Text to show in the link

  * `anchor` - Optional anchor (e.g., method name)

  

  # Example Transformation

  

  ```text

  Input:

    id = Id(123)  (rustdoc's internal ID for ConfigBuilder)

    current_file = "ureq/agent/index.md"

    display_name = "ConfigBuilder"

    anchor = None

  

  Step 1: Look up ID in registry

    registry.get_path("ureq", Id(123)) → Some("config/index.md")

  

  Step 2: Build markdown link

    build_markdown_link("ureq/agent/index.md", "ureq", "config/index.md", "ConfigBuilder", None)

    → "[`ConfigBuilder`](../config/index.md)"

  

  Output: Some("[`ConfigBuilder`](../config/index.md)")

  ```

- <span id="singlecrateview-resolve-crate-path"></span>`fn resolve_crate_path(&self, path_without_crate: &str, display_name: &str, current_file: &str) -> Option<String>`

  Resolve `crate::path::Item` or `crate::path::Item::method` patterns.

  

  This handles the common rustdoc pattern where docs reference items using

  crate-relative paths. The tricky part is distinguishing between:

  - `crate::module::Type` (link to Type, no anchor)

  - `crate::module::Type::method` (link to Type with #method anchor)

  - `crate::module::Type::Variant` (link to Type with #Variant anchor)

  

  # Arguments

  * `path_without_crate` - The path after stripping "`crate::`" prefix

  * `display_name` - Full original text for display (includes "`crate::`")

  * `current_file` - Source file for relative path computation

  

  # Example Transformation

  

  ```text

  Input:

    path_without_crate = "config::ConfigBuilder::http_status_as_error"

    display_name = "crate::config::ConfigBuilder::http_status_as_error"

    current_file = "ureq/index.md"

  

  Step 1: Split into type path and anchor

    split_type_and_anchor("config::ConfigBuilder::http_status_as_error")

    → ("config::ConfigBuilder", Some("http_status_as_error"))

    (lowercase "http_status_as_error" indicates a method)

  

  Step 2: Extract the type name (last segment of type path)

    "config::ConfigBuilder".rsplit("::").next() → "ConfigBuilder"

  

  Step 3: Resolve type name in registry

    registry.resolve_name("ConfigBuilder", "ureq") → Some(("ureq", Id(123)))

    registry.get_path("ureq", Id(123)) → Some("config/index.md")

  

  Step 4: Build markdown link with anchor

    build_markdown_link("ureq/index.md", "ureq", "config/index.md",

                        "crate::config::ConfigBuilder::http_status_as_error",

                        Some("http_status_as_error"))

    → "[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)"

  

  Output: Some("[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)")

  ```

- <span id="singlecrateview-split-type-and-anchor"></span>`fn split_type_and_anchor(path: &str) -> (&str, Option<&str>)`

  Split `config::ConfigBuilder::method` into (`config::ConfigBuilder`, Some("method")).

  

  Detects methods (lowercase) and enum variants (`Type::Variant` pattern).

  

  # Detection Rules

  

  1. **Methods/fields**: Last segment starts with lowercase

     - `Type::method` → (Type, method)

     - `mod::Type::field_name` → (`mod::Type`, `field_name`)

  

  2. **Enum variants**: Two consecutive uppercase segments

     - `Option::Some` → (Option, Some)

     - `mod::Error::IoError` → (`mod::Error`, `IoError`)

  

  3. **Nested types**: Uppercase but no uppercase predecessor

     - `mod::OuterType::InnerType` → (`mod::OuterType::InnerType`, None)

  

  # Examples

  

  ```text

  "ConfigBuilder::http_status_as_error"

    Last segment "http_status_as_error" starts lowercase → method

    → ("ConfigBuilder", Some("http_status_as_error"))

  

  "config::ConfigBuilder::new"

    Last segment "new" starts lowercase → method

    → ("config::ConfigBuilder", Some("new"))

  

  "Option::Some"

    "Option" uppercase, "Some" uppercase → enum variant

    → ("Option", Some("Some"))

  

  "error::Error::Io"

    "Error" uppercase, "Io" uppercase → enum variant

    → ("error::Error", Some("Io"))

  

  "config::ConfigBuilder"

    "config" lowercase, "ConfigBuilder" uppercase → not a variant

    → ("config::ConfigBuilder", None)

  

  "Vec"

    No "::" separator

    → ("Vec", None)

  ```

- <span id="singlecrateview-build-markdown-link"></span>`fn build_markdown_link(&self, current_file: &str, target_crate: &str, target_path: &str, display_name: &str, anchor: Option<&str>) -> String`

  Build a markdown link, handling same-crate and cross-crate cases.

  

  This is the core function that computes relative paths between markdown

  files and formats the final link.

  

  # Arguments

  * `current_file` - The file we're generating (e.g., "ureq/agent/index.md")

  * `target_crate` - The crate containing the target item

  * `target_path` - Path to target within its crate (e.g., "config/index.md")

  * `display_name` - Text to show in the link

  * `anchor` - Optional anchor suffix (e.g., "`method_name`")

  

  # Path Computation Examples

  

  ## Same Crate Examples

  

  ```text

  Example 1: Link from index to nested module

     current_file = "ureq/index.md"

     target_crate = "ureq"

     target_path = "config/index.md"

  

     Step 1: Strip crate prefix from current

       "ureq/index.md" -> "index.md"

  

     Step 2: Compute relative path

       from "index.md" to "config/index.md"

       -> "config/index.md"

  

     Output: "[`display`](config/index.md)"

  

  Example 2: Link from nested to sibling module

     current_file = "ureq/agent/index.md"

     target_crate = "ureq"

     target_path = "config/index.md"

  

     Step 1: Strip crate prefix

       "ureq/agent/index.md" -> "agent/index.md"

  

     Step 2: Compute relative path

       from "agent/index.md" to "config/index.md"

       -> "config/index.md"

  

     Output: "[`display`][../config/index.md]"

  

  ## Cross-Crate Examples

  

  ```text

  Example 3: Link from one crate to another

    current_file = "ureq/agent/index.md"

    target_crate = "http"

    target_path  = "status/index.md"

  

    Step 1: Strip crate prefix

      "ureq/agent/index.md" → "agent/index.md"

  

    Step 2: Count depth (number of '/' in local path)

      "agent/index.md" has 1 slash → depth = 1

  

    Step 3: Build cross-crate path

      Go up (depth + 1) levels: "../" * 2 = "../../"

      Then into target crate: "../../http/status/index.md"

  

    Output: "[`display`](../../http/status/index.md)"

  

  Example 4: Cross-crate from root

    current_file = "ureq/index.md"

    target_crate = "http"

    target_path  = "index.md"

  

    depth = 0 (no slashes in "index.md")

    prefix = "../" * 1 = "../"

  

    Output: "[`display`](../http/index.md)"

  ```rust

- <span id="singlecrateview-compute-cross-crate-path"></span>`fn compute_cross_crate_path(current_local: &str, target_crate: &str, target_path: &str) -> String`

  Compute a relative path for cross-crate linking.

  

  Given the local portion of the current file path (without crate prefix),

  computes the `../` prefix needed to navigate to another crate's file.

  

  # Arguments

  * `current_local` - Current file path within crate (e.g., "agent/index.md")

  * `target_crate` - Name of the target crate

  * `target_path` - Path within target crate (e.g., "status/index.md")

  

  # Examples

  

  ```text

  // From root of one crate to another

  compute_cross_crate_path("index.md", "http", "index.md")

    → "../http/index.md"

  

  // From nested module to another crate

  compute_cross_crate_path("agent/index.md", "http", "status/index.md")

    → "../../http/status/index.md"

  

  // From deeply nested to another crate root

  compute_cross_crate_path("a/b/c/index.md", "other", "index.md")

    → "../../../../other/index.md"

  ```

- <span id="singlecrateview-strip-crate-prefix"></span>`fn strip_crate_prefix(path: &str) -> &str`

  Strip the crate prefix from a file path.

  

  File paths in our system includes the crate name as the first directory.

  This helper removes it to get the crate-local path.

  

  # Examples

  

  ```text

  "ureq/config/index.md" -> "config/index.md"

  "ureq/index.md"        -> "index.md"

  "http/status/index.md" -> "status/index.md"

  "simple.md"            -> "simple.md" (no slash returns as is)

  ```

- <span id="singlecrateview-looks-like-external-reference"></span>`fn looks_like_external_reference(link_text: &str) -> bool`

  Check if a link text looks like an intentional external crate reference.

  

  Simple names like "Wide", "Error", "Default" are often meant to be

  local anchors or type aliases, not cross-crate links.

#### Trait Implementations

##### `impl Any for SingleCrateView<'a>`

- <span id="singlecrateview-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SingleCrateView<'a>`

- <span id="singlecrateview-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SingleCrateView<'a>`

- <span id="singlecrateview-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SingleCrateView<'a>`

- <span id="singlecrateview-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SingleCrateView<'a>`

##### `impl<U> Into for SingleCrateView<'a>`

- <span id="singlecrateview-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SingleCrateView<'a>`

##### `impl ItemAccess for SingleCrateView<'_>`

- <span id="singlecrateview-itemaccess-krate"></span>`fn krate(&self) -> &Crate`

- <span id="singlecrateview-itemaccess-crate-name"></span>`fn crate_name(&self) -> &str`

- <span id="singlecrateview-itemaccess-get-item"></span>`fn get_item(&self, id: &Id) -> Option<&Item>`

- <span id="singlecrateview-itemaccess-get-impls"></span>`fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

- <span id="singlecrateview-itemaccess-crate-version"></span>`fn crate_version(&self) -> Option<&str>`

- <span id="singlecrateview-itemaccess-render-config"></span>`fn render_config(&self) -> &RenderConfig` — [`RenderConfig`](../generator/config/index.md#renderconfig)

- <span id="singlecrateview-itemaccess-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../generator/render_shared/index.md#sourcepathconfig)

##### `impl ItemFilter for SingleCrateView<'_>`

- <span id="singlecrateview-itemfilter-should-include-item"></span>`fn should_include_item(&self, item: &Item) -> bool`

- <span id="singlecrateview-itemfilter-include-private"></span>`fn include_private(&self) -> bool`

- <span id="singlecrateview-itemfilter-include-blanket-impls"></span>`fn include_blanket_impls(&self) -> bool`

##### `impl LinkResolver for SingleCrateView<'_>`

- <span id="singlecrateview-linkresolver-link-registry"></span>`fn link_registry(&self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../linker/index.md#linkregistry)

- <span id="singlecrateview-linkresolver-process-docs"></span>`fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

- <span id="singlecrateview-linkresolver-create-link"></span>`fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

##### `impl OwoColorize for SingleCrateView<'a>`

##### `impl Pointable for SingleCrateView<'a>`

- <span id="singlecrateview-pointable-const-align"></span>`const ALIGN: usize`

- <span id="singlecrateview-pointable-type-init"></span>`type Init = T`

- <span id="singlecrateview-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="singlecrateview-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="singlecrateview-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="singlecrateview-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl RenderContext for SingleCrateView<'a>`

##### `impl<U> TryFrom for SingleCrateView<'a>`

- <span id="singlecrateview-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="singlecrateview-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SingleCrateView<'a>`

- <span id="singlecrateview-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="singlecrateview-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SingleCrateView<'a>`

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    ctx: crate::multi_crate::MultiCrateContext<'a>,
    args: &'a crate::Args,
}
```

*Defined in `src/multi_crate/generator.rs:397-403`*

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

- <span id="multicrategenerator-new"></span>`fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self` — [`CrateCollection`](collection/index.md#cratecollection), [`Args`](../index.md#args), [`RenderConfig`](../generator/config/index.md#renderconfig)

  Create a new multi-crate generator.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `args` - CLI arguments

  * `config` - Rendering configuration options

- <span id="multicrategenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

  Generate documentation for all crates.

  

  Creates the output directory structure, generates docs for each crate

  in parallel using rayon, and optionally generates SUMMARY.md for

  mdBook compatibility.

  

  # Errors

  

  Returns an error if any file operation fails.

- <span id="multicrategenerator-collect-rendered-items"></span>`fn collect_rendered_items(&self) -> HashMap<String, HashSet<Id>>`

  Collect the IDs of all items that would be rendered.

  

  This walks the module tree for each crate using the same visibility

  rules as rendering, collecting the IDs of items that will have

  documentation generated for them.

- <span id="multicrategenerator-collect-crate-items"></span>`fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](context/index.md#singlecrateview)

  Collect rendered item IDs for a single crate.

- <span id="multicrategenerator-collect-module-items"></span>`fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](context/index.md#singlecrateview)

  Recursively collect rendered item IDs from a module.

- <span id="multicrategenerator-generate-crate"></span>`fn generate_crate(&self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](context/index.md#singlecrateview), [`Error`](../error/index.md#error)

  Generate documentation for a single crate.

- <span id="multicrategenerator-generate-module"></span>`fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](context/index.md#singlecrateview), [`Error`](../error/index.md#error)

  Generate a module directory with index.md and child modules.

- <span id="multicrategenerator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../error/index.md#error)

  Create a progress bar.

  

  # Errors

  

  Returns an error if the progress bar template is invalid.

#### Trait Implementations

##### `impl Any for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MultiCrateGenerator<'a>`

##### `impl<U> Into for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiCrateGenerator<'a>`

##### `impl OwoColorize for MultiCrateGenerator<'a>`

##### `impl Pointable for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrategenerator-pointable-type-init"></span>`type Init = T`

- <span id="multicrategenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrategenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrategenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrategenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multicrategenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multicrategenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MultiCrateGenerator<'a>`

### `MultiCrateParser`

```rust
struct MultiCrateParser;
```

*Defined in `src/multi_crate/parser.rs:26`*

Parser for multiple rustdoc JSON files in a directory.

Discovers JSON files and parses each one, extracting the crate name
from the root module item.

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
println!("Found {} crates", crates.len());
```

#### Implementations

- <span id="multicrateparser-parse-directory"></span>`fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](collection/index.md#cratecollection), [`Error`](../error/index.md#error)

  Parse all rustdoc JSON files in a directory.

  

  Scans the top level of the directory for `*.json` files and

  attempts to parse each one as rustdoc JSON. Files that aren't

  valid rustdoc JSON (e.g., search indices) are silently skipped.

  

  # Arguments

  

  * `dir` - Path to directory containing JSON files

  

  # Returns

  

  A `CrateCollection` containing all successfully parsed crates.

  

  # Errors

  

  - [`Error::InvalidDirectory`](../index.md) if the path is invalid

  - [`Error::NoJsonFiles`](../index.md) if no valid JSON files found

  - [`Error::DuplicateCrate`](../index.md) if multiple files define the same crate

  - [`Error::NoCrateName`](../index.md) if a JSON file has no root module

- <span id="multicrateparser-extract-crate-name"></span>`fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](../error/index.md#error)

  Extract the crate name from a parsed Crate.

  

  The crate name is stored in the root item's `name` field.

#### Trait Implementations

##### `impl Any for MultiCrateParser`

- <span id="multicrateparser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiCrateParser`

- <span id="multicrateparser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiCrateParser`

- <span id="multicrateparser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MultiCrateParser`

- <span id="multicrateparser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MultiCrateParser`

##### `impl<U> Into for MultiCrateParser`

- <span id="multicrateparser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiCrateParser`

##### `impl OwoColorize for MultiCrateParser`

##### `impl Pointable for MultiCrateParser`

- <span id="multicrateparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrateparser-pointable-type-init"></span>`type Init = T`

- <span id="multicrateparser-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrateparser-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrateparser-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrateparser-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MultiCrateParser`

- <span id="multicrateparser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multicrateparser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiCrateParser`

- <span id="multicrateparser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multicrateparser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MultiCrateParser`

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

*Defined in `src/multi_crate/registry.rs:94-116`*

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

- <span id="unifiedlinkregistry-build"></span>`fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](collection/index.md#cratecollection)

  Build a unified registry from a collection of crates.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `primary_crate` - Optional primary crate for disambiguation

  

  # Returns

  

  A populated registry ready for link resolution.

- <span id="unifiedlinkregistry-register-crate"></span>`fn register_crate(&mut self, crate_name: &str, krate: &Crate)`

  Register all items from a single crate.

- <span id="unifiedlinkregistry-register-from-paths"></span>`fn register_from_paths(&mut self, crate_name: &str, krate: &Crate)`

  Register items using the `paths` field from rustdoc JSON.

  

  The `paths` field contains canonical paths for all items, including

  those in private modules that are re-exported publicly. Since we only

  generate docs for public modules, items in private modules are

  documented at their public re-export location (typically root).

- <span id="unifiedlinkregistry-item-enum-to-kind"></span>`const fn item_enum_to_kind(inner: &ItemEnum) -> ItemKind`

  Convert `ItemEnum` to `ItemKind` for the name index.

- <span id="unifiedlinkregistry-register-item-recursive"></span>`fn register_item_recursive(&mut self, krate: &Crate, crate_name: &str, item_id: Id, item: &rustdoc_types::Item, parent_path: &str)`

  Recursively register an item and its children.

- <span id="unifiedlinkregistry-register-item"></span>`fn register_item(&mut self, crate_name: &str, id: Id, name: &str, path: &str, kind: ItemKind)`

  Register a single item in the registry.

- <span id="unifiedlinkregistry-get-path"></span>`fn get_path(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

  Get the file path for an item in a specific crate.

  

  Uses raw entry API for zero-allocation lookup.

- <span id="unifiedlinkregistry-get-name"></span>`fn get_name(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

  Get the display name for an item.

  

  Uses raw entry API for zero-allocation lookup.

- <span id="unifiedlinkregistry-get-re-export-source"></span>`fn get_re_export_source(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

  Get the original source path for an external re-export.

  

  Returns `Some("crate::path::Item")` if this item is a re-export

  from another crate, `None` otherwise.

- <span id="unifiedlinkregistry-resolve-reexport"></span>`fn resolve_reexport(&self, crate_name: &str, id: Id) -> Option<(compact_str::CompactString, Id)>`

  Resolve through re-export chain to find the canonical item.

  

  If the item is an external re-export, follows the source path

  to find the original crate and ID. Returns the original if found,

  otherwise returns `None`.

  

  # Arguments

  

  * `crate_name` - The crate where the re-export appears

  * `id` - The ID of the re-export Use item

  

  # Returns

  

  `Some((original_crate, original_id))` if the re-export chain can be resolved,

  `None` if there's no re-export source or the original can't be found.

- <span id="unifiedlinkregistry-resolve-name"></span>`fn resolve_name(&self, name: &str, current_crate: &str) -> Option<(compact_str::CompactString, Id)>`

  Resolve an item name to its crate and ID.

  

  Uses disambiguation priority:

  1. Current crate (modules preferred over macros)

  2. Primary crate (if set, modules preferred)

  3. First module match, then first non-module match

- <span id="unifiedlinkregistry-resolve-path"></span>`fn resolve_path(&self, path: &str) -> Option<(compact_str::CompactString, Id)>`

  Resolve a full path like `regex_automata::Regex` to its crate and ID.

  

  This is used for resolving external re-exports where `use_item.id` is `None`

  but the source path is available.

  

  # Arguments

  

  * `path` - Full path like `regex_automata::Regex` or `tracing_core::span::Span`

  

  # Returns

  

  The (`crate_name`, `item_id`) if found in the registry.

- <span id="unifiedlinkregistry-create-link"></span>`fn create_link(&self, from_crate: &str, from_path: &str, to_crate: &str, to_id: Id) -> Option<String>`

  Create a markdown link from one file to another across crates.

  

  # Arguments

  

  * `from_crate` - The crate where the link appears

  * `from_path` - The file path where the link appears

  * `to_crate` - The target crate

  * `to_id` - The target item's ID

  

  # Returns

  

  A formatted markdown link like `[`Name`](relative/path.md)`,

  or `None` if the target item isn't registered.

- <span id="unifiedlinkregistry-compute-cross-crate-path"></span>`fn compute_cross_crate_path(from: &str, to: &str) -> String`

  Compute relative path between files potentially in different crates.

  

  # Examples

  

  - `tracing/span/index.md` to `tracing_core/subscriber/index.md`

    = `../../tracing_core/subscriber/index.md`

  - `tracing/index.md` to `tracing/span/index.md`

    = `span/index.md`

- <span id="unifiedlinkregistry-get-anchor"></span>`fn get_anchor(&self, crate_name: &str, id: Id) -> Option<String>`

  Get an anchor string for an item within its page.

  

  # Arguments

  

  * `crate_name` - The crate containing the item

  * `id` - The item's ID

  

  # Returns

  

  An anchor like `#span` or `#enter` for linking to specific items.

- <span id="unifiedlinkregistry-contains"></span>`fn contains(&self, crate_name: &str, id: Id) -> bool`

  Check if an item exists in the registry.

  

  Uses raw entry API for zero-allocation lookup.

- <span id="unifiedlinkregistry-len"></span>`fn len(&self) -> usize`

  Get the number of registered items.

- <span id="unifiedlinkregistry-is-empty"></span>`fn is_empty(&self) -> bool`

  Check if the registry is empty.

#### Trait Implementations

##### `impl Any for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-default"></span>`fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](registry/index.md#unifiedlinkregistry)

##### `impl<T> From for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for UnifiedLinkRegistry`

##### `impl<U> Into for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UnifiedLinkRegistry`

##### `impl OwoColorize for UnifiedLinkRegistry`

##### `impl Pointable for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unifiedlinkregistry-pointable-type-init"></span>`type Init = T`

- <span id="unifiedlinkregistry-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unifiedlinkregistry-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unifiedlinkregistry-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unifiedlinkregistry-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unifiedlinkregistry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unifiedlinkregistry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for UnifiedLinkRegistry`

### `SearchIndex`

```rust
struct SearchIndex {
    pub items: Vec<SearchEntry>,
}
```

*Defined in `src/multi_crate/search.rs:75-78`*

The complete search index containing all searchable items.

Serialized to `search_index.json` for client-side consumption.

#### Fields

- **`items`**: `Vec<SearchEntry>`

  All searchable items across all crates.

#### Trait Implementations

##### `impl Any for SearchIndex`

- <span id="searchindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SearchIndex`

- <span id="searchindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SearchIndex`

- <span id="searchindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SearchIndex`

- <span id="searchindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SearchIndex`

- <span id="searchindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SearchIndex`

##### `impl<U> Into for SearchIndex`

- <span id="searchindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SearchIndex`

##### `impl OwoColorize for SearchIndex`

##### `impl Pointable for SearchIndex`

- <span id="searchindex-pointable-const-align"></span>`const ALIGN: usize`

- <span id="searchindex-pointable-type-init"></span>`type Init = T`

- <span id="searchindex-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindex-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindex-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindex-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SearchIndex`

- <span id="searchindex-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl<U> TryFrom for SearchIndex`

- <span id="searchindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searchindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SearchIndex`

- <span id="searchindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searchindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SearchIndex`

### `SearchIndexGenerator<'a>`

```rust
struct SearchIndexGenerator<'a> {
    crates: &'a super::CrateCollection,
    include_private: bool,
    rendered_items: std::collections::HashMap<String, std::collections::HashSet<rustdoc_types::Id>>,
}
```

*Defined in `src/multi_crate/search.rs:93-108`*

Generator for multi-crate search indices.

Traverses all crates in a [`CrateCollection`](collection/index.md) and builds a comprehensive
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

- <span id="searchindexgenerator-new"></span>`const fn new(crates: &'a CrateCollection, include_private: bool, rendered_items: HashMap<String, HashSet<Id>>) -> Self` — [`CrateCollection`](collection/index.md#cratecollection)

  Create a new search index generator.

  

  # Arguments

  

  * `crates` - Collection of parsed crates to index

  * `include_private` - Whether to include non-public items

  * `rendered_items` - Map of crate name to set of rendered item IDs

- <span id="searchindexgenerator-generate"></span>`fn generate(&self) -> SearchIndex` — [`SearchIndex`](search/index.md#searchindex)

  Generate the complete search index.

  

  Traverses all crates and collects searchable items including:

  - Modules

  - Structs

  - Enums

  - Traits

  - Functions

  - Type aliases

  - Constants

  - Macros

  

  Items are sorted alphabetically by name for consistent output.

- <span id="searchindexgenerator-write"></span>`fn write(&self, output_dir: &Path) -> std::io::Result<()>`

  Write the search index to `search_index.json` in the output directory.

  

  # Arguments

  

  * `output_dir` - Directory where `search_index.json` will be written

  

  # Errors

  

  Returns an error if the file cannot be written.

- <span id="searchindexgenerator-index-crate"></span>`fn index_crate(&self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](search/index.md#searchentry)

  Index all items in a single crate.

  

  Only indexes items that were actually rendered (present in `rendered_items`).

- <span id="searchindexgenerator-build-path-map"></span>`fn build_path_map(krate: &Crate) -> HashMap<Id, String>`

  Build a map from item ID to its module path.

  

  This allows us to reconstruct the full path for each item.

- <span id="searchindexgenerator-compute-file-path"></span>`fn compute_file_path(crate_name: &str, module_path: &str, kind: &str) -> String`

  Compute the file path for an item based on its module location.

#### Trait Implementations

##### `impl Any for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SearchIndexGenerator<'a>`

##### `impl<U> Into for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SearchIndexGenerator<'a>`

##### `impl OwoColorize for SearchIndexGenerator<'a>`

##### `impl Pointable for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="searchindexgenerator-pointable-type-init"></span>`type Init = T`

- <span id="searchindexgenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindexgenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindexgenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindexgenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searchindexgenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searchindexgenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SearchIndexGenerator<'a>`

### `SummaryGenerator<'a>`

```rust
struct SummaryGenerator<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    output_dir: &'a std::path::Path,
    include_private: bool,
}
```

*Defined in `src/multi_crate/summary.rs:31-40`*

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

- <span id="summarygenerator-new"></span>`const fn new(crates: &'a CrateCollection, output_dir: &'a Path, include_private: bool) -> Self` — [`CrateCollection`](collection/index.md#cratecollection)

  Create a new summary generator.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `output_dir` - Directory to write SUMMARY.md

  * `include_private` - Whether to include private modules

- <span id="summarygenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

  Generate the SUMMARY.md file.

  

  # Errors

  

  Returns an error if the file cannot be written.

- <span id="summarygenerator-add-modules"></span>`fn add_modules(&self, content: &mut String, krate: &rustdoc_types::Crate, items: &[rustdoc_types::Id], path_prefix: &str, indent: usize)`

  Add module entries recursively.

#### Trait Implementations

##### `impl Any for SummaryGenerator<'a>`

- <span id="summarygenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SummaryGenerator<'a>`

- <span id="summarygenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SummaryGenerator<'a>`

- <span id="summarygenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SummaryGenerator<'a>`

- <span id="summarygenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SummaryGenerator<'a>`

##### `impl<U> Into for SummaryGenerator<'a>`

- <span id="summarygenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SummaryGenerator<'a>`

##### `impl OwoColorize for SummaryGenerator<'a>`

##### `impl Pointable for SummaryGenerator<'a>`

- <span id="summarygenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="summarygenerator-pointable-type-init"></span>`type Init = T`

- <span id="summarygenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="summarygenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="summarygenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="summarygenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SummaryGenerator<'a>`

- <span id="summarygenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="summarygenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SummaryGenerator<'a>`

- <span id="summarygenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="summarygenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SummaryGenerator<'a>`

## Constants

### `RUST_PATH_SEP`
```rust
const RUST_PATH_SEP: &str;
```

*Defined in `src/multi_crate/mod.rs:29`*

Rust module path separator (e.g., `serde_json::de::from_str`).

### `FILE_PATH_SEP`
```rust
const FILE_PATH_SEP: char = '/';
```

*Defined in `src/multi_crate/mod.rs:32`*

File system path separator for generated documentation.

