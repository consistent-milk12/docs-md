*[rayon](../../index.md) / [slice](../index.md) / [rchunks](index.md)*

---

# Module `rchunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RChunks`](#rchunks) | struct | Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end. |
| [`RChunksProducer`](#rchunksproducer) | struct |  |
| [`RChunksExact`](#rchunksexact) | struct | Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end. |
| [`RChunksExactProducer`](#rchunksexactproducer) | struct |  |
| [`RChunksMut`](#rchunksmut) | struct | Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end. |
| [`RChunksMutProducer`](#rchunksmutproducer) | struct |  |
| [`RChunksExactMut`](#rchunksexactmut) | struct | Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end. |
| [`RChunksExactMutProducer`](#rchunksexactmutproducer) | struct |  |

## Structs

### `RChunks<'data, T>`

```rust
struct RChunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:6-9`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L6-L9)*

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Any for RChunks<'data, T>`

- <span id="rchunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunks<'data, T>`

- <span id="rchunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunks<'data, T>`

- <span id="rchunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for RChunks<'_, T>`

- <span id="rchunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for RChunks<'data, T>`

- <span id="rchunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for RChunks<'data, T>`

- <span id="rchunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RChunks<'data, T>`

- <span id="rchunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Sync> IndexedParallelIterator for RChunks<'_, T>`

- <span id="rchunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for RChunks<'data, T>`

- <span id="rchunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunks<'data, T>`

##### `impl<T> IntoParallelIterator for RChunks<'data, T>`

- <span id="rchunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for RChunks<'data, T>`

- <span id="rchunks-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="rchunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunks-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunks<'data, T>`

- <span id="rchunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunks-pointable-type-init"></span>`type Init = T`

- <span id="rchunks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for RChunks<'data, T>`

- <span id="rchunks-toowned-type-owned"></span>`type Owned = T`

- <span id="rchunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rchunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RChunks<'data, T>`

- <span id="rchunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunks<'data, T>`

- <span id="rchunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RChunksProducer<'data, T: Sync>`

```rust
struct RChunksProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:61-64`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L61-L64)*

#### Trait Implementations

##### `impl<T> Any for RChunksProducer<'data, T>`

- <span id="rchunksproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunksProducer<'data, T>`

- <span id="rchunksproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunksProducer<'data, T>`

- <span id="rchunksproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RChunksProducer<'data, T>`

- <span id="rchunksproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RChunksProducer<'data, T>`

- <span id="rchunksproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunksProducer<'data, T>`

##### `impl<T> Pointable for RChunksProducer<'data, T>`

- <span id="rchunksproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for RChunksProducer<'data, T>`

- <span id="rchunksproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="rchunksproducer-producer-type-intoiter"></span>`type IntoIter = RChunks<'data, T>`

- <span id="rchunksproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for RChunksProducer<'data, T>`

- <span id="rchunksproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunksproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunksProducer<'data, T>`

- <span id="rchunksproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunksproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RChunksExact<'data, T>`

```rust
struct RChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:92-96`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L92-L96)*

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksexact-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- <span id="rchunksexact-remainder"></span>`fn remainder(&self) -> &'data [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements.

#### Trait Implementations

##### `impl<T> Any for RChunksExact<'data, T>`

- <span id="rchunksexact-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunksExact<'data, T>`

- <span id="rchunksexact-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunksExact<'data, T>`

- <span id="rchunksexact-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for RChunksExact<'_, T>`

- <span id="rchunksexact-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for RChunksExact<'data, T>`

- <span id="rchunksexact-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for RChunksExact<'data, T>`

- <span id="rchunksexact-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RChunksExact<'data, T>`

- <span id="rchunksexact-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Sync> IndexedParallelIterator for RChunksExact<'_, T>`

- <span id="rchunksexact-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexact-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexact-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for RChunksExact<'data, T>`

- <span id="rchunksexact-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksexact-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexact-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="rchunksexact-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexact-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExact<'data, T>`

- <span id="rchunksexact-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexact-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexact-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexact-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexact-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexact-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for RChunksExact<'data, T>`

- <span id="rchunksexact-toowned-type-owned"></span>`type Owned = T`

- <span id="rchunksexact-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rchunksexact-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for RChunksExact<'data, T>`

- <span id="rchunksexact-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunksexact-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunksExact<'data, T>`

- <span id="rchunksexact-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunksexact-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RChunksExactProducer<'data, T: Sync>`

```rust
struct RChunksExactProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:161-164`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L161-L164)*

#### Trait Implementations

##### `impl<T> Any for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunksExactProducer<'data, T>`

##### `impl<T> Pointable for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexactproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexactproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexactproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexactproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexactproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="rchunksexactproducer-producer-type-intoiter"></span>`type IntoIter = RChunksExact<'data, T>`

- <span id="rchunksexactproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksexactproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunksexactproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunksexactproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RChunksMut<'data, T>`

```rust
struct RChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:192-195`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L192-L195)*

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<T> Any for RChunksMut<'data, T>`

- <span id="rchunksmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunksMut<'data, T>`

- <span id="rchunksmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunksMut<'data, T>`

- <span id="rchunksmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for RChunksMut<'data, T>`

- <span id="rchunksmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RChunksMut<'data, T>`

- <span id="rchunksmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for RChunksMut<'_, T>`

- <span id="rchunksmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunksmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for RChunksMut<'data, T>`

- <span id="rchunksmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksmut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksmut-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksMut<'data, T>`

- <span id="rchunksmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksmut-pointable-type-init"></span>`type Init = T`

- <span id="rchunksmut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksmut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksmut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksmut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for RChunksMut<'data, T>`

- <span id="rchunksmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunksmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunksMut<'data, T>`

- <span id="rchunksmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunksmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RChunksMutProducer<'data, T: Send>`

```rust
struct RChunksMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:241-244`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L241-L244)*

#### Trait Implementations

##### `impl<T> Any for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunksMutProducer<'data, T>`

##### `impl<T> Pointable for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksmutproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksmutproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksmutproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksmutproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksmutproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-producer-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksmutproducer-producer-type-intoiter"></span>`type IntoIter = RChunksMut<'data, T>`

- <span id="rchunksmutproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksmutproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunksmutproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunksmutproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RChunksExactMut<'data, T: Send>`

```rust
struct RChunksExactMut<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:272-276`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L272-L276)*

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksexactmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- <span id="rchunksexactmut-into-remainder"></span>`fn into_remainder(self) -> &'data mut [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements.

  

  Note that this has to consume `self` to return the original lifetime of

  the data, which prevents this from actually being used as a parallel

  iterator since that also consumes. This method is provided for parity

  with `std::iter::RChunksExactMut`, but consider calling `remainder()` or

  `take_remainder()` as alternatives.

- <span id="rchunksexactmut-remainder"></span>`fn remainder(&mut self) -> &mut [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements.

  

  Consider `take_remainder()` if you need access to the data with its

  original lifetime, rather than borrowing through `&mut self` here.

- <span id="rchunksexactmut-take-remainder"></span>`fn take_remainder(&mut self) -> &'data mut [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements. Subsequent calls will return an empty slice.

#### Trait Implementations

##### `impl<T> Any for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug + Send> Debug for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send + 'data> IndexedParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexactmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexactmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksexactmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexactmut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send + 'data> ParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksexactmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexactmut-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexactmut-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexactmut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexactmut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexactmut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexactmut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunksexactmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunksexactmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RChunksExactMutProducer<'data, T: Send>`

```rust
struct RChunksExactMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:358-361`](../../../../.source_1765633015/rayon-1.11.0/src/slice/rchunks.rs#L358-L361)*

#### Trait Implementations

##### `impl<T> Any for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for RChunksExactMutProducer<'data, T>`

##### `impl<T> Pointable for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexactmutproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexactmutproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexactmutproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexactmutproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexactmutproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-producer-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksexactmutproducer-producer-type-intoiter"></span>`type IntoIter = RChunksExactMut<'data, T>`

- <span id="rchunksexactmutproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksexactmutproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rchunksexactmutproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rchunksexactmutproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

