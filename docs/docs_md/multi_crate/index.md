*[docs_md](../index.md) / [multi_crate](index.md)*

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
- [`SummaryGenerator`](../index.md): Creates mdBook-compatible SUMMARY.md

# Usage

```ignore
use docs_md::multi_crate::{MultiCrateParser, MultiCrateGenerator};

let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let generator = MultiCrateGenerator::new(&crates, &args);
generator.generate()?;
```

## Structs

### `CrateCollection`

```rust
struct CrateCollection {
    // [REDACTED: Private Fields]
}
```

Collection of parsed crates ready for documentation generation.

Maintains crates in a deterministic processing order for reproducible
output. The order is typically alphabetical by crate name.

# Example

```ignore
let mut collection = CrateCollection::new();
collection.insert("tracing".to_string(), tracing_crate);
collection.insert("tracing_core".to_string(), tracing_core_crate);

for (name, krate) in collection.iter() {
    println!("Processing {name}");
}
```

#### Implementations

- `fn new() -> Self`
  Create an empty crate collection.

- `fn insert(self: &mut Self, name: String, krate: Crate) -> Option<Crate>`
  Insert a crate into the collection.

- `fn get(self: &Self, name: &str) -> Option<&Crate>`
  Get a crate by name.

- `fn contains(self: &Self, name: &str) -> bool`
  Check if a crate exists in the collection.

- `fn iter(self: &Self) -> impl Iterator<Item = (&String, &Crate)>`
  Iterate over crates in processing order.

- `fn len(self: &Self) -> usize`
  Get the number of crates in the collection.

- `fn is_empty(self: &Self) -> bool`
  Check if the collection is empty.

- `fn names(self: &Self) -> &[String]`
  Get crate names in processing order.

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

- `fn default() -> CrateCollection`

### `MultiCrateContext<'a>`

```rust
struct MultiCrateContext<'a> {
    // [REDACTED: Private Fields]
}
```

Shared context for multi-crate documentation generation.

Holds references to all crates, the unified link registry, and
CLI configuration. Used by [`MultiCrateGenerator`](../index.md) to coordinate
generation across crates.


