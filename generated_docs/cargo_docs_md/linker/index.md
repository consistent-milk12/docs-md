*[cargo_docs_md](../index.md) / [linker](index.md)*

---

# Module `linker`

Cross-reference linking for markdown documentation.

This module provides the `LinkRegistry` which maps rustdoc item IDs to their
corresponding markdown file paths. This enables creating clickable links
between items in the generated documentation.

# How It Works

1. During initialization, `LinkRegistry::build()` traverses the entire crate
   and records where each item's documentation will be written.

2. During markdown generation, `create_link()` is called to generate
   relative links from one file to another.

# Path Formats

The registry supports two output formats:

- **Flat**: `module.md`, `parent__child.md` (double-underscore separators)
- **Nested**: `module/index.md`, `parent/child/index.md` (directory structure)

# Example

```ignore
let registry = LinkRegistry::build(&krate, true); // flat format
let link = registry.create_link(&some_id, "index.md");
// Returns: Some("[`ItemName`](module.md)")
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LinkRegistry`](#linkregistry) | struct | Registry mapping item IDs to their documentation file paths. |
| [`method_anchor`](#method_anchor) | fn | Generate a compound anchor for a method on a type. |
| [`slugify_anchor`](#slugify_anchor) | fn | Convert a name to a GitHub-style markdown anchor slug. |
| [`slugify_anchor_ascii`](#slugify_anchor_ascii) | fn | Fast ASCII-only slugification (no allocation for normalization). |
| [`slugify_anchor_impl`](#slugify_anchor_impl) | fn | Unicode-aware slugification with full lowercase support. |
| [`item_has_anchor`](#item_has_anchor) | fn | Check if an item kind generates a heading anchor in markdown. |

## Structs

### `LinkRegistry`

```rust
struct LinkRegistry {
    item_paths: std::collections::HashMap<rustdoc_types::Id, String>,
    item_names: std::collections::HashMap<rustdoc_types::Id, String>,
}
```

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

- <span id="linkregistry-default"></span>`fn default() -> LinkRegistry` — [`LinkRegistry`](../index.md)

##### `impl<T> Instrument for LinkRegistry`

##### `impl<T> IntoEither for LinkRegistry`

##### `impl<D> OwoColorize for LinkRegistry`

##### `impl<T> Pointable for LinkRegistry`

- <span id="linkregistry-align"></span>`const ALIGN: usize`

- <span id="linkregistry-init"></span>`type Init = T`

- <span id="linkregistry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="linkregistry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="linkregistry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="linkregistry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for LinkRegistry`

## Functions

### `method_anchor`

```rust
fn method_anchor(type_name: &str, method_name: &str) -> String
```

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

### `slugify_anchor`

```rust
fn slugify_anchor(name: &str) -> String
```

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

### `slugify_anchor_ascii`

```rust
fn slugify_anchor_ascii(name: &str) -> String
```

Fast ASCII-only slugification (no allocation for normalization).

### `slugify_anchor_impl`

```rust
fn slugify_anchor_impl(name: &str) -> String
```

Unicode-aware slugification with full lowercase support.

### `item_has_anchor`

```rust
const fn item_has_anchor(kind: rustdoc_types::ItemKind) -> bool
```

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

