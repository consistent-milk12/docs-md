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

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:142-145`](../../../.source_1765210505/rayon-1.11.0/src/slice/chunk_by.rs#L142-L145)*

Parallel iterator over slice in (non-overlapping) chunks separated by a predicate.

This struct is created by the `par_chunk_by` method on `&[T]`.


#### Implementations

- <span id="chunkby-new"></span>`fn new(slice: &'data [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T, P: Clone> Clone for ChunkBy<'_, T, P>`

- <span id="chunkby-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug, P> Debug for ChunkBy<'_, T, P>`

- <span id="chunkby-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkBy<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunkby-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkby-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="chunkby-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for ChunkBy<'data, T, P>`

- <span id="chunkby-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkby-pointable-type-init"></span>`type Init = T`

- <span id="chunkby-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkby-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkby-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkby-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunkByMut<'data, T, P>`

```rust
struct ChunkByMut<'data, T, P> {
    pred: P,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:199-202`](../../../.source_1765210505/rayon-1.11.0/src/slice/chunk_by.rs#L199-L202)*

Parallel iterator over slice in (non-overlapping) mutable chunks
separated by a predicate.

This struct is created by the `par_chunk_by_mut` method on `&mut [T]`.


#### Implementations

- <span id="chunkbymut-new"></span>`fn new(slice: &'data mut [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug, P> Debug for ChunkByMut<'_, T, P>`

- <span id="chunkbymut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkByMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunkbymut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkbymut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunkbymut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkbymut-pointable-type-init"></span>`type Init = T`

- <span id="chunkbymut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkbymut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkbymut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkbymut-drop"></span>`unsafe fn drop(ptr: usize)`

### `Chunks<'data, T>`

```rust
struct Chunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:6-9`](../../../.source_1765210505/rayon-1.11.0/src/slice/chunks.rs#L6-L9)*

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for Chunks<'_, T>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Chunks<'data, T>`

- <span id="chunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Chunks<'_, T>`

- <span id="chunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunks-len"></span>`fn len(&self) -> usize`

- <span id="chunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Chunks<'data, T>`

##### `impl<T> IntoParallelIterator for Chunks<'data, T>`

- <span id="chunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Chunks<'data, T>`

- <span id="chunks-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="chunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<'data, T>`

- <span id="chunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunks-pointable-type-init"></span>`type Init = T`

- <span id="chunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunks-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunksExact<'data, T>`

```rust
struct ChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:92-96`](../../../.source_1765210505/rayon-1.11.0/src/slice/chunks.rs#L92-L96)*

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksexact-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- <span id="chunksexact-remainder"></span>`fn remainder(&self) -> &'data [T]`

#### Trait Implementations

##### `impl<T> Clone for ChunksExact<'_, T>`

- <span id="chunksexact-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for ChunksExact<'data, T>`

- <span id="chunksexact-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for ChunksExact<'_, T>`

- <span id="chunksexact-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksexact-len"></span>`fn len(&self) -> usize`

- <span id="chunksexact-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for ChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunksexact-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexact-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="chunksexact-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksexact-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExact<'data, T>`

- <span id="chunksexact-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksexact-pointable-type-init"></span>`type Init = T`

- <span id="chunksexact-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexact-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexact-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexact-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunksExactMut<'data, T>`

```rust
struct ChunksExactMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:273-277`](../../../.source_1765210505/rayon-1.11.0/src/slice/chunks.rs#L273-L277)*

Parallel iterator over mutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksexactmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- <span id="chunksexactmut-into-remainder"></span>`fn into_remainder(self) -> &'data mut [T]`

- <span id="chunksexactmut-remainder"></span>`fn remainder(&mut self) -> &mut [T]`

- <span id="chunksexactmut-take-remainder"></span>`fn take_remainder(&mut self) -> &'data mut [T]`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for ChunksExactMut<'_, T>`

- <span id="chunksexactmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksexactmut-len"></span>`fn len(&self) -> usize`

- <span id="chunksexactmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for ChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunksexactmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexactmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunksexactmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksexactmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksexactmut-pointable-type-init"></span>`type Init = T`

- <span id="chunksexactmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexactmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexactmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexactmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunksMut<'data, T>`

```rust
struct ChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunks.rs:193-196`](../../../.source_1765210505/rayon-1.11.0/src/slice/chunks.rs#L193-L196)*

Parallel iterator over mutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for ChunksMut<'data, T>`

- <span id="chunksmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for ChunksMut<'_, T>`

- <span id="chunksmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksmut-len"></span>`fn len(&self) -> usize`

- <span id="chunksmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for ChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunksmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunksmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="chunksmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksMut<'data, T>`

- <span id="chunksmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunksmut-pointable-type-init"></span>`type Init = T`

- <span id="chunksmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunks<'data, T>`

```rust
struct RChunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:6-9`](../../../.source_1765210505/rayon-1.11.0/src/slice/rchunks.rs#L6-L9)*

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for RChunks<'_, T>`

- <span id="rchunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for RChunks<'data, T>`

- <span id="rchunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for RChunks<'_, T>`

- <span id="rchunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunks-len"></span>`fn len(&self) -> usize`

- <span id="rchunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunks<'data, T>`

##### `impl<T> IntoParallelIterator for RChunks<'data, T>`

- <span id="rchunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for RChunks<'data, T>`

- <span id="rchunks-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="rchunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunks<'data, T>`

- <span id="rchunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunks-pointable-type-init"></span>`type Init = T`

- <span id="rchunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunks-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunksExact<'data, T>`

```rust
struct RChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:92-96`](../../../.source_1765210505/rayon-1.11.0/src/slice/rchunks.rs#L92-L96)*

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksexact-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- <span id="rchunksexact-remainder"></span>`fn remainder(&self) -> &'data [T]`

#### Trait Implementations

##### `impl<T> Clone for RChunksExact<'_, T>`

- <span id="rchunksexact-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for RChunksExact<'data, T>`

- <span id="rchunksexact-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for RChunksExact<'_, T>`

- <span id="rchunksexact-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksexact-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexact-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksexact-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexact-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="rchunksexact-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksexact-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExact<'data, T>`

- <span id="rchunksexact-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexact-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexact-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexact-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexact-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexact-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunksExactMut<'data, T: Send>`

```rust
struct RChunksExactMut<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:272-276`](../../../.source_1765210505/rayon-1.11.0/src/slice/rchunks.rs#L272-L276)*

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksexactmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- <span id="rchunksexactmut-into-remainder"></span>`fn into_remainder(self) -> &'data mut [T]`

- <span id="rchunksexactmut-remainder"></span>`fn remainder(&mut self) -> &mut [T]`

- <span id="rchunksexactmut-take-remainder"></span>`fn take_remainder(&mut self) -> &'data mut [T]`

#### Trait Implementations

##### `impl<T: fmt::Debug + Send> Debug for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send + 'data> IndexedParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksexactmut-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexactmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksexactmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexactmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send + 'data> ParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksexactmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksexactmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexactmut-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexactmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexactmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexactmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexactmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunksMut<'data, T>`

```rust
struct RChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:192-195`](../../../.source_1765210505/rayon-1.11.0/src/slice/rchunks.rs#L192-L195)*

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for RChunksMut<'data, T>`

- <span id="rchunksmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for RChunksMut<'_, T>`

- <span id="rchunksmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksmut-len"></span>`fn len(&self) -> usize`

- <span id="rchunksmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="rchunksmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksMut<'data, T>`

- <span id="rchunksmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksmut-pointable-type-init"></span>`type Init = T`

- <span id="rchunksmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `Iter<'data, T>`

```rust
struct Iter<'data, T> {
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:801-803`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L801-L803)*