#### Implementations

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self`
  Create a new multi-crate context.

- `fn crates(self: &Self) -> &CrateCollection`
  Get the crate collection.

- `fn registry(self: &Self) -> &UnifiedLinkRegistry`
  Get the unified link registry.

- `fn args(self: &Self) -> &Args`
  Get CLI arguments.

- `fn single_crate_view(self: &'a Self, crate_name: &str) -> Option<SingleCrateView<'a>>`
  Create a single-crate view for rendering one crate.

- `fn find_item(self: &Self, id: &Id) -> Option<(&str, &Item)>`
  Find an item across all crates by ID.

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

### `SingleCrateView<'a>`

```rust
struct SingleCrateView<'a> {
    // [REDACTED: Private Fields]
}
```

View of a single crate within multi-crate context.

Provides an interface similar to [`GeneratorContext`](../index.md) but uses
[`UnifiedLinkRegistry`](../index.md) for cross-crate link resolution. This
allows existing rendering code to work with minimal changes.


#### Implementations

- `fn crate_name(self: &Self) -> &str`
  Get the crate name.

- `fn krate(self: &Self) -> &Crate`
  Get the crate being rendered.

- `fn registry(self: &Self) -> &UnifiedLinkRegistry`
  Get the unified registry.

- `fn args(self: &Self) -> &Args`
  Get CLI arguments.

- `fn get_path(self: &Self, id: Id) -> Option<&Vec<String>>`
  Get module path for an item.

- `fn get_impls(self: &Self, id: Id) -> Option<&Vec<&'a Impl>>`
  Get impl blocks for a type.

- `fn should_include_item(self: &Self, item: &rustdoc_types::Item) -> bool`
  Check if an item should be included based on visibility.

- `fn count_modules(self: &Self) -> usize`
  Count modules for progress reporting.

- `fn create_link(self: &Self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String>`
  Create a markdown link using the unified registry.

- `fn resolve_name(self: &Self, name: &str) -> Option<(String, Id)>`
  Resolve a name to a crate and ID.

- `fn lookup_item_across_crates(self: &Self, id: &Id) -> Option<(&str, &Item)>`
  Look up an item across all crates by ID.

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

### `MultiCrateGenerator<'a>`

```rust
struct MultiCrateGenerator<'a> {
    // [REDACTED: Private Fields]
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

#### Implementations

- `fn new(crates: &'a CrateCollection, args: &'a Args) -> Self`
  Create a new multi-crate generator.

- `fn generate(self: &Self) -> Result<(), Error>`
  Generate documentation for all crates.

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

- `fn parse_directory(dir: &Path) -> Result<CrateCollection, Error>`
  Parse all rustdoc JSON files in a directory.

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

### `UnifiedLinkRegistry`

```rust
struct UnifiedLinkRegistry {
    // [REDACTED: Private Fields]
}
```

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

#### Implementations

- `fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self`
  Build a unified registry from a collection of crates.

- `fn get_path(self: &Self, crate_name: &str, id: Id) -> Option<&String>`
  Get the file path for an item in a specific crate.

- `fn get_name(self: &Self, crate_name: &str, id: Id) -> Option<&String>`
  Get the display name for an item.

- `fn resolve_name(self: &Self, name: &str, current_crate: &str) -> Option<(String, Id)>`
  Resolve an item name to its crate and ID.

- `fn create_link(self: &Self, from_crate: &str, from_path: &str, to_crate: &str, to_id: Id) -> Option<String>`
  Create a markdown link from one file to another across crates.

- `fn compute_cross_crate_path(from: &str, to: &str) -> String`
  Compute relative path between files potentially in different crates.

- `fn get_anchor(self: &Self, crate_name: &str, id: Id) -> Option<String>`
  Get an anchor string for an item within its page.

- `fn contains(self: &Self, crate_name: &str, id: Id) -> bool`
  Check if an item exists in the registry.

- `fn len(self: &Self) -> usize`
  Get the number of registered items.

- `fn is_empty(self: &Self) -> bool`
  Check if the registry is empty.

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

- `fn default() -> UnifiedLinkRegistry`

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

##### `impl Serialize`

- `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

### `SearchIndexGenerator<'a>`

```rust
struct SearchIndexGenerator<'a> {
    // [REDACTED: Private Fields]
}
```

Generator for multi-crate search indices.

Traverses all crates in a [`CrateCollection`](../index.md) and builds a comprehensive
search index of all public items.

# Example

```ignore
let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
let generator = SearchIndexGenerator::new(&crates);
generator.write(Path::new("docs/"))?;
```

#### Implementations

- `fn new(crates: &'a CrateCollection) -> Self`
  Create a new search index generator.

- `fn generate(self: &Self) -> SearchIndex`
  Generate the complete search index.

- `fn write(self: &Self, output_dir: &Path) -> std::io::Result<()>`
  Write the search index to `search_index.json` in the output directory.

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

### `SummaryGenerator<'a>`

```rust
struct SummaryGenerator<'a> {
    // [REDACTED: Private Fields]
}
```

Generates mdBook-compatible SUMMARY.md file.

Creates a table of contents linking all crates and their modules,
allowing the documentation to be built as an mdBook site.

# Output Format

```markdown
# Summary

- [tracing](tracing/index.md)
  - [span](tracing/span/index.md)
  - [field](tracing/field/index.md)
- [tracing_core](tracing_core/index.md)
  - [subscriber](tracing_core/subscriber/index.md)
```

#### Implementations

- `fn new(crates: &'a CrateCollection, output_dir: &'a Path) -> Self`
  Create a new summary generator.

- `fn generate(self: &Self) -> Result<(), Error>`
  Generate the SUMMARY.md file.

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

