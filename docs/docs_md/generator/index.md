*[docs_md](../index.md) / [generator](index.md)*

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
- [`render_shared`](render_shared/index.md) - Shared rendering functions for documentation generation.

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

- `const fn new(module_path: &'a [String], crate_name: &'a str) -> Self`

- `fn generate(self: &Self) -> String`

#### Trait Implementations

##### `impl<T> Instrument for BreadcrumbGenerator<'a>`

##### `impl<T> IntoEither for BreadcrumbGenerator<'a>`

##### `impl<D> OwoColorize for BreadcrumbGenerator<'a>`

##### `impl<T> Pointable for BreadcrumbGenerator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn default() -> MarkdownCapture` — [`MarkdownCapture`](../index.md)

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

### `GeneratorContext<'a>`

```rust
struct GeneratorContext<'a> {
    pub krate: &'a rustdoc_types::Crate,
    crate_name: String,
    pub impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    pub link_registry: crate::linker::LinkRegistry,
    pub args: &'a crate::Args,
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

#### Implementations

- `fn new(krate: &'a Crate, args: &'a Args) -> Self` — [`Args`](../index.md)

- `fn build_impl_map(krate: &'a Crate) -> HashMap<Id, Vec<&'a Impl>>`

- `fn impl_sort_key(impl_block: &Impl) -> (u8, String)`

- `const fn get_type_id(ty: &rustdoc_types::Type) -> Option<Id>`

- `const fn should_include_item(self: &Self, item: &Item) -> bool`

- `fn count_modules(self: &Self, item: &Item) -> usize`

#### Trait Implementations

##### `impl<T> Instrument for GeneratorContext<'a>`

##### `impl<T> IntoEither for GeneratorContext<'a>`

##### `impl ItemAccess for GeneratorContext<'_>`

- `fn krate(self: &Self) -> &Crate`

- `fn crate_name(self: &Self) -> &str`

- `fn get_item(self: &Self, id: &Id) -> Option<&Item>`

- `fn get_impls(self: &Self, id: &Id) -> Option<&[&Impl]>`

- `fn crate_version(self: &Self) -> Option<&str>`

##### `impl ItemFilter for GeneratorContext<'_>`

- `fn should_include_item(self: &Self, item: &Item) -> bool`

- `fn include_private(self: &Self) -> bool`

- `fn include_blanket_impls(self: &Self) -> bool`

##### `impl LinkResolver for GeneratorContext<'_>`

- `fn link_registry(self: &Self) -> Option<&LinkRegistry>` — [`LinkRegistry`](../index.md)

- `fn process_docs(self: &Self, item: &Item, current_file: &str) -> Option<String>`

- `fn create_link(self: &Self, id: Id, current_file: &str) -> Option<String>`

##### `impl<D> OwoColorize for GeneratorContext<'a>`

##### `impl<T> Pointable for GeneratorContext<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self` — [`LinkRegistry`](../index.md)

- `fn process(self: &Self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_links_protected(self: &Self, docs: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_line(self: &Self, line: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_reference_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_path_reference_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_method_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_backtick_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_plain_links(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn process_html_links_with_context(self: &Self, text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn resolve_html_link_to_url(self: &Self, item_name: &str, item_kind: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- `fn kind_matches(html_kind: &str, item_kind: ItemKind) -> bool`

- `fn clean_blank_lines(docs: &str) -> String`

- `fn resolve_to_url(self: &Self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- `fn get_url_for_id(self: &Self, id: Id) -> Option<String>`

- `fn get_docs_rs_url(path_info: &rustdoc_types::ItemSummary) -> Option<String>`

- `fn resolve_method_link(self: &Self, type_name: &str, method_name: &str, item_links: &HashMap<String, Id>) -> Option<String>`

- `fn resolve_link(self: &Self, link_text: &str, item_links: &HashMap<String, Id>) -> String`

- `fn create_link_for_id(self: &Self, id: Id, display_name: &str) -> Option<String>`

- `fn create_docs_rs_link(path_info: &rustdoc_types::ItemSummary, display_name: &str) -> Option<String>`

#### Trait Implementations

##### `impl<T> Instrument for DocLinkProcessor<'a>`

##### `impl<T> IntoEither for DocLinkProcessor<'a>`

##### `impl<D> OwoColorize for DocLinkProcessor<'a>`

##### `impl<T> Pointable for DocLinkProcessor<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self` — [`RenderContext`](#rendercontext)

- `fn process_docs(self: &Self, item: &Item) -> Option<String>`

- `fn render(self: &Self, item: &Item) -> String`

- `fn categorize_items(self: &Self, item_ids: &'a [Id]) -> CategorizedItems<'a>` — [`CategorizedItems`](module/index.md)

- `fn expand_glob_reexport(self: &Self, items: &mut CategorizedItems<'a>, use_item: &rustdoc_types::Use, seen_items: &mut HashSet<&'a Id>)` — [`CategorizedItems`](module/index.md)

- `fn render_all_sections(self: &Self, md: &mut String, items: &CategorizedItems<'_>)` — [`CategorizedItems`](module/index.md)

- `fn render_modules_section(self: &Self, md: &mut String, modules: &[(&Id, &Item)])`

- `fn render_structs_section(self: &Self, md: &mut String, structs: &[(&Id, &Item)])`

- `fn render_enums_section(self: &Self, md: &mut String, enums: &[(&Id, &Item)])`

- `fn render_traits_section(self: &Self, md: &mut String, traits: &[(&Id, &Item)])`

- `fn render_functions_section(self: &Self, md: &mut String, functions: &[&Item])`

- `fn render_macros_section(self: &Self, md: &mut String, macros: &[&Item])`

- `fn render_constants_section(self: &Self, md: &mut String, constants: &[&Item])`

- `fn render_type_aliases_section(self: &Self, md: &mut String, type_aliases: &[&Item])`

#### Trait Implementations

##### `impl<T> Instrument for ModuleRenderer<'a>`

##### `impl<T> IntoEither for ModuleRenderer<'a>`

##### `impl<D> OwoColorize for ModuleRenderer<'a>`

##### `impl<T> Pointable for ModuleRenderer<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(krate: &'a Crate, args: &'a Args) -> Result<Self, Error>` — [`Args`](../index.md), [`Error`](../error/index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](../error/index.md)

- `fn create_progress_bar(total: usize) -> Result<ProgressBar, Error>` — [`Error`](../error/index.md)

- `fn generate_to_capture(krate: &Crate, format: CliOutputFormat, include_private: bool) -> Result<MarkdownCapture, Error>` — [`CliOutputFormat`](../index.md), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- `fn generate_flat_to_capture(ctx: &GeneratorContext<'_>, root: &Item, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](#generatorcontext), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- `fn generate_flat_recursive_capture(ctx: &GeneratorContext<'_>, item: &Item, prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](#generatorcontext), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- `fn generate_nested_to_capture(ctx: &GeneratorContext<'_>, root: &Item, path_prefix: &str, capture: &mut MarkdownCapture) -> Result<(), Error>` — [`GeneratorContext`](#generatorcontext), [`MarkdownCapture`](../index.md), [`Error`](../error/index.md)

- `fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error>` — [`Args`](../index.md), [`Error`](../error/index.md)

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

## Traits

## Functions

