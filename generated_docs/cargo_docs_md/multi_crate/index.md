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

- <span id="cratecollection-insert"></span>`fn insert(&mut self, name: String, krate: Crate) -> Option<Crate>`

- <span id="cratecollection-get"></span>`fn get(&self, name: &str) -> Option<&Crate>`

- <span id="cratecollection-get-with-name"></span>`fn get_with_name(&self, name: &str) -> Option<(&str, &Crate)>`

- <span id="cratecollection-contains"></span>`fn contains(&self, name: &str) -> bool`

- <span id="cratecollection-iter"></span>`fn iter(&self) -> impl Iterator<Item = (&String, &Crate)>`

- <span id="cratecollection-len"></span>`fn len(&self) -> usize`

- <span id="cratecollection-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="cratecollection-names"></span>`fn names(&self) -> Vec<&String>`

#### Trait Implementations

##### `impl Debug for CrateCollection`

- <span id="cratecollection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateCollection`

- <span id="cratecollection-default"></span>`fn default() -> CrateCollection` — [`CrateCollection`](collection/index.md#cratecollection)

##### `impl Instrument for CrateCollection`

##### `impl IntoEither for CrateCollection`

##### `impl OwoColorize for CrateCollection`

##### `impl Pointable for CrateCollection`

- <span id="cratecollection-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cratecollection-pointable-type-init"></span>`type Init = T`

- <span id="cratecollection-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratecollection-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratecollection-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratecollection-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="multicratecontext-set-source-dir"></span>`fn set_source_dir(&mut self, source_dir: &Path)`

- <span id="multicratecontext-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../generator/render_shared/index.md#sourcepathconfig)

- <span id="multicratecontext-build-cross-crate-impls"></span>`fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](collection/index.md#cratecollection)

- <span id="multicratecontext-crates"></span>`const fn crates(&self) -> &CrateCollection` — [`CrateCollection`](collection/index.md#cratecollection)

- <span id="multicratecontext-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](registry/index.md#unifiedlinkregistry)

- <span id="multicratecontext-args"></span>`const fn args(&self) -> &Args` — [`Args`](../index.md#args)

- <span id="multicratecontext-single-crate-view"></span>`fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](context/index.md#singlecrateview)

- <span id="multicratecontext-find-item"></span>`fn find_item(&self, id: &Id) -> Option<(&str, &Item)>`

- <span id="multicratecontext-get-cross-crate-impls"></span>`fn get_cross_crate_impls(&self, target_crate: &str) -> Option<&HashMap<String, Vec<&'a Impl>>>`

- <span id="multicratecontext-get-impl-target-path"></span>`fn get_impl_target_path(impl_block: &Impl) -> Option<String>`

#### Trait Implementations

##### `impl Instrument for MultiCrateContext<'a>`

##### `impl IntoEither for MultiCrateContext<'a>`

##### `impl OwoColorize for MultiCrateContext<'a>`

##### `impl Pointable for MultiCrateContext<'a>`

- <span id="multicratecontext-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicratecontext-pointable-type-init"></span>`type Init = T`

- <span id="multicratecontext-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicratecontext-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicratecontext-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicratecontext-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="singlecrateview-build-impl-map"></span>`fn build_impl_map(&mut self)`

- <span id="singlecrateview-build-type-name-map"></span>`fn build_type_name_map(&mut self)`

- <span id="singlecrateview-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- <span id="singlecrateview-crate-name"></span>`const fn crate_name(&self) -> &str`

- <span id="singlecrateview-krate"></span>`const fn krate(&self) -> &Crate`

- <span id="singlecrateview-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](registry/index.md#unifiedlinkregistry)

- <span id="singlecrateview-args"></span>`const fn args(&self) -> &Args` — [`Args`](../index.md#args)

- <span id="singlecrateview-get-impls"></span>`fn get_impls(&self, id: Id) -> Option<&Vec<&'a Impl>>`

- <span id="singlecrateview-get-all-impls"></span>`fn get_all_impls(&self, id: Id) -> Vec<&'a Impl>`

- <span id="singlecrateview-get-impls-from-crate"></span>`fn get_impls_from_crate(&self, id: Id, source_krate: &'a Crate) -> Vec<&'a Impl>`

- <span id="singlecrateview-get-impl-target-id-from-type"></span>`const fn get_impl_target_id_from_type(ty: &rustdoc_types::Type) -> Option<Id>`

- <span id="singlecrateview-should-include-item"></span>`const fn should_include_item(&self, item: &rustdoc_types::Item) -> bool`

- <span id="singlecrateview-count-modules"></span>`fn count_modules(&self) -> usize`

- <span id="singlecrateview-create-link"></span>`fn create_link(&self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`

- <span id="singlecrateview-resolve-name"></span>`fn resolve_name(&self, name: &str) -> Option<(String, Id)>`

- <span id="singlecrateview-lookup-item-across-crates"></span>`fn lookup_item_across_crates(&self, id: &Id) -> Option<(&str, &Item)>`

- <span id="singlecrateview-get-crate"></span>`fn get_crate(&self, name: &str) -> Option<&Crate>`

- <span id="singlecrateview-resolve-external-path"></span>`fn resolve_external_path(&self, path: &str) -> Option<(&str, &Item, Id)>`

- <span id="singlecrateview-process-backtick-links"></span>`fn process_backtick_links(&self, docs: &str, item_links: &HashMap<String, Id>, current_file: &str) -> String`

- <span id="singlecrateview-process-plain-links"></span>`fn process_plain_links(&self, docs: &str, current_file: &str) -> String`

- <span id="singlecrateview-resolve-plain-link"></span>`fn resolve_plain_link(&self, link_text: &str, current_file: &str) -> Option<String>`

- <span id="singlecrateview-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>, current_file: &str) -> Option<String>`

- <span id="singlecrateview-build-link-to-id"></span>`fn build_link_to_id(&self, id: Id, current_file: &str, display_name: &str, anchor: Option<&str>) -> Option<String>`

- <span id="singlecrateview-resolve-crate-path"></span>`fn resolve_crate_path(&self, path_without_crate: &str, display_name: &str, current_file: &str) -> Option<String>`

- <span id="singlecrateview-split-type-and-anchor"></span>`fn split_type_and_anchor(path: &str) -> (&str, Option<&str>)`

- <span id="singlecrateview-build-markdown-link"></span>`fn build_markdown_link(&self, current_file: &str, target_crate: &str, target_path: &str, display_name: &str, anchor: Option<&str>) -> String`

- <span id="singlecrateview-compute-cross-crate-path"></span>`fn compute_cross_crate_path(current_local: &str, target_crate: &str, target_path: &str) -> String`

- <span id="singlecrateview-strip-crate-prefix"></span>`fn strip_crate_prefix(path: &str) -> &str`

- <span id="singlecrateview-looks-like-external-reference"></span>`fn looks_like_external_reference(link_text: &str) -> bool`

#### Trait Implementations

##### `impl Instrument for SingleCrateView<'a>`

##### `impl IntoEither for SingleCrateView<'a>`

##### `impl ItemAccess for SingleCrateView<'_>`

- <span id="singlecrateview-krate"></span>`fn krate(&self) -> &Crate`

- <span id="singlecrateview-crate-name"></span>`fn crate_name(&self) -> &str`

- <span id="singlecrateview-get-item"></span>`fn get_item(&self, id: &Id) -> Option<&Item>`

- <span id="singlecrateview-get-impls"></span>`fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

- <span id="singlecrateview-crate-version"></span>`fn crate_version(&self) -> Option<&str>`

- <span id="singlecrateview-render-config"></span>`fn render_config(&self) -> &RenderConfig` — [`RenderConfig`](../generator/config/index.md#renderconfig)

- <span id="singlecrateview-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](../generator/render_shared/index.md#sourcepathconfig)

##### `impl ItemFilter for SingleCrateView<'_>`

- <span id="singlecrateview-should-include-item"></span>`fn should_include_item(&self, item: &Item) -> bool`

- <span id="singlecrateview-include-private"></span>`fn include_private(&self) -> bool`

- <span id="singlecrateview-include-blanket-impls"></span>`fn include_blanket_impls(&self) -> bool`

##### `impl LinkResolver for SingleCrateView<'_>`

- <span id="singlecrateview-link-registry"></span>`fn link_registry(&self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../linker/index.md#linkregistry)

- <span id="singlecrateview-process-docs"></span>`fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

- <span id="singlecrateview-create-link"></span>`fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

##### `impl OwoColorize for SingleCrateView<'a>`

##### `impl Pointable for SingleCrateView<'a>`

- <span id="singlecrateview-pointable-const-align"></span>`const ALIGN: usize`

- <span id="singlecrateview-pointable-type-init"></span>`type Init = T`

- <span id="singlecrateview-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="singlecrateview-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="singlecrateview-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="singlecrateview-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl RenderContext for SingleCrateView<'a>`

##### `impl WithSubscriber for SingleCrateView<'a>`

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    ctx: crate::multi_crate::MultiCrateContext<'a>,
    args: &'a crate::Args,
}
```

*Defined in `src/multi_crate/generator.rs:416-422`*

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

- <span id="multicrategenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

- <span id="multicrategenerator-collect-rendered-items"></span>`fn collect_rendered_items(&self) -> HashMap<String, HashSet<Id>>`

- <span id="multicrategenerator-collect-crate-items"></span>`fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](context/index.md#singlecrateview)

- <span id="multicrategenerator-collect-module-items"></span>`fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](context/index.md#singlecrateview)

- <span id="multicrategenerator-generate-crate"></span>`fn generate_crate(&self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](context/index.md#singlecrateview), [`Error`](../error/index.md#error)

- <span id="multicrategenerator-generate-module"></span>`fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](context/index.md#singlecrateview), [`Error`](../error/index.md#error)

- <span id="multicrategenerator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../error/index.md#error)

#### Trait Implementations

##### `impl Instrument for MultiCrateGenerator<'a>`

##### `impl IntoEither for MultiCrateGenerator<'a>`

##### `impl OwoColorize for MultiCrateGenerator<'a>`

##### `impl Pointable for MultiCrateGenerator<'a>`

- <span id="multicrategenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrategenerator-pointable-type-init"></span>`type Init = T`

- <span id="multicrategenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrategenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrategenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrategenerator-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="multicrateparser-extract-crate-name"></span>`fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](../error/index.md#error)

#### Trait Implementations

##### `impl Instrument for MultiCrateParser`

##### `impl IntoEither for MultiCrateParser`

##### `impl OwoColorize for MultiCrateParser`

##### `impl Pointable for MultiCrateParser`

- <span id="multicrateparser-pointable-const-align"></span>`const ALIGN: usize`

- <span id="multicrateparser-pointable-type-init"></span>`type Init = T`

- <span id="multicrateparser-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="multicrateparser-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="multicrateparser-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="multicrateparser-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="unifiedlinkregistry-register-crate"></span>`fn register_crate(&mut self, crate_name: &str, krate: &Crate)`

- <span id="unifiedlinkregistry-register-from-paths"></span>`fn register_from_paths(&mut self, crate_name: &str, krate: &Crate)`

- <span id="unifiedlinkregistry-item-enum-to-kind"></span>`const fn item_enum_to_kind(inner: &ItemEnum) -> ItemKind`

- <span id="unifiedlinkregistry-register-item-recursive"></span>`fn register_item_recursive(&mut self, krate: &Crate, crate_name: &str, item_id: Id, item: &rustdoc_types::Item, parent_path: &str)`

- <span id="unifiedlinkregistry-register-item"></span>`fn register_item(&mut self, crate_name: &str, id: Id, name: &str, path: &str, kind: ItemKind)`

- <span id="unifiedlinkregistry-get-path"></span>`fn get_path(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- <span id="unifiedlinkregistry-get-name"></span>`fn get_name(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- <span id="unifiedlinkregistry-get-re-export-source"></span>`fn get_re_export_source(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- <span id="unifiedlinkregistry-resolve-reexport"></span>`fn resolve_reexport(&self, crate_name: &str, id: Id) -> Option<(compact_str::CompactString, Id)>`

- <span id="unifiedlinkregistry-resolve-name"></span>`fn resolve_name(&self, name: &str, current_crate: &str) -> Option<(compact_str::CompactString, Id)>`

- <span id="unifiedlinkregistry-resolve-path"></span>`fn resolve_path(&self, path: &str) -> Option<(compact_str::CompactString, Id)>`

- <span id="unifiedlinkregistry-create-link"></span>`fn create_link(&self, from_crate: &str, from_path: &str, to_crate: &str, to_id: Id) -> Option<String>`

- <span id="unifiedlinkregistry-compute-cross-crate-path"></span>`fn compute_cross_crate_path(from: &str, to: &str) -> String`

- <span id="unifiedlinkregistry-get-anchor"></span>`fn get_anchor(&self, crate_name: &str, id: Id) -> Option<String>`

- <span id="unifiedlinkregistry-contains"></span>`fn contains(&self, crate_name: &str, id: Id) -> bool`

- <span id="unifiedlinkregistry-len"></span>`fn len(&self) -> usize`

- <span id="unifiedlinkregistry-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Debug for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-default"></span>`fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](registry/index.md#unifiedlinkregistry)

##### `impl Instrument for UnifiedLinkRegistry`

##### `impl IntoEither for UnifiedLinkRegistry`

##### `impl OwoColorize for UnifiedLinkRegistry`

##### `impl Pointable for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="unifiedlinkregistry-pointable-type-init"></span>`type Init = T`

- <span id="unifiedlinkregistry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unifiedlinkregistry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unifiedlinkregistry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unifiedlinkregistry-drop"></span>`unsafe fn drop(ptr: usize)`

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

##### `impl Debug for SearchIndex`

- <span id="searchindex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for SearchIndex`

##### `impl IntoEither for SearchIndex`

##### `impl OwoColorize for SearchIndex`

##### `impl Pointable for SearchIndex`

- <span id="searchindex-pointable-const-align"></span>`const ALIGN: usize`

- <span id="searchindex-pointable-type-init"></span>`type Init = T`

- <span id="searchindex-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindex-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindex-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindex-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Serialize for SearchIndex`

- <span id="searchindex-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

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

- <span id="searchindexgenerator-generate"></span>`fn generate(&self) -> SearchIndex` — [`SearchIndex`](search/index.md#searchindex)

- <span id="searchindexgenerator-write"></span>`fn write(&self, output_dir: &Path) -> std::io::Result<()>`

- <span id="searchindexgenerator-index-crate"></span>`fn index_crate(&self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](search/index.md#searchentry)

- <span id="searchindexgenerator-build-path-map"></span>`fn build_path_map(krate: &Crate) -> HashMap<Id, String>`

- <span id="searchindexgenerator-compute-file-path"></span>`fn compute_file_path(crate_name: &str, module_path: &str, kind: &str) -> String`

#### Trait Implementations

##### `impl Instrument for SearchIndexGenerator<'a>`

##### `impl IntoEither for SearchIndexGenerator<'a>`

##### `impl OwoColorize for SearchIndexGenerator<'a>`

##### `impl Pointable for SearchIndexGenerator<'a>`

- <span id="searchindexgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="searchindexgenerator-pointable-type-init"></span>`type Init = T`

- <span id="searchindexgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="searchindexgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="searchindexgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="searchindexgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="summarygenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../error/index.md#error)

- <span id="summarygenerator-add-modules"></span>`fn add_modules(&self, content: &mut String, krate: &rustdoc_types::Crate, items: &[rustdoc_types::Id], path_prefix: &str, indent: usize)`

#### Trait Implementations

##### `impl Instrument for SummaryGenerator<'a>`

##### `impl IntoEither for SummaryGenerator<'a>`

##### `impl OwoColorize for SummaryGenerator<'a>`

##### `impl Pointable for SummaryGenerator<'a>`

- <span id="summarygenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="summarygenerator-pointable-type-init"></span>`type Init = T`

- <span id="summarygenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="summarygenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="summarygenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="summarygenerator-drop"></span>`unsafe fn drop(ptr: usize)`

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

