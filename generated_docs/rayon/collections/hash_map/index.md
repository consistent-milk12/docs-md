*[rayon](../../index.md) / [collections](../index.md) / [hash_map](index.md)*

---

# Module `hash_map`

This module contains the parallel iterator types for hash maps
(`HashMap<K, V>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a hash map |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a hash map |
| [`IterMut`](#itermut) | struct | Parallel iterator over a mutable reference to a hash map |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves out of a hash map, but keeps the total capacity. |

## Structs

### `IntoIter<K, V>`

```rust
struct IntoIter<K, V> {
    inner: vec::IntoIter<(K, V)>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:15-17`](../../../../.source_1765521767/rayon-1.11.0/src/collections/hash_map.rs#L15-L17)*

Parallel iterator over a hash map

#### Trait Implementations

##### `impl Any for IntoIter<K, V>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<K, V>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<K, V>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IntoIter<K, V>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntoIter<K, V>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntoIter<K, V>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntoIter<K, V>`

##### `impl IntoParallelIterator for IntoIter<K, V>`

- <span id="intoiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intoiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Send, V: Send> ParallelIterator for IntoIter<K, V>`

- <span id="intoiter-paralleliterator-type-item"></span>`type Item = (K, V)`

- <span id="intoiter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="intoiter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for IntoIter<K, V>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

- <span id="intoiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for IntoIter<K, V>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoIter<K, V>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a V)>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:31-33`](../../../../.source_1765521767/rayon-1.11.0/src/collections/hash_map.rs#L31-L33)*

Parallel iterator over an immutable reference to a hash map

#### Trait Implementations

##### `impl Any for Iter<'a, K, V>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a, K, V>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a, K, V>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, V> Clone for Iter<'_, K, V>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Iter<'a, K, V>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Iter<'a, K, V>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Iter<'a, K, V>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Iter<'a, K, V>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Iter<'a, K, V>`

##### `impl IntoParallelIterator for Iter<'a, K, V>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Sync, V: Sync> ParallelIterator for Iter<'a, K, V>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="iter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Iter<'a, K, V>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Iter<'a, K, V>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Iter<'a, K, V>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iter<'a, K, V>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a mut V)>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:55-57`](../../../../.source_1765521767/rayon-1.11.0/src/collections/hash_map.rs#L55-L57)*

Parallel iterator over a mutable reference to a hash map

#### Trait Implementations

##### `impl Any for IterMut<'a, K, V>`

- <span id="itermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMut<'a, K, V>`

- <span id="itermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMut<'a, K, V>`

- <span id="itermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IterMut<'a, K, V>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IterMut<'a, K, V>`

- <span id="itermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterMut<'a, K, V>`

- <span id="itermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IterMut<'a, K, V>`

##### `impl IntoParallelIterator for IterMut<'a, K, V>`

- <span id="itermut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="itermut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Sync, V: Send> ParallelIterator for IterMut<'a, K, V>`

- <span id="itermut-paralleliterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="itermut-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for IterMut<'a, K, V>`

- <span id="itermut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermut-pointable-type-init"></span>`type Init = T`

- <span id="itermut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for IterMut<'a, K, V>`

- <span id="itermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterMut<'a, K, V>`

- <span id="itermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'a, K, V>`

```rust
struct Drain<'a, K, V> {
    inner: vec::IntoIter<(K, V)>,
    marker: std::marker::PhantomData<&'a mut std::collections::HashMap<K, V>>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:72-75`](../../../../.source_1765521767/rayon-1.11.0/src/collections/hash_map.rs#L72-L75)*

Draining parallel iterator that moves out of a hash map,
but keeps the total capacity.

#### Trait Implementations

##### `impl Any for Drain<'a, K, V>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'a, K, V>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a, K, V>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Drain<'a, K, V>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Drain<'a, K, V>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Drain<'a, K, V>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Drain<'a, K, V>`

##### `impl IntoParallelIterator for Drain<'a, K, V>`

- <span id="drain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="drain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Send, V: Send> ParallelIterator for Drain<'_, K, V>`

- <span id="drain-paralleliterator-type-item"></span>`type Item = (K, V)`

- <span id="drain-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="drain-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Drain<'a, K, V>`

- <span id="drain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drain-pointable-type-init"></span>`type Init = T`

- <span id="drain-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Drain<'a, K, V>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Drain<'a, K, V>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

