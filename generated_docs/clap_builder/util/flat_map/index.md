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

*Defined in [`clap_builder-4.5.53/src/util/flat_map.rs:9-12`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/flat_map.rs#L9-L12)*

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

##### `impl Any for FlatMap<K, V>`

- <span id="flatmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMap<K, V>`

- <span id="flatmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMap<K, V>`

- <span id="flatmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: clone::Clone, V: clone::Clone> Clone for FlatMap<K, V>`

- <span id="flatmap-clone"></span>`fn clone(&self) -> FlatMap<K, V>` — [`FlatMap`](#flatmap)

##### `impl CloneToUninit for FlatMap<K, V>`

- <span id="flatmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for FlatMap<K, V>`

- <span id="flatmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K: PartialEq + Eq, V> Default for FlatMap<K, V>`

- <span id="flatmap-default"></span>`fn default() -> Self`

##### `impl<K: cmp::Eq, V: cmp::Eq> Eq for FlatMap<K, V>`

##### `impl<T> From for FlatMap<K, V>`

- <span id="flatmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMap<K, V>`

- <span id="flatmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<K: cmp::PartialEq, V: cmp::PartialEq> PartialEq for FlatMap<K, V>`

- <span id="flatmap-partialeq-eq"></span>`fn eq(&self, other: &FlatMap<K, V>) -> bool` — [`FlatMap`](#flatmap)

##### `impl<K, V> StructuralPartialEq for FlatMap<K, V>`

##### `impl ToOwned for FlatMap<K, V>`

- <span id="flatmap-toowned-type-owned"></span>`type Owned = T`

- <span id="flatmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlatMap<K, V>`

- <span id="flatmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMap<K, V>`

- <span id="flatmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VacantEntry<'a, K, V>`

```rust
struct VacantEntry<'a, K, V> {
    v: &'a mut FlatMap<K, V>,
    key: K,
}
```

*Defined in [`clap_builder-4.5.53/src/util/flat_map.rs:178-181`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/flat_map.rs#L178-L181)*

#### Trait Implementations

##### `impl Any for VacantEntry<'a, K, V>`

- <span id="vacantentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VacantEntry<'a, K, V>`

- <span id="vacantentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VacantEntry<'a, K, V>`

- <span id="vacantentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for VacantEntry<'a, K, V>`

- <span id="vacantentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VacantEntry<'a, K, V>`

- <span id="vacantentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for VacantEntry<'a, K, V>`

- <span id="vacantentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vacantentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VacantEntry<'a, K, V>`

- <span id="vacantentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vacantentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OccupiedEntry<'a, K, V>`

```rust
struct OccupiedEntry<'a, K, V> {
    v: &'a mut FlatMap<K, V>,
    index: usize,
}
```

*Defined in [`clap_builder-4.5.53/src/util/flat_map.rs:183-186`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/flat_map.rs#L183-L186)*

#### Trait Implementations

##### `impl Any for OccupiedEntry<'a, K, V>`

- <span id="occupiedentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OccupiedEntry<'a, K, V>`

- <span id="occupiedentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OccupiedEntry<'a, K, V>`

- <span id="occupiedentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for OccupiedEntry<'a, K, V>`

- <span id="occupiedentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OccupiedEntry<'a, K, V>`

- <span id="occupiedentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OccupiedEntry<'a, K, V>`

- <span id="occupiedentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="occupiedentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OccupiedEntry<'a, K, V>`

- <span id="occupiedentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="occupiedentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    keys: std::slice::Iter<'a, K>,
    values: std::slice::Iter<'a, V>,
}
```

*Defined in [`clap_builder-4.5.53/src/util/flat_map.rs:188-191`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/flat_map.rs#L188-L191)*

#### Trait Implementations

##### `impl Any for Iter<'a, K, V>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a, K, V>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a, K, V>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V> DoubleEndedIterator for Iter<'a, K, V>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<(&'a K, &'a V)>`

##### `impl<K, V> ExactSizeIterator for Iter<'_, K, V>`

##### `impl<T> From for Iter<'a, K, V>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Iter<'a, K, V>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Iter<'a, K, V>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for Iter<'a, K, V>`

- <span id="iter-iterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<(&'a K, &'a V)>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for Iter<'a, K, V>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<'a, K, V>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    keys: std::slice::IterMut<'a, K>,
    values: std::slice::IterMut<'a, V>,
}
```

*Defined in [`clap_builder-4.5.53/src/util/flat_map.rs:224-227`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/flat_map.rs#L224-L227)*

#### Trait Implementations

##### `impl Any for IterMut<'a, K, V>`

- <span id="itermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMut<'a, K, V>`

- <span id="itermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMut<'a, K, V>`

- <span id="itermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V> DoubleEndedIterator for IterMut<'a, K, V>`

- <span id="itermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<(&'a K, &'a mut V)>`

##### `impl<K, V> ExactSizeIterator for IterMut<'_, K, V>`

##### `impl<T> From for IterMut<'a, K, V>`

- <span id="itermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterMut<'a, K, V>`

- <span id="itermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, V> Iterator for IterMut<'a, K, V>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<(&'a K, &'a mut V)>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for IterMut<'a, K, V>`

- <span id="itermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterMut<'a, K, V>`

- <span id="itermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Entry<'a, K, V>`

```rust
enum Entry<'a, K, V> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}
```

*Defined in [`clap_builder-4.5.53/src/util/flat_map.rs:149-152`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/flat_map.rs#L149-L152)*

#### Implementations

- <span id="entry-or-insert"></span>`fn or_insert(self, default: V) -> &'a mut V`

- <span id="entry-or-insert-with"></span>`fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V`

#### Trait Implementations

##### `impl Any for Entry<'a, K, V>`

- <span id="entry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Entry<'a, K, V>`

- <span id="entry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Entry<'a, K, V>`

- <span id="entry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Entry<'a, K, V>`

- <span id="entry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Entry<'a, K, V>`

- <span id="entry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Entry<'a, K, V>`

- <span id="entry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="entry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Entry<'a, K, V>`

- <span id="entry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="entry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

