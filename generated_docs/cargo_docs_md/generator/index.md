*[cargo_docs_md](../index.md) / [generator](index.md)*

---

# Module `generator`

Markdown documentation generator for rustdoc JSON.

This is the core module that transforms rustdoc JSON data into markdown files.
It handles the complete generation pipeline: traversing modules, rendering
different item types, and creating cross-reference links.

# Architecture

The generation process follows these steps:

1. **Setup**: Create output directory, build path and impl maps
2. **Link Registry**: Build a registry mapping item IDs to file paths
3. **Generation**: Recursively traverse modules and write markdown files

# Module Structure

- [`context`](context/index.md) - Shared state for generation (crate data, maps, config)
- [`module`](module/index.md) - Module-level markdown rendering
- [`items`](items/index.md) - Individual item rendering (structs, enums, traits, etc.)
- [`impls`](impls/index.md) - Implementation block rendering
- [`flat`](flat/index.md) - Flat output format generator
- [`nested`](nested/index.md) - Nested output format generator

# Output Formats

Two output formats are supported:

- **Flat**: All files in one directory (`module.md`, `parent__child.md`)
- **Nested**: Directory hierarchy (`module/index.md`, `parent/child/index.md`)

# Usage

```ignore
use docs_md::generator::Generator;

let generator = Generator::new(&krate, &args)?;
generator.generate()?;
```

## Contents

