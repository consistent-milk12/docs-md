*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [registry](index.md)*

---

# Module `registry`

Unified link registry for cross-crate documentation.

This module provides [`UnifiedLinkRegistry`](#unifiedlinkregistry) which maps item IDs across
multiple crates to their documentation file paths, enabling cross-crate
linking in the generated markdown.

# `rustdoc_types` Path Types

There are two distinct path representations in `rustdoc_types`:

- **`ItemSummary`**: Contains metadata about items without full content.
  - `path: Vec<String>` - Structured path segments like `["std", "vec", "Vec"]`
  - `kind: ItemKind` - The item's kind (Struct, Enum, Trait, etc.)
  - Use for: kind filtering, path lookups, metadata queries

- **`Item`**: Full item content including inner details.
  - `inner: ItemEnum` - The actual item content (Struct/Enum/Trait data)
  - Use for: rendering, accessing item members, documentation content

**Optimization tip**: When only the item kind is needed, prefer
`krate.paths.get(&id).map(|p| p.kind)` over looking up the full `Item`.



## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BorrowedKey`](#borrowedkey) | struct | Borrowed key for zero-allocation lookups. |
| [`UnifiedLinkRegistry`](#unifiedlinkregistry) | struct | Registry mapping item IDs to documentation paths across multiple crates. |
| [`keys_match`](#keys-match) | fn | Allow comparing `BorrowedKey` with `RegistryKey`. |
| [`Str`](#str) | type | Compact string type for memory-efficient storage. |
| [`RegistryKey`](#registrykey) | type | Key type for registry lookups: `(crate_name, item_id)`. |

## Structs

### `BorrowedKey<'a>`

```rust
struct BorrowedKey<'a>(&'a str, rustdoc_types::Id);
```

*Defined in `src/multi_crate/registry.rs:51`*

Borrowed key for zero-allocation lookups.

Must hash identically to `RegistryKey` tuple of `(CompactString, Id)`.

#### Trait Implementations

##### `impl Any for BorrowedKey<'a>`

- <span id="borrowedkey-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BorrowedKey<'a>`

- <span id="borrowedkey-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BorrowedKey<'a>`

- <span id="borrowedkey-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Eq for BorrowedKey<'a>`

##### `impl<K> Equivalent for BorrowedKey<'a>`

- <span id="borrowedkey-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<T> From for BorrowedKey<'a>`

- <span id="borrowedkey-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for BorrowedKey<'_>`

- <span id="borrowedkey-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl Instrument for BorrowedKey<'a>`

##### `impl<U> Into for BorrowedKey<'a>`

- <span id="borrowedkey-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for BorrowedKey<'a>`

##### `impl OwoColorize for BorrowedKey<'a>`

##### `impl PartialEq for BorrowedKey<'a>`

- <span id="borrowedkey-partialeq-eq"></span>`fn eq(&self, other: &BorrowedKey<'a>) -> bool` — [`BorrowedKey`](#borrowedkey)

##### `impl Pointable for BorrowedKey<'a>`

- <span id="borrowedkey-pointable-const-align"></span>`const ALIGN: usize`

- <span id="borrowedkey-pointable-type-init"></span>`type Init = T`

- <span id="borrowedkey-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="borrowedkey-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="borrowedkey-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="borrowedkey-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl StructuralPartialEq for BorrowedKey<'a>`

##### `impl<U> TryFrom for BorrowedKey<'a>`

- <span id="borrowedkey-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="borrowedkey-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BorrowedKey<'a>`

- <span id="borrowedkey-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="borrowedkey-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for BorrowedKey<'a>`

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

Unlike [`LinkRegistry`](../../linker/index.md) which handles a single crate, this registry
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

- <span id="unifiedlinkregistry-build"></span>`fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](../collection/index.md#cratecollection)

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

- <span id="unifiedlinkregistry-default"></span>`fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](#unifiedlinkregistry)

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

## Functions

### `keys_match`

```rust
fn keys_match(stored: &(compact_str::CompactString, rustdoc_types::Id), borrowed: &BorrowedKey<'_>) -> bool
```

*Defined in `src/multi_crate/registry.rs:63-65`*

Allow comparing `BorrowedKey` with `RegistryKey`.

## Type Aliases

### `Str`

```rust
type Str = compact_str::CompactString;
```

*Defined in `src/multi_crate/registry.rs:39`*

Compact string type for memory-efficient storage.
Strings ≤24 bytes are stored inline (no heap allocation).
Most crate names, item names, and short paths fit inline.

### `RegistryKey`

```rust
type RegistryKey = (compact_str::CompactString, rustdoc_types::Id);
```

*Defined in `src/multi_crate/registry.rs:45`*

Key type for registry lookups: `(crate_name, item_id)`.

Uses `CompactString` for memory efficiency - most crate names are short
and stored inline without heap allocation.