Parallel iterator over immutable items in a slice

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Iter<'data, T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Iter<'_, T>`

- <span id="iter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-len"></span>`fn len(&self) -> usize`

- <span id="iter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Iter<'data, T>`

##### `impl<T> IntoParallelIterator for Iter<'data, T>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Iter<'data, T>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = &'data T`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'data, T>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

### `IterProducer<'data, T: Sync>`

```rust
struct IterProducer<'data, T: Sync> {
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:846-848`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L846-L848)*

#### Trait Implementations

##### `impl<T> IntoEither for IterProducer<'data, T>`

##### `impl<T> Pointable for IterProducer<'data, T>`

- <span id="iterproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iterproducer-pointable-type-init"></span>`type Init = T`

- <span id="iterproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for IterProducer<'data, T>`

- <span id="iterproducer-producer-type-item"></span>`type Item = &'data T`

- <span id="iterproducer-producer-type-intoiter"></span>`type IntoIter = Iter<'data, T>`

- <span id="iterproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="iterproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `Windows<'data, T>`

```rust
struct Windows<'data, T> {
    window_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:866-869`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L866-L869)*

Parallel iterator over immutable overlapping windows of a slice

#### Trait Implementations

##### `impl<T> Clone for Windows<'_, T>`

- <span id="windows-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Windows<'data, T>`

- <span id="windows-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Windows<'_, T>`

- <span id="windows-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="windows-len"></span>`fn len(&self) -> usize`

- <span id="windows-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Windows<'data, T>`

##### `impl<T> IntoParallelIterator for Windows<'data, T>`

- <span id="windows-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="windows-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="windows-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Windows<'data, T>`

- <span id="windows-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="windows-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="windows-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Windows<'data, T>`

- <span id="windows-pointable-const-align"></span>`const ALIGN: usize`

- <span id="windows-pointable-type-init"></span>`type Init = T`

- <span id="windows-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="windows-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="windows-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="windows-drop"></span>`unsafe fn drop(ptr: usize)`

### `WindowsProducer<'data, T: Sync>`

```rust
struct WindowsProducer<'data, T: Sync> {
    window_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:916-919`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L916-L919)*

#### Trait Implementations

##### `impl<T> IntoEither for WindowsProducer<'data, T>`

##### `impl<T> Pointable for WindowsProducer<'data, T>`

- <span id="windowsproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="windowsproducer-pointable-type-init"></span>`type Init = T`

