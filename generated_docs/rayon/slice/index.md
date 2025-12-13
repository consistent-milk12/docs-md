*[rayon](../index.md) / [slice](index.md)*

---

# Module `slice`

Parallel iterator types for [slices]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Contents

- [Modules](#modules)
  - [`chunk_by`](#chunk-by)
  - [`chunks`](#chunks)
  - [`rchunks`](#rchunks)
  - [`sort`](#sort)
- [Structs](#structs)
  - [`ChunkBy`](#chunkby)
  - [`ChunkByMut`](#chunkbymut)
  - [`Chunks`](#chunks)
  - [`ChunksExact`](#chunksexact)
  - [`ChunksExactMut`](#chunksexactmut)
  - [`ChunksMut`](#chunksmut)
  - [`RChunks`](#rchunks)
  - [`RChunksExact`](#rchunksexact)
  - [`RChunksExactMut`](#rchunksexactmut)
  - [`RChunksMut`](#rchunksmut)
  - [`Iter`](#iter)
  - [`IterProducer`](#iterproducer)
  - [`Windows`](#windows)
  - [`WindowsProducer`](#windowsproducer)
  - [`IterMut`](#itermut)
  - [`IterMutProducer`](#itermutproducer)
  - [`Split`](#split)
  - [`SplitInclusive`](#splitinclusive)
  - [`SplitMut`](#splitmut)
  - [`SplitInclusiveMut`](#splitinclusivemut)
- [Traits](#traits)
  - [`ParallelSlice`](#parallelslice)
  - [`ParallelSliceMut`](#parallelslicemut)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`chunk_by`](#chunk-by) | mod |  |
| [`chunks`](#chunks) | mod |  |
| [`rchunks`](#rchunks) | mod |  |
| [`sort`](#sort) | mod | **Parallel** Slice sorting |
| [`ChunkBy`](#chunkby) | struct |  |
| [`ChunkByMut`](#chunkbymut) | struct |  |
| [`Chunks`](#chunks) | struct |  |
| [`ChunksExact`](#chunksexact) | struct |  |
| [`ChunksExactMut`](#chunksexactmut) | struct |  |
| [`ChunksMut`](#chunksmut) | struct |  |
| [`RChunks`](#rchunks) | struct |  |
| [`RChunksExact`](#rchunksexact) | struct |  |
| [`RChunksExactMut`](#rchunksexactmut) | struct |  |
| [`RChunksMut`](#rchunksmut) | struct |  |
| [`Iter`](#iter) | struct | Parallel iterator over immutable items in a slice |
| [`IterProducer`](#iterproducer) | struct |  |
| [`Windows`](#windows) | struct | Parallel iterator over immutable overlapping windows of a slice |
| [`WindowsProducer`](#windowsproducer) | struct |  |
| [`IterMut`](#itermut) | struct | Parallel iterator over mutable items in a slice |
| [`IterMutProducer`](#itermutproducer) | struct |  |
| [`Split`](#split) | struct | Parallel iterator over slices separated by a predicate |
| [`SplitInclusive`](#splitinclusive) | struct | Parallel iterator over slices separated by a predicate, including the matched part as a terminator. |
| [`SplitMut`](#splitmut) | struct | Parallel iterator over mutable slices separated by a predicate |
| [`SplitInclusiveMut`](#splitinclusivemut) | struct | Parallel iterator over mutable slices separated by a predicate, including the matched part as a terminator. |
| [`ParallelSlice`](#parallelslice) | trait | Parallel extensions for slices. |
| [`ParallelSliceMut`](#parallelslicemut) | trait | Parallel extensions for mutable slices. |

## Modules

- [`chunk_by`](chunk_by/index.md)
- [`chunks`](chunks/index.md)
- [`rchunks`](rchunks/index.md)
- [`sort`](sort/index.md) — **Parallel** Slice sorting

## Structs

### `ChunkBy<'data, T, P>`

```rust
struct ChunkBy<'data, T, P> {
    pred: P,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:142-145`](../../../.source_1765521767/rayon-1.11.0/src/slice/chunk_by.rs#L142-L145)*

Parallel iterator over slice in (non-overlapping) chunks separated by a predicate.

This struct is created by the `par_chunk_by` method on `&[T]`.


#### Implementations

- <span id="chunkby-new"></span>`fn new(slice: &'data [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T> Any for ChunkBy<'data, T, P>`

- <span id="chunkby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkBy<'data, T, P>`

- <span id="chunkby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkBy<'data, T, P>`

- <span id="chunkby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P: Clone> Clone for ChunkBy<'_, T, P>`

- <span id="chunkby-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for ChunkBy<'data, T, P>`

- <span id="chunkby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug, P> Debug for ChunkBy<'_, T, P>`

- <span id="chunkby-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunkBy<'data, T, P>`

- <span id="chunkby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunkBy<'data, T, P>`

- <span id="chunkby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunkBy<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunkby-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkby-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="chunkby-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for ChunkBy<'data, T, P>`

- <span id="chunkby-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkby-pointable-type-init"></span>`type Init = T`

- <span id="chunkby-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkby-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkby-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkby-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for ChunkBy<'data, T, P>`

- <span id="chunkby-toowned-type-owned"></span>`type Owned = T`

- <span id="chunkby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunkby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for ChunkBy<'data, T, P>`

- <span id="chunkby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunkBy<'data, T, P>`

- <span id="chunkby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunkByMut<'data, T, P>`

```rust
struct ChunkByMut<'data, T, P> {
    pred: P,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:199-202`](../../../.source_1765521767/rayon-1.11.0/src/slice/chunk_by.rs#L199-L202)*

Parallel iterator over slice in (non-overlapping) mutable chunks
separated by a predicate.

This struct is created by the `par_chunk_by_mut` method on `&mut [T]`.


#### Implementations

- <span id="chunkbymut-new"></span>`fn new(slice: &'data mut [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T> Any for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, P> Debug for ChunkByMut<'_, T, P>`

- <span id="chunkbymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunkByMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunkbymut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkbymut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunkbymut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkbymut-pointable-type-init"></span>`type Init = T`

- <span id="chunkbymut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkbymut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkbymut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkbymut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkbymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkbymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Chunks<'data, T>`

```rust
struct Chunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:6-9`](../../../.source_1765521767/rayon-1.11.0/src/slice/chunks.rs#L6-L9)*

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

- <span id="chunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="chunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `ChunksExact<'data, T>`

```rust
struct ChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:92-96`](../../../.source_1765521767/rayon-1.11.0/src/slice/chunks.rs#L92-L96)*

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

- <span id="chunksexact-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksexact-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunksexact-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="chunksexact-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `ChunksExactMut<'data, T>`

```rust
struct ChunksExactMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:273-277`](../../../.source_1765521767/rayon-1.11.0/src/slice/chunks.rs#L273-L277)*

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

- <span id="chunksexactmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksexactmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunksexactmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="chunksexactmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `ChunksMut<'data, T>`

```rust
struct ChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:193-196`](../../../.source_1765521767/rayon-1.11.0/src/slice/chunks.rs#L193-L196)*

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

- <span id="chunksmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunksmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="chunksmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `RChunks<'data, T>`

```rust
struct RChunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:6-9`](../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L6-L9)*

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

- <span id="rchunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="rchunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `RChunksExact<'data, T>`

```rust
struct RChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:92-96`](../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L92-L96)*

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

- <span id="rchunksexact-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksexact-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexact-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="rchunksexact-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `RChunksExactMut<'data, T: Send>`

```rust
struct RChunksExactMut<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:272-276`](../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L272-L276)*

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

- <span id="rchunksexactmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksexactmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexactmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="rchunksexactmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `RChunksMut<'data, T>`

```rust
struct RChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:192-195`](../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L192-L195)*

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

- <span id="rchunksmut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksmut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rchunksmut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

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

- <span id="rchunksmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

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

### `Iter<'data, T>`

```rust
struct Iter<'data, T> {
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:801-803`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L801-L803)*

Parallel iterator over immutable items in a slice

#### Trait Implementations

##### `impl<T> Any for Iter<'data, T>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'data, T>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'data, T>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Iter<'data, T>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Iter<'data, T>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Iter<'data, T>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Sync> IndexedParallelIterator for Iter<'_, T>`

- <span id="iter-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="iter-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for Iter<'data, T>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Iter<'data, T>`

##### `impl<T> IntoParallelIterator for Iter<'data, T>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Iter<'data, T>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = &'data T`

- <span id="iter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'data, T>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Iter<'data, T>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Iter<'data, T>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Iter<'data, T>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterProducer<'data, T: Sync>`

```rust
struct IterProducer<'data, T: Sync> {
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:846-848`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L846-L848)*

#### Trait Implementations

##### `impl<T> Any for IterProducer<'data, T>`

- <span id="iterproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterProducer<'data, T>`

- <span id="iterproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterProducer<'data, T>`

- <span id="iterproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IterProducer<'data, T>`

- <span id="iterproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for IterProducer<'data, T>`

- <span id="iterproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IterProducer<'data, T>`

##### `impl<T> Pointable for IterProducer<'data, T>`

- <span id="iterproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iterproducer-pointable-type-init"></span>`type Init = T`

- <span id="iterproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for IterProducer<'data, T>`

- <span id="iterproducer-producer-type-item"></span>`type Item = &'data T`

- <span id="iterproducer-producer-type-intoiter"></span>`type IntoIter = Iter<'data, T>`

- <span id="iterproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="iterproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for IterProducer<'data, T>`

- <span id="iterproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterProducer<'data, T>`

- <span id="iterproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Windows<'data, T>`

```rust
struct Windows<'data, T> {
    window_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:866-869`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L866-L869)*

Parallel iterator over immutable overlapping windows of a slice

#### Trait Implementations

##### `impl<T> Any for Windows<'data, T>`

- <span id="windows-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Windows<'data, T>`

- <span id="windows-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Windows<'data, T>`

- <span id="windows-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Windows<'_, T>`

- <span id="windows-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Windows<'data, T>`

- <span id="windows-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Windows<'data, T>`

- <span id="windows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Windows<'data, T>`

- <span id="windows-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Sync> IndexedParallelIterator for Windows<'_, T>`

- <span id="windows-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="windows-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="windows-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for Windows<'data, T>`

- <span id="windows-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Windows<'data, T>`

##### `impl<T> IntoParallelIterator for Windows<'data, T>`

- <span id="windows-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="windows-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="windows-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Windows<'data, T>`

- <span id="windows-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="windows-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="windows-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Windows<'data, T>`

- <span id="windows-pointable-const-align"></span>`const ALIGN: usize`

- <span id="windows-pointable-type-init"></span>`type Init = T`

- <span id="windows-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="windows-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="windows-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="windows-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Windows<'data, T>`

- <span id="windows-toowned-type-owned"></span>`type Owned = T`

- <span id="windows-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="windows-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Windows<'data, T>`

- <span id="windows-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="windows-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Windows<'data, T>`

- <span id="windows-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="windows-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WindowsProducer<'data, T: Sync>`

```rust
struct WindowsProducer<'data, T: Sync> {
    window_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:916-919`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L916-L919)*

#### Trait Implementations

##### `impl<T> Any for WindowsProducer<'data, T>`

- <span id="windowsproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WindowsProducer<'data, T>`

- <span id="windowsproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WindowsProducer<'data, T>`

- <span id="windowsproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for WindowsProducer<'data, T>`

- <span id="windowsproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for WindowsProducer<'data, T>`

- <span id="windowsproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for WindowsProducer<'data, T>`

##### `impl<T> Pointable for WindowsProducer<'data, T>`

- <span id="windowsproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="windowsproducer-pointable-type-init"></span>`type Init = T`

- <span id="windowsproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="windowsproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="windowsproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="windowsproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for WindowsProducer<'data, T>`

- <span id="windowsproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="windowsproducer-producer-type-intoiter"></span>`type IntoIter = Windows<'data, T>`

- <span id="windowsproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="windowsproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for WindowsProducer<'data, T>`

- <span id="windowsproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="windowsproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for WindowsProducer<'data, T>`

- <span id="windowsproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="windowsproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'data, T>`

```rust
struct IterMut<'data, T> {
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:948-950`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L948-L950)*

Parallel iterator over mutable items in a slice

#### Trait Implementations

##### `impl<T> Any for IterMut<'data, T>`

- <span id="itermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMut<'data, T>`

- <span id="itermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMut<'data, T>`

- <span id="itermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for IterMut<'data, T>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IterMut<'data, T>`

- <span id="itermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for IterMut<'_, T>`

- <span id="itermut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="itermut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for IterMut<'data, T>`

- <span id="itermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IterMut<'data, T>`

##### `impl<T> IntoParallelIterator for IterMut<'data, T>`

- <span id="itermut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="itermut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IterMut<'data, T>`

- <span id="itermut-paralleliterator-type-item"></span>`type Item = &'data mut T`

- <span id="itermut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'data, T>`

- <span id="itermut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermut-pointable-type-init"></span>`type Init = T`

- <span id="itermut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for IterMut<'data, T>`

- <span id="itermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterMut<'data, T>`

- <span id="itermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMutProducer<'data, T: Send>`

```rust
struct IterMutProducer<'data, T: Send> {
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:987-989`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L987-L989)*

#### Trait Implementations

##### `impl<T> Any for IterMutProducer<'data, T>`

- <span id="itermutproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMutProducer<'data, T>`

- <span id="itermutproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMutProducer<'data, T>`

- <span id="itermutproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IterMutProducer<'data, T>`

- <span id="itermutproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for IterMutProducer<'data, T>`

- <span id="itermutproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IterMutProducer<'data, T>`

##### `impl<T> Pointable for IterMutProducer<'data, T>`

- <span id="itermutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermutproducer-pointable-type-init"></span>`type Init = T`

- <span id="itermutproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermutproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermutproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermutproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for IterMutProducer<'data, T>`

- <span id="itermutproducer-producer-type-item"></span>`type Item = &'data mut T`

- <span id="itermutproducer-producer-type-intoiter"></span>`type IntoIter = IterMut<'data, T>`

- <span id="itermutproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="itermutproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for IterMutProducer<'data, T>`

- <span id="itermutproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermutproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterMutProducer<'data, T>`

- <span id="itermutproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermutproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Split<'data, T, P>`

```rust
struct Split<'data, T, P> {
    slice: &'data [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1009-1012`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L1009-L1012)*

Parallel iterator over slices separated by a predicate

#### Trait Implementations

##### `impl<T> Any for Split<'data, T, P>`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split<'data, T, P>`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split<'data, T, P>`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P: Clone> Clone for Split<'_, T, P>`

- <span id="split-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Split<'data, T, P>`

- <span id="split-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: Debug, P> Debug for Split<'_, T, P>`

- <span id="split-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Split<'data, T, P>`

- <span id="split-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Split<'data, T, P>`

- <span id="split-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Split<'data, T, P>`

##### `impl<T> IntoParallelIterator for Split<'data, T, P>`

- <span id="split-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="split-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="split-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for Split<'data, T, P>`

- <span id="split-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="split-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for Split<'data, T, P>`

- <span id="split-pointable-const-align"></span>`const ALIGN: usize`

- <span id="split-pointable-type-init"></span>`type Init = T`

- <span id="split-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="split-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="split-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="split-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Split<'data, T, P>`

- <span id="split-toowned-type-owned"></span>`type Owned = T`

- <span id="split-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="split-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Split<'data, T, P>`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Split<'data, T, P>`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitInclusive<'data, T, P>`

```rust
struct SplitInclusive<'data, T, P> {
    slice: &'data [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1047-1050`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L1047-L1050)*

Parallel iterator over slices separated by a predicate,
including the matched part as a terminator.

#### Trait Implementations

##### `impl<T> Any for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P: Clone> Clone for SplitInclusive<'_, T, P>`

- <span id="splitinclusive-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: Debug, P> Debug for SplitInclusive<'_, T, P>`

- <span id="splitinclusive-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for SplitInclusive<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitinclusive-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusive-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="splitinclusive-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitinclusive-pointable-type-init"></span>`type Init = T`

- <span id="splitinclusive-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitinclusive-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitinclusive-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitinclusive-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-toowned-type-owned"></span>`type Owned = T`

- <span id="splitinclusive-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="splitinclusive-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitinclusive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitinclusive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitMut<'data, T, P>`

```rust
struct SplitMut<'data, T, P> {
    slice: &'data mut [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1135-1138`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L1135-L1138)*

Parallel iterator over mutable slices separated by a predicate

#### Trait Implementations

##### `impl<T> Any for SplitMut<'data, T, P>`

- <span id="splitmut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitMut<'data, T, P>`

- <span id="splitmut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitMut<'data, T, P>`

- <span id="splitmut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Debug, P> Debug for SplitMut<'_, T, P>`

- <span id="splitmut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitMut<'data, T, P>`

- <span id="splitmut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for SplitMut<'data, T, P>`

- <span id="splitmut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for SplitMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitMut<'data, T, P>`

- <span id="splitmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitmut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for SplitMut<'data, T, P>`

- <span id="splitmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="splitmut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitMut<'data, T, P>`

- <span id="splitmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitmut-pointable-type-init"></span>`type Init = T`

- <span id="splitmut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitmut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitmut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitmut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for SplitMut<'data, T, P>`

- <span id="splitmut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitmut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SplitMut<'data, T, P>`

- <span id="splitmut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitmut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitInclusiveMut<'data, T, P>`

```rust
struct SplitInclusiveMut<'data, T, P> {
    slice: &'data mut [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1166-1169`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L1166-L1169)*

Parallel iterator over mutable slices separated by a predicate,
including the matched part as a terminator.

#### Trait Implementations

##### `impl<T> Any for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Debug, P> Debug for SplitInclusiveMut<'_, T, P>`

- <span id="splitinclusivemut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for SplitInclusiveMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitinclusivemut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusivemut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="splitinclusivemut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitinclusivemut-pointable-type-init"></span>`type Init = T`

- <span id="splitinclusivemut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitinclusivemut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitinclusivemut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitinclusivemut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitinclusivemut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitinclusivemut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ParallelSlice<T: Sync>`

```rust
trait ParallelSlice<T: Sync> { ... }
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:29-199`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L29-L199)*

Parallel extensions for slices.

#### Required Methods

- `fn as_parallel_slice(&self) -> &[T]`

  Returns a plain slice, which is used to implement the rest of the

#### Provided Methods

- `fn par_split<P>(&self, separator: P) -> Split<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that

- `fn par_split_inclusive<P>(&self, separator: P) -> SplitInclusive<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that

- `fn par_windows(&self, window_size: usize) -> Windows<'_, T>`

  Returns a parallel iterator over all contiguous windows of length

- `fn par_chunks(&self, chunk_size: usize) -> Chunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of

- `fn par_chunks_exact(&self, chunk_size: usize) -> ChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of

- `fn par_rchunks(&self, chunk_size: usize) -> RChunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- `fn par_rchunks_exact(&self, chunk_size: usize) -> RChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- `fn par_chunk_by<F>(&self, pred: F) -> ChunkBy<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping runs

#### Implementors

- `[T]`

### `ParallelSliceMut<T: Send>`

```rust
trait ParallelSliceMut<T: Send> { ... }
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:209-754`](../../../.source_1765521767/rayon-1.11.0/src/slice/mod.rs#L209-L754)*

Parallel extensions for mutable slices.

#### Required Methods

- `fn as_parallel_slice_mut(&mut self) -> &mut [T]`

  Returns a plain mutable slice, which is used to implement the rest of

#### Provided Methods

- `fn par_split_mut<P>(&mut self, separator: P) -> SplitMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by

- `fn par_split_inclusive_mut<P>(&mut self, separator: P) -> SplitInclusiveMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by elements

- `fn par_chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of

- `fn par_chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of

- `fn par_rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- `fn par_rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- `fn par_sort(&mut self)`

  Sorts the slice in parallel.

- `fn par_sort_by<F>(&mut self, compare: F)`

  Sorts the slice in parallel with a comparator function.

- `fn par_sort_by_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function.

- `fn par_sort_by_cached_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function.

- `fn par_sort_unstable(&mut self)`

  Sorts the slice in parallel, but might not preserve the order of equal elements.

- `fn par_sort_unstable_by<F>(&mut self, compare: F)`

  Sorts the slice in parallel with a comparator function, but might not preserve the order of

- `fn par_sort_unstable_by_key<K, F>(&mut self, f: F)`

  Sorts the slice in parallel with a key extraction function, but might not preserve the order

- `fn par_chunk_by_mut<F>(&mut self, pred: F) -> ChunkByMut<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping mutable

#### Implementors

- `[T]`

