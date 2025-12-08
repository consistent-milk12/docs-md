*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [registry](index.md)*

---

# Module `registry`

Unified link registry for cross-crate documentation.

This module provides [`UnifiedLinkRegistry`](../../index.md) which maps item IDs across
multiple crates to their documentation file paths, enabling cross-crate
linking in the generated markdown.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BorrowedKey`](#borrowedkey) | struct | Borrowed key for zero-allocation lookups. |
| [`UnifiedLinkRegistry`](#unifiedlinkregistry) | struct | Registry mapping item IDs to documentation paths across multiple crates. |
| [`keys_match`](#keys_match) | fn | Allow comparing `BorrowedKey` with `RegistryKey`. |
| [`Str`](#str) | type | Compact string type for memory-efficient storage. |
| [`RegistryKey`](#registrykey) | type | Key type for registry lookups: `(crate_name, item_id)`. |

## Structs

### `BorrowedKey<'a>`

```rust
struct BorrowedKey<'a>(&'a str, rustdoc_types::Id);
```

Borrowed key for zero-allocation lookups.

Must hash identically to `RegistryKey` tuple of `(CompactString, Id)`.

#### Trait Implementations

##### `impl<'a> Eq for BorrowedKey<'a>`

##### `impl<Q, K> Equivalent for BorrowedKey<'a>`

- <span id="borrowedkey-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for BorrowedKey<'_>`

- <span id="borrowedkey-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T> Instrument for BorrowedKey<'a>`

##### `impl<T> IntoEither for BorrowedKey<'a>`

##### `impl<D> OwoColorize for BorrowedKey<'a>`

##### `impl<'a> PartialEq for BorrowedKey<'a>`

- <span id="borrowedkey-eq"></span>`fn eq(&self, other: &BorrowedKey<'a>) -> bool` — [`BorrowedKey`](#borrowedkey)

##### `impl<T> Pointable for BorrowedKey<'a>`

- <span id="borrowedkey-align"></span>`const ALIGN: usize`

- <span id="borrowedkey-init"></span>`type Init = T`

- <span id="borrowedkey-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="borrowedkey-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="borrowedkey-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="borrowedkey-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a> StructuralPartialEq for BorrowedKey<'a>`

##### `impl<T> WithSubscriber for BorrowedKey<'a>`

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

Registry mapping item IDs to documentation paths across multiple crates.

Unlike [`LinkRegistry`](../../index.md) which handles a single crate, this registry
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

- <span id="unifiedlinkregistry-build"></span>`fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](../../index.md)

- <span id="unifiedlinkregistry-register-crate"></span>`fn register_crate(&mut self, crate_name: &str, krate: &Crate)`

- <span id="unifiedlinkregistry-register-from-paths"></span>`fn register_from_paths(&mut self, crate_name: &str, krate: &Crate)`

- <span id="unifiedlinkregistry-item-enum-to-kind"></span>`const fn item_enum_to_kind(inner: &ItemEnum) -> ItemKind`

- <span id="unifiedlinkregistry-register-item-recursive"></span>`fn register_item_recursive(&mut self, krate: &Crate, crate_name: &str, item_id: Id, item: &rustdoc_types::Item, parent_path: &str)`

- <span id="unifiedlinkregistry-register-item"></span>`fn register_item(&mut self, crate_name: &str, id: Id, name: &str, path: &str, kind: ItemKind)`

- <span id="unifiedlinkregistry-get-path"></span>`fn get_path(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- <span id="unifiedlinkregistry-get-name"></span>`fn get_name(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- <span id="unifiedlinkregistry-get-re-export-source"></span>`fn get_re_export_source(&self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- <span id="unifiedlinkregistry-resolve-reexport"></span>`fn resolve_reexport(&self, crate_name: &str, id: Id) -> Option<(compact_str::CompactString, Id)>`

- <span id="unifiedlinkregistry-resolve-name"></span>`fn resolve_name(&self, name: &str, current_crate: &str) -> Option<(compact_str::CompactString, Id)>`

- <span id="unifiedlinkregistry-resolve-path"></span>`fn resolve_path(&self, path: &str) -> Option<(compact_str::CompactString, Id)>`

- <span id="unifiedlinkregistry-create-link"></span>`fn create_link(&self, from_crate: &str, from_path: &str, to_crate: &str, to_id: Id) -> Option<String>`

- <span id="unifiedlinkregistry-compute-cross-crate-path"></span>`fn compute_cross_crate_path(from: &str, to: &str) -> String`

- <span id="unifiedlinkregistry-get-anchor"></span>`fn get_anchor(&self, crate_name: &str, id: Id) -> Option<String>`

- <span id="unifiedlinkregistry-contains"></span>`fn contains(&self, crate_name: &str, id: Id) -> bool`

- <span id="unifiedlinkregistry-len"></span>`fn len(&self) -> usize`

- <span id="unifiedlinkregistry-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Debug for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-default"></span>`fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../../index.md)

##### `impl<T> Instrument for UnifiedLinkRegistry`

##### `impl<T> IntoEither for UnifiedLinkRegistry`

##### `impl<D> OwoColorize for UnifiedLinkRegistry`

##### `impl<T> Pointable for UnifiedLinkRegistry`

- <span id="unifiedlinkregistry-align"></span>`const ALIGN: usize`

- <span id="unifiedlinkregistry-init"></span>`type Init = T`

- <span id="unifiedlinkregistry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="unifiedlinkregistry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="unifiedlinkregistry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="unifiedlinkregistry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for UnifiedLinkRegistry`

## Functions

### `keys_match`

```rust
fn keys_match(stored: &(compact_str::CompactString, rustdoc_types::Id), borrowed: &BorrowedKey<'_>) -> bool
```

Allow comparing `BorrowedKey` with `RegistryKey`.

## Type Aliases

### `Str`

```rust
type Str = compact_str::CompactString;
```

Compact string type for memory-efficient storage.
Strings ≤24 bytes are stored inline (no heap allocation).
Most crate names, item names, and short paths fit inline.

### `RegistryKey`

```rust
type RegistryKey = (compact_str::CompactString, rustdoc_types::Id);
```

Key type for registry lookups: `(crate_name, item_id)`.

Uses `CompactString` for memory efficiency - most crate names are short
and stored inline without heap allocation.

