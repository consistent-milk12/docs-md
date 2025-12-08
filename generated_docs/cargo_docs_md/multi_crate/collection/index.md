*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [collection](index.md)*

---

# Module `collection`

Crate collection for multi-crate documentation.

This module provides [`CrateCollection`](../../index.md), a container for multiple parsed
rustdoc crates that maintains a consistent processing order.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CrateCollection`](#cratecollection) | struct | Collection of parsed crates ready for documentation generation. |

## Structs

### `CrateCollection`

```rust
struct CrateCollection {
    crates: std::collections::HashMap<String, rustdoc_types::Crate>,
}
```

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

- <span id="cratecollection-insert"></span>`fn insert(&mut self, name: String, krate: Crate) -> Option<Crate>`

- <span id="cratecollection-get"></span>`fn get(&self, name: &str) -> Option<&Crate>`

- <span id="cratecollection-get-with-name"></span>`fn get_with_name(&self, name: &str) -> Option<(&str, &Crate)>`

- <span id="cratecollection-contains"></span>`fn contains(&self, name: &str) -> bool`

- <span id="cratecollection-iter"></span>`fn iter(&self) -> impl Iterator<Item = (&String, &Crate)>`

- <span id="cratecollection-len"></span>`fn len(&self) -> usize`

- <span id="cratecollection-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="cratecollection-names"></span>`fn names(&self) -> Vec<&String>`

#### Trait Implementations

##### `impl Debug for CrateCollection`

- <span id="cratecollection-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateCollection`

- <span id="cratecollection-default"></span>`fn default() -> CrateCollection` — [`CrateCollection`](../../index.md)

##### `impl<T> Instrument for CrateCollection`

##### `impl<T> IntoEither for CrateCollection`

##### `impl<D> OwoColorize for CrateCollection`

##### `impl<T> Pointable for CrateCollection`

- <span id="cratecollection-align"></span>`const ALIGN: usize`

- <span id="cratecollection-init"></span>`type Init = T`

- <span id="cratecollection-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratecollection-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratecollection-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratecollection-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for CrateCollection`

