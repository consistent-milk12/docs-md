*[rayon](../../index.md) / [slice](../index.md) / [chunks](index.md)*

---

# Module `chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Chunks`](#chunks) | struct | Parallel iterator over immutable non-overlapping chunks of a slice |
| [`ChunksProducer`](#chunksproducer) | struct |  |
| [`ChunksExact`](#chunksexact) | struct | Parallel iterator over immutable non-overlapping chunks of a slice |
| [`ChunksExactProducer`](#chunksexactproducer) | struct |  |
| [`ChunksMut`](#chunksmut) | struct | Parallel iterator over mutable non-overlapping chunks of a slice |
| [`ChunksMutProducer`](#chunksmutproducer) | struct |  |
| [`ChunksExactMut`](#chunksexactmut) | struct | Parallel iterator over mutable non-overlapping chunks of a slice |
| [`ChunksExactMutProducer`](#chunksexactmutproducer) | struct |  |

## Structs

### `Chunks<'data, T>`

```rust
struct Chunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:6-9`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L6-L9)*

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Any for Chunks<'data, T>`

- <span id="chunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chunks<'data, T>`

- <span id="chunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chunks<'data, T>`

- <span id="chunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Chunks<'_, T>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Chunks<'data, T>`

- <span id="chunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Chunks<'data, T>`

- <span id="chunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Chunks<'data, T>`

- <span id="chunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Sync> IndexedParallelIterator for Chunks<'_, T>`

- <span id="chunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for Chunks<'data, T>`

- <span id="chunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Chunks<'data, T>`

##### `impl<T> IntoParallelIterator for Chunks<'data, T>`

- <span id="chunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Chunks<'data, T>`

- <span id="chunks-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="chunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunks-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<'data, T>`

- <span id="chunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunks-pointable-type-init"></span>`type Init = T`

- <span id="chunks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Chunks<'data, T>`

- <span id="chunks-toowned-type-owned"></span>`type Owned = T`

- <span id="chunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Chunks<'data, T>`

- <span id="chunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Chunks<'data, T>`

- <span id="chunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunksProducer<'data, T: Sync>`

```rust
struct ChunksProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:61-64`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L61-L64)*

#### Trait Implementations

##### `impl<T> Any for ChunksProducer<'data, T>`

- <span id="chunksproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunksProducer<'data, T>`

- <span id="chunksproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunksProducer<'data, T>`

- <span id="chunksproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunksProducer<'data, T>`

- <span id="chunksproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunksProducer<'data, T>`

- <span id="chunksproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunksProducer<'data, T>`

##### `impl<T> Pointable for ChunksProducer<'data, T>`

- <span id="chunksproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksproducer-pointable-type-init"></span>`type Init = T`

- <span id="chunksproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for ChunksProducer<'data, T>`

- <span id="chunksproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="chunksproducer-producer-type-intoiter"></span>`type IntoIter = Chunks<'data, T>`

- <span id="chunksproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="chunksproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for ChunksProducer<'data, T>`

- <span id="chunksproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunksproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunksProducer<'data, T>`

- <span id="chunksproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunksproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunksExact<'data, T>`

```rust
struct ChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:92-96`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L92-L96)*

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksexact-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- <span id="chunksexact-remainder"></span>`fn remainder(&self) -> &'data [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements.

#### Trait Implementations

##### `impl<T> Any for ChunksExact<'data, T>`

- <span id="chunksexact-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunksExact<'data, T>`

- <span id="chunksexact-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunksExact<'data, T>`

- <span id="chunksexact-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for ChunksExact<'_, T>`

- <span id="chunksexact-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for ChunksExact<'data, T>`

- <span id="chunksexact-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for ChunksExact<'data, T>`

- <span id="chunksexact-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunksExact<'data, T>`

- <span id="chunksexact-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Sync> IndexedParallelIterator for ChunksExact<'_, T>`

- <span id="chunksexact-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunksexact-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunksexact-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for ChunksExact<'data, T>`

- <span id="chunksexact-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunksexact-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexact-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="chunksexact-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunksexact-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExact<'data, T>`

- <span id="chunksexact-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksexact-pointable-type-init"></span>`type Init = T`

- <span id="chunksexact-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexact-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexact-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexact-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for ChunksExact<'data, T>`

- <span id="chunksexact-toowned-type-owned"></span>`type Owned = T`

- <span id="chunksexact-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunksexact-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for ChunksExact<'data, T>`

- <span id="chunksexact-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunksexact-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunksExact<'data, T>`

- <span id="chunksexact-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunksexact-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunksExactProducer<'data, T: Sync>`

```rust
struct ChunksExactProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:162-165`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L162-L165)*

#### Trait Implementations

##### `impl<T> Any for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunksExactProducer<'data, T>`

##### `impl<T> Pointable for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksexactproducer-pointable-type-init"></span>`type Init = T`

- <span id="chunksexactproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexactproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexactproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexactproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="chunksexactproducer-producer-type-intoiter"></span>`type IntoIter = ChunksExact<'data, T>`

- <span id="chunksexactproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="chunksexactproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunksexactproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunksexactproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunksMut<'data, T>`

```rust
struct ChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:193-196`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L193-L196)*

Parallel iterator over mutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<T> Any for ChunksMut<'data, T>`

- <span id="chunksmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunksMut<'data, T>`

- <span id="chunksmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunksMut<'data, T>`

- <span id="chunksmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for ChunksMut<'data, T>`

- <span id="chunksmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunksMut<'data, T>`

- <span id="chunksmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for ChunksMut<'_, T>`

- <span id="chunksmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunksmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunksmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for ChunksMut<'data, T>`

- <span id="chunksmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunksmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksmut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunksmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunksmut-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksMut<'data, T>`

- <span id="chunksmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksmut-pointable-type-init"></span>`type Init = T`

- <span id="chunksmut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksmut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksmut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksmut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ChunksMut<'data, T>`

- <span id="chunksmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunksmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunksMut<'data, T>`

- <span id="chunksmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunksmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunksMutProducer<'data, T: Send>`

```rust
struct ChunksMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:242-245`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L242-L245)*

#### Trait Implementations

##### `impl<T> Any for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunksMutProducer<'data, T>`

##### `impl<T> Pointable for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksmutproducer-pointable-type-init"></span>`type Init = T`

- <span id="chunksmutproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksmutproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksmutproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksmutproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-producer-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunksmutproducer-producer-type-intoiter"></span>`type IntoIter = ChunksMut<'data, T>`

- <span id="chunksmutproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="chunksmutproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunksmutproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunksmutproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunksExactMut<'data, T>`

```rust
struct ChunksExactMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:273-277`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L273-L277)*

