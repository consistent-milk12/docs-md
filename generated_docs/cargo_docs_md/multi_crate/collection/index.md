*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [collection](index.md)*

---

# Module `collection`

Crate collection for multi-crate documentation.

This module provides [`CrateCollection`](#cratecollection), a container for multiple parsed
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

*Defined in `src/multi_crate/collection.rs:30-34`*

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

  Create an empty crate collection.

- <span id="cratecollection-insert"></span>`fn insert(&mut self, name: String, krate: Crate) -> Option<Crate>`

  Insert a crate into the collection.

  

  If a crate with the same name already exists, it is replaced

  and `Some(old_crate)` is returned.

- <span id="cratecollection-get"></span>`fn get(&self, name: &str) -> Option<&Crate>`

  Get a crate by name.

- <span id="cratecollection-get-with-name"></span>`fn get_with_name(&self, name: &str) -> Option<(&str, &Crate)>`

  Get a crate by name, returning the stored key as well.

  

  This is useful when you need a reference to the crate name that

  has the same lifetime as the collection.

- <span id="cratecollection-contains"></span>`fn contains(&self, name: &str) -> bool`

  Check if a crate exists in the collection.

- <span id="cratecollection-iter"></span>`fn iter(&self) -> impl Iterator<Item = (&String, &Crate)>`

  Iterate over crates in alphabetical order.

  

  Returns tuples of `(&crate_name, &Crate)` sorted alphabetically

  by crate name for deterministic output.

  

  Sorting is done on-demand since collection size is small (10-50 crates).

- <span id="cratecollection-len"></span>`fn len(&self) -> usize`

  Get the number of crates in the collection.

- <span id="cratecollection-is-empty"></span>`fn is_empty(&self) -> bool`

  Check if the collection is empty.

- <span id="cratecollection-names"></span>`fn names(&self) -> Vec<&String>`

  Get crate names in alphabetical order.

  

  Returns a sorted `Vec` of crate names for deterministic processing.

  Sorting is done on-demand since collection size is small.

#### Trait Implementations

##### `impl Any for CrateCollection`

- <span id="cratecollection-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CrateCollection`

- <span id="cratecollection-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CrateCollection`

- <span id="cratecollection-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CrateCollection`

- <span id="cratecollection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CrateCollection`

- <span id="cratecollection-default"></span>`fn default() -> CrateCollection` — [`CrateCollection`](#cratecollection)

##### `impl<T> From for CrateCollection`

- <span id="cratecollection-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for CrateCollection`

##### `impl<U> Into for CrateCollection`

- <span id="cratecollection-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CrateCollection`

##### `impl OwoColorize for CrateCollection`

##### `impl Pointable for CrateCollection`

- <span id="cratecollection-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cratecollection-pointable-type-init"></span>`type Init = T`

- <span id="cratecollection-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cratecollection-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cratecollection-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cratecollection-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CrateCollection`

- <span id="cratecollection-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cratecollection-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CrateCollection`

- <span id="cratecollection-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cratecollection-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for CrateCollection`

