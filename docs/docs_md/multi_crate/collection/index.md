*[docs_md](../../index.md) / [multi_crate](../index.md) / [collection](index.md)*

---

# Module `collection`

Crate collection for multi-crate documentation.

This module provides [`CrateCollection`](../../index.md), a container for multiple parsed
rustdoc crates that maintains a consistent processing order.

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

- `fn new() -> Self`

- `fn insert(self: &mut Self, name: String, krate: Crate) -> Option<Crate>`

- `fn get(self: &Self, name: &str) -> Option<&Crate>`

- `fn get_with_name(self: &Self, name: &str) -> Option<(&str, &Crate)>`

- `fn contains(self: &Self, name: &str) -> bool`

- `fn iter(self: &Self) -> impl Iterator<Item = (&String, &Crate)>`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn names(self: &Self) -> Vec<&String>`

#### Trait Implementations

##### `impl Debug for CrateCollection`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for CrateCollection`

- `fn default() -> CrateCollection` — [`CrateCollection`](../../index.md)

##### `impl<T> Instrument for CrateCollection`

##### `impl<T> IntoEither for CrateCollection`

##### `impl<D> OwoColorize for CrateCollection`

##### `impl<T> Pointable for CrateCollection`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for CrateCollection`

