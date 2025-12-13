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
  - [`source`](#source)
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
  - [`CollectSourcesArgs`](#collectsourcesargs)
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
| [`source`](#source) | mod | Source code parsing for enhanced documentation. |
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
| [`CollectSourcesArgs`](#collectsourcesargs) | struct | Arguments for the `collect-sources` subcommand. |
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
- [`source`](source/index.md) — Source code parsing for enhanced documentation.
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

  Create a new generator for the given crate and arguments.

  

  This initializes the shared context including:

  - Path map (item ID → module path)

  - Impl map (type ID → impl blocks)

  - Link registry for cross-references

  

  # Arguments

  

  * `krate` - The parsed rustdoc JSON crate

  * `args` - CLI arguments containing output path, format, and options

  * `config` - Rendering configuration options

  

  # Errors

  

  Returns an error if the root item cannot be found in the crate index.

- <span id="generator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](error/index.md#error)

  Generate markdown documentation.

  

  This is the main entry point for documentation generation. It:

  

  1. Creates the output directory

  2. Sets up a progress bar

  3. Dispatches to the format-specific generator (flat or nested)

  

  # Errors

  

  Returns an error if any file operation fails.

- <span id="generator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](error/index.md#error)

  Create a progress bar for user feedback.

  

  # Errors

  

  Returns an error if the progress bar template is invalid.

- <span id="generator-generate-to-capture"></span>`fn generate_to_capture(krate: &Crate, format: CliOutputFormat, include_private: bool) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](#clioutputformat), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

  Generate documentation to memory instead of disk.

  

  This function mirrors `generate()` but captures all output in a

  `MarkdownCapture` struct instead of writing to the filesystem.

  Useful for testing and programmatic access to generated docs.

  

  # Arguments

  

  * `krate` - The parsed rustdoc JSON crate

  * `format` - Output format (Flat or Nested)

  * `include_private` - Whether to include private items

  

  # Returns

  

  A `MarkdownCapture` containing all generated markdown files.

  

  # Errors

  

  Returns an error if the root item cannot be found in the crate index.

- <span id="generator-generate-to-capture-with-config"></span>`fn generate_to_capture_with_config(krate: &Crate, format: CliOutputFormat, include_private: bool, config: RenderConfig) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](#clioutputformat), [`RenderConfig`](generator/config/index.md#renderconfig), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

  Generate markdown to an in-memory capture with custom configuration.

  

  This variant allows specifying a custom [`RenderConfig`](generator/config/index.md) for testing

  different rendering options like `hide_trivial_derives`.

  

  # Arguments

  

  * `krate` - The parsed rustdoc JSON crate

  * `format` - Output format (Flat or Nested)

  * `include_private` - Whether to include private items

  * `config` - Custom rendering configuration

  

  # Returns

  

  A `MarkdownCapture` containing all generated markdown files.

  

  # Errors

  

  Returns an error if the root item cannot be found in the crate index.

- <span id="generator-generate-flat-to-capture"></span>`fn generate_flat_to_capture(ctx: &GeneratorContext<'_>, root: &Item, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/context/index.md#generatorcontext), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

  Generate flat structure to capture.

- <span id="generator-generate-flat-recursive-capture"></span>`fn generate_flat_recursive_capture(ctx: &GeneratorContext<'_>, item: &Item, prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/context/index.md#generatorcontext), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

  Recursive flat generation to capture.

- <span id="generator-generate-nested-to-capture"></span>`fn generate_nested_to_capture(ctx: &GeneratorContext<'_>, root: &Item, path_prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](generator/context/index.md#generatorcontext), [`MarkdownCapture`](generator/capture/index.md#markdowncapture), [`Error`](error/index.md#error)

  Generate nested structure to capture.

- <span id="generator-run"></span>`fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error>` — [`Args`](#args), [`Error`](error/index.md#error)

  Convenience method to generate documentation in one call.

  

  Creates a `Generator` and runs it immediately. For more control

  over the generation process, use `new()` and `generate()` separately.

  

  Uses default `RenderConfig`. For custom configuration, use `new()` directly.

  

  # Arguments

  

  * `krate` - The parsed rustdoc JSON crate

  * `args` - CLI arguments containing output path, format, and options

  

  # Returns

  

  `Ok(())` on success, or an error if any file operation fails.

  

  # Errors

  

  Returns an error if the root item cannot be found or if file operations fail.

#### Trait Implementations

##### `impl Any for Generator<'a>`

- <span id="generator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Generator<'a>`

- <span id="generator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Generator<'a>`

- <span id="generator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Generator<'a>`

- <span id="generator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for Generator<'a>`

##### `impl<U> Into for Generator<'a>`

- <span id="generator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Generator<'a>`

##### `impl OwoColorize for Generator<'a>`

##### `impl Pointable for Generator<'a>`

- <span id="generator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="generator-pointable-type-init"></span>`type Init = T`

- <span id="generator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Generator<'a>`

- <span id="generator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="generator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Generator<'a>`

- <span id="generator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="generator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new empty capture.

- <span id="markdowncapture-insert"></span>`fn insert(&mut self, path: String, content: String)`

  Add a file to the capture.

  

  # Arguments

  * `path` - Relative path of the file (e.g., "index.md" or "span/index.md")

  * `content` - The markdown content for this file

- <span id="markdowncapture-get"></span>`fn get(&self, path: &str) -> Option<&String>`

  Get the content of a specific file.

- <span id="markdowncapture-paths"></span>`fn paths(&self) -> Vec<&String>`

  Get all file paths in sorted order.

- <span id="markdowncapture-len"></span>`fn len(&self) -> usize`

  Get the number of captured files.

- <span id="markdowncapture-is-empty"></span>`fn is_empty(&self) -> bool`

  Check if the capture is empty.

- <span id="markdowncapture-to-snapshot-string"></span>`fn to_snapshot_string(&self) -> String`

  Convert all captured files to a single string for snapshot testing.

  

  Files are sorted by path and separated with clear headers.

- <span id="markdowncapture-into-inner"></span>`fn into_inner(self) -> HashMap<String, String>`

  Consume self and return the underlying `HashMap`.

#### Trait Implementations

##### `impl Any for MarkdownCapture`

- <span id="markdowncapture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MarkdownCapture`

- <span id="markdowncapture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MarkdownCapture`

- <span id="markdowncapture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for MarkdownCapture`

- <span id="markdowncapture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MarkdownCapture`

- <span id="markdowncapture-default"></span>`fn default() -> MarkdownCapture` — [`MarkdownCapture`](generator/capture/index.md#markdowncapture)

##### `impl<T> From for MarkdownCapture`

- <span id="markdowncapture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MarkdownCapture`

##### `impl<U> Into for MarkdownCapture`

- <span id="markdowncapture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MarkdownCapture`

##### `impl OwoColorize for MarkdownCapture`

##### `impl Pointable for MarkdownCapture`

- <span id="markdowncapture-pointable-const-align"></span>`const ALIGN: usize`

- <span id="markdowncapture-pointable-type-init"></span>`type Init = T`

- <span id="markdowncapture-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="markdowncapture-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="markdowncapture-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="markdowncapture-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MarkdownCapture`

- <span id="markdowncapture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="markdowncapture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MarkdownCapture`

- <span id="markdowncapture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="markdowncapture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MarkdownCapture`

### `RenderConfig`

```rust
struct RenderConfig {
    pub toc_threshold: usize,
    pub quick_reference: bool,
    pub group_impls: bool,
    pub hide_trivial_derives: bool,
    pub method_anchors: bool,
    pub full_method_docs: bool,
    pub include_source: SourceConfig,
}
```

*Defined in `src/generator/config.rs:14-39`*

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

- **`full_method_docs`**: `bool`

  Include full method documentation instead of first-paragraph summaries.
  
  When `false` (default), method docs in impl blocks show only the first
  paragraph (up to the first blank line). When `true`, the complete
  documentation is included.

- **`include_source`**: `SourceConfig`

  Source code integration options.

#### Trait Implementations

##### `impl Any for RenderConfig`

- <span id="renderconfig-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RenderConfig`

- <span id="renderconfig-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RenderConfig`

- <span id="renderconfig-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RenderConfig`

- <span id="renderconfig-clone"></span>`fn clone(&self) -> RenderConfig` — [`RenderConfig`](generator/config/index.md#renderconfig)

##### `impl CloneToUninit for RenderConfig`

- <span id="renderconfig-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RenderConfig`

- <span id="renderconfig-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RenderConfig`

- <span id="renderconfig-default"></span>`fn default() -> Self`

##### `impl<T> From for RenderConfig`

- <span id="renderconfig-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for RenderConfig`

##### `impl<U> Into for RenderConfig`

- <span id="renderconfig-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RenderConfig`

##### `impl OwoColorize for RenderConfig`

##### `impl Pointable for RenderConfig`

- <span id="renderconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="renderconfig-pointable-type-init"></span>`type Init = T`

- <span id="renderconfig-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="renderconfig-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="renderconfig-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="renderconfig-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for RenderConfig`

- <span id="renderconfig-toowned-type-owned"></span>`type Owned = T`

- <span id="renderconfig-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="renderconfig-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RenderConfig`

- <span id="renderconfig-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="renderconfig-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RenderConfig`

- <span id="renderconfig-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="renderconfig-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in `src/generator/config.rs:49-68`*

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

##### `impl Any for SourceConfig`

- <span id="sourceconfig-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SourceConfig`

- <span id="sourceconfig-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SourceConfig`

- <span id="sourceconfig-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SourceConfig`

- <span id="sourceconfig-clone"></span>`fn clone(&self) -> SourceConfig` — [`SourceConfig`](generator/config/index.md#sourceconfig)

##### `impl CloneToUninit for SourceConfig`

- <span id="sourceconfig-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SourceConfig`

- <span id="sourceconfig-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SourceConfig`

- <span id="sourceconfig-default"></span>`fn default() -> SourceConfig` — [`SourceConfig`](generator/config/index.md#sourceconfig)

##### `impl<T> From for SourceConfig`

- <span id="sourceconfig-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SourceConfig`

##### `impl<U> Into for SourceConfig`

- <span id="sourceconfig-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SourceConfig`

##### `impl OwoColorize for SourceConfig`

##### `impl Pointable for SourceConfig`

- <span id="sourceconfig-pointable-const-align"></span>`const ALIGN: usize`

- <span id="sourceconfig-pointable-type-init"></span>`type Init = T`

- <span id="sourceconfig-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="sourceconfig-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="sourceconfig-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="sourceconfig-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for SourceConfig`

- <span id="sourceconfig-toowned-type-owned"></span>`type Owned = T`

- <span id="sourceconfig-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sourceconfig-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SourceConfig`

- <span id="sourceconfig-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sourceconfig-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SourceConfig`

- <span id="sourceconfig-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sourceconfig-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SourceConfig`

### `AnchorUtils`

```rust
struct AnchorUtils;
```

*Defined in `src/linker.rs:68`*

Utilify functions to handle anchors

#### Implementations

- <span id="anchorutils-assoc-item-anchor"></span>`fn assoc_item_anchor(type_name: &str, item_name: &str, kind: AssocItemKind) -> String` — [`AssocItemKind`](linker/index.md#associtemkind)

  Generate a compound anchor for an associated item on a type.

  

  This creates a unique anchor that combines the type name, item kind, and item name,

  enabling deep linking to specific items. The format is `typename-itemname` for methods

  (backward compatible), and `typename-kind-itemname` for constants and types to avoid

  collisions.

  

  # Arguments

  

  * `type_name` - The name of the type (struct, enum, trait, etc.)

  * `item_name` - The name of the method or associated item

  * `kind` - The kind of associated item (method, const, or type)

  

  # Examples

  

  ```ignore

  assert_eq!(assoc_item_anchor("Parser", "parse", AssocItemKind::Method), "parser-parse");

  assert_eq!(assoc_item_anchor("HashMap", "new", AssocItemKind::Method), "hashmap-new");

  assert_eq!(assoc_item_anchor("Vec", "Item", AssocItemKind::Type), "vec-type-item");

  assert_eq!(assoc_item_anchor("Vec", "ALIGN", AssocItemKind::Const), "vec-const-align");

  ```

- <span id="anchorutils-method-anchor"></span>`fn method_anchor(type_name: &str, method_name: &str) -> String`

  Generate a compound anchor for a method on a type.

  

  This creates a unique anchor that combines the type name and method name,

  enabling deep linking to specific methods. The format is `typename-methodname`,

  where both parts are slugified.

  

  # Arguments

  

  * `type_name` - The name of the type (struct, enum, trait, etc.)

  * `method_name` - The name of the method or associated item

  

  # Examples

  

  ```ignore

  assert_eq!(method_anchor("Parser", "parse"), "parser-parse");

  assert_eq!(method_anchor("HashMap", "new"), "hashmap-new");

  assert_eq!(method_anchor("Vec<T>", "push"), "vec-push");

  ```

- <span id="anchorutils-impl-item-anchor"></span>`fn impl_item_anchor(type_name: &str, item_name: &str, kind: AssocItemKind, impl_ctx: ImplContext<'_>) -> String` — [`AssocItemKind`](linker/index.md#associtemkind), [`ImplContext`](linker/index.md#implcontext)

  Generate an anchor for an associated item in an impl block, with trait disambiguation.

  

  This extends `assoc_item_anchor` to handle trait impls, where multiple traits

  may define the same associated type (e.g., `Output` in both `Add` and `Sub`).

  For trait impls, the trait name is included in the anchor to ensure uniqueness.

  

  # Arguments

  

  * `type_name` - The name of the implementing type

  * `item_name` - The name of the associated item

  * `kind` - The kind of associated item

  * `impl_ctx` - Whether this is an inherent or trait impl

  

  # Anchor Formats

  

  | Context | Kind | Format | Example |

  |---------|------|--------|---------|

  | Inherent | Method | `type-method` | `vec-push` |

  | Inherent | Type | `type-type-item` | `vec-type-item` |

  | Inherent | Const | `type-const-item` | `vec-const-align` |

  | Trait(Add) | Method | `type-add-method` | `vec-add-add` |

  | Trait(Add) | Type | `type-add-type-item` | `vec-add-type-output` |

  | Trait(Add) | Const | `type-add-const-item` | `vec-add-const-max` |

- <span id="anchorutils-slugify-anchor"></span>`fn slugify_anchor(name: &str) -> String`

  Convert a name to a GitHub-style markdown anchor slug.

  

  This normalizes item names to match the anchor IDs generated by markdown

  renderers (GitHub, mdBook, etc.) when they process headings.

  

  # Rules Applied

  

  1. Apply Unicode NFC normalization (canonical composition)

  2. Convert to lowercase (full Unicode, not just ASCII)

  3. Remove backticks (markdown code formatting)

  4. Remove generics (`<T>`, `<K, V>`) by stripping `<...>` content

  5. Replace spaces and underscores with hyphens

  6. Remove non-alphanumeric characters (except hyphens)

  7. Collapse consecutive hyphens

  8. Trim leading/trailing hyphens

  

  # Examples

  

  ```ignore

  assert_eq!(slugify_anchor("HashMap"), "hashmap");

  assert_eq!(slugify_anchor("HashMap<K, V>"), "hashmap");

  assert_eq!(slugify_anchor("my_function"), "my-function");

  assert_eq!(slugify_anchor("Into<T>"), "into");

  assert_eq!(slugify_anchor("Größe"), "größe");

  ```

- <span id="anchorutils-slugify-anchor-ascii"></span>`fn slugify_anchor_ascii(name: &str) -> String`

  Fast ASCII-only slugification (no allocation for normalization).

- <span id="anchorutils-slugify-anchor-impl"></span>`fn slugify_anchor_impl(name: &str) -> String`

  Unicode-aware slugification with full lowercase support.

- <span id="anchorutils-item-has-anchor"></span>`const fn item_has_anchor(kind: ItemKind) -> bool`

  Check if an item kind generates a heading anchor in markdown.

  

  Only certain item types get `### \`Name\` headings in the generated output.

  Other items (methods, fields, variants) are rendered as bullet points

  without heading anchors.

  

  # Items with anchors

  

  - Struct, Enum, Trait, Function, Constant, `TypeAlias`, Macro, Module

  

  # Items without anchors

  

  - Methods (in impl blocks)

  - Struct fields

  - Enum variants

  - Associated types/constants

  - Trait methods

#### Trait Implementations

##### `impl Any for AnchorUtils`

- <span id="anchorutils-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AnchorUtils`

- <span id="anchorutils-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AnchorUtils`

- <span id="anchorutils-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AnchorUtils`

- <span id="anchorutils-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for AnchorUtils`

##### `impl<U> Into for AnchorUtils`

- <span id="anchorutils-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for AnchorUtils`

##### `impl OwoColorize for AnchorUtils`

##### `impl Pointable for AnchorUtils`

- <span id="anchorutils-pointable-const-align"></span>`const ALIGN: usize`

- <span id="anchorutils-pointable-type-init"></span>`type Init = T`

- <span id="anchorutils-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="anchorutils-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="anchorutils-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="anchorutils-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for AnchorUtils`

- <span id="anchorutils-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anchorutils-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AnchorUtils`

- <span id="anchorutils-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anchorutils-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for AnchorUtils`

### `LinkRegistry`

```rust
struct LinkRegistry {
    item_paths: std::collections::HashMap<rustdoc_types::Id, String>,
    item_names: std::collections::HashMap<rustdoc_types::Id, String>,
}
```

*Defined in `src/linker.rs:346-358`*

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

  Build a link registry by traversing all items in the crate.

  

  This function walks the module tree starting from the root and records

  the file path where each item will be documented. The paths depend on

  the output format (flat vs nested).

  

  # Arguments

  

  * `krate` - The parsed rustdoc crate containing all items

  * `flat_format` - If true, use flat paths (`mod.md`); if false, use nested (`mod/index.md`)

  * `include_private` - If true, include non-public items; if false, only public items

  

  # Returns

  

  A populated `LinkRegistry` ready for link creation.

  

  # Algorithm

  

  1. Start at the crate root module

  2. For each top-level module: register it and recursively process children

  3. For structs/enums/traits at root level: register them to `index.md`

  4. Other items (functions, constants) are registered within their module's file

  5. Items are filtered by visibility unless `include_private` is true

- <span id="linkregistry-register-module-items"></span>`fn register_module_items(&mut self, krate: &Crate, module_id: Id, module_item: &rustdoc_types::Item, path: &str, module_prefix: &str, flat_format: bool, include_private: bool)`

  Recursively register all items within a module.

  

  This is called for each module in the crate to populate the registry

  with all items that can be linked to.

  

  # Arguments

  

  * `krate` - The full crate for looking up item details

  * `module_id` - ID of the module being registered

  * `module_item` - The module's Item data

  * `path` - File path where this module's docs will be written

  * `module_prefix` - Prefix for building child paths (e.g., "parent" or "`parent__child`")

  * `flat_format` - Whether to use flat naming convention

  * `include_private` - Whether to include non-public items

- <span id="linkregistry-register-glob-items"></span>`fn register_glob_items(&mut self, krate: &Crate, use_item: &rustdoc_types::Use, path: &str, include_private: bool)`

  Register items from a glob re-export target module.

- <span id="linkregistry-get-path"></span>`fn get_path(&self, id: Id) -> Option<&String>`

  Get the file path where an item is documented.

  

  # Arguments

  

  * `id` - The item's unique ID from rustdoc JSON

  

  # Returns

  

  The relative file path (e.g., `"span.md"` or `"span/index.md"`),

  or `None` if the item isn't registered.

- <span id="linkregistry-get-name"></span>`fn get_name(&self, id: Id) -> Option<&String>`

  Get the display name for an item.

  

  # Arguments

  

  * `id` - The item's unique ID from rustdoc JSON

  

  # Returns

  

  The item's name for display in links (e.g., `"Span"`),

  or `None` if the item isn't registered.

- <span id="linkregistry-create-link"></span>`fn create_link(&self, id: Id, from_path: &str) -> Option<String>`

  Create a markdown link to an item from a given source file.

  

  This is the main method used during markdown generation to create

  clickable links between documented items.

  

  # Arguments

  

  * `id` - The target item's ID

  * `from_path` - The source file creating the link (e.g., `"index.md"`)

  

  # Returns

  

  A formatted markdown link like `[``ItemName``](path/to/file.md)`,

  or `None` if the target item isn't registered.

  

  # Link Types

  

  - **Same file**: Returns an anchor link (`#itemname`)

  - **Different file**: Returns a relative path (`../other/file.md`)

  

  # Example

  

  ```ignore

  // From index.md linking to span.md

  registry.create_link(&span_id, "index.md")

  // Returns: Some("[`Span`](span.md)")

  

  // From span/index.md linking to index.md

  registry.create_link(&root_id, "span/index.md")

  // Returns: Some("[`crate`](../index.md)")

  ```

- <span id="linkregistry-compute-relative-path"></span>`fn compute_relative_path(from: &str, to: &str) -> String`

  Compute the relative path from one file to another.

  

  This function calculates the relative path needed to navigate from

  one markdown file to another within the generated documentation.

  Uses `pathdiff` for robust cross-platform path calculation.

  

  # Arguments

  

  * `from` - The source file path (e.g., `"span/index.md"`)

  * `to` - The target file path (e.g., `"field/index.md"`)

  

  # Returns

  

  A relative path string (e.g., `"../field/index.md"`)

  

  # Examples

  

  - Same directory: `"index.md"` → `"span.md"` = `"span.md"`

  - Into subdirectory: `"index.md"` → `"span/index.md"` = `"span/index.md"`

  - Up to parent: `"span/index.md"` → `"index.md"` = `"../index.md"`

  - Sibling directory: `"span/index.md"` → `"field/index.md"` = `"../field/index.md"`

#### Trait Implementations

##### `impl Any for LinkRegistry`

- <span id="linkregistry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LinkRegistry`

- <span id="linkregistry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LinkRegistry`

- <span id="linkregistry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LinkRegistry`

- <span id="linkregistry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LinkRegistry`

- <span id="linkregistry-default"></span>`fn default() -> LinkRegistry` — [`LinkRegistry`](linker/index.md#linkregistry)

##### `impl<T> From for LinkRegistry`

- <span id="linkregistry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for LinkRegistry`

##### `impl<U> Into for LinkRegistry`

- <span id="linkregistry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for LinkRegistry`

##### `impl OwoColorize for LinkRegistry`

##### `impl Pointable for LinkRegistry`

- <span id="linkregistry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="linkregistry-pointable-type-init"></span>`type Init = T`

- <span id="linkregistry-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="linkregistry-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="linkregistry-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="linkregistry-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for LinkRegistry`

- <span id="linkregistry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="linkregistry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LinkRegistry`

- <span id="linkregistry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="linkregistry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

- <span id="cratecollection-default"></span>`fn default() -> CrateCollection` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

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

- <span id="multicratecontext-source-path-config-for-file"></span>`fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig>` — [`SourcePathConfig`](generator/render_shared/index.md#sourcepathconfig)

  Get source path config for a specific file.

  

  Returns `None` if source locations are disabled or no source dir configured.

- <span id="multicratecontext-build-cross-crate-impls"></span>`fn build_cross_crate_impls(crates: &'a CrateCollection) -> HashMap<String, HashMap<String, Vec<&'a Impl>>>` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

  Build the cross-crate impl map for all crates.

  

  Scans all crates once and groups impl blocks by their target crate

  and type name. This avoids O(n*m) scanning per view creation.

- <span id="multicratecontext-crates"></span>`const fn crates(&self) -> &CrateCollection` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection)

  Get the crate collection.

- <span id="multicratecontext-registry"></span>`const fn registry(&self) -> &UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](multi_crate/registry/index.md#unifiedlinkregistry)

  Get the unified link registry.

- <span id="multicratecontext-args"></span>`const fn args(&self) -> &Args` — [`Args`](#args)

  Get CLI arguments.

- <span id="multicratecontext-single-crate-view"></span>`fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview)

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

- <span id="multicrategenerator-new"></span>`fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection), [`Args`](#args), [`RenderConfig`](generator/config/index.md#renderconfig)

  Create a new multi-crate generator.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `args` - CLI arguments

  * `config` - Rendering configuration options

- <span id="multicrategenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](error/index.md#error)

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

- <span id="multicrategenerator-collect-crate-items"></span>`fn collect_crate_items(view: &SingleCrateView<'_>, ids: &mut HashSet<Id>)` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview)

  Collect rendered item IDs for a single crate.

- <span id="multicrategenerator-collect-module-items"></span>`fn collect_module_items(view: &SingleCrateView<'_>, item: &Item, ids: &mut HashSet<Id>)` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview)

  Recursively collect rendered item IDs from a module.

- <span id="multicrategenerator-generate-crate"></span>`fn generate_crate(&self, view: &SingleCrateView<'_>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview), [`Error`](error/index.md#error)

  Generate documentation for a single crate.

- <span id="multicrategenerator-generate-module"></span>`fn generate_module(view: &SingleCrateView<'_>, item: &Item, parent_dir: &Path, module_path: Vec<String>, progress: &Arc<ProgressBar>) -> Result<(), Error>` — [`SingleCrateView`](multi_crate/context/index.md#singlecrateview), [`Error`](error/index.md#error)

  Generate a module directory with index.md and child modules.

- <span id="multicrategenerator-create-progress-bar"></span>`fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](error/index.md#error)

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

- <span id="multicrateparser-parse-directory"></span>`fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>` — [`CrateCollection`](multi_crate/collection/index.md#cratecollection), [`Error`](error/index.md#error)

  Parse all rustdoc JSON files in a directory.

  

  Scans the top level of the directory for `*.json` files and

  attempts to parse each one as rustdoc JSON. Files that aren't

  valid rustdoc JSON (e.g., search indices) are silently skipped.

  

  # Arguments

  

  * `dir` - Path to directory containing JSON files

  

  # Returns

  

  A `CrateCollection` containing all successfully parsed crates.

  

  # Errors

  

  - [`Error::InvalidDirectory`](#errorinvaliddirectory) if the path is invalid

  - [`Error::NoJsonFiles`](#errornojsonfiles) if no valid JSON files found

  - [`Error::DuplicateCrate`](#errorduplicatecrate) if multiple files define the same crate

  - [`Error::NoCrateName`](#errornocratename) if a JSON file has no root module

- <span id="multicrateparser-extract-crate-name"></span>`fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error>` — [`Error`](error/index.md#error)

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

  Create a new search index generator.

  

  # Arguments

  

  * `crates` - Collection of parsed crates to index

  * `include_private` - Whether to include non-public items

  * `rendered_items` - Map of crate name to set of rendered item IDs

- <span id="searchindexgenerator-generate"></span>`fn generate(&self) -> SearchIndex` — [`SearchIndex`](multi_crate/search/index.md#searchindex)

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

- <span id="searchindexgenerator-index-crate"></span>`fn index_crate(&self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate)` — [`SearchEntry`](multi_crate/search/index.md#searchentry)

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

- <span id="unifiedlinkregistry-default"></span>`fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](multi_crate/registry/index.md#unifiedlinkregistry)

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

##### `impl Any for Cli`

- <span id="cli-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Args for Cli`

- <span id="cli-args-group-id"></span>`fn group_id() -> Option<clap::Id>`

- <span id="cli-args-augment-args"></span>`fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="cli-args-augment-args-for-update"></span>`fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl<T> Borrow for Cli`

- <span id="cli-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cli`

- <span id="cli-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl CommandFactory for Cli`

- <span id="cli-commandfactory-command"></span>`fn command<'b>() -> clap::Command`

- <span id="cli-commandfactory-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for Cli`

- <span id="cli-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Cli`

- <span id="cli-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromArgMatches for Cli`

- <span id="cli-fromargmatches-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cli-fromargmatches-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cli-fromargmatches-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="cli-fromargmatches-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for Cli`

##### `impl<U> Into for Cli`

- <span id="cli-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Cli`

##### `impl OwoColorize for Cli`

##### `impl Parser for Cli`

##### `impl Pointable for Cli`

- <span id="cli-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cli-pointable-type-init"></span>`type Init = T`

- <span id="cli-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cli-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cli-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cli-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Cli`

- <span id="cli-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cli-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cli`

- <span id="cli-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cli-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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
    pub toc_threshold: usize,
    pub no_quick_reference: bool,
    pub no_group_impls: bool,
    pub hide_trivial_derives: bool,
    pub no_method_anchors: bool,
    pub source_locations: bool,
    pub full_method_docs: bool,
    pub cargo_args: Vec<String>,
}
```

*Defined in `src/lib.rs:128-208`*

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

- **`toc_threshold`**: `usize`

  Minimum number of items before generating a table of contents.
  
  Modules with fewer items than this threshold won't have a TOC.
  Default: 10

- **`no_quick_reference`**: `bool`

  Disable quick reference tables at the top of modules.

- **`no_group_impls`**: `bool`

  Disable grouping impl blocks by category (Derive, Conversion, etc.).

- **`hide_trivial_derives`**: `bool`

  Hide trivial derive implementations (Clone, Copy, Debug, etc.).

- **`no_method_anchors`**: `bool`

  Disable method-level anchors for deep linking.

- **`source_locations`**: `bool`

  Include source file locations for items.

- **`full_method_docs`**: `bool`

  Include full method documentation instead of first-line summaries.
  
  By default, only the first paragraph of method docs is shown in impl blocks.
  Enable this to include the complete documentation for each method.

- **`cargo_args`**: `Vec<String>`

  Additional arguments to pass to cargo doc.
  
  Example: `docs-md docs -- --all-features`

#### Trait Implementations

##### `impl Any for DocsArgs`

- <span id="docsargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Args for DocsArgs`

- <span id="docsargs-args-group-id"></span>`fn group_id() -> Option<clap::Id>`

- <span id="docsargs-args-augment-args"></span>`fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="docsargs-args-augment-args-for-update"></span>`fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl<T> Borrow for DocsArgs`

- <span id="docsargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DocsArgs`

- <span id="docsargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl CommandFactory for DocsArgs`

- <span id="docsargs-commandfactory-command"></span>`fn command<'b>() -> clap::Command`

- <span id="docsargs-commandfactory-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for DocsArgs`

- <span id="docsargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DocsArgs`

- <span id="docsargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromArgMatches for DocsArgs`

- <span id="docsargs-fromargmatches-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="docsargs-fromargmatches-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="docsargs-fromargmatches-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="docsargs-fromargmatches-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for DocsArgs`

##### `impl<U> Into for DocsArgs`

- <span id="docsargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for DocsArgs`

##### `impl OwoColorize for DocsArgs`

##### `impl Parser for DocsArgs`

##### `impl Pointable for DocsArgs`

- <span id="docsargs-pointable-const-align"></span>`const ALIGN: usize`

- <span id="docsargs-pointable-type-init"></span>`type Init = T`

- <span id="docsargs-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="docsargs-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="docsargs-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="docsargs-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for DocsArgs`

- <span id="docsargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="docsargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DocsArgs`

- <span id="docsargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="docsargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for DocsArgs`

### `CollectSourcesArgs`

```rust
struct CollectSourcesArgs {
    pub output: Option<std::path::PathBuf>,
    pub include_dev: bool,
    pub dry_run: bool,
    pub manifest_path: Option<std::path::PathBuf>,
}
```

*Defined in `src/lib.rs:213-233`*

Arguments for the `collect-sources` subcommand.

#### Fields

- **`output`**: `Option<std::path::PathBuf>`

  Output directory for collected sources.
  
  If not specified, creates `.source_{timestamp}/` in the workspace root.

- **`include_dev`**: `bool`

  Include dev-dependencies in collection.
  
  By default, only regular dependencies are collected.

- **`dry_run`**: `bool`

  Dry run - show what would be collected without copying.

- **`manifest_path`**: `Option<std::path::PathBuf>`

  Path to Cargo.toml (defaults to current directory).

#### Trait Implementations

##### `impl Any for CollectSourcesArgs`

- <span id="collectsourcesargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Args for CollectSourcesArgs`

- <span id="collectsourcesargs-args-group-id"></span>`fn group_id() -> Option<clap::Id>`

- <span id="collectsourcesargs-args-augment-args"></span>`fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="collectsourcesargs-args-augment-args-for-update"></span>`fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl<T> Borrow for CollectSourcesArgs`

- <span id="collectsourcesargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CollectSourcesArgs`

- <span id="collectsourcesargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl CommandFactory for CollectSourcesArgs`

- <span id="collectsourcesargs-commandfactory-command"></span>`fn command<'b>() -> clap::Command`

- <span id="collectsourcesargs-commandfactory-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for CollectSourcesArgs`

- <span id="collectsourcesargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CollectSourcesArgs`

- <span id="collectsourcesargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromArgMatches for CollectSourcesArgs`

- <span id="collectsourcesargs-fromargmatches-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="collectsourcesargs-fromargmatches-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="collectsourcesargs-fromargmatches-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="collectsourcesargs-fromargmatches-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for CollectSourcesArgs`

##### `impl<U> Into for CollectSourcesArgs`

- <span id="collectsourcesargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CollectSourcesArgs`

##### `impl OwoColorize for CollectSourcesArgs`

##### `impl Parser for CollectSourcesArgs`

##### `impl Pointable for CollectSourcesArgs`

- <span id="collectsourcesargs-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectsourcesargs-pointable-type-init"></span>`type Init = T`

- <span id="collectsourcesargs-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectsourcesargs-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectsourcesargs-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectsourcesargs-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CollectSourcesArgs`

- <span id="collectsourcesargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collectsourcesargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CollectSourcesArgs`

- <span id="collectsourcesargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collectsourcesargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for CollectSourcesArgs`

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
    pub toc_threshold: usize,
    pub no_quick_reference: bool,
    pub no_group_impls: bool,
    pub hide_trivial_derives: bool,
    pub no_method_anchors: bool,
    pub source_locations: bool,
    pub full_method_docs: bool,
}
```

*Defined in `src/lib.rs:245-367`*

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

- **`toc_threshold`**: `usize`

  Minimum number of items before generating a table of contents.
  
  Modules with fewer items than this threshold won't have a TOC.
  Default: 10

- **`no_quick_reference`**: `bool`

  Disable quick reference tables at the top of modules.

- **`no_group_impls`**: `bool`

  Disable grouping impl blocks by category (Derive, Conversion, etc.).

- **`hide_trivial_derives`**: `bool`

  Hide trivial derive implementations (Clone, Copy, Debug, etc.).

- **`no_method_anchors`**: `bool`

  Disable method-level anchors for deep linking.

- **`source_locations`**: `bool`

  Include source file locations for items.

- **`full_method_docs`**: `bool`

  Include full method documentation instead of first-line summaries.
  
  By default, only the first paragraph of method docs is shown in impl blocks.
  Enable this to include the complete documentation for each method.

#### Trait Implementations

##### `impl Any for GenerateArgs`

- <span id="generateargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Args for GenerateArgs`

- <span id="generateargs-args-group-id"></span>`fn group_id() -> Option<clap::Id>`

- <span id="generateargs-args-augment-args"></span>`fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="generateargs-args-augment-args-for-update"></span>`fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl<T> Borrow for GenerateArgs`

- <span id="generateargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GenerateArgs`

- <span id="generateargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl CommandFactory for GenerateArgs`

- <span id="generateargs-commandfactory-command"></span>`fn command<'b>() -> clap::Command`

- <span id="generateargs-commandfactory-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for GenerateArgs`

- <span id="generateargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GenerateArgs`

- <span id="generateargs-default"></span>`fn default() -> GenerateArgs` — [`GenerateArgs`](#generateargs)

##### `impl<T> From for GenerateArgs`

- <span id="generateargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromArgMatches for GenerateArgs`

- <span id="generateargs-fromargmatches-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="generateargs-fromargmatches-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="generateargs-fromargmatches-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="generateargs-fromargmatches-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for GenerateArgs`

##### `impl<U> Into for GenerateArgs`

- <span id="generateargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for GenerateArgs`

##### `impl OwoColorize for GenerateArgs`

##### `impl Parser for GenerateArgs`

##### `impl Pointable for GenerateArgs`

- <span id="generateargs-pointable-const-align"></span>`const ALIGN: usize`

- <span id="generateargs-pointable-type-init"></span>`type Init = T`

- <span id="generateargs-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="generateargs-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="generateargs-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="generateargs-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for GenerateArgs`

- <span id="generateargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="generateargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GenerateArgs`

- <span id="generateargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="generateargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

##### `impl Any for OutputFormat`

- <span id="outputformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OutputFormat`

- <span id="outputformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OutputFormat`

- <span id="outputformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OutputFormat`

- <span id="outputformat-clone"></span>`fn clone(&self) -> OutputFormat` — [`OutputFormat`](#outputformat)

##### `impl CloneToUninit for OutputFormat`

- <span id="outputformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for OutputFormat`

##### `impl Debug for OutputFormat`

- <span id="outputformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OutputFormat`

- <span id="outputformat-default"></span>`fn default() -> OutputFormat` — [`OutputFormat`](#outputformat)

##### `impl<T> From for OutputFormat`

- <span id="outputformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for OutputFormat`

##### `impl<U> Into for OutputFormat`

- <span id="outputformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for OutputFormat`

##### `impl OwoColorize for OutputFormat`

##### `impl Pointable for OutputFormat`

- <span id="outputformat-pointable-const-align"></span>`const ALIGN: usize`

- <span id="outputformat-pointable-type-init"></span>`type Init = T`

- <span id="outputformat-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="outputformat-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="outputformat-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="outputformat-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for OutputFormat`

- <span id="outputformat-toowned-type-owned"></span>`type Owned = T`

- <span id="outputformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="outputformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OutputFormat`

- <span id="outputformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="outputformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OutputFormat`

- <span id="outputformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="outputformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl ValueEnum for OutputFormat`

- <span id="outputformat-valueenum-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="outputformat-valueenum-to-possible-value"></span>`fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>`

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

##### `impl Any for Cargo`

- <span id="cargo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cargo`

- <span id="cargo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cargo`

- <span id="cargo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl CommandFactory for Cargo`

- <span id="cargo-commandfactory-command"></span>`fn command<'b>() -> clap::Command`

- <span id="cargo-commandfactory-command-for-update"></span>`fn command_for_update<'b>() -> clap::Command`

##### `impl Debug for Cargo`

- <span id="cargo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Cargo`

- <span id="cargo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromArgMatches for Cargo`

- <span id="cargo-fromargmatches-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cargo-fromargmatches-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="cargo-fromargmatches-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="cargo-fromargmatches-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut<'b>(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for Cargo`

##### `impl<U> Into for Cargo`

- <span id="cargo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Cargo`

##### `impl OwoColorize for Cargo`

##### `impl Parser for Cargo`

##### `impl Pointable for Cargo`

- <span id="cargo-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cargo-pointable-type-init"></span>`type Init = T`

- <span id="cargo-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cargo-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cargo-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cargo-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Subcommand for Cargo`

- <span id="cargo-subcommand-augment-subcommands"></span>`fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="cargo-subcommand-augment-subcommands-for-update"></span>`fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="cargo-subcommand-has-subcommand"></span>`fn has_subcommand(__clap_name: &str) -> bool`

##### `impl<U> TryFrom for Cargo`

- <span id="cargo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cargo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cargo`

- <span id="cargo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cargo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for Cargo`

### `Command`

```rust
enum Command {
    Docs(DocsArgs),
    CollectSources(CollectSourcesArgs),
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

- **`CollectSources`**

  Collect dependency sources to a local directory.
  
  Copies source code from `~/.cargo/registry/src/` into a local
  `.source_{timestamp}/` directory for parsing and documentation.
  
  Example: `cargo docs-md collect-sources --include-dev`

#### Trait Implementations

##### `impl Any for Command`

- <span id="command-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Command`

- <span id="command-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Command`

- <span id="command-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Command`

- <span id="command-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Command`

- <span id="command-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromArgMatches for Command`

- <span id="command-fromargmatches-from-arg-matches"></span>`fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="command-fromargmatches-from-arg-matches-mut"></span>`fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- <span id="command-fromargmatches-update-from-arg-matches"></span>`fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- <span id="command-fromargmatches-update-from-arg-matches-mut"></span>`fn update_from_arg_matches_mut<'b>(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Instrument for Command`

##### `impl<U> Into for Command`

- <span id="command-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Command`

##### `impl OwoColorize for Command`

##### `impl Pointable for Command`

- <span id="command-pointable-const-align"></span>`const ALIGN: usize`

- <span id="command-pointable-type-init"></span>`type Init = T`

- <span id="command-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="command-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="command-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="command-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Subcommand for Command`

- <span id="command-subcommand-augment-subcommands"></span>`fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="command-subcommand-augment-subcommands-for-update"></span>`fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

- <span id="command-subcommand-has-subcommand"></span>`fn has_subcommand(__clap_name: &str) -> bool`

##### `impl<U> TryFrom for Command`

- <span id="command-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="command-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Command`

- <span id="command-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="command-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for Command`

### `CliOutputFormat`

```rust
enum CliOutputFormat {
    Flat,
    Nested,
}
```

*Defined in `src/lib.rs:374-381`*

CLI-compatible output format enum (for clap `ValueEnum` derive).

#### Variants

- **`Flat`**

  Flat structure with double-underscore separators in filenames.

- **`Nested`**

  Nested directory structure mirroring the module hierarchy.

#### Trait Implementations

##### `impl Any for CliOutputFormat`

- <span id="clioutputformat-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CliOutputFormat`

- <span id="clioutputformat-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CliOutputFormat`

- <span id="clioutputformat-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CliOutputFormat`

- <span id="clioutputformat-clone"></span>`fn clone(&self) -> CliOutputFormat` — [`CliOutputFormat`](#clioutputformat)

##### `impl CloneToUninit for CliOutputFormat`

- <span id="clioutputformat-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CliOutputFormat`

##### `impl Debug for CliOutputFormat`

- <span id="clioutputformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CliOutputFormat`

- <span id="clioutputformat-default"></span>`fn default() -> CliOutputFormat` — [`CliOutputFormat`](#clioutputformat)

##### `impl<T> From for CliOutputFormat`

- <span id="clioutputformat-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CliOutputFormat`

##### `impl<U> Into for CliOutputFormat`

- <span id="clioutputformat-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CliOutputFormat`

##### `impl OwoColorize for CliOutputFormat`

##### `impl Pointable for CliOutputFormat`

- <span id="clioutputformat-pointable-const-align"></span>`const ALIGN: usize`

- <span id="clioutputformat-pointable-type-init"></span>`type Init = T`

- <span id="clioutputformat-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clioutputformat-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clioutputformat-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clioutputformat-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for CliOutputFormat`

- <span id="clioutputformat-toowned-type-owned"></span>`type Owned = T`

- <span id="clioutputformat-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="clioutputformat-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CliOutputFormat`

- <span id="clioutputformat-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="clioutputformat-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CliOutputFormat`

- <span id="clioutputformat-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="clioutputformat-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl ValueEnum for CliOutputFormat`

- <span id="clioutputformat-valueenum-value-variants"></span>`fn value_variants<'a>() -> &'a [Self]`

- <span id="clioutputformat-valueenum-to-possible-value"></span>`fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl WithSubscriber for CliOutputFormat`

## Type Aliases

### `Args`

```rust
type Args = GenerateArgs;
```

*Defined in `src/lib.rs:370`*

Backwards-compatible type alias for existing code.