- <span id="windowsproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="windowsproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="windowsproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="windowsproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for WindowsProducer<'data, T>`

- <span id="windowsproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="windowsproducer-producer-type-intoiter"></span>`type IntoIter = Windows<'data, T>`

- <span id="windowsproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="windowsproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `IterMut<'data, T>`

```rust
struct IterMut<'data, T> {
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:948-950`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L948-L950)*

Parallel iterator over mutable items in a slice

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for IterMut<'data, T>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IterMut<'_, T>`

- <span id="itermut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-len"></span>`fn len(&self) -> usize`

- <span id="itermut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for IterMut<'data, T>`

##### `impl<T> IntoParallelIterator for IterMut<'data, T>`

- <span id="itermut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="itermut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IterMut<'data, T>`

- <span id="itermut-paralleliterator-type-item"></span>`type Item = &'data mut T`

- <span id="itermut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'data, T>`

- <span id="itermut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermut-pointable-type-init"></span>`type Init = T`

- <span id="itermut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-drop"></span>`unsafe fn drop(ptr: usize)`

### `IterMutProducer<'data, T: Send>`

```rust
struct IterMutProducer<'data, T: Send> {
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:987-989`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L987-L989)*

#### Trait Implementations

##### `impl<T> IntoEither for IterMutProducer<'data, T>`

##### `impl<T> Pointable for IterMutProducer<'data, T>`

- <span id="itermutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermutproducer-pointable-type-init"></span>`type Init = T`

- <span id="itermutproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermutproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermutproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermutproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for IterMutProducer<'data, T>`

- <span id="itermutproducer-producer-type-item"></span>`type Item = &'data mut T`

- <span id="itermutproducer-producer-type-intoiter"></span>`type IntoIter = IterMut<'data, T>`

- <span id="itermutproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="itermutproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `Split<'data, T, P>`

```rust
struct Split<'data, T, P> {
    slice: &'data [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1009-1012`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L1009-L1012)*

Parallel iterator over slices separated by a predicate

#### Trait Implementations

##### `impl<T, P: Clone> Clone for Split<'_, T, P>`

- <span id="split-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Debug, P> Debug for Split<'_, T, P>`

- <span id="split-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Split<'data, T, P>`

##### `impl<T> IntoParallelIterator for Split<'data, T, P>`

- <span id="split-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="split-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="split-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for Split<'data, T, P>`

- <span id="split-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="split-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for Split<'data, T, P>`

- <span id="split-pointable-const-align"></span>`const ALIGN: usize`

- <span id="split-pointable-type-init"></span>`type Init = T`

- <span id="split-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="split-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="split-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="split-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitInclusive<'data, T, P>`

```rust
struct SplitInclusive<'data, T, P> {
    slice: &'data [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1047-1050`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L1047-L1050)*

Parallel iterator over slices separated by a predicate,
including the matched part as a terminator.

#### Trait Implementations

##### `impl<T, P: Clone> Clone for SplitInclusive<'_, T, P>`

- <span id="splitinclusive-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Debug, P> Debug for SplitInclusive<'_, T, P>`

- <span id="splitinclusive-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitInclusive<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitinclusive-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusive-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="splitinclusive-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitinclusive-pointable-type-init"></span>`type Init = T`

- <span id="splitinclusive-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitinclusive-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitinclusive-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitinclusive-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitMut<'data, T, P>`

```rust
struct SplitMut<'data, T, P> {
    slice: &'data mut [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1135-1138`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L1135-L1138)*

Parallel iterator over mutable slices separated by a predicate

#### Trait Implementations

##### `impl<T: Debug, P> Debug for SplitMut<'_, T, P>`

- <span id="splitmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitMut<'data, T, P>`

- <span id="splitmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for SplitMut<'data, T, P>`

- <span id="splitmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="splitmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitMut<'data, T, P>`

- <span id="splitmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitmut-pointable-type-init"></span>`type Init = T`

- <span id="splitmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `SplitInclusiveMut<'data, T, P>`

```rust
struct SplitInclusiveMut<'data, T, P> {
    slice: &'data mut [T],
    separator: P,
}
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:1166-1169`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L1166-L1169)*

Parallel iterator over mutable slices separated by a predicate,
including the matched part as a terminator.

#### Trait Implementations

##### `impl<T: Debug, P> Debug for SplitInclusiveMut<'_, T, P>`

- <span id="splitinclusivemut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitInclusiveMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="splitinclusivemut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusivemut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="splitinclusivemut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="splitinclusivemut-pointable-type-init"></span>`type Init = T`

- <span id="splitinclusivemut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitinclusivemut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitinclusivemut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitinclusivemut-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `ParallelSlice<T: Sync>`

```rust
trait ParallelSlice<T: Sync> { ... }
```

*Defined in [`rayon-1.11.0/src/slice/mod.rs:29-199`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L29-L199)*

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

*Defined in [`rayon-1.11.0/src/slice/mod.rs:209-754`](../../../.source_1765210505/rayon-1.11.0/src/slice/mod.rs#L209-L754)*

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

