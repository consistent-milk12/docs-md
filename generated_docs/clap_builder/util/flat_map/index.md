*[clap_builder](../../index.md) / [util](../index.md) / [flat_map](index.md)*

---

# Module `flat_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlatMap`](#flatmap) | struct | Flat (Vec) backed map |
| [`VacantEntry`](#vacantentry) | struct |  |
| [`OccupiedEntry`](#occupiedentry) | struct |  |
| [`Iter`](#iter) | struct |  |
| [`IterMut`](#itermut) | struct |  |
| [`Entry`](#entry) | enum |  |

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

- <span id="flatmap-new"></span>`fn new() -> Self`

- <span id="flatmap-insert"></span>`fn insert(&mut self, key: K, value: V) -> Option<V>`

- <span id="flatmap-insert-unchecked"></span>`fn insert_unchecked(&mut self, key: K, value: V)`

- <span id="flatmap-extend-unchecked"></span>`fn extend_unchecked(&mut self, iter: impl IntoIterator<Item = (K, V)>)`

- <span id="flatmap-contains-key"></span>`fn contains_key<Q>(&self, key: &Q) -> bool`

- <span id="flatmap-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<V>`

- <span id="flatmap-remove-entry"></span>`fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>`

- <span id="flatmap-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="flatmap-entry"></span>`fn entry(&mut self, key: K) -> Entry<'_, K, V>` — [`Entry`](#entry)

- <span id="flatmap-get"></span>`fn get<Q>(&self, k: &Q) -> Option<&V>`

- <span id="flatmap-get-mut"></span>`fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>`

- <span id="flatmap-keys"></span>`fn keys(&self) -> std::slice::Iter<'_, K>`

- <span id="flatmap-values"></span>`fn values(&self) -> std::slice::Iter<'_, V>`

- <span id="flatmap-iter"></span>`fn iter(&self) -> Iter<'_, K, V>` — [`Iter`](#iter)

- <span id="flatmap-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, K, V>` — [`IterMut`](#itermut)

#### Trait Implementations

##### `impl<K: clone::Clone, V: clone::Clone> Clone for FlatMap<K, V>`

- <span id="flatmap-clone"></span>`fn clone(&self) -> FlatMap<K, V>` — [`FlatMap`](#flatmap)

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for FlatMap<K, V>`

- <span id="flatmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K: PartialEq + Eq, V> Default for FlatMap<K, V>`

- <span id="flatmap-default"></span>`fn default() -> Self`

##### `impl<K: cmp::Eq, V: cmp::Eq> Eq for FlatMap<K, V>`

##### `impl<K: cmp::PartialEq, V: cmp::PartialEq> PartialEq for FlatMap<K, V>`

- <span id="flatmap-eq"></span>`fn eq(&self, other: &FlatMap<K, V>) -> bool` — [`FlatMap`](#flatmap)

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

- <span id="iter-next-back"></span>`fn next_back(&mut self) -> Option<(&'a K, &'a V)>`

##### `impl<K, V> ExactSizeIterator for Iter<'_, K, V>`

##### `impl<I> IntoIterator for Iter<'a, K, V>`

- <span id="iter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, K, V> Iterator for Iter<'a, K, V>`

- <span id="iter-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<(&'a K, &'a V)>`

- <span id="iter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    keys: std::slice::IterMut<'a, K>,
    values: std::slice::IterMut<'a, V>,
}
```

#### Trait Implementations

##### `impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V>`

- <span id="itermut-next-back"></span>`fn next_back(&mut self) -> Option<(&'a K, &'a mut V)>`

##### `impl<K, V> ExactSizeIterator for IterMut<'_, K, V>`

##### `impl<I> IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiter"></span>`type IntoIter = I`

- <span id="itermut-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, K, V> Iterator for IterMut<'a, K, V>`

- <span id="itermut-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-next"></span>`fn next(&mut self) -> Option<(&'a K, &'a mut V)>`

- <span id="itermut-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `Entry<'a, K, V>`

```rust
enum Entry<'a, K, V> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}
```

#### Implementations

- <span id="entry-or-insert"></span>`fn or_insert(self, default: V) -> &'a mut V`

- <span id="entry-or-insert-with"></span>`fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V`

