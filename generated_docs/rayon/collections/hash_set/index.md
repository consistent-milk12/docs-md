*[rayon](../../index.md) / [collections](../index.md) / [hash_set](index.md)*

---

# Module `hash_set`

This module contains the parallel iterator types for hash sets
(`HashSet<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a hash set |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a hash set |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves out of a hash set, but keeps the total capacity. |

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_set.rs:15-17`](../../../../.source_1765633015/rayon-1.11.0/src/collections/hash_set.rs#L15-L17)*

Parallel iterator over a hash set

#### Trait Implementations

##### `impl<T> Any for IntoIter<T>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<T>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<T>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for IntoIter<T>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntoIter<T>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for IntoIter<T>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IntoIter<T>`

##### `impl<T> IntoParallelIterator for IntoIter<T>`

- <span id="intoiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intoiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T>`

- <span id="intoiter-paralleliterator-type-item"></span>`type Item = T`

- <span id="intoiter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="intoiter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

- <span id="intoiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for IntoIter<T>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntoIter<T>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: vec::IntoIter<&'a T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_set.rs:31-33`](../../../../.source_1765633015/rayon-1.11.0/src/collections/hash_set.rs#L31-L33)*

Parallel iterator over an immutable reference to a hash set

#### Trait Implementations

##### `impl<T> Any for Iter<'a, T>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a, T>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a, T>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Iter<'a, T>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Iter<'a, T>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Iter<'a, T>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Iter<'a, T>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Iter<'a, T>`

##### `impl<T> IntoParallelIterator for Iter<'a, T>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Iter<'a, T>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = &'a T`

- <span id="iter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="iter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, T>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Iter<'a, T>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Iter<'a, T>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Iter<'a, T>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    inner: vec::IntoIter<T>,
    marker: std::marker::PhantomData<&'a mut std::collections::HashSet<T>>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_set.rs:58-61`](../../../../.source_1765633015/rayon-1.11.0/src/collections/hash_set.rs#L58-L61)*

Draining parallel iterator that moves out of a hash set,
but keeps the total capacity.

#### Trait Implementations

##### `impl<T> Any for Drain<'a, T>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'a, T>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a, T>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for Drain<'a, T>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Drain<'a, T>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Drain<'a, T>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Drain<'a, T>`

##### `impl<T> IntoParallelIterator for Drain<'a, T>`

- <span id="drain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="drain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Drain<'_, T>`

- <span id="drain-paralleliterator-type-item"></span>`type Item = T`

- <span id="drain-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="drain-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Drain<'a, T>`

- <span id="drain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drain-pointable-type-init"></span>`type Init = T`

- <span id="drain-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Drain<'a, T>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Drain<'a, T>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

