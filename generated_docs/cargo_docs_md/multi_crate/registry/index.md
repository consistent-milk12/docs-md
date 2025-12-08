*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [registry](index.md)*

---

# Module `registry`

Unified link registry for cross-crate documentation.

This module provides [`UnifiedLinkRegistry`](../../index.md) which maps item IDs across
multiple crates to their documentation file paths, enabling cross-crate
linking in the generated markdown.

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

- `fn equivalent(self: &Self, key: &K) -> bool`

##### `impl Hash for BorrowedKey<'_>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<T> Instrument for BorrowedKey<'a>`

##### `impl<T> IntoEither for BorrowedKey<'a>`

##### `impl<D> OwoColorize for BorrowedKey<'a>`

##### `impl<'a> PartialEq for BorrowedKey<'a>`

- `fn eq(self: &Self, other: &BorrowedKey<'a>) -> bool` — [`BorrowedKey`](#borrowedkey)

##### `impl<T> Pointable for BorrowedKey<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self` — [`CrateCollection`](../../index.md)

- `fn register_crate(self: &mut Self, crate_name: &str, krate: &Crate)`

- `fn register_from_paths(self: &mut Self, crate_name: &str, krate: &Crate)`

- `fn item_enum_to_kind(inner: &ItemEnum) -> ItemKind`

- `fn register_item_recursive(self: &mut Self, krate: &Crate, crate_name: &str, item_id: Id, item: &rustdoc_types::Item, parent_path: &str)`

- `fn register_item(self: &mut Self, crate_name: &str, id: Id, name: &str, path: &str, kind: ItemKind)`

- `fn get_path(self: &Self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- `fn get_name(self: &Self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- `fn get_re_export_source(self: &Self, crate_name: &str, id: Id) -> Option<&compact_str::CompactString>`

- `fn resolve_reexport(self: &Self, crate_name: &str, id: Id) -> Option<(compact_str::CompactString, Id)>`

- `fn resolve_name(self: &Self, name: &str, current_crate: &str) -> Option<(compact_str::CompactString, Id)>`

- `fn resolve_path(self: &Self, path: &str) -> Option<(compact_str::CompactString, Id)>`

- `fn create_link(self: &Self, from_crate: &str, from_path: &str, to_crate: &str, to_id: Id) -> Option<String>`

- `fn compute_cross_crate_path(from: &str, to: &str) -> String`

- `fn get_anchor(self: &Self, crate_name: &str, id: Id) -> Option<String>`

- `fn contains(self: &Self, crate_name: &str, id: Id) -> bool`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for UnifiedLinkRegistry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for UnifiedLinkRegistry`

- `fn default() -> UnifiedLinkRegistry` — [`UnifiedLinkRegistry`](../../index.md)

##### `impl<T> Instrument for UnifiedLinkRegistry`

##### `impl<T> IntoEither for UnifiedLinkRegistry`

##### `impl<D> OwoColorize for UnifiedLinkRegistry`

##### `impl<T> Pointable for UnifiedLinkRegistry`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

