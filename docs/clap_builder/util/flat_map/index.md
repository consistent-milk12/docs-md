*[clap_builder](../../index.md) / [util](../index.md) / [flat_map](index.md)*

---

# Module `flat_map`

## Structs

### `FlatMap<K, V>`

```rust
struct FlatMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}
```

Flat (Vec) backed map

This preserves insertion order

#### Implementations

- `fn new() -> Self`

- `fn insert(self: &mut Self, key: K, value: V) -> Option<V>`

- `fn insert_unchecked(self: &mut Self, key: K, value: V)`

- `fn extend_unchecked(self: &mut Self, iter: impl IntoIterator<Item = (K, V)>)`

- `fn contains_key<Q>(self: &Self, key: &Q) -> bool`

- `fn remove<Q>(self: &mut Self, key: &Q) -> Option<V>`

- `fn remove_entry<Q>(self: &mut Self, key: &Q) -> Option<(K, V)>`

- `fn is_empty(self: &Self) -> bool`

- `fn entry(self: &mut Self, key: K) -> Entry<'_, K, V>` — [`Entry`](#entry)

- `fn get<Q>(self: &Self, k: &Q) -> Option<&V>`

- `fn get_mut<Q>(self: &mut Self, k: &Q) -> Option<&mut V>`

- `fn keys(self: &Self) -> std::slice::Iter<'_, K>`

- `fn values(self: &Self) -> std::slice::Iter<'_, V>`

- `fn iter(self: &Self) -> Iter<'_, K, V>` — [`Iter`](#iter)

- `fn iter_mut(self: &mut Self) -> IterMut<'_, K, V>` — [`IterMut`](#itermut)

#### Trait Implementations

##### `impl<K: $crate::clone::Clone, V: $crate::clone::Clone> Clone for FlatMap<K, V>`

- `fn clone(self: &Self) -> FlatMap<K, V>` — [`FlatMap`](#flatmap)

##### `impl<K: $crate::fmt::Debug, V: $crate::fmt::Debug> Debug for FlatMap<K, V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<K: PartialEq + Eq, V> Default for FlatMap<K, V>`

- `fn default() -> Self`

##### `impl<K: $crate::cmp::Eq, V: $crate::cmp::Eq> Eq for FlatMap<K, V>`

##### `impl<K: $crate::cmp::PartialEq, V: $crate::cmp::PartialEq> PartialEq for FlatMap<K, V>`

- `fn eq(self: &Self, other: &FlatMap<K, V>) -> bool` — [`FlatMap`](#flatmap)

##### `impl<K, V> StructuralPartialEq for FlatMap<K, V>`

### `VacantEntry<'a, K, V>`

```rust
struct VacantEntry<'a, K, V> {
    v: &'a mut FlatMap<K, V>,
    key: K,
}
```

### `OccupiedEntry<'a, K, V>`

```rust
struct OccupiedEntry<'a, K, V> {
    v: &'a mut FlatMap<K, V>,
    index: usize,
}
```

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    keys: std::slice::Iter<'a, K>,
    values: std::slice::Iter<'a, V>,
}
```

#### Trait Implementations

##### `impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V>`

- `fn next_back(self: &mut Self) -> Option<(&'a K, &'a V)>`

##### `impl<K, V> ExactSizeIterator for Iter<'_, K, V>`

##### `impl<I> IntoIterator for Iter<'a, K, V>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K, V> Iterator for Iter<'a, K, V>`

- `type Item = (&'a K, &'a V)`

- `fn next(self: &mut Self) -> Option<(&'a K, &'a V)>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    keys: std::slice::IterMut<'a, K>,
    values: std::slice::IterMut<'a, V>,
}
```

#### Trait Implementations

##### `impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V>`

- `fn next_back(self: &mut Self) -> Option<(&'a K, &'a mut V)>`

##### `impl<K, V> ExactSizeIterator for IterMut<'_, K, V>`

##### `impl<I> IntoIterator for IterMut<'a, K, V>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, K, V> Iterator for IterMut<'a, K, V>`

- `type Item = (&'a K, &'a mut V)`

- `fn next(self: &mut Self) -> Option<(&'a K, &'a mut V)>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

## Enums

### `Entry<'a, K, V>`

```rust
enum Entry<'a, K, V> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}
```

#### Implementations

- `fn or_insert(self: Self, default: V) -> &'a mut V`

- `fn or_insert_with<F: FnOnce() -> V>(self: Self, default: F) -> &'a mut V`

