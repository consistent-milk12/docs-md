# Crate `docs_md`

docs-md library interface for testing and reuse.

This module exposes the core functionality of docs-md as a library,
allowing integration tests and external tools to use the markdown
generation capabilities programmatically.

## Modules

- [`error`](error/index.md) - Error types for docs-md.
- [`generator`](generator/index.md) - Markdown documentation generator for rustdoc JSON.
- [`linker`](linker/index.md) - Cross-reference linking for markdown documentation.
- [`multi_crate`](multi_crate/index.md) - Multi-crate documentation generation.
- [`parser`](parser/index.md) - Rustdoc JSON parsing module.
- [`types`](types/index.md) - Type rendering utilities for converting rustdoc types to string representations.

## Structs

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

### `LinkRegistry`

```rust
struct LinkRegistry {
    // [REDACTED: Private Fields]
}
```

Registry mapping item IDs to their documentation file paths.

This is the central data structure for cross-reference resolution.
It's built once during generation and queried whenever we need to
create links between items.

#### Implementations

- `fn build(krate: &Crate, flat_format: bool) -> Self`
  Build a link registry by traversing all items in the crate.

- `fn get_path(self: &Self, id: Id) -> Option<&String>`
  Get the file path where an item is documented.

- `fn get_name(self: &Self, id: Id) -> Option<&String>`
  Get the display name for an item.

- `fn create_link(self: &Self, id: Id, from_path: &str) -> Option<String>`
  Create a markdown link to an item from a given source file.

- `fn compute_relative_path(from: &str, to: &str) -> String`
  Compute the relative path from one file to another.

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

- `fn default() -> LinkRegistry`

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
CLI configuration. Used by [`MultiCrateGenerator`](#multicrategenerator) to coordinate
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

Traverses all crates in a [`CrateCollection`](#cratecollection) and builds a comprehensive
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

### `UnifiedLinkRegistry`

```rust
struct UnifiedLinkRegistry {
    // [REDACTED: Private Fields]
}
```

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

### `Args`

```rust
struct Args {
    pub path: Option<std::path::PathBuf>,
    pub crate_name: Option<String>,
    pub dir: Option<std::path::PathBuf>,
    pub mdbook: bool,
    pub search_index: bool,
    pub primary_crate: Option<String>,
    pub crate_version: Option<String>,
    pub output: std::path::PathBuf,
    pub format: CliOutputFormat,
    pub include_private: bool,
}
```

Command-line arguments for docs-md.

The tool accepts input from two mutually exclusive sources:
1. A local rustdoc JSON file (`--path`)
2. A crate name to fetch from docs.rs (`--crate-name`) - Note: currently non-functional
   as docs.rs doesn't serve rustdoc JSON files publicly.

#### Fields

- **`path`**: `Option<std::path::PathBuf>`

  Path to a local rustdoc JSON file.
  
  Generate this file with: `cargo doc --output-format json`
  The JSON file will be in `target/doc/{crate_name}.json`
  
  Mutually exclusive with `--crate-name` and `--dir`.

- **`crate_name`**: `Option<String>`

  Crate name to fetch from docs.rs (experimental).
  
  **Note**: This feature is currently non-functional because docs.rs
  doesn't expose rustdoc JSON files publicly. Use `--path` instead.
  
  Mutually exclusive with `--path` and `--dir`.

- **`dir`**: `Option<std::path::PathBuf>`

  Directory containing multiple rustdoc JSON files.
  
  Use this for multi-crate documentation generation. The tool will
  scan the directory for all `*.json` files (rustdoc format) and
  generate documentation for each crate with cross-crate linking.
  
  Generate JSON files with:
  `RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc`
  
  Mutually exclusive with `--path` and `--crate-name`.

- **`mdbook`**: `bool`

  Generate mdBook-compatible SUMMARY.md file.
  
  Only valid with `--dir` for multi-crate documentation.
  Creates a `SUMMARY.md` file in the output directory that can be
  used as the entry point for an mdBook documentation site.

- **`search_index`**: `bool`

  Generate search_index.json for client-side search.
  
  Only valid with `--dir` for multi-crate documentation.
  Creates a `search_index.json` file containing all documented items,
  which can be used with client-side search libraries like Fuse.js,
  Lunr.js, or FlexSearch.

- **`primary_crate`**: `Option<String>`

  Primary crate name for preferential link resolution.
  
  When specified with `--dir`, links to items in this crate take
  precedence over items with the same name in dependencies.
  This helps resolve ambiguous links like `exit` to the intended
  crate rather than `std::process::exit`.

- **`crate_version`**: `Option<String>`

  Specific version to fetch from docs.rs.
  
  Only used with `--crate-name`. Defaults to "latest" if not specified.

- **`output`**: `std::path::PathBuf`

  Output directory for generated markdown files.
  
  The directory will be created if it doesn't exist.
  Defaults to `docs/` in the current directory.

- **`format`**: `CliOutputFormat`

  Output format (flat or nested).
  
  - `flat`: All files in one directory (default)
  - `nested`: Directory hierarchy mirroring modules

- **`include_private`**: `bool`

  Include private (non-public) items in the output.
  
  By default, only public items are documented. Enable this
  to also include `pub(crate)`, `pub(super)`, and private items.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl FromArgMatches`

- `fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error>`

- `fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

- `fn update_from_arg_matches_mut(self: &mut Self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Args`

- `fn group_id() -> Option<clap::Id>`

- `fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command`

- `fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl CommandFactory`

- `fn command<'b>() -> clap::Command`

- `fn command_for_update<'b>() -> clap::Command`

##### `impl OwoColorize<D>`

##### `impl Parser`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `OutputFormat`

```rust
enum OutputFormat {
    Flat,
    Nested,
}
```

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

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(cli: CliOutputFormat) -> Self`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> OutputFormat`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl OwoColorize<D>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl ValueEnum`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value<'a>(self: &Self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> OutputFormat`

### `CliOutputFormat`

```rust
enum CliOutputFormat {
    Flat,
    Nested,
}
```

CLI-compatible output format enum (for clap `ValueEnum` derive).

#### Variants

- **`Flat`**

  Flat structure with double-underscore separators in filenames.

- **`Nested`**

  Nested directory structure mirroring the module hierarchy.

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

##### `impl Clone`

- `fn clone(self: &Self) -> CliOutputFormat`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl OwoColorize<D>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl ValueEnum`

- `fn value_variants<'a>() -> &'a [Self]`

- `fn to_possible_value<'a>(self: &Self) -> ::std::option::Option<clap::builder::PossibleValue>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> CliOutputFormat`

