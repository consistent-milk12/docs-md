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

*Defined in [`clap_builder-4.5.53/src/util/flat_set.rs:9-11`](../../../../.source_1765521767/clap_builder-4.5.53/src/util/flat_set.rs#L9-L11)*

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

##### `impl<T> Any for FlatSet<T>`

- <span id="flatset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatSet<T>`

- <span id="flatset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatSet<T>`

- <span id="flatset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for FlatSet<T>`

- <span id="flatset-clone"></span>`fn clone(&self) -> FlatSet<T>` — [`FlatSet`](#flatset)

##### `impl<T> CloneToUninit for FlatSet<T>`

- <span id="flatset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for FlatSet<T>`

- <span id="flatset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: PartialEq + Eq> Default for FlatSet<T>`

- <span id="flatset-default"></span>`fn default() -> Self`

##### `impl<T: cmp::Eq> Eq for FlatSet<T>`

##### `impl<T: PartialEq + Eq> Extend for FlatSet<T>`

- <span id="flatset-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T> From for FlatSet<T>`

- <span id="flatset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: PartialEq + Eq> FromIterator for FlatSet<T>`

- <span id="flatset-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self`

##### `impl<T, U> Into for FlatSet<T>`

- <span id="flatset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: PartialEq + Eq> IntoIterator for FlatSet<T>`

- <span id="flatset-intoiterator-type-item"></span>`type Item = T`

- <span id="flatset-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<T>`

- <span id="flatset-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T: cmp::PartialEq> PartialEq for FlatSet<T>`

- <span id="flatset-partialeq-eq"></span>`fn eq(&self, other: &FlatSet<T>) -> bool` — [`FlatSet`](#flatset)

##### `impl<T> StructuralPartialEq for FlatSet<T>`

##### `impl<T> ToOwned for FlatSet<T>`

- <span id="flatset-toowned-type-owned"></span>`type Owned = T`

- <span id="flatset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for FlatSet<T>`

- <span id="flatset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FlatSet<T>`

- <span id="flatset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

