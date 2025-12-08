*[clap_builder](../../index.md) / [util](../index.md) / [flat_set](index.md)*

---

# Module `flat_set`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlatSet`](#flatset) | struct | Flat (Vec) backed set |

## Structs

### `FlatSet<T>`

```rust
struct FlatSet<T> {
    inner: Vec<T>,
}
```

Flat (Vec) backed set

This preserves insertion order

#### Implementations

- <span id="flatset-new"></span>`fn new() -> Self`

- <span id="flatset-insert"></span>`fn insert(&mut self, value: T) -> bool`

- <span id="flatset-contains"></span>`fn contains<Q>(&self, value: &Q) -> bool`

- <span id="flatset-retain"></span>`fn retain<F>(&mut self, f: F)`

- <span id="flatset-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="flatset-iter"></span>`fn iter(&self) -> std::slice::Iter<'_, T>`

- <span id="flatset-sort-by-key"></span>`fn sort_by_key<K, F>(&mut self, f: F)`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for FlatSet<T>`

- <span id="flatset-clone"></span>`fn clone(&self) -> FlatSet<T>` — [`FlatSet`](#flatset)

##### `impl<T: fmt::Debug> Debug for FlatSet<T>`

- <span id="flatset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: PartialEq + Eq> Default for FlatSet<T>`

- <span id="flatset-default"></span>`fn default() -> Self`

##### `impl<T: cmp::Eq> Eq for FlatSet<T>`

##### `impl<T: PartialEq + Eq> Extend for FlatSet<T>`

- <span id="flatset-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T: PartialEq + Eq> FromIterator for FlatSet<T>`

- <span id="flatset-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<T: PartialEq + Eq> IntoIterator for FlatSet<T>`

- <span id="flatset-item"></span>`type Item = T`

- <span id="flatset-intoiter"></span>`type IntoIter = IntoIter<T>`

- <span id="flatset-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T: cmp::PartialEq> PartialEq for FlatSet<T>`

- <span id="flatset-eq"></span>`fn eq(&self, other: &FlatSet<T>) -> bool` — [`FlatSet`](#flatset)

##### `impl<T> StructuralPartialEq for FlatSet<T>`

