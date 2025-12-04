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

- [`context`](#context) - Shared state for generation (crate data, maps, config)
- [`module`](module/index.md) - Module-level markdown rendering
- [`items`](#items) - Individual item rendering (structs, enums, traits, etc.)
- [`impls`](#impls) - Implementation block rendering
- [`flat`](#flat) - Flat output format generator
- [`nested`](#nested) - Nested output format generator

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
- [`module`](module/index.md) - Module markdown rendering for documentation generation.

## Structs

### `BreadcrumbGenerator<'a>`

```rust
struct BreadcrumbGenerator<'a> {
    // [REDACTED: Private Fields]
}
```

Generates breadcrumb navigation for nested module pages.

Creates a navigation line showing the path from the crate root to
the current module, with each segment being a clickable link.

#### Implementations

- `fn new(module_path: &'a [String], crate_name: &'a str) -> Self`
  Create a new breadcrumb generator.

- `fn generate(self: &Self) -> String`
  Generate breadcrumb navigation markdown.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `MarkdownCapture`

```rust
struct MarkdownCapture {
    // [REDACTED: Private Fields]
}
```

Captures generated markdown in memory for testing.

Instead of writing files to disk, this struct stores all generated
markdown content in a HashMap, keyed by relative file path. This
enables snapshot testing and verification of output without filesystem
side effects.

#### Implementations

- `fn new() -> Self`
  Create a new empty capture.

- `fn insert(self: &mut Self, path: String, content: String)`
  Add a file to the capture.

- `fn get(self: &Self, path: &str) -> Option<&String>`
  Get the content of a specific file.

- `fn paths(self: &Self) -> Vec<&String>`
  Get all file paths in sorted order.

- `fn len(self: &Self) -> usize`
  Get the number of captured files.

- `fn is_empty(self: &Self) -> bool`
  Check if the capture is empty.

- `fn to_snapshot_string(self: &Self) -> String`
  Convert all captured files to a single string for snapshot testing.

- `fn into_inner(self: Self) -> HashMap<String, String>`
  Consume self and return the underlying HashMap.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> MarkdownCapture`

### `GeneratorContext<'a>`

```rust
struct GeneratorContext<'a> {
    pub krate: &'a rustdoc_types::Crate,
    pub path_map: std::collections::HashMap<rustdoc_types::Id, Vec<String>>,
    pub impl_map: std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>,
    pub link_registry: crate::linker::LinkRegistry,
    pub args: &'a crate::Args,
}
```

Shared context containing all data needed for documentation generation.

This struct is passed to all rendering components and provides:
- Access to the parsed crate data
- Path mapping for "defined in" information
- Impl block lookup for rendering implementations
- Link registry for cross-references
- CLI configuration options

#### Fields

- **`krate`**: `&'a rustdoc_types::Crate`

  The parsed rustdoc JSON crate.

- **`path_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<String>>`

  Maps item IDs to their full module paths.
  
  Used for generating "defined in" information.
  For `std::collections::HashMap`, the path would be `["std", "collections", "HashMap"]`.

- **`impl_map`**: `std::collections::HashMap<rustdoc_types::Id, Vec<&'a rustdoc_types::Impl>>`

  Maps type IDs to all impl blocks for that type.
  
  Used for rendering "Implementations" and "Trait Implementations" sections.

- **`link_registry`**: `crate::linker::LinkRegistry`

  Registry for creating cross-reference links between items.

- **`args`**: `&'a crate::Args`

  CLI arguments containing output path, format, and options.

#### Implementations

- `fn new(krate: &'a Crate, args: &'a Args) -> Self`
  Create a new generator context from crate data and CLI arguments.

- `fn should_include_item(self: &Self, item: &Item) -> bool`
  Check if an item should be included based on visibility settings.

- `fn count_modules(self: &Self, item: &Item) -> usize`
  Count the total number of modules that will be generated.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `DocLinkProcessor<'a>`

```rust
struct DocLinkProcessor<'a> {
    // [REDACTED: Private Fields]
}
```

Processes doc comments to resolve intra-doc links to markdown links.

Rustdoc JSON includes a `links` field on each Item that maps intra-doc
link text to item IDs. This processor uses that map along with the
`LinkRegistry` to convert these to relative markdown links.

# Supported Patterns

- `` [`Name`](#name) `` - Backtick code links (most common)
- `` `path::to::Item` `` - Qualified path links

# External Crate Links

Items from external crates are linked to docs.rs when possible.

#### Implementations

- `fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self`
  Create a new processor for the given context.

- `fn process(self: &Self, docs: &str, item_links: &HashMap<String, Id>) -> String`
  Process a doc string and resolve all intra-doc links.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `ModuleRenderer<'a>`

```rust
struct ModuleRenderer<'a> {
    // [REDACTED: Private Fields]
}
```

Renders a module to markdown.

This struct handles the complete rendering of a module's documentation page,
including:
- Title (Crate or Module heading)
- Module-level documentation
- Sections for each item type (Modules, Structs, Enums, etc.)

#### Implementations

- `fn new(ctx: &'a GeneratorContext<'a>, current_file: &'a str, is_root: bool) -> Self`
  Create a new module renderer.

- `fn render(self: &Self, item: &Item) -> String`
  Generate the complete markdown content for a module.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `Generator<'a>`

```rust
struct Generator<'a> {
    // [REDACTED: Private Fields]
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

#### Implementations

- `fn new(krate: &'a Crate, args: &'a Args) -> Result<Self, Error>`
  Create a new generator for the given crate and arguments.

- `fn generate(self: &Self) -> Result<(), Error>`
  Generate markdown documentation.

- `fn generate_to_capture(krate: &Crate, format: CliOutputFormat, include_private: bool) -> Result<MarkdownCapture, Error>`
  Generate documentation to memory instead of disk.

- `fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error>`
  Convenience method to generate documentation in one call.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl OwoColorize<D>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

## Functions

