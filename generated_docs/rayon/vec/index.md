*[rayon](../index.md) / [vec](index.md)*

---

# Module `vec`

Parallel iterator types for [vectors] (`Vec<T>`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator that moves out of a vector. |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves a range out of a vector, but keeps the total capacity. |
| [`DrainProducer`](#drainproducer) | struct |  |
| [`SliceDrain`](#slicedrain) | struct |  |

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    vec: Vec<T>,
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:38-40`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L38-L40)*

Parallel iterator that moves out of a vector.

#### Trait Implementations

##### `impl<T> Any for IntoIter<T>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<T>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<T>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T> CloneToUninit for IntoIter<T>`

- <span id="intoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for IntoIter<T>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntoIter<T>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for IntoIter<T>`

- <span id="intoiter-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="intoiter-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="intoiter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

- <span id="intoiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for IntoIter<T>`

- <span id="intoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="intoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IntoIter<T>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntoIter<T>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Drain<'data, T: Send>`

```rust
struct Drain<'data, T: Send> {
    vec: &'data mut Vec<T>,
    range: std::ops::Range<usize>,
    orig_len: usize,
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:111-115`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L111-L115)*

Draining parallel iterator that moves a range out of a vector, but keeps the total capacity.

#### Trait Implementations

##### `impl<T> Any for Drain<'data, T>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'data, T>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'data, T>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug + Send> Debug for Drain<'data, T>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> Drop for Drain<'data, T>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Drain<'data, T>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for Drain<'data, T>`

- <span id="drain-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="drain-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="drain-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for Drain<'data, T>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Drain<'data, T>`

##### `impl<T> IntoParallelIterator for Drain<'data, T>`

- <span id="drain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="drain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Drain<'data, T>`

- <span id="drain-paralleliterator-type-item"></span>`type Item = T`

- <span id="drain-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="drain-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Drain<'data, T>`

- <span id="drain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drain-pointable-type-init"></span>`type Init = T`

- <span id="drain-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for Drain<'data, T>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Drain<'data, T>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DrainProducer<'data, T: Send>`

```rust
struct DrainProducer<'data, T: Send> {
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:188-190`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L188-L190)*

#### Implementations

- <span id="drainproducer-new"></span>`unsafe fn new(slice: &mut [T]) -> DrainProducer<'_, T>` — [`DrainProducer`](#drainproducer)

  Creates a draining producer, which *moves* items from the slice.

  

  Unsafe because `!Copy` data must not be read after the borrow is released.

- <span id="drainproducer-from-vec"></span>`unsafe fn from_vec(vec: &mut Vec<T>, len: usize) -> DrainProducer<'_, T>` — [`DrainProducer`](#drainproducer)

  Creates a draining producer, which *moves* items from the tail of the vector.

  

  Unsafe because we're moving from beyond `vec.len()`, so the caller must ensure

  that data is initialized and not read after the borrow is released.

#### Trait Implementations

##### `impl<T> Any for DrainProducer<'data, T>`

- <span id="drainproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DrainProducer<'data, T>`

- <span id="drainproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DrainProducer<'data, T>`

- <span id="drainproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: 'data + Send> Drop for DrainProducer<'data, T>`

- <span id="drainproducer-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for DrainProducer<'data, T>`

- <span id="drainproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for DrainProducer<'data, T>`

- <span id="drainproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for DrainProducer<'data, T>`

##### `impl<T> Pointable for DrainProducer<'data, T>`

- <span id="drainproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drainproducer-pointable-type-init"></span>`type Init = T`

- <span id="drainproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drainproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drainproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drainproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for DrainProducer<'data, T>`

- <span id="drainproducer-producer-type-item"></span>`type Item = T`

- <span id="drainproducer-producer-type-intoiter"></span>`type IntoIter = SliceDrain<'data, T>`

- <span id="drainproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="drainproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for DrainProducer<'data, T>`

- <span id="drainproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drainproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for DrainProducer<'data, T>`

- <span id="drainproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drainproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SliceDrain<'data, T>`

```rust
struct SliceDrain<'data, T> {
    iter: slice::IterMut<'data, T>,
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:246-248`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L246-L248)*

#### Trait Implementations

##### `impl<T> Any for SliceDrain<'data, T>`

- <span id="slicedrain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SliceDrain<'data, T>`

- <span id="slicedrain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SliceDrain<'data, T>`

- <span id="slicedrain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: 'data> DoubleEndedIterator for SliceDrain<'data, T>`

- <span id="slicedrain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: 'data> Drop for SliceDrain<'data, T>`

- <span id="slicedrain-drop"></span>`fn drop(&mut self)`

##### `impl<T: 'data> ExactSizeIterator for SliceDrain<'data, T>`

- <span id="slicedrain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for SliceDrain<'data, T>`

- <span id="slicedrain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: 'data> FusedIterator for SliceDrain<'data, T>`

##### `impl<T, U> Into for SliceDrain<'data, T>`

- <span id="slicedrain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for SliceDrain<'data, T>`

##### `impl IntoIterator for SliceDrain<'data, T>`

- <span id="slicedrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="slicedrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="slicedrain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'data> Iterator for SliceDrain<'data, T>`

- <span id="slicedrain-iterator-type-item"></span>`type Item = T`

- <span id="slicedrain-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="slicedrain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="slicedrain-iterator-count"></span>`fn count(self) -> usize`

##### `impl<T> Pointable for SliceDrain<'data, T>`

- <span id="slicedrain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="slicedrain-pointable-type-init"></span>`type Init = T`

- <span id="slicedrain-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="slicedrain-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="slicedrain-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="slicedrain-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for SliceDrain<'data, T>`

- <span id="slicedrain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slicedrain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SliceDrain<'data, T>`

- <span id="slicedrain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slicedrain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

