*[clap_builder](../../index.md) / [util](../index.md) / [flat_set](index.md)*

---

# Module `flat_set`

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

- `fn new() -> Self`

- `fn insert(self: &mut Self, value: T) -> bool`

- `fn contains<Q>(self: &Self, value: &Q) -> bool`

- `fn retain<F>(self: &mut Self, f: F)`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> std::slice::Iter<'_, T>`

- `fn sort_by_key<K, F>(self: &mut Self, f: F)`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for FlatSet<T>`

- `fn clone(self: &Self) -> FlatSet<T>` — [`FlatSet`](#flatset)

##### `impl<T: $crate::fmt::Debug> Debug for FlatSet<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: PartialEq + Eq> Default for FlatSet<T>`

- `fn default() -> Self`

##### `impl<T: $crate::cmp::Eq> Eq for FlatSet<T>`

##### `impl<T: PartialEq + Eq> Extend for FlatSet<T>`

- `fn extend<I: IntoIterator<Item = T>>(self: &mut Self, iter: I)`

##### `impl<T: PartialEq + Eq> FromIterator for FlatSet<T>`

- `fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<T: PartialEq + Eq> IntoIterator for FlatSet<T>`

- `type Item = T`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for FlatSet<T>`

- `fn eq(self: &Self, other: &FlatSet<T>) -> bool` — [`FlatSet`](#flatset)

##### `impl<T> StructuralPartialEq for FlatSet<T>`