- [Modules](#modules)
  - [`breadcrumbs`](#breadcrumbs)
  - [`capture`](#capture)
  - [`context`](#context)
  - [`doc_links`](#doc_links)
  - [`flat`](#flat)
  - [`impl_category`](#impl_category)
  - [`impls`](#impls)
  - [`items`](#items)
  - [`module`](#module)
  - [`nested`](#nested)
  - [`quick_ref`](#quick_ref)
  - [`render_shared`](#render_shared)
  - [`toc`](#toc)
  - [`config`](#config)
- [Structs](#structs)
  - [`BreadcrumbGenerator`](#breadcrumbgenerator)
  - [`MarkdownCapture`](#markdowncapture)
  - [`RenderConfig`](#renderconfig)
  - [`SourceConfig`](#sourceconfig)
  - [`GeneratorContext`](#generatorcontext)
  - [`DocLinkProcessor`](#doclinkprocessor)
  - [`ModuleRenderer`](#modulerenderer)
  - [`QuickRefEntry`](#quickrefentry)
  - [`QuickRefGenerator`](#quickrefgenerator)
  - [`TocEntry`](#tocentry)
  - [`TocGenerator`](#tocgenerator)
  - [`Generator`](#generator)
- [Enums](#enums)
  - [`ImplCategory`](#implcategory)
- [Traits](#traits)
  - [`ItemAccess`](#itemaccess)
  - [`ItemFilter`](#itemfilter)
  - [`LinkResolver`](#linkresolver)
  - [`RenderContext`](#rendercontext)
- [Functions](#functions)
  - [`convert_html_links`](#convert_html_links)
  - [`convert_path_reference_links`](#convert_path_reference_links)
  - [`strip_duplicate_title`](#strip_duplicate_title)
  - [`strip_reference_definitions`](#strip_reference_definitions)
  - [`extract_summary`](#extract_summary)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`breadcrumbs`](#breadcrumbs) | mod | Breadcrumb navigation generation for nested module pages. |
| [`capture`](#capture) | mod | In-memory markdown capture for testing. |
| [`context`](#context) | mod | Shared context for documentation generation. |
| [`doc_links`](#doc_links) | mod | Intra-doc link processing for documentation generation. |
| [`flat`](#flat) | mod | Flat format documentation generation. |
| [`impl_category`](#impl_category) | mod | Categorization system for trait implementations. |
| [`impls`](#impls) | mod | Implementation block rendering for documentation generation. |
| [`items`](#items) | mod | Item rendering for documentation generation. |
| [`module`](#module) | mod | Module markdown rendering for documentation generation. |
| [`nested`](#nested) | mod | Nested format documentation generation. |
| [`quick_ref`](#quick_ref) | mod | Quick Reference table generation for module documentation. |
| [`render_shared`](#render_shared) | mod | Shared rendering functions for documentation generation. |
| [`toc`](#toc) | mod | Table of Contents generation for module documentation. |
| [`config`](#config) | mod | Configuration for markdown rendering. |
| [`BreadcrumbGenerator`](#breadcrumbgenerator) | struct |  |
| [`MarkdownCapture`](#markdowncapture) | struct |  |
| [`RenderConfig`](#renderconfig) | struct |  |
| [`SourceConfig`](#sourceconfig) | struct |  |
| [`GeneratorContext`](#generatorcontext) | struct |  |
| [`DocLinkProcessor`](#doclinkprocessor) | struct |  |
| [`ModuleRenderer`](#modulerenderer) | struct |  |
| [`QuickRefEntry`](#quickrefentry) | struct |  |
| [`QuickRefGenerator`](#quickrefgenerator) | struct |  |
| [`TocEntry`](#tocentry) | struct |  |
| [`TocGenerator`](#tocgenerator) | struct |  |
| [`Generator`](#generator) | struct | Main documentation generator. |
| [`ImplCategory`](#implcategory) | enum |  |
| [`ItemAccess`](#itemaccess) | trait |  |
| [`ItemFilter`](#itemfilter) | trait |  |
| [`LinkResolver`](#linkresolver) | trait |  |
| [`RenderContext`](#rendercontext) | trait |  |
| [`convert_html_links`](#convert_html_links) | fn |  |
| [`convert_path_reference_links`](#convert_path_reference_links) | fn |  |
| [`strip_duplicate_title`](#strip_duplicate_title) | fn |  |
| [`strip_reference_definitions`](#strip_reference_definitions) | fn |  |
| [`extract_summary`](#extract_summary) | fn |  |

## Modules

- [`breadcrumbs`](breadcrumbs/index.md) — Breadcrumb navigation generation for nested module pages.
- [`capture`](capture/index.md) — In-memory markdown capture for testing.
- [`context`](context/index.md) — Shared context for documentation generation.
- [`doc_links`](doc_links/index.md) — Intra-doc link processing for documentation generation.
- [`flat`](flat/index.md) — Flat format documentation generation.
- [`impl_category`](impl_category/index.md) — Categorization system for trait implementations.
- [`impls`](impls/index.md) — Implementation block rendering for documentation generation.
- [`items`](items/index.md) — Item rendering for documentation generation.
- [`module`](module/index.md) — Module markdown rendering for documentation generation.
- [`nested`](nested/index.md) — Nested format documentation generation.
- [`quick_ref`](quick_ref/index.md) — Quick Reference table generation for module documentation.
- [`render_shared`](render_shared/index.md) — Shared rendering functions for documentation generation.
- [`toc`](toc/index.md) — Table of Contents generation for module documentation.
- [`config`](config/index.md) — Configuration for markdown rendering.

## Structs

### `BreadcrumbGenerator<'a>`

```rust
struct BreadcrumbGenerator<'a> {
    module_path: &'a [String],
    crate_name: &'a str,
}
```

*Defined in `src/generator/breadcrumbs.rs:10-16`*

Generates breadcrumb navigation for nested module pages.

Creates a navigation line showing the path from the crate root to
the current module, with each segment being a clickable link.

#### Fields

- **`module_path`**: `&'a [String]`

  The module path segments (e.g., `["error", "types"]`).

- **`crate_name`**: `&'a str`

  The name of the crate for the root link.

#### Implementations

- <span id="breadcrumbgenerator-new"></span>`const fn new(module_path: &'a [String], crate_name: &'a str) -> Self`

- <span id="breadcrumbgenerator-generate"></span>`fn generate(&self) -> String`

#### Trait Implementations

##### `impl Instrument for BreadcrumbGenerator<'a>`

##### `impl IntoEither for BreadcrumbGenerator<'a>`

##### `impl OwoColorize for BreadcrumbGenerator<'a>`

##### `impl Pointable for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-const-align"></span>`const ALIGN: usize`

- <span id="breadcrumbgenerator-type-init"></span>`type Init = T`

- <span id="breadcrumbgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="breadcrumbgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="breadcrumbgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="breadcrumbgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for BreadcrumbGenerator<'a>`

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

- <span id="markdowncapture-default"></span>`fn default() -> MarkdownCapture` — [`MarkdownCapture`](capture/index.md)

##### `impl Instrument for MarkdownCapture`

##### `impl IntoEither for MarkdownCapture`

##### `impl OwoColorize for MarkdownCapture`

##### `impl Pointable for MarkdownCapture`

- <span id="markdowncapture-const-align"></span>`const ALIGN: usize`

- <span id="markdowncapture-type-init"></span>`type Init = T`

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

- <span id="renderconfig-clone"></span>`fn clone(&self) -> RenderConfig` — [`RenderConfig`](config/index.md)

##### `impl Debug for RenderConfig`

- <span id="renderconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RenderConfig`

- <span id="renderconfig-default"></span>`fn default() -> Self`

##### `impl Instrument for RenderConfig`

##### `impl IntoEither for RenderConfig`

##### `impl OwoColorize for RenderConfig`

##### `impl Pointable for RenderConfig`

- <span id="renderconfig-const-align"></span>`const ALIGN: usize`

- <span id="renderconfig-type-init"></span>`type Init = T`

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

- <span id="sourceconfig-clone"></span>`fn clone(&self) -> SourceConfig` — [`SourceConfig`](config/index.md)

##### `impl Debug for SourceConfig`

- <span id="sourceconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceConfig`

- <span id="sourceconfig-default"></span>`fn default() -> SourceConfig` — [`SourceConfig`](config/index.md)

##### `impl Instrument for SourceConfig`

##### `impl IntoEither for SourceConfig`

##### `impl OwoColorize for SourceConfig`

##### `impl Pointable for SourceConfig`

- <span id="sourceconfig-const-align"></span>`const ALIGN: usize`

- <span id="sourceconfig-type-init"></span>`type Init = T`

- <span id="sourceconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for SourceConfig`

### `GeneratorContext<'a>`

```rust
struct GeneratorContext<'a> {
    pub krate: &'a rustdoc_types::Crate,
    crate_name: String,
    pub impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    pub link_registry: crate::linker::LinkRegistry,
    pub args: &'a crate::Args,
    pub config: crate::generator::config::RenderConfig,
    path_name_index: std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>,
    source_path_config: Option<crate::generator::render_shared::SourcePathConfig>,
}
```

*Defined in `src/generator/context.rs:135-168`*

Shared context containing all data needed for documentation generation.

This struct is passed to all rendering components and provides:
- Access to the parsed crate data
- Impl block lookup for rendering implementations
- Link registry for cross-references
- CLI configuration options

#### Fields

- **`krate`**: `&'a rustdoc_types::Crate`

  The parsed rustdoc JSON crate.

- **`crate_name`**: `String`

  The crate name (extracted from root module).

- **`impl_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>`

  Maps type IDs to all impl blocks for that type.
  
  Used for rendering "Implementations" and "Trait Implementations" sections.

- **`link_registry`**: `crate::linker::LinkRegistry`

  Registry for creating cross-reference links between items.

- **`args`**: `&'a crate::Args`

  CLI arguments containing output path, format, and options.

- **`config`**: `crate::generator::config::RenderConfig`

  Rendering configuration options.

- **`path_name_index`**: `std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>`

  Pre-built index mapping item names to their IDs for fast lookup.
  
  Built once at construction time from `krate.paths` and shared across
  all `DocLinkProcessor` instances for efficiency.

- **`source_path_config`**: `Option<crate::generator::render_shared::SourcePathConfig>`

  Base source path configuration for transforming cargo registry paths.
  
  `None` if source locations are disabled or no `.source_*` dir detected.
  The `depth` field is set to 0; use `source_path_config_for_file()` to
  get a config with the correct depth for a specific file.

#### Implementations

- <span id="generatorcontext-new"></span>`fn new(krate: &'a Crate, args: &'a Args, config: RenderConfig) -> Self` — [`Args`](../index.md), [`RenderConfig`](config/index.md)

- <span id="generatorcontext-set-source-dir"></span>`fn set_source_dir(&mut self, source_dir: &Path)`

- <span id="generatorcontext-build-impl-map"></span>`fn build_impl_map(krate: &'a Crate) -> HashMap<Id, Vec<&'a Impl>>`

- <span id="generatorcontext-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- <span id="generatorcontext-should-include-item"></span>`const fn should_include_item(&self, item: &Item) -> bool`

- <span id="generatorcontext-count-modules"></span>`fn count_modules(&self, item: &Item) -> usize`

- <span id="generatorcontext-build-path-name-index"></span>`fn build_path_name_index(krate: &'a Crate) -> HashMap<&'a str, Vec<Id>>`

#### Trait Implementations

##### `impl Instrument for GeneratorContext<'a>`

##### `impl IntoEither for GeneratorContext<'a>`

##### `impl ItemAccess for GeneratorContext<'_>`

- <span id="generatorcontext-krate"></span>`fn krate(&self) -> &Crate`

- <span id="generatorcontext-crate-name"></span>`fn crate_name(&self) -> &str`

- <span id="generatorcontext-get-item"></span>`fn get_item(&self, id: &Id) -> Option<&Item>`

- <span id="generatorcontext-get-impls"></span>`fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

- <span id="generatorcontext-crate-version"></span>`fn crate_version(&self) -> Option<&str>`

- <span id="generatorcontext-render-config"></span>`fn render_config(&self) -> &RenderConfig` — [`RenderConfig`](config/index.md)

- <span id="generatorcontext-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](render_shared/index.md)

##### `impl ItemFilter for GeneratorContext<'_>`

- <span id="generatorcontext-should-include-item"></span>`fn should_include_item(&self, item: &Item) -> bool`

- <span id="generatorcontext-include-private"></span>`fn include_private(&self) -> bool`

- <span id="generatorcontext-include-blanket-impls"></span>`fn include_blanket_impls(&self) -> bool`

##### `impl LinkResolver for GeneratorContext<'_>`

- <span id="generatorcontext-link-registry"></span>`fn link_registry(&self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../linker/index.md)

- <span id="generatorcontext-process-docs"></span>`fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

- <span id="generatorcontext-create-link"></span>`fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

##### `impl OwoColorize for GeneratorContext<'a>`

##### `impl Pointable for GeneratorContext<'a>`

- <span id="generatorcontext-const-align"></span>`const ALIGN: usize`

- <span id="generatorcontext-type-init"></span>`type Init = T`

- <span id="generatorcontext-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generatorcontext-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generatorcontext-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generatorcontext-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl RenderContext for GeneratorContext<'a>`

##### `impl WithSubscriber for GeneratorContext<'a>`

### `DocLinkProcessor<'a>`

```rust
struct DocLinkProcessor<'a> {
    krate: &'a rustdoc_types::Crate,
    link_registry: &'a crate::linker::LinkRegistry,
    current_file: &'a str,
    path_name_index: std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>,
}
```

*Defined in `src/generator/doc_links.rs:416-429`*

Processes doc comments to resolve intra-doc links to markdown links.

Rustdoc JSON includes a `links` field on each Item that maps intra-doc
link text to item IDs. This processor uses that map along with the
`LinkRegistry` to convert these to relative markdown links.

# Supported Patterns

- `` [`Name`](#name) `` - Backtick code links (most common)
- `` `path::to::Item` `` - Qualified path links
- `` `Type::method` `` - Method/associated item links
- `[name]` - Plain identifier links
- `[text]`ref`` - Reference-style links
- ``text`` - Path reference links

# External Crate Links

Items from external crates are linked to docs.rs when possible.

# Code Block Protection

Links inside fenced code blocks are not processed.

#### Fields

- **`krate`**: `&'a rustdoc_types::Crate`

  The crate being documented (for looking up items).

- **`link_registry`**: `&'a crate::linker::LinkRegistry`

  Registry mapping IDs to file paths.

- **`current_file`**: `&'a str`

  The current file path (for relative link calculation).

- **`path_name_index`**: `std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>`

  Index mapping item names to their IDs for fast lookup.
  Built from `krate.paths` at construction time.

#### Implementations

- <span id="doclinkprocessor-with-index"></span>`fn with_index(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str, path_name_index: &HashMap<&'a str, Vec<Id>>) -> Self` — [`LinkRegistry`](../linker/index.md)

- <span id="doclinkprocessor-new"></span>`fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self` — [`LinkRegistry`](../linker/index.md)

- <span id="doclinkprocessor-process"></span>`fn process(&self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-links-protected"></span>`fn process_links_protected(&self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-line"></span>`fn process_line(&self, line: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-reference-links"></span>`fn process_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-path-reference-links"></span>`fn process_path_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-method-links"></span>`fn process_method_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-backtick-links"></span>`fn process_backtick_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-plain-links"></span>`fn process_plain_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-process-html-links-with-context"></span>`fn process_html_links_with_context(&self, text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-resolve-html-link-to-url"></span>`fn resolve_html_link_to_url(&self, item_name: &str, item_kind: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-kind-matches"></span>`fn kind_matches(html_kind: &str, item_kind: ItemKind) -> bool`

- <span id="doclinkprocessor-clean-blank-lines"></span>`fn clean_blank_lines(docs: &str) -> String`

- <span id="doclinkprocessor-resolve-to-url"></span>`fn resolve_to_url(&self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-get-url-for-id"></span>`fn get_url_for_id(&self, id: Id) -> Option<String>`

- <span id="doclinkprocessor-get-docs-rs-url"></span>`fn get_docs_rs_url(path_info: &rustdoc_types::ItemSummary) -> Option<String>`

- <span id="doclinkprocessor-resolve-method-link"></span>`fn resolve_method_link(&self, type_name: &str, method_name: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- <span id="doclinkprocessor-resolve-link"></span>`fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>) -> String`

- <span id="doclinkprocessor-create-link-for-id"></span>`fn create_link_for_id(&self, id: Id, display_name: &str) -> Option<String>`

- <span id="doclinkprocessor-create-docs-rs-link"></span>`fn create_docs_rs_link(path_info: &rustdoc_types::ItemSummary, display_name: &str) -> Option<String>`

#### Trait Implementations

##### `impl Instrument for DocLinkProcessor<'a>`

##### `impl IntoEither for DocLinkProcessor<'a>`

##### `impl OwoColorize for DocLinkProcessor<'a>`

##### `impl Pointable for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-const-align"></span>`const ALIGN: usize`

- <span id="doclinkprocessor-type-init"></span>`type Init = T`

- <span id="doclinkprocessor-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="doclinkprocessor-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="doclinkprocessor-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="doclinkprocessor-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for DocLinkProcessor<'a>`

### `ModuleRenderer<'a>`

```rust
struct ModuleRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    is_root: bool,
}
```

*Defined in `src/generator/module.rs:28-37`*

Renders a module to markdown.

This struct handles the complete rendering of a module's documentation page,
including:
- Title (Crate or Module heading)
- Module-level documentation
- Sections for each item type (Modules, Structs, Enums, etc.)

The renderer is generic over [`RenderContext`](context/index.md), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`is_root`**: `bool`

  Whether this is the crate root module.

#### Implementations

- <span id="modulerenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](context/index.md)

- <span id="modulerenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

- <span id="modulerenderer-render"></span>`fn render(&self, item: &Item) -> String`

- <span id="modulerenderer-categorize-items"></span>`fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](module/index.md)

- <span id="modulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport(&self, items: &mut CategorizedItems<'a>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<&'a Id>)` — [`CategorizedItems`](module/index.md)

- <span id="modulerenderer-render-all-sections"></span>`fn render_all_sections(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](module/index.md)

- <span id="modulerenderer-render-types-section"></span>`fn render_types_section(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](module/index.md)

- <span id="modulerenderer-render-statics-section"></span>`fn render_statics_section(&self, md: &mut String, statics: &[&Item])`

- <span id="modulerenderer-build-toc-entries"></span>`fn build_toc_entries(items: &CategorizedItems<'_>) -> Vec<TocEntry>` — [`CategorizedItems`](module/index.md), [`TocEntry`](toc/index.md)

- <span id="modulerenderer-build-quick-ref-entries"></span>`fn build_quick_ref_entries(&self, items: &CategorizedItems<'_>) -> Vec<QuickRefEntry>` — [`CategorizedItems`](module/index.md), [`QuickRefEntry`](quick_ref/index.md)

- <span id="modulerenderer-get-item-summary"></span>`fn get_item_summary(&self, item: &Item, item_id: Id) -> String`

- <span id="modulerenderer-render-modules-section"></span>`fn render_modules_section(&self, md: &mut String, modules: &[(&Id, &Item)])`

- <span id="modulerenderer-get-module-summary"></span>`fn get_module_summary(&self, item: &Item, item_id: Id) -> String`

- <span id="modulerenderer-render-traits-section"></span>`fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)])`

- <span id="modulerenderer-render-functions-section"></span>`fn render_functions_section(&self, md: &mut String, functions: &[&Item])`

- <span id="modulerenderer-render-macros-section"></span>`fn render_macros_section(&self, md: &mut String, macros: &[&Item])`

- <span id="modulerenderer-render-constants-section"></span>`fn render_constants_section(&self, md: &mut String, constants: &[&Item])`

#### Trait Implementations

##### `impl Instrument for ModuleRenderer<'a>`

##### `impl IntoEither for ModuleRenderer<'a>`

##### `impl OwoColorize for ModuleRenderer<'a>`

##### `impl Pointable for ModuleRenderer<'a>`

- <span id="modulerenderer-const-align"></span>`const ALIGN: usize`

- <span id="modulerenderer-type-init"></span>`type Init = T`

- <span id="modulerenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="modulerenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="modulerenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="modulerenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for ModuleRenderer<'a>`

### `QuickRefEntry`

```rust
struct QuickRefEntry {
    pub name: String,
    pub kind: &'static str,
    pub anchor: String,
    pub summary: String,
}
```

*Defined in `src/generator/quick_ref.rs:26-38`*

An entry in the quick reference table.

Each entry represents a single public item with its name, kind,
anchor link, and first-sentence summary.

#### Fields

- **`name`**: `String`

  Display name for this entry.

- **`kind`**: `&'static str`

  Item kind (struct, enum, trait, fn, etc.).

- **`anchor`**: `String`

  Anchor link target (without the `#` prefix).

- **`summary`**: `String`

  First-sentence summary from doc comment.

#### Implementations

- <span id="quickrefentry-new"></span>`fn new(name: impl Into<String>, kind: &'static str, anchor: impl Into<String>, summary: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for QuickRefEntry`

- <span id="quickrefentry-clone"></span>`fn clone(&self) -> QuickRefEntry` — [`QuickRefEntry`](quick_ref/index.md)

##### `impl Debug for QuickRefEntry`

- <span id="quickrefentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for QuickRefEntry`

##### `impl IntoEither for QuickRefEntry`

##### `impl OwoColorize for QuickRefEntry`

##### `impl Pointable for QuickRefEntry`

- <span id="quickrefentry-const-align"></span>`const ALIGN: usize`

- <span id="quickrefentry-type-init"></span>`type Init = T`

- <span id="quickrefentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for QuickRefEntry`

### `QuickRefGenerator`

```rust
struct QuickRefGenerator;
```

*Defined in `src/generator/quick_ref.rs:70`*

Generator for markdown quick reference tables.

The generator creates a table summarizing all items with links,
kinds, and first-sentence descriptions.

#### Implementations

- <span id="quickrefgenerator-new"></span>`const fn new() -> Self`

- <span id="quickrefgenerator-generate"></span>`fn generate(&self, entries: &[QuickRefEntry]) -> String` — [`QuickRefEntry`](quick_ref/index.md)

#### Trait Implementations

##### `impl Clone for QuickRefGenerator`

- <span id="quickrefgenerator-clone"></span>`fn clone(&self) -> QuickRefGenerator` — [`QuickRefGenerator`](quick_ref/index.md)

##### `impl Debug for QuickRefGenerator`

- <span id="quickrefgenerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for QuickRefGenerator`

- <span id="quickrefgenerator-default"></span>`fn default() -> QuickRefGenerator` — [`QuickRefGenerator`](quick_ref/index.md)

##### `impl Instrument for QuickRefGenerator`

##### `impl IntoEither for QuickRefGenerator`

##### `impl OwoColorize for QuickRefGenerator`

##### `impl Pointable for QuickRefGenerator`

- <span id="quickrefgenerator-const-align"></span>`const ALIGN: usize`

- <span id="quickrefgenerator-type-init"></span>`type Init = T`

- <span id="quickrefgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for QuickRefGenerator`

### `TocEntry`

```rust
struct TocEntry {
    pub title: String,
    pub anchor: String,
    pub children: Vec<Self>,
}
```

*Defined in `src/generator/toc.rs:28-37`*

An entry in the table of contents.

Each entry represents either a section heading (like "Structs") or an
individual item (like a specific struct name). Entries can have children
for nested navigation.

#### Fields

- **`title`**: `String`

  Display title for this entry.

- **`anchor`**: `String`

  Anchor link target (without the `#` prefix).

- **`children`**: `Vec<Self>`

  Child entries for nested navigation.

#### Implementations

- <span id="tocentry-new"></span>`fn new(title: impl Into<String>, anchor: impl Into<String>) -> Self`

- <span id="tocentry-with-children"></span>`fn with_children(title: impl Into<String>, anchor: impl Into<String>, children: Vec<Self>) -> Self`

- <span id="tocentry-count"></span>`fn count(&self) -> usize`

#### Trait Implementations

##### `impl Clone for TocEntry`

- <span id="tocentry-clone"></span>`fn clone(&self) -> TocEntry` — [`TocEntry`](toc/index.md)

##### `impl Debug for TocEntry`

- <span id="tocentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TocEntry`

##### `impl IntoEither for TocEntry`

##### `impl OwoColorize for TocEntry`

##### `impl Pointable for TocEntry`

- <span id="tocentry-const-align"></span>`const ALIGN: usize`

- <span id="tocentry-type-init"></span>`type Init = T`

- <span id="tocentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TocEntry`

### `TocGenerator`

```rust
struct TocGenerator {
    threshold: usize,
}
```

*Defined in `src/generator/toc.rs:88-91`*

Generator for markdown table of contents.

The generator only produces output when the total number of items
exceeds the configured threshold. This prevents cluttering small
modules with unnecessary navigation.

#### Fields

- **`threshold`**: `usize`

  Minimum items required to generate a TOC.

#### Implementations

- <span id="tocgenerator-new"></span>`const fn new(threshold: usize) -> Self`

- <span id="tocgenerator-generate"></span>`fn generate(&self, entries: &[TocEntry]) -> Option<String>` — [`TocEntry`](toc/index.md)

- <span id="tocgenerator-render-entry"></span>`fn render_entry(md: &mut String, entry: &TocEntry, depth: usize)` — [`TocEntry`](toc/index.md)

#### Trait Implementations

##### `impl Clone for TocGenerator`

- <span id="tocgenerator-clone"></span>`fn clone(&self) -> TocGenerator` — [`TocGenerator`](toc/index.md)

##### `impl Debug for TocGenerator`

- <span id="tocgenerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TocGenerator`

##### `impl IntoEither for TocGenerator`

##### `impl OwoColorize for TocGenerator`

##### `impl Pointable for TocGenerator`

- <span id="tocgenerator-const-align"></span>`const ALIGN: usize`

- <span id="tocgenerator-type-init"></span>`type Init = T`

- <span id="tocgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TocGenerator`

### `Generator<'a>`

```rust
struct Generator<'a> {
    ctx: GeneratorContext<'a>,
    args: &'a crate::Args,
    root_item: &'a rustdoc_types::Item,
}
```

*Defined in `src/generator/mod.rs:89-98`*

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

- <span id="generator-new"></span>`fn new(krate: &'a Crate, args: &'a Args, config: RenderConfig) -> Result<Self, Error>` — [`Args`](../index.md), [`RenderConfig`](config/index.md), [`Error`](../error/index.md)

- <span id="generator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../error/index.md)

- <span id="generator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../error/index.md)

- <span id="generator-generate-to-capture"></span>`fn generate_to_capture(krate: &Crate, format: CliOutputFormat, include_private: bool) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](../index.md), [`MarkdownCapture`](capture/index.md), [`Error`](../error/index.md)

- <span id="generator-generate-to-capture-with-config"></span>`fn generate_to_capture_with_config(krate: &Crate, format: CliOutputFormat, include_private: bool, config: RenderConfig) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](../index.md), [`RenderConfig`](config/index.md), [`MarkdownCapture`](capture/index.md), [`Error`](../error/index.md)

- <span id="generator-generate-flat-to-capture"></span>`fn generate_flat_to_capture(ctx: &GeneratorContext<'_>, root: &Item, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](context/index.md), [`MarkdownCapture`](capture/index.md), [`Error`](../error/index.md)

- <span id="generator-generate-flat-recursive-capture"></span>`fn generate_flat_recursive_capture(ctx: &GeneratorContext<'_>, item: &Item, prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](context/index.md), [`MarkdownCapture`](capture/index.md), [`Error`](../error/index.md)

- <span id="generator-generate-nested-to-capture"></span>`fn generate_nested_to_capture(ctx: &GeneratorContext<'_>, root: &Item, path_prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](context/index.md), [`MarkdownCapture`](capture/index.md), [`Error`](../error/index.md)

- <span id="generator-run"></span>`fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error>` — [`Args`](../index.md), [`Error`](../error/index.md)

#### Trait Implementations

##### `impl Instrument for Generator<'a>`

##### `impl IntoEither for Generator<'a>`

##### `impl OwoColorize for Generator<'a>`

##### `impl Pointable for Generator<'a>`

- <span id="generator-const-align"></span>`const ALIGN: usize`

- <span id="generator-type-init"></span>`type Init = T`

- <span id="generator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for Generator<'a>`

## Enums

### `ImplCategory`

```rust
enum ImplCategory {
    Inherent,
    Derive,
    Conversion,
    Iterator,
    Io,
    Operator,
    Access,
    Formatting,
    Other,
}
```

*Defined in `src/generator/impl_category.rs:66-144`*

Logical category for trait implementations.

Each variant represents a group of related traits that serve a similar purpose.
This categorization enables documentation to be organized by functionality rather
than alphabetically, making it easier for users to find relevant implementations.

# Variant Order

The variants are ordered by their typical importance/frequency of use:
1. `Inherent` - Direct methods on the type (most commonly referenced)
2. `Derive` - Standard derived traits (`Clone`, `Debug`, etc.)
3. `Conversion` - Type conversion traits (`From`, `Into`, etc.)
4. `Access` - Smart pointer/indexing traits (`Deref`, `Index`)
5. `Iterator` - Iteration support
6. `Operator` - Operator overloading
7. `Formatting` - Display/formatting traits
8. `Io` - I/O traits (less common)
9. `Other` - Catch-all for unrecognized traits

#### Variants

- **`Inherent`**

  Inherent implementations (no trait).
  
  These are methods defined directly on the type:
  ```rust,ignore
  impl MyType {
      fn new() -> Self { ... }
      fn method(&self) { ... }
  }
  ```

- **`Derive`**

  Common derived traits from `#[derive(...)]`.
  
  Includes: `Clone`, `Copy`, `Debug`, `Default`, `PartialEq`, `Eq`,
  `Hash`, `PartialOrd`, `Ord`.
  
  These traits have standard, predictable implementations that users
  typically don't need to examine in detail.

- **`Conversion`**

  Type conversion traits.
  
  Includes: `From`, `Into`, `TryFrom`, `TryInto`, `AsRef`, `AsMut`,
  `Borrow`, `BorrowMut`.
  
  These traits define how a type can be converted to/from other types,
  which is essential for understanding type interoperability.

- **`Iterator`**

  Iterator-related traits.
  
  Includes: `Iterator`, `IntoIterator`, `FromIterator`, `Extend`,
  `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`.
  
  These traits define how a type participates in Rust's iteration ecosystem.

- **`Io`**

  I/O traits from `std::io`.
  
  Includes: `Read`, `Write`, `Seek`, `BufRead`, `BufWrite`.
  
  These traits define how a type can be used for input/output operations.

- **`Operator`**

  Operator overloading traits from `std::ops`.
  
  Includes all arithmetic, bitwise, and compound assignment operators:
  `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`, `Not`, `BitAnd`, `BitOr`,
  `BitXor`, `Shl`, `Shr`, and their `*Assign` variants.
  
  These traits define custom behavior for Rust's operators (`+`, `-`, etc.).

- **`Access`**

  Smart pointer and indexing traits.
  
  Includes: `Deref`, `DerefMut`, `Index`, `IndexMut`.
  
  These traits define how a type can be dereferenced or indexed,
  which is crucial for wrapper types and collections.

- **`Formatting`**

  Display and formatting traits.
  
  Includes: `Display`, `LowerHex`, `UpperHex`, `Octal`, `Binary`,
  `Pointer`, `LowerExp`, `UpperExp`.
  
  These traits define how a type is formatted for output. `Display` is
  particularly important as it defines user-facing string representation.

- **`Other`**

  Any trait that doesn't fit other categories.
  
  This is the fallback for:
  - Third-party traits (e.g., `serde::Serialize`)
  - Less common std traits (e.g., `Drop`, `Send`, `Sync`)
  - Domain-specific traits

#### Implementations

- <span id="implcategory-from-trait-path"></span>`fn from_trait_path(path: Option<&str>) -> Self`

- <span id="implcategory-display-name"></span>`const fn display_name(&self) -> &'static str`

- <span id="implcategory-sort-order"></span>`const fn sort_order(self) -> u8`

#### Trait Implementations

##### `impl Clone for ImplCategory`

- <span id="implcategory-clone"></span>`fn clone(&self) -> ImplCategory` — [`ImplCategory`](impl_category/index.md)

##### `impl Comparable for ImplCategory`

- <span id="implcategory-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for ImplCategory`

##### `impl Debug for ImplCategory`

- <span id="implcategory-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ImplCategory`

##### `impl Equivalent for ImplCategory`

- <span id="implcategory-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ImplCategory`

- <span id="implcategory-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Instrument for ImplCategory`

##### `impl IntoEither for ImplCategory`

##### `impl Ord for ImplCategory`

- <span id="implcategory-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl OwoColorize for ImplCategory`

##### `impl PartialEq for ImplCategory`

- <span id="implcategory-eq"></span>`fn eq(&self, other: &ImplCategory) -> bool` — [`ImplCategory`](impl_category/index.md)

##### `impl PartialOrd for ImplCategory`

- <span id="implcategory-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Pointable for ImplCategory`

- <span id="implcategory-const-align"></span>`const ALIGN: usize`

- <span id="implcategory-type-init"></span>`type Init = T`

- <span id="implcategory-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="implcategory-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="implcategory-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="implcategory-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for ImplCategory`

##### `impl WithSubscriber for ImplCategory`

## Traits

### `ItemAccess`

```rust
trait ItemAccess { ... }
```

*Defined in `src/generator/context.rs:37-63`*

Core data access for crate documentation.

Provides read-only access to the crate structure, items, and impl blocks.

#### Required Methods

- `fn krate(&self) -> &Crate`

  Get the crate being documented.

- `fn crate_name(&self) -> &str`

  Get the crate name.

- `fn get_item(&self, id: &Id) -> Option<&Item>`

  Get an item by its ID.

- `fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

  Get impl blocks for a type.

- `fn crate_version(&self) -> Option<&str>`

  Get the crate version for display in headers.

- `fn render_config(&self) -> &RenderConfig`

  Get the rendering configuration.

#### Provided Methods

- `fn source_path_config_for_file(&self, _current_file: &str) -> Option<SourcePathConfig>`

  Get source path config for a specific file.

#### Implementors

- [`GeneratorContext`](context/index.md)
- [`SingleCrateView`](../multi_crate/context/index.md)

### `ItemFilter`

```rust
trait ItemFilter { ... }
```

*Defined in `src/generator/context.rs:68-79`*

Item visibility and filtering logic.

Determines which items should be included in the generated documentation.

#### Required Methods

- `fn should_include_item(&self, item: &Item) -> bool`

  Check if an item should be included based on visibility.

- `fn include_private(&self) -> bool`

  Whether private items should be included.

- `fn include_blanket_impls(&self) -> bool`

  Whether blanket trait implementations should be included.

#### Implementors

- [`GeneratorContext`](context/index.md)
- [`SingleCrateView`](../multi_crate/context/index.md)

### `LinkResolver`

```rust
trait LinkResolver { ... }
```

*Defined in `src/generator/context.rs:84-113`*

Link creation and documentation processing.

Handles intra-doc link resolution and markdown link generation.

#### Required Methods

- `fn link_registry(&self) -> Option<&LinkRegistry>`

  Get the link registry for single-crate mode.

- `fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

  Process documentation string with intra-doc link resolution.

- `fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

  Create a markdown link to an item.

#### Implementors

- [`GeneratorContext`](context/index.md)
- [`SingleCrateView`](../multi_crate/context/index.md)

### `RenderContext`

```rust
trait RenderContext: ItemAccess + ItemFilter + LinkResolver { ... }
```

*Defined in `src/generator/context.rs:126`*

Combined rendering context trait.

This trait combines [`ItemAccess`](context/index.md), [`ItemFilter`](context/index.md), and [`LinkResolver`](context/index.md)
for components that need full access to the rendering context.

Most renderers should use this trait for convenience, but components
with limited requirements can depend on individual sub-traits.

#### Implementors

- `T`

## Functions

*Defined in `src/generator/mod.rs:60`*

*Defined in `src/generator/mod.rs:60`*

*Defined in `src/generator/mod.rs:60`*

*Defined in `src/generator/mod.rs:61`*

*Defined in `src/generator/mod.rs:69`*