Parallel iterator over mutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksexactmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- <span id="chunksexactmut-into-remainder"></span>`fn into_remainder(self) -> &'data mut [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements.

  

  Note that this has to consume `self` to return the original lifetime of

  the data, which prevents this from actually being used as a parallel

  iterator since that also consumes. This method is provided for parity

  with `std::iter::ChunksExactMut`, but consider calling `remainder()` or

  `take_remainder()` as alternatives.

- <span id="chunksexactmut-remainder"></span>`fn remainder(&mut self) -> &mut [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements.

  

  Consider `take_remainder()` if you need access to the data with its

  original lifetime, rather than borrowing through `&mut self` here.

- <span id="chunksexactmut-take-remainder"></span>`fn take_remainder(&mut self) -> &'data mut [T]`

  Return the remainder of the original slice that is not going to be

  returned by the iterator. The returned slice has at most `chunk_size-1`

  elements. Subsequent calls will return an empty slice.

#### Trait Implementations

##### `impl<T> Any for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for ChunksExactMut<'_, T>`

- <span id="chunksexactmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunksexactmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunksexactmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunksexactmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexactmut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunksexactmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="chunksexactmut-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksexactmut-pointable-type-init"></span>`type Init = T`

- <span id="chunksexactmut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexactmut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexactmut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexactmut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunksexactmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunksexactmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunksExactMutProducer<'data, T: Send>`

```rust
struct ChunksExactMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:360-363`](../../../../.source_1765633015/rayon-1.11.0/src/slice/chunks.rs#L360-L363)*

#### Trait Implementations

##### `impl<T> Any for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunksExactMutProducer<'data, T>`

##### `impl<T> Pointable for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksexactmutproducer-pointable-type-init"></span>`type Init = T`

- <span id="chunksexactmutproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexactmutproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexactmutproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexactmutproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-producer-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunksexactmutproducer-producer-type-intoiter"></span>`type IntoIter = ChunksExactMut<'data, T>`

- <span id="chunksexactmutproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="chunksexactmutproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunksexactmutproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunksexactmutproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

