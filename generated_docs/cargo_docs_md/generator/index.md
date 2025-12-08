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
  - [`impls`](#impls)
  - [`items`](#items)
  - [`module`](#module)
  - [`nested`](#nested)
  - [`quick_ref`](#quick_ref)
  - [`render_shared`](#render_shared)
  - [`toc`](#toc)
  - [`config`](#config)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`Generator`](#generator)
- [Traits](#traits)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
- [Functions](#functions)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`breadcrumbs`](#breadcrumbs) | mod | Breadcrumb navigation generation for nested module pages. |
| [`capture`](#capture) | mod | In-memory markdown capture for testing. |
| [`context`](#context) | mod | Shared context for documentation generation. |
| [`doc_links`](#doc_links) | mod | Intra-doc link processing for documentation generation. |
| [`flat`](#flat) | mod | Flat format documentation generation. |
| [`impls`](#impls) | mod | Implementation block rendering for documentation generation. |
| [`items`](#items) | mod | Item rendering for documentation generation. |
| [`module`](#module) | mod | Module markdown rendering for documentation generation. |
| [`nested`](#nested) | mod | Nested format documentation generation. |
| [`quick_ref`](#quick_ref) | mod | Quick Reference table generation for module documentation. |
| [`render_shared`](#render_shared) | mod | Shared rendering functions for documentation generation. |
| [`toc`](#toc) | mod | Table of Contents generation for module documentation. |
| [`config`](#config) | mod | Configuration for markdown rendering. |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`Generator`](#generator) | struct | Main documentation generator. |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | trait |  |
| [`unnamed`](#unnamed) | fn |  |
| [`unnamed`](#unnamed) | fn |  |
| [`unnamed`](#unnamed) | fn |  |
| [`unnamed`](#unnamed) | fn |  |
| [`unnamed`](#unnamed) | fn |  |

## Modules

- [`breadcrumbs`](breadcrumbs/index.md) - Breadcrumb navigation generation for nested module pages.
- [`capture`](capture/index.md) - In-memory markdown capture for testing.
- [`context`](context/index.md) - Shared context for documentation generation.
- [`doc_links`](doc_links/index.md) - Intra-doc link processing for documentation generation.
- [`flat`](flat/index.md) - Flat format documentation generation.
- [`impls`](impls/index.md) - Implementation block rendering for documentation generation.
- [`items`](items/index.md) - Item rendering for documentation generation.
- [`module`](module/index.md) - Module markdown rendering for documentation generation.
- [`nested`](nested/index.md) - Nested format documentation generation.
- [`quick_ref`](quick_ref/index.md) - Quick Reference table generation for module documentation.
- [`render_shared`](render_shared/index.md) - Shared rendering functions for documentation generation.
- [`toc`](toc/index.md) - Table of Contents generation for module documentation.
- [`config`](config/index.md) - Configuration for markdown rendering.

## Structs

### `BreadcrumbGenerator<'a>`

```rust
struct BreadcrumbGenerator<'a> {
    module_path: &'a [String],
    crate_name: &'a str,
}
```

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

##### `impl<T> Instrument for BreadcrumbGenerator<'a>`

##### `impl<T> IntoEither for BreadcrumbGenerator<'a>`

##### `impl<D> OwoColorize for BreadcrumbGenerator<'a>`

##### `impl<T> Pointable for BreadcrumbGenerator<'a>`

- <span id="breadcrumbgenerator-align"></span>`const ALIGN: usize`

- <span id="breadcrumbgenerator-init"></span>`type Init = T`

- <span id="breadcrumbgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="breadcrumbgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="breadcrumbgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="breadcrumbgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for BreadcrumbGenerator<'a>`

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

- <span id="markdowncapture-default"></span>`fn default() -> MarkdownCapture` — [`MarkdownCapture`](../index.md)

##### `impl<T> Instrument for MarkdownCapture`

##### `impl<T> IntoEither for MarkdownCapture`

##### `impl<D> OwoColorize for MarkdownCapture`

##### `impl<T> Pointable for MarkdownCapture`

- <span id="markdowncapture-align"></span>`const ALIGN: usize`

- <span id="markdowncapture-init"></span>`type Init = T`

- <span id="markdowncapture-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="markdowncapture-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="markdowncapture-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="markdowncapture-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MarkdownCapture`

### `QuickRefEntry`

```rust
struct QuickRefEntry {
    pub name: String,
    pub kind: &'static str,
    pub anchor: String,
    pub summary: String,
}
```

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

- <span id="quickrefentry-clone"></span>`fn clone(&self) -> QuickRefEntry` — [`QuickRefEntry`](#quickrefentry)

##### `impl Debug for QuickRefEntry`

- <span id="quickrefentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for QuickRefEntry`

##### `impl<T> IntoEither for QuickRefEntry`

##### `impl<D> OwoColorize for QuickRefEntry`

##### `impl<T> Pointable for QuickRefEntry`

- <span id="quickrefentry-align"></span>`const ALIGN: usize`

- <span id="quickrefentry-init"></span>`type Init = T`

- <span id="quickrefentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for QuickRefEntry`

### `QuickRefGenerator`

```rust
struct QuickRefGenerator;
```

Generator for markdown quick reference tables.

The generator creates a table summarizing all items with links,
kinds, and first-sentence descriptions.

#### Implementations

- <span id="quickrefgenerator-new"></span>`const fn new() -> Self`

- <span id="quickrefgenerator-generate"></span>`fn generate(&self, entries: &[QuickRefEntry]) -> String` — [`QuickRefEntry`](#quickrefentry)

#### Trait Implementations

##### `impl Clone for QuickRefGenerator`

- <span id="quickrefgenerator-clone"></span>`fn clone(&self) -> QuickRefGenerator` — [`QuickRefGenerator`](#quickrefgenerator)

##### `impl Debug for QuickRefGenerator`

- <span id="quickrefgenerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for QuickRefGenerator`

- <span id="quickrefgenerator-default"></span>`fn default() -> QuickRefGenerator` — [`QuickRefGenerator`](#quickrefgenerator)

##### `impl<T> Instrument for QuickRefGenerator`

##### `impl<T> IntoEither for QuickRefGenerator`

##### `impl<D> OwoColorize for QuickRefGenerator`

##### `impl<T> Pointable for QuickRefGenerator`

- <span id="quickrefgenerator-align"></span>`const ALIGN: usize`

- <span id="quickrefgenerator-init"></span>`type Init = T`

- <span id="quickrefgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for QuickRefGenerator`

### `TocEntry`

```rust
struct TocEntry {
    pub title: String,
    pub anchor: String,
    pub children: Vec<TocEntry>,
}
```

An entry in the table of contents.

Each entry represents either a section heading (like "Structs") or an
individual item (like a specific struct name). Entries can have children
for nested navigation.

#### Fields

- **`title`**: `String`

  Display title for this entry.

- **`anchor`**: `String`

  Anchor link target (without the `#` prefix).

- **`children`**: `Vec<TocEntry>`

  Child entries for nested navigation.

#### Implementations

- <span id="tocentry-new"></span>`fn new(title: impl Into<String>, anchor: impl Into<String>) -> Self`

- <span id="tocentry-with-children"></span>`fn with_children(title: impl Into<String>, anchor: impl Into<String>, children: Vec<TocEntry>) -> Self` — [`TocEntry`](#tocentry)

- <span id="tocentry-count"></span>`fn count(&self) -> usize`

#### Trait Implementations

##### `impl Clone for TocEntry`

- <span id="tocentry-clone"></span>`fn clone(&self) -> TocEntry` — [`TocEntry`](#tocentry)

##### `impl Debug for TocEntry`

- <span id="tocentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for TocEntry`

##### `impl<T> IntoEither for TocEntry`

##### `impl<D> OwoColorize for TocEntry`

##### `impl<T> Pointable for TocEntry`

- <span id="tocentry-align"></span>`const ALIGN: usize`

- <span id="tocentry-init"></span>`type Init = T`

- <span id="tocentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for TocEntry`

### `TocGenerator`

```rust
struct TocGenerator {
    threshold: usize,
}
```

Generator for markdown table of contents.

The generator only produces output when the total number of items
exceeds the configured threshold. This prevents cluttering small
modules with unnecessary navigation.

#### Fields

- **`threshold`**: `usize`

  Minimum items required to generate a TOC.

#### Implementations

- <span id="tocgenerator-new"></span>`const fn new(threshold: usize) -> Self`

- <span id="tocgenerator-generate"></span>`fn generate(&self, entries: &[TocEntry]) -> Option<String>` — [`TocEntry`](#tocentry)

- <span id="tocgenerator-render-entry"></span>`fn render_entry(&self, md: &mut String, entry: &TocEntry, depth: usize)` — [`TocEntry`](#tocentry)

#### Trait Implementations

##### `impl Clone for TocGenerator`

- <span id="tocgenerator-clone"></span>`fn clone(&self) -> TocGenerator` — [`TocGenerator`](#tocgenerator)

##### `impl Debug for TocGenerator`

- <span id="tocgenerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for TocGenerator`

##### `impl<T> IntoEither for TocGenerator`

##### `impl<D> OwoColorize for TocGenerator`

##### `impl<T> Pointable for TocGenerator`

- <span id="tocgenerator-align"></span>`const ALIGN: usize`

- <span id="tocgenerator-init"></span>`type Init = T`

- <span id="tocgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for TocGenerator`

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

- <span id="renderconfig-clone"></span>`fn clone(&self) -> RenderConfig` — [`RenderConfig`](../index.md)

##### `impl Debug for RenderConfig`

- <span id="renderconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RenderConfig`

- <span id="renderconfig-default"></span>`fn default() -> Self`

##### `impl<T> Instrument for RenderConfig`

##### `impl<T> IntoEither for RenderConfig`

##### `impl<D> OwoColorize for RenderConfig`

##### `impl<T> Pointable for RenderConfig`

- <span id="renderconfig-align"></span>`const ALIGN: usize`

- <span id="renderconfig-init"></span>`type Init = T`

- <span id="renderconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="renderconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="renderconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="renderconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for RenderConfig`

### `SourceConfig`

```rust
struct SourceConfig {
    pub function_bodies: bool,
    pub const_values: bool,
    pub private_items: bool,
    pub source_locations: bool,
}
```

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

  Add file:line references to items.

#### Trait Implementations

##### `impl Clone for SourceConfig`

- <span id="sourceconfig-clone"></span>`fn clone(&self) -> SourceConfig` — [`SourceConfig`](../index.md)

##### `impl Debug for SourceConfig`

- <span id="sourceconfig-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceConfig`

- <span id="sourceconfig-default"></span>`fn default() -> Self`

##### `impl<T> Instrument for SourceConfig`

##### `impl<T> IntoEither for SourceConfig`

##### `impl<D> OwoColorize for SourceConfig`

##### `impl<T> Pointable for SourceConfig`

- <span id="sourceconfig-align"></span>`const ALIGN: usize`

- <span id="sourceconfig-init"></span>`type Init = T`

- <span id="sourceconfig-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceconfig-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceconfig-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceconfig-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for SourceConfig`

### `GeneratorContext<'a>`

```rust
struct GeneratorContext<'a> {
    pub krate: &'a rustdoc_types::Crate,
    crate_name: String,
    pub impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    pub link_registry: crate::linker::LinkRegistry,
    pub args: &'a crate::Args,
    pub config: crate::generator::config::RenderConfig,
}
```

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

#### Implementations

- <span id="generatorcontext-new"></span>`fn new(krate: &'a Crate, args: &'a Args, config: RenderConfig) -> Self` — [`Args`](../index.md), [`RenderConfig`](../index.md)

- <span id="generatorcontext-build-impl-map"></span>`fn build_impl_map(krate: &'a Crate) -> HashMap<Id, Vec<&'a Impl>>`

- <span id="generatorcontext-impl-sort-key"></span>`fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- <span id="generatorcontext-get-type-id"></span>`const fn get_type_id(ty: &rustdoc_types::Type) -> Option<Id>`

- <span id="generatorcontext-should-include-item"></span>`const fn should_include_item(&self, item: &Item) -> bool`

- <span id="generatorcontext-count-modules"></span>`fn count_modules(&self, item: &Item) -> usize`

#### Trait Implementations

##### `impl<T> Instrument for GeneratorContext<'a>`

##### `impl<T> IntoEither for GeneratorContext<'a>`

##### `impl ItemAccess for GeneratorContext<'_>`

- <span id="generatorcontext-krate"></span>`fn krate(&self) -> &Crate`

- <span id="generatorcontext-crate-name"></span>`fn crate_name(&self) -> &str`

- <span id="generatorcontext-get-item"></span>`fn get_item(&self, id: &Id) -> Option<&Item>`

- <span id="generatorcontext-get-impls"></span>`fn get_impls(&self, id: &Id) -> Option<&[&Impl]>`

- <span id="generatorcontext-crate-version"></span>`fn crate_version(&self) -> Option<&str>`

- <span id="generatorcontext-render-config"></span>`fn render_config(&self) -> &RenderConfig` — [`RenderConfig`](../index.md)

##### `impl ItemFilter for GeneratorContext<'_>`

- <span id="generatorcontext-should-include-item"></span>`fn should_include_item(&self, item: &Item) -> bool`

- <span id="generatorcontext-include-private"></span>`fn include_private(&self) -> bool`

- <span id="generatorcontext-include-blanket-impls"></span>`fn include_blanket_impls(&self) -> bool`

##### `impl LinkResolver for GeneratorContext<'_>`

- <span id="generatorcontext-link-registry"></span>`fn link_registry(&self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../index.md)

- <span id="generatorcontext-process-docs"></span>`fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>`

- <span id="generatorcontext-create-link"></span>`fn create_link(&self, id: Id, current_file: &str) -> Option<String>`

##### `impl<D> OwoColorize for GeneratorContext<'a>`

##### `impl<T> Pointable for GeneratorContext<'a>`

- <span id="generatorcontext-align"></span>`const ALIGN: usize`

- <span id="generatorcontext-init"></span>`type Init = T`

- <span id="generatorcontext-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generatorcontext-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generatorcontext-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generatorcontext-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> RenderContext for GeneratorContext<'a>`

##### `impl<T> WithSubscriber for GeneratorContext<'a>`

### `DocLinkProcessor<'a>`

```rust
struct DocLinkProcessor<'a> {
    krate: &'a rustdoc_types::Crate,
    link_registry: &'a crate::linker::LinkRegistry,
    current_file: &'a str,
    path_name_index: std::collections::HashMap<&'a str, Vec<rustdoc_types::Id>>,
}
```

Processes doc comments to resolve intra-doc links to markdown links.

Rustdoc JSON includes a `links` field on each Item that maps intra-doc
link text to item IDs. This processor uses that map along with the
`LinkRegistry` to convert these to relative markdown links.

# Supported Patterns

- `` `Name` `` - Backtick code links (most common)
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

- <span id="doclinkprocessor-new"></span>`fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self` — [`LinkRegistry`](../index.md)

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

##### `impl<T> Instrument for DocLinkProcessor<'a>`

##### `impl<T> IntoEither for DocLinkProcessor<'a>`

##### `impl<D> OwoColorize for DocLinkProcessor<'a>`

##### `impl<T> Pointable for DocLinkProcessor<'a>`

- <span id="doclinkprocessor-align"></span>`const ALIGN: usize`

- <span id="doclinkprocessor-init"></span>`type Init = T`

- <span id="doclinkprocessor-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="doclinkprocessor-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="doclinkprocessor-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="doclinkprocessor-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for DocLinkProcessor<'a>`

### `ModuleRenderer<'a>`

```rust
struct ModuleRenderer<'a> {
    ctx: &'a dyn RenderContext,
    current_file: &'a str,
    is_root: bool,
}
```

Renders a module to markdown.

This struct handles the complete rendering of a module's documentation page,
including:
- Title (Crate or Module heading)
- Module-level documentation
- Sections for each item type (Modules, Structs, Enums, etc.)

The renderer is generic over [`RenderContext`](#rendercontext), allowing it to work with
both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.

#### Fields

- **`ctx`**: `&'a dyn RenderContext`

  Reference to the render context (either single-crate or multi-crate).

- **`current_file`**: `&'a str`

  Path of the current file being generated (for relative link calculation).

- **`is_root`**: `bool`

  Whether this is the crate root module.

#### Implementations

- <span id="modulerenderer-new"></span>`fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](#rendercontext)

- <span id="modulerenderer-process-docs"></span>`fn process_docs(&self, item: &Item) -> Option<String>`

- <span id="modulerenderer-render"></span>`fn render(&self, item: &Item) -> String`

- <span id="modulerenderer-categorize-items"></span>`fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](module/index.md)

- <span id="modulerenderer-expand-glob-reexport"></span>`fn expand_glob_reexport(&self, items: &mut CategorizedItems<'a>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<&'a Id>)` — [`CategorizedItems`](module/index.md)

- <span id="modulerenderer-render-all-sections"></span>`fn render_all_sections(&self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](module/index.md)

- <span id="modulerenderer-build-toc-entries"></span>`fn build_toc_entries(&self, items: &CategorizedItems<'_>) -> Vec<TocEntry>` — [`CategorizedItems`](module/index.md), [`TocEntry`](#tocentry)

- <span id="modulerenderer-build-quick-ref-entries"></span>`fn build_quick_ref_entries(&self, items: &CategorizedItems<'_>) -> Vec<QuickRefEntry>` — [`CategorizedItems`](module/index.md), [`QuickRefEntry`](#quickrefentry)

- <span id="modulerenderer-render-modules-section"></span>`fn render_modules_section(&self, md: &mut String, modules: &[(&Id, &Item)])`

- <span id="modulerenderer-render-structs-section"></span>`fn render_structs_section(&self, md: &mut String, structs: &[(&Id, &Item)])`

- <span id="modulerenderer-render-enums-section"></span>`fn render_enums_section(&self, md: &mut String, enums: &[(&Id, &Item)])`

- <span id="modulerenderer-render-traits-section"></span>`fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)])`

- <span id="modulerenderer-render-functions-section"></span>`fn render_functions_section(&self, md: &mut String, functions: &[&Item])`

- <span id="modulerenderer-render-macros-section"></span>`fn render_macros_section(&self, md: &mut String, macros: &[&Item])`

- <span id="modulerenderer-render-constants-section"></span>`fn render_constants_section(&self, md: &mut String, constants: &[&Item])`

- <span id="modulerenderer-render-type-aliases-section"></span>`fn render_type_aliases_section(&self, md: &mut String, type_aliases: &[&Item])`

#### Trait Implementations

##### `impl<T> Instrument for ModuleRenderer<'a>`

##### `impl<T> IntoEither for ModuleRenderer<'a>`

##### `impl<D> OwoColorize for ModuleRenderer<'a>`

##### `impl<T> Pointable for ModuleRenderer<'a>`

- <span id="modulerenderer-align"></span>`const ALIGN: usize`

- <span id="modulerenderer-init"></span>`type Init = T`

- <span id="modulerenderer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="modulerenderer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="modulerenderer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="modulerenderer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for ModuleRenderer<'a>`

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

- <span id="generator-new"></span>`fn new(krate: &'a Crate, args: &'a Args, config: RenderConfig) -> Result<Self, Error>` — [`Args`](../index.md), [`RenderConfig`](../index.md), [`Error`](../error/index.md)

- <span id="generator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../error/index.md)

- <span id="generator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../error/index.md)

- <span id="generator-generate-to-capture"></span>`fn generate_to_capture(krate: &Crate, format: CliOutputFormat, include_private: bool) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](../index.md), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- <span id="generator-generate-flat-to-capture"></span>`fn generate_flat_to_capture(ctx: &GeneratorContext<'_>, root: &Item, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](#generatorcontext), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- <span id="generator-generate-flat-recursive-capture"></span>`fn generate_flat_recursive_capture(ctx: &GeneratorContext<'_>, item: &Item, prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](#generatorcontext), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- <span id="generator-generate-nested-to-capture"></span>`fn generate_nested_to_capture(ctx: &GeneratorContext<'_>, root: &Item, path_prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](#generatorcontext), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- <span id="generator-run"></span>`fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error>` — [`Args`](../index.md), [`Error`](../error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for Generator<'a>`

##### `impl<T> IntoEither for Generator<'a>`

##### `impl<D> OwoColorize for Generator<'a>`

##### `impl<T> Pointable for Generator<'a>`

- <span id="generator-align"></span>`const ALIGN: usize`

- <span id="generator-init"></span>`type Init = T`

- <span id="generator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for Generator<'a>`

## Traits

## Functions

