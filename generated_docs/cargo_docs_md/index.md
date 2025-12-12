# Crate `cargo_docs_md`

docs-md library interface for testing and reuse.

This module exposes the core functionality of docs-md as a library,
allowing integration tests and external tools to use the markdown
generation capabilities programmatically.

## Contents

- [Modules](#modules)
  - [`error`](#error)
  - [`generator`](#generator)
  - [`linker`](#linker)
  - [`multi_crate`](#multi-crate)
  - [`parser`](#parser)
  - [`types`](#types)
  - [`utils`](#utils)
- [Structs](#structs)
  - [`Generator`](#generator)
  - [`MarkdownCapture`](#markdowncapture)
  - [`RenderConfig`](#renderconfig)
  - [`SourceConfig`](#sourceconfig)
  - [`AnchorUtils`](#anchorutils)
  - [`LinkRegistry`](#linkregistry)
  - [`CrateCollection`](#cratecollection)
  - [`MultiCrateContext`](#multicratecontext)
  - [`MultiCrateGenerator`](#multicrategenerator)
  - [`MultiCrateParser`](#multicrateparser)
  - [`SearchIndex`](#searchindex)
  - [`SearchIndexGenerator`](#searchindexgenerator)
  - [`UnifiedLinkRegistry`](#unifiedlinkregistry)
  - [`Cli`](#cli)
  - [`DocsArgs`](#docsargs)
  - [`GenerateArgs`](#generateargs)
- [Enums](#enums)
  - [`OutputFormat`](#outputformat)
  - [`Cargo`](#cargo)
  - [`Command`](#command)
  - [`CliOutputFormat`](#clioutputformat)
- [Type Aliases](#type-aliases)
  - [`Args`](#args)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`error`](#error) | mod | Error types for docs-md. |
| [`generator`](#generator) | mod | Markdown documentation generator for rustdoc JSON. |
| [`linker`](#linker) | mod | Cross-reference linking for markdown documentation. |
| [`multi_crate`](#multi-crate) | mod | Multi-crate documentation generation. |
| [`parser`](#parser) | mod | Rustdoc JSON parsing module. |
| [`types`](#types) | mod | Type rendering utilities for converting rustdoc types to string representations. |
| [`utils`](#utils) | mod | Shared utility functions used across the documentation generator. |
| [`Generator`](#generator) | struct |  |
| [`MarkdownCapture`](#markdowncapture) | struct |  |
| [`RenderConfig`](#renderconfig) | struct |  |
| [`SourceConfig`](#sourceconfig) | struct |  |
| [`AnchorUtils`](#anchorutils) | struct |  |
| [`LinkRegistry`](#linkregistry) | struct |  |
| [`CrateCollection`](#cratecollection) | struct |  |
| [`MultiCrateContext`](#multicratecontext) | struct |  |
| [`MultiCrateGenerator`](#multicrategenerator) | struct |  |
| [`MultiCrateParser`](#multicrateparser) | struct |  |
| [`SearchIndex`](#searchindex) | struct |  |
| [`SearchIndexGenerator`](#searchindexgenerator) | struct |  |
| [`UnifiedLinkRegistry`](#unifiedlinkregistry) | struct |  |
| [`Cli`](#cli) | struct | Top-level CLI for docs-md. |
| [`DocsArgs`](#docsargs) | struct | Arguments for the `docs` subcommand (build + generate). |
| [`GenerateArgs`](#generateargs) | struct | Command-line arguments for direct generation (no subcommand). |
| [`OutputFormat`](#outputformat) | enum | Output format for the generated markdown documentation. |
| [`Cargo`](#cargo) | enum | Cargo wrapper for subcommand invocation. |
| [`Command`](#command) | enum | Available subcommands |
| [`CliOutputFormat`](#clioutputformat) | enum | CLI-compatible output format enum (for clap `ValueEnum` derive). |
| [`Args`](#args) | type | Backwards-compatible type alias for existing code. |

## Modules

- [`error`](error/index.md) — Error types for docs-md.
- [`generator`](generator/index.md) — Markdown documentation generator for rustdoc JSON.
- [`linker`](linker/index.md) — Cross-reference linking for markdown documentation.
- [`multi_crate`](multi_crate/index.md) — Multi-crate documentation generation.
- [`parser`](parser/index.md) — Rustdoc JSON parsing module.
- [`types`](types/index.md) — Type rendering utilities for converting rustdoc types to string representations.
- [`utils`](utils/index.md) — Shared utility functions used across the documentation generator.

## Structs

### `Generator<'a>`

```rust
struct Generator<'a> {
    ctx: GeneratorContext<'a>,
    args: &'a crate::Args,
    root_item: &'a rustdoc_types::Item,
}
```

*Defined in `src/generator/mod.rs:86-95`*

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

- <span id="generator-new"></span>`fn new(krate: &'a Crate, args: &'a Args, config: RenderConfig) -> Result<Self, Error>` — [`Args`](#args), [`RenderConfig`](generator/config/index.md#renderconfig), [`Error`](error/index.md#error)

- <span id="generator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](error/index.md#error)

- <span id="generator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](error/index.md#error)

- <span id="generator-generate-to-capture"></span>`fn generate_to_capture(krate: &Crate, format: CliOutputFormat, include_private: bool) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](#clioutputformat), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

- <span id="generator-generate-to-capture-with-config"></span>`fn generate_to_capture_with_config(krate: &Crate, format: CliOutputFormat, include_private: bool, config: RenderConfig) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](#clioutputformat), [`RenderConfig`](generator/config/index.md#renderconfig), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

- <span id="generator-generate-flat-to-capture"></span>`fn generate_flat_to_capture(ctx: &GeneratorContext<'_>, root: &Item, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/context/index.md#generatorcontext), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

- <span id="generator-generate-flat-recursive-capture"></span>`fn generate_flat_recursive_capture(ctx: &GeneratorContext<'_>, item: &Item, prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/context/index.md#generatorcontext), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

- <span id="generator-generate-nested-to-capture"></span>`fn generate_nested_to_capture(ctx: &GeneratorContext<'_>, root: &Item, path_prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/context/index.md#generatorcontext), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

- <span id="generator-run"></span>`fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error>` — [`Args`](#args), [`Error`](error/index.md#error)

#### Trait Implementations

##### `impl Instrument for Generator<'a>`

##### `impl IntoEither for Generator<'a>`

##### `impl OwoColorize for Generator<'a>`

##### `impl Pointable for Generator<'a>`

- <span id="generator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="generator-pointable-type-init"></span>`type Init = T`

- <span id="generator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for Generator<'a>`

### `MarkdownCapture`

```rust
struct MarkdownCapture {
    files: std::collections::HashMap<String, String>,
}
```

*Defined in `src/generator/capture.rs:15-18`*

Captures generated markdown in memory for testing.

Instead of writing files to disk, this struct stores all generated
markdown content in a `HashMap`, keyed by relative file path. This
enables snapshot testing and verification of output without filesystem
side effects.

#### Fields

- **`files`**: `std::collections::HashMap<String, String>`

  Maps file paths (relative to output directory) to their generated content.

#### Implementations

- <span id="markdowncapture-new"></span>`fn new() -> Self`

- <span id="markdowncapture-insert"></span>`fn insert(&mut self, path: String, content: String)`

- <span id="markdowncapture-get"></span>`fn get(&self, path: &str) -> Option<&String>`

- <span id="markdowncapture-paths"></span>`fn paths(&self) -> Vec<&String>`

- <span id="markdowncapture-len"></span>`fn len(&self) -> usize`

- <span id="markdowncapture-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="markdowncapture-to-snapshot-string"></span>`fn to_snapshot_string(&self) -> String`

- <span id="markdowncapture-into-inner"></span>`fn into_inner(self) -> HashMap<String, String>`

#### Trait Implementations

##### `impl Debug for MarkdownCapture`

- <span id="markdowncapture-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MarkdownCapture`

- <span id="markdowncapture-default"></span>`fn default() -> MarkdownCapture` — [`MarkdownCapture`](generator/capture/index.md#markdowncapture)

##### `impl Instrument for MarkdownCapture`

##### `impl IntoEither for MarkdownCapture`

##### `impl OwoColorize for MarkdownCapture`

##### `impl Pointable for MarkdownCapture`

- <span id="markdowncapture-pointable-const-align"></span>`const ALIGN: usize`

- <span id="markdowncapture-pointable-type-init"></span>`type Init = T`

- <span id="markdowncapture-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="markdowncapture-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="markdowncapture-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="markdowncapture-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for MarkdownCapture`

### `RenderConfig`

```rust
struct RenderConfig {
    pub toc_threshold: usize,
    pub quick_reference: bool,
    pub group_impls: bool,
    pub hide_trivial_derives: bool,
    pub method_anchors: bool,
    pub include_source: SourceConfig,
}
```

*Defined in `src/generator/config.rs:14-32`*

Configuration options for markdown rendering.

#### Fields

- **`toc_threshold`**: `usize`

  Generate table of contents for modules with more than this many items.

- **`quick_reference`**: `bool`

  Generate quick reference tables at the top of modules.

- **`group_impls`**: `bool`

  Group impl blocks by category (Derive, Conversion, Iterator, etc.).

- **`hide_trivial_derives`**: `bool`

  Hide trivial derive implementations (Clone, Copy, Debug, etc.).

- **`method_anchors`**: `bool`

  Generate method-level anchors for deep linking.

- **`include_source`**: `SourceConfig`

  Source code integration options.

#### Trait Implementations

##### `impl Clone for RenderConfig`

- <span id="renderconfig-clone"></span>`fn clone(&self) -> RenderConfig` — [`RenderConfig`](generator/config/index.md#renderconfig)

##### `impl Debug for RenderConfig`

- <span id="renderconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RenderConfig`

- <span id="renderconfig-default"></span>`fn default() -> Self`

##### `impl Instrument for RenderConfig`

##### `impl IntoEither for RenderConfig`

##### `impl OwoColorize for RenderConfig`

##### `impl Pointable for RenderConfig`

- <span id="renderconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="renderconfig-pointable-type-init"></span>`type Init = T`

- <span id="renderconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="renderconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="renderconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="renderconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for RenderConfig`

### `SourceConfig`

```rust
struct SourceConfig {
    pub function_bodies: bool,
    pub const_values: bool,
    pub private_items: bool,
    pub source_locations: bool,
    pub source_dir: Option<std::path::PathBuf>,
}
```

*Defined in `src/generator/config.rs:42-61`*

Configuration for source code integration.

Requires the `source-parsing` feature to have any effect.

#### Fields

- **`function_bodies`**: `bool`

  Include function bodies in collapsible sections.

- **`const_values`**: `bool`

  Show actual values for constants and statics.

- **`private_items`**: `bool`

  Include private items in a separate section.

- **`source_locations`**: `bool`

  Add <file:line> references to items.

- **`source_dir`**: `Option<std::path::PathBuf>`

  Path to the `.source_*` directory containing collected dependency sources.
  
  When set, source location references will use paths relative to this directory
  and generate clickable links. When `None`, absolute paths from rustdoc JSON
  are displayed without links.

#### Trait Implementations

##### `impl Clone for SourceConfig`

- <span id="sourceconfig-clone"></span>`fn clone(&self) -> SourceConfig` — [`SourceConfig`](generator/config/index.md#sourceconfig)

##### `impl Debug for SourceConfig`

- <span id="sourceconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceConfig`

- <span id="sourceconfig-default"></span>`fn default() -> SourceConfig` — [`SourceConfig`](generator/config/index.md#sourceconfig)

##### `impl Instrument for SourceConfig`

##### `impl IntoEither for SourceConfig`

##### `impl OwoColorize for SourceConfig`

##### `impl Pointable for SourceConfig`

- <span id="sourceconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourceconfig-pointable-type-init"></span>`type Init = T`

- <span id="sourceconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourceConfig`

### `AnchorUtils`

```rust
struct AnchorUtils;
```

*Defined in `src/linker.rs:53`*

Utilify functions to handle anchors

#### Implementations

- <span id="anchorutils-assoc-item-anchor"></span>`fn assoc_item_anchor(type_name: &str, item_name: &str, kind: AssocItemKind) -> String` — [`AssocItemKind`](linker/index.md#associtemkind)

- <span id="anchorutils-method-anchor"></span>`fn method_anchor(type_name: &str, method_name: &str) -> String`

- <span id="anchorutils-slugify-anchor"></span>`fn slugify_anchor(name: &str) -> String`

- <span id="anchorutils-slugify-anchor-ascii"></span>`fn slugify_anchor_ascii(name: &str) -> String`

- <span id="anchorutils-slugify-anchor-impl"></span>`fn slugify_anchor_impl(name: &str) -> String`

- <span id="anchorutils-item-has-anchor"></span>`const fn item_has_anchor(kind: ItemKind) -> bool`

#### Trait Implementations

##### `impl Instrument for AnchorUtils`

##### `impl IntoEither for AnchorUtils`

##### `impl OwoColorize for AnchorUtils`

##### `impl Pointable for AnchorUtils`

- <span id="anchorutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="anchorutils-pointable-type-init"></span>`type Init = T`

- <span id="anchorutils-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="anchorutils-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="anchorutils-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="anchorutils-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for AnchorUtils`

### `LinkRegistry`

```rust
struct LinkRegistry {
    item_paths: std::collections::HashMap<rustdoc_types::Id, String>,
    item_names: std::collections::HashMap<rustdoc_types::Id, String>,
}
```

*Defined in `src/linker.rs:276-288`*

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

- <span id="linkregistry-build"></span>`fn build(krate: &Crate, flat_format: bool, include_private: bool) -> Self`

- <span id="linkregistry-register-module-items"></span>`fn register_module_items(&mut self, krate: &Crate, module_id: Id, module_item: &rustdoc_types::Item, path: &str, module_prefix: &str, flat_format: bool, include_private: bool)`

- <span id="linkregistry-register-glob-items"></span>`fn register_glob_items(&mut self, krate: &Crate, use_item: &rustdoc_types::Use, path: &str, include_private: bool)`

- <span id="linkregistry-get-path"></span>`fn get_path(&self, id: Id) -> Option<&String>`

- <span id="linkregistry-get-name"></span>`fn get_name(&self, id: Id) -> Option<&String>`

- <span id="linkregistry-create-link"></span>`fn create_link(&self, id: Id, from_path: &str) -> Option<String>`

- <span id="linkregistry-compute-relative-path"></span>`fn compute_relative_path(from: &str, to: &str) -> String`

#### Trait Implementations

##### `impl Debug for LinkRegistry`

- <span id="linkregistry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LinkRegistry`

- <span id="linkregistry-default"></span>`fn default() -> LinkRegistry` — [`LinkRegistry`](linker/index.md#linkregistry)

##### `impl Instrument for LinkRegistry`

##### `impl IntoEither for LinkRegistry`

##### `impl OwoColorize for LinkRegistry`

##### `impl Pointable for LinkRegistry`

- <span id="linkregistry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="linkregistry-pointable-type-init"></span>`type Init = T`

- <span id="linkregistry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="linkregistry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="linkregistry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="linkregistry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for LinkRegistry`

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

- <span id="cratecollection-default"></span>`fn default() -> CrateCollection` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

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
CLI configuration. Used by [`MultiCrateGenerator`](multi_crate/generator/index.md) to coordinate
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

- <span id="multicratecontext-new"></span>`fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection), [`Args`](#args), [`RenderConfig`](generator/config/index.md#renderconfig)

- <span id="multicratecontext-set-source-dir"></span>`fn set_source_dir(&mut self, source_dir: &Path)`

- <span id="multicratecontext-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](generator/render_shared/index.md#sourcepathconfig)

- <span id="multicratecontext-build-cross-crate-impls"></span>`fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

- <span id="multicratecontext-crates"></span>`const fn crates(&self) -> &CrateCollection` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

- <span id="multicratecontext-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](multi_crate/registry/index.md#unifiedlinkregistry)

- <span id="multicratecontext-args"></span>`const fn args(&self) -> &Args` — [`Args`](#args)

- <span id="multicratecontext-single-crate-view"></span>`fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview)

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

- <span id="multicrategenerator-new"></span>`fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection), [`Args`](#args), [`RenderConfig`](generator/config/index.md#renderconfig)

- <span id="multicrategenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](error/index.md#error)

- <span id="multicrategenerator-collect-rendered-items"></span>`fn collect_rendered_items(&self) -> HashMap<String, HashSet<Id>>`

- <span id="multicrategenerator-collect-crate-items"></span>`fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview)

- <span id="multicrategenerator-collect-module-items"></span>`fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview)

- <span id="multicrategenerator-generate-crate"></span>`fn generate_crate(&self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview), [`Error`](error/index.md#error)

- <span id="multicrategenerator-generate-module"></span>`fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview), [`Error`](error/index.md#error)

- <span id="multicrategenerator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](error/index.md#error)

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

- <span id="multicrateparser-parse-directory"></span>`fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection), [`Error`](error/index.md#error)

- <span id="multicrateparser-extract-crate-name"></span>`fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](error/index.md#error)

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

Traverses all crates in a [`CrateCollection`](multi_crate/collection/index.md) and builds a comprehensive
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

- <span id="searchindexgenerator-new"></span>`const fn new(crates: &'a CrateCollection, include_private: bool, rendered_items: HashMap<String, HashSet<Id>>) -> Self` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

- <span id="searchindexgenerator-generate"></span>`fn generate(&self) -> SearchIndex` — [`SearchIndex`](multi_crate/search/index.md#searchindex)

- <span id="searchindexgenerator-write"></span>`fn write(&self, output_dir: &Path) -> std::io::Result<()>`

- <span id="searchindexgenerator-index-crate"></span>`fn index_crate(&self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](multi_crate/search/index.md#searchentry)

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

Unlike [`LinkRegistry`](linker/index.md) which handles a single crate, this registry
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

- <span id="unifiedlinkregistry-build"></span>`fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

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

- <span id="unifiedlinkregistry-default"></span>`fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](multi_crate/registry/index.md#unifiedlinkregistry)

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

### `Cli`

```rust
struct Cli {
    pub command: Option<Command>,
    pub args: GenerateArgs,
}
```

*Defined in `src/lib.rs:75-99`*

Top-level CLI for docs-md.

#### Fields

- **`command`**: `Option<Command>`

  Subcommand to run

- **`args`**: `GenerateArgs`

  Generation options (used when no subcommand is specified)

#### Trait Implementations

##### `impl Args for Cli`

- <span id="cli-group-id"></span>`fn group_id() -> Option<clap::Id>`

- <span id="cli-augment-args"></span>`fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="cli-augment-args-for-update"></span>`fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl CommandFactory for Cli`

- <span id="cli-command"></span>`fn command<'b>() -> clap::Command`

- <span id="cli-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for Cli`

- <span id="cli-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromArgMatches for Cli`

- <span id="cli-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cli-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cli-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="cli-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for Cli`

##### `impl IntoEither for Cli`

##### `impl OwoColorize for Cli`

##### `impl Parser for Cli`

##### `impl Pointable for Cli`

- <span id="cli-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cli-pointable-type-init"></span>`type Init = T`

- <span id="cli-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cli-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cli-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cli-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for Cli`

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

*Defined in `src/lib.rs:128-173`*

Arguments for the `docs` subcommand (build + generate).

#### Fields

- **`output`**: `std::path::PathBuf`

  Output directory for generated markdown files.
  
  Defaults to `generated_docs/` in the current directory.

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

- <span id="docsargs-group-id"></span>`fn group_id() -> Option<clap::Id>`

- <span id="docsargs-augment-args"></span>`fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="docsargs-augment-args-for-update"></span>`fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl CommandFactory for DocsArgs`

- <span id="docsargs-command"></span>`fn command<'b>() -> clap::Command`

- <span id="docsargs-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for DocsArgs`

- <span id="docsargs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromArgMatches for DocsArgs`

- <span id="docsargs-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="docsargs-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="docsargs-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="docsargs-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for DocsArgs`

##### `impl IntoEither for DocsArgs`

##### `impl OwoColorize for DocsArgs`

##### `impl Parser for DocsArgs`

##### `impl Pointable for DocsArgs`

- <span id="docsargs-pointable-const-align"></span>`const ALIGN: usize`

- <span id="docsargs-pointable-type-init"></span>`type Init = T`

- <span id="docsargs-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="docsargs-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="docsargs-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="docsargs-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for DocsArgs`

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

*Defined in `src/lib.rs:210-297`*

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
  Defaults to `generated_docs/` in the current directory.

- **`format`**: `CliOutputFormat`

  Output format (flat or nested).
  
  - `flat`: All files in one directory
  - `nested`: Directory hierarchy mirroring modules (default)

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

- <span id="generateargs-group-id"></span>`fn group_id() -> Option<clap::Id>`

- <span id="generateargs-augment-args"></span>`fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="generateargs-augment-args-for-update"></span>`fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl CommandFactory for GenerateArgs`

- <span id="generateargs-command"></span>`fn command<'b>() -> clap::Command`

- <span id="generateargs-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for GenerateArgs`

- <span id="generateargs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GenerateArgs`

- <span id="generateargs-default"></span>`fn default() -> GenerateArgs` — [`GenerateArgs`](#generateargs)

##### `impl FromArgMatches for GenerateArgs`

- <span id="generateargs-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="generateargs-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="generateargs-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="generateargs-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for GenerateArgs`

##### `impl IntoEither for GenerateArgs`

##### `impl OwoColorize for GenerateArgs`

##### `impl Parser for GenerateArgs`

##### `impl Pointable for GenerateArgs`

- <span id="generateargs-pointable-const-align"></span>`const ALIGN: usize`

- <span id="generateargs-pointable-type-init"></span>`type Init = T`

- <span id="generateargs-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generateargs-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generateargs-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generateargs-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for GenerateArgs`

## Enums

### `OutputFormat`

```rust
enum OutputFormat {
    Flat,
    Nested,
}
```

*Defined in `src/lib.rs:40-53`*

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

- <span id="outputformat-clone"></span>`fn clone(&self) -> OutputFormat` — [`OutputFormat`](#outputformat)

##### `impl Copy for OutputFormat`

##### `impl Debug for OutputFormat`

- <span id="outputformat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OutputFormat`

- <span id="outputformat-default"></span>`fn default() -> OutputFormat` — [`OutputFormat`](#outputformat)

##### `impl Instrument for OutputFormat`

##### `impl IntoEither for OutputFormat`

##### `impl OwoColorize for OutputFormat`

##### `impl Pointable for OutputFormat`

- <span id="outputformat-pointable-const-align"></span>`const ALIGN: usize`

- <span id="outputformat-pointable-type-init"></span>`type Init = T`

- <span id="outputformat-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="outputformat-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="outputformat-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="outputformat-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ValueEnum for OutputFormat`

- <span id="outputformat-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="outputformat-to-possible-value"></span>`fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl WithSubscriber for OutputFormat`

### `Cargo`

```rust
enum Cargo {
    DocsMd(Cli),
}
```

*Defined in `src/lib.rs:61-65`*

Cargo wrapper for subcommand invocation.

When invoked as `cargo docs-md`, cargo passes "docs-md" as the first argument.
This wrapper handles that by making `docs-md` a subcommand that contains the real CLI.

#### Variants

- **`DocsMd`**

  Generate per-module markdown from rustdoc JSON

#### Trait Implementations

##### `impl CommandFactory for Cargo`

- <span id="cargo-command"></span>`fn command<'b>() -> clap::Command`

- <span id="cargo-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for Cargo`

- <span id="cargo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromArgMatches for Cargo`

- <span id="cargo-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cargo-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cargo-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="cargo-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut<'b>(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for Cargo`

##### `impl IntoEither for Cargo`

##### `impl OwoColorize for Cargo`

##### `impl Parser for Cargo`

##### `impl Pointable for Cargo`

- <span id="cargo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cargo-pointable-type-init"></span>`type Init = T`

- <span id="cargo-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cargo-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cargo-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cargo-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Subcommand for Cargo`

- <span id="cargo-augment-subcommands"></span>`fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="cargo-augment-subcommands-for-update"></span>`fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="cargo-has-subcommand"></span>`fn has_subcommand(__clap_name: &str) -> bool`

##### `impl WithSubscriber for Cargo`

### `Command`

```rust
enum Command {
    Docs(DocsArgs),
}
```

*Defined in `src/lib.rs:103-120`*

Available subcommands

#### Variants

- **`Docs`**

  Build rustdoc JSON and generate markdown in one step.
  
  This runs `cargo +nightly doc` with JSON output, then generates
  markdown documentation from the result. Requires nightly toolchain.
  
  Example: `cargo docs-md docs --primary-crate my_crate`

#### Trait Implementations

##### `impl Debug for Command`

- <span id="command-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromArgMatches for Command`

- <span id="command-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="command-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="command-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="command-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut<'b>(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for Command`

##### `impl IntoEither for Command`

##### `impl OwoColorize for Command`

##### `impl Pointable for Command`

- <span id="command-pointable-const-align"></span>`const ALIGN: usize`

- <span id="command-pointable-type-init"></span>`type Init = T`

- <span id="command-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="command-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="command-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="command-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Subcommand for Command`

- <span id="command-augment-subcommands"></span>`fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="command-augment-subcommands-for-update"></span>`fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="command-has-subcommand"></span>`fn has_subcommand(__clap_name: &str) -> bool`

##### `impl WithSubscriber for Command`

### `CliOutputFormat`

```rust
enum CliOutputFormat {
    Flat,
    Nested,
}
```

*Defined in `src/lib.rs:304-311`*

CLI-compatible output format enum (for clap `ValueEnum` derive).

#### Variants

- **`Flat`**

  Flat structure with double-underscore separators in filenames.

- **`Nested`**

  Nested directory structure mirroring the module hierarchy.

#### Trait Implementations

##### `impl Clone for CliOutputFormat`

- <span id="clioutputformat-clone"></span>`fn clone(&self) -> CliOutputFormat` — [`CliOutputFormat`](#clioutputformat)

##### `impl Copy for CliOutputFormat`

##### `impl Debug for CliOutputFormat`

- <span id="clioutputformat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CliOutputFormat`

- <span id="clioutputformat-default"></span>`fn default() -> CliOutputFormat` — [`CliOutputFormat`](#clioutputformat)

##### `impl Instrument for CliOutputFormat`

##### `impl IntoEither for CliOutputFormat`

##### `impl OwoColorize for CliOutputFormat`

##### `impl Pointable for CliOutputFormat`

- <span id="clioutputformat-pointable-const-align"></span>`const ALIGN: usize`

- <span id="clioutputformat-pointable-type-init"></span>`type Init = T`

- <span id="clioutputformat-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clioutputformat-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clioutputformat-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clioutputformat-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ValueEnum for CliOutputFormat`

- <span id="clioutputformat-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="clioutputformat-to-possible-value"></span>`fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl WithSubscriber for CliOutputFormat`

## Type Aliases

### `Args`

```rust
type Args = GenerateArgs;
```

*Defined in `src/lib.rs:300`*

Backwards-compatible type alias for existing code.

