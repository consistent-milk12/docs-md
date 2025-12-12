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
| [`AnchorUtils`](#anchorutils) | struct | Utilify functions to handle anchors |
| [`LinkRegistry`](#linkregistry) | struct | Registry mapping item IDs to their documentation file paths. |
| [`AssocItemKind`](#associtemkind) | enum | Kind of associated item for anchor generation. |

## Structs

### `AnchorUtils`

```rust
struct AnchorUtils;
```

*Defined in `src/linker.rs:53`*

Utilify functions to handle anchors

#### Implementations

- <span id="anchorutils-assoc-item-anchor"></span>`fn assoc_item_anchor(type_name: &str, item_name: &str, kind: AssocItemKind) -> String` — [`AssocItemKind`](#associtemkind)

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

- <span id="linkregistry-default"></span>`fn default() -> LinkRegistry` — [`LinkRegistry`](#linkregistry)

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

## Enums

### `AssocItemKind`

```rust
enum AssocItemKind {
    Method,
    Const,
    Type,
}
```

*Defined in `src/linker.rs:41-50`*

Kind of associated item for anchor generation.

Used to disambiguate anchors when multiple items share the same name
(e.g., `type Init` and `fn init` in the same impl block).

#### Variants

- **`Method`**

  A method or function (`fn`)

- **`Const`**

  An associated constant (`const`)

- **`Type`**

  An associated type (`type`)

#### Trait Implementations

##### `impl Clone for AssocItemKind`

- <span id="associtemkind-clone"></span>`fn clone(&self) -> AssocItemKind` — [`AssocItemKind`](#associtemkind)

##### `impl Copy for AssocItemKind`

##### `impl Debug for AssocItemKind`

- <span id="associtemkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AssocItemKind`

##### `impl<K> Equivalent for AssocItemKind`

- <span id="associtemkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Instrument for AssocItemKind`

##### `impl IntoEither for AssocItemKind`

##### `impl OwoColorize for AssocItemKind`

##### `impl PartialEq for AssocItemKind`

- <span id="associtemkind-eq"></span>`fn eq(&self, other: &AssocItemKind) -> bool` — [`AssocItemKind`](#associtemkind)

##### `impl Pointable for AssocItemKind`

- <span id="associtemkind-pointable-const-align"></span>`const ALIGN: usize`

- <span id="associtemkind-pointable-type-init"></span>`type Init = T`

- <span id="associtemkind-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="associtemkind-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="associtemkind-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="associtemkind-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for AssocItemKind`

##### `impl WithSubscriber for AssocItemKind`

