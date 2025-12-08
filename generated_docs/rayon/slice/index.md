*[rayon](../index.md) / [slice](index.md)*

---

# Module `slice`

Parallel iterator types for [slices]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Contents

- [Modules](#modules)
  - [`chunk_by`](#chunk_by)
  - [`chunks`](#chunks)
  - [`rchunks`](#rchunks)
  - [`sort`](#sort)
- [Structs](#structs)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
  - [`unnamed`](#unnamed)
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
| [`chunk_by`](#chunk_by) | mod |  |
| [`chunks`](#chunks) | mod |  |
| [`rchunks`](#rchunks) | mod |  |
| [`sort`](#sort) | mod | **Parallel** Slice sorting |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`unnamed`](#unnamed) | struct |  |
| [`Iter`](#iter) | struct | Parallel iterator over immutable items in a slice |
| [`IterProducer`](#iterproducer) | struct |  |
| [`Windows`](#windows) | struct | Parallel iterator over immutable overlapping windows of a slice |
| [`WindowsProducer`](#windowsproducer) | struct |  |
| [`IterMut`](#itermut) | struct | Parallel iterator over mutable items in a slice |
| [`IterMutProducer`](#itermutproducer) | struct |  |
| [`Split`](#split) | struct | Parallel iterator over slices separated by a predicate |
| [`SplitInclusive`](#splitinclusive) | struct | Parallel iterator over slices separated by a predicate |
| [`SplitMut`](#splitmut) | struct | Parallel iterator over mutable slices separated by a predicate |
| [`SplitInclusiveMut`](#splitinclusivemut) | struct | Parallel iterator over mutable slices separated by a predicate |
| [`ParallelSlice`](#parallelslice) | trait | Parallel extensions for slices. |
| [`ParallelSliceMut`](#parallelslicemut) | trait | Parallel extensions for mutable slices. |

## Modules

- [`chunk_by`](chunk_by/index.md) - 
- [`chunks`](chunks/index.md) - 
- [`rchunks`](rchunks/index.md) - 
- [`sort`](sort/index.md) - **Parallel** Slice sorting

## Structs

### `ChunkBy<'data, T, P>`

```rust
struct ChunkBy<'data, T, P> {
    pred: P,
    slice: &'data [T],
}
```

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

- <span id="chunkby-iter"></span>`type Iter = T`

- <span id="chunkby-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkby-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-item"></span>`type Item = &'data [T]`

- <span id="chunkby-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkBy<'data, T, P>`

- <span id="chunkby-align"></span>`const ALIGN: usize`

- <span id="chunkby-init"></span>`type Init = T`

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

- <span id="chunkbymut-iter"></span>`type Iter = T`

- <span id="chunkbymut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkbymut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-item"></span>`type Item = &'data mut [T]`

- <span id="chunkbymut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-align"></span>`const ALIGN: usize`

- <span id="chunkbymut-init"></span>`type Init = T`

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

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for Chunks<'_, T>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<'data, T: fmt::Debug> Debug for Chunks<'data, T>`

- <span id="chunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Chunks<'_, T>`

- <span id="chunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunks-len"></span>`fn len(&self) -> usize`

- <span id="chunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Chunks<'data, T>`

##### `impl<T> IntoParallelIterator for Chunks<'data, T>`

- <span id="chunks-iter"></span>`type Iter = T`

- <span id="chunks-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Chunks<'data, T>`

- <span id="chunks-item"></span>`type Item = &'data [T]`

- <span id="chunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<'data, T>`

- <span id="chunks-align"></span>`const ALIGN: usize`

- <span id="chunks-init"></span>`type Init = T`

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

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksexact-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- <span id="chunksexact-remainder"></span>`fn remainder(&self) -> &'data [T]`

#### Trait Implementations

##### `impl<T> Clone for ChunksExact<'_, T>`

- <span id="chunksexact-clone"></span>`fn clone(&self) -> Self`

##### `impl<'data, T: fmt::Debug> Debug for ChunksExact<'data, T>`

- <span id="chunksexact-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for ChunksExact<'_, T>`

- <span id="chunksexact-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunksexact-len"></span>`fn len(&self) -> usize`

- <span id="chunksexact-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-iter"></span>`type Iter = T`

- <span id="chunksexact-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexact-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-item"></span>`type Item = &'data [T]`

- <span id="chunksexact-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunksexact-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExact<'data, T>`

- <span id="chunksexact-align"></span>`const ALIGN: usize`

- <span id="chunksexact-init"></span>`type Init = T`

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

Parallel iterator over mutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksexactmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- <span id="chunksexactmut-into-remainder"></span>`fn into_remainder(self) -> &'data mut [T]`

- <span id="chunksexactmut-remainder"></span>`fn remainder(&mut self) -> &mut [T]`

- <span id="chunksexactmut-take-remainder"></span>`fn take_remainder(&mut self) -> &'data mut [T]`

#### Trait Implementations

##### `impl<'data, T: fmt::Debug> Debug for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for ChunksExactMut<'_, T>`

- <span id="chunksexactmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunksexactmut-len"></span>`fn len(&self) -> usize`

- <span id="chunksexactmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-iter"></span>`type Iter = T`

- <span id="chunksexactmut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexactmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-item"></span>`type Item = &'data mut [T]`

- <span id="chunksexactmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunksexactmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-align"></span>`const ALIGN: usize`

- <span id="chunksexactmut-init"></span>`type Init = T`

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

Parallel iterator over mutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunksmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<'data, T: fmt::Debug> Debug for ChunksMut<'data, T>`

- <span id="chunksmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for ChunksMut<'_, T>`

- <span id="chunksmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunksmut-len"></span>`fn len(&self) -> usize`

- <span id="chunksmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-iter"></span>`type Iter = T`

- <span id="chunksmut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-item"></span>`type Item = &'data mut [T]`

- <span id="chunksmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="chunksmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksMut<'data, T>`

- <span id="chunksmut-align"></span>`const ALIGN: usize`

- <span id="chunksmut-init"></span>`type Init = T`

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

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for RChunks<'_, T>`

- <span id="rchunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<'data, T: fmt::Debug> Debug for RChunks<'data, T>`

- <span id="rchunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for RChunks<'_, T>`

- <span id="rchunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunks-len"></span>`fn len(&self) -> usize`

- <span id="rchunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunks<'data, T>`

##### `impl<T> IntoParallelIterator for RChunks<'data, T>`

- <span id="rchunks-iter"></span>`type Iter = T`

- <span id="rchunks-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for RChunks<'data, T>`

- <span id="rchunks-item"></span>`type Item = &'data [T]`

- <span id="rchunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunks<'data, T>`

- <span id="rchunks-align"></span>`const ALIGN: usize`

- <span id="rchunks-init"></span>`type Init = T`

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

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksexact-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- <span id="rchunksexact-remainder"></span>`fn remainder(&self) -> &'data [T]`

#### Trait Implementations

##### `impl<T> Clone for RChunksExact<'_, T>`

- <span id="rchunksexact-clone"></span>`fn clone(&self) -> Self`

##### `impl<'data, T: fmt::Debug> Debug for RChunksExact<'data, T>`

- <span id="rchunksexact-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for RChunksExact<'_, T>`

- <span id="rchunksexact-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunksexact-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexact-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-iter"></span>`type Iter = T`

- <span id="rchunksexact-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexact-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-item"></span>`type Item = &'data [T]`

- <span id="rchunksexact-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunksexact-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExact<'data, T>`

- <span id="rchunksexact-align"></span>`const ALIGN: usize`

- <span id="rchunksexact-init"></span>`type Init = T`

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

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksexactmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- <span id="rchunksexactmut-into-remainder"></span>`fn into_remainder(self) -> &'data mut [T]`

- <span id="rchunksexactmut-remainder"></span>`fn remainder(&mut self) -> &mut [T]`

- <span id="rchunksexactmut-take-remainder"></span>`fn take_remainder(&mut self) -> &'data mut [T]`

#### Trait Implementations

##### `impl<'data, T: fmt::Debug + Send> Debug for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'data, T: Send + 'data> IndexedParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunksexactmut-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexactmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-iter"></span>`type Iter = T`

- <span id="rchunksexactmut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexactmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Send + 'data> ParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksexactmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunksexactmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-align"></span>`const ALIGN: usize`

- <span id="rchunksexactmut-init"></span>`type Init = T`

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

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<'data, T: fmt::Debug> Debug for RChunksMut<'data, T>`

- <span id="rchunksmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for RChunksMut<'_, T>`

- <span id="rchunksmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunksmut-len"></span>`fn len(&self) -> usize`

- <span id="rchunksmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-iter"></span>`type Iter = T`

- <span id="rchunksmut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Send> ParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="rchunksmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksMut<'data, T>`

- <span id="rchunksmut-align"></span>`const ALIGN: usize`

- <span id="rchunksmut-init"></span>`type Init = T`

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

Parallel iterator over immutable items in a slice

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<'data, T: fmt::Debug> Debug for Iter<'data, T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Iter<'_, T>`

- <span id="iter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="iter-len"></span>`fn len(&self) -> usize`

- <span id="iter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Iter<'data, T>`

##### `impl<T> IntoParallelIterator for Iter<'data, T>`

- <span id="iter-iter"></span>`type Iter = T`

- <span id="iter-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Iter<'data, T>`

- <span id="iter-item"></span>`type Item = &'data T`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'data, T>`

- <span id="iter-align"></span>`const ALIGN: usize`

- <span id="iter-init"></span>`type Init = T`

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

#### Trait Implementations

##### `impl<T> IntoEither for IterProducer<'data, T>`

##### `impl<T> Pointable for IterProducer<'data, T>`

- <span id="iterproducer-align"></span>`const ALIGN: usize`

- <span id="iterproducer-init"></span>`type Init = T`

- <span id="iterproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for IterProducer<'data, T>`

- <span id="iterproducer-item"></span>`type Item = &'data T`

- <span id="iterproducer-intoiter"></span>`type IntoIter = Iter<'data, T>`

- <span id="iterproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md)

- <span id="iterproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `Windows<'data, T>`

```rust
struct Windows<'data, T> {
    window_size: usize,
    slice: &'data [T],
}
```

Parallel iterator over immutable overlapping windows of a slice

#### Trait Implementations

##### `impl<T> Clone for Windows<'_, T>`

- <span id="windows-clone"></span>`fn clone(&self) -> Self`

##### `impl<'data, T: fmt::Debug> Debug for Windows<'data, T>`

- <span id="windows-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Windows<'_, T>`

- <span id="windows-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="windows-len"></span>`fn len(&self) -> usize`

- <span id="windows-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Windows<'data, T>`

##### `impl<T> IntoParallelIterator for Windows<'data, T>`

- <span id="windows-iter"></span>`type Iter = T`

- <span id="windows-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="windows-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Windows<'data, T>`

- <span id="windows-item"></span>`type Item = &'data [T]`

- <span id="windows-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="windows-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Windows<'data, T>`

- <span id="windows-align"></span>`const ALIGN: usize`

- <span id="windows-init"></span>`type Init = T`

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

#### Trait Implementations

##### `impl<T> IntoEither for WindowsProducer<'data, T>`

##### `impl<T> Pointable for WindowsProducer<'data, T>`

- <span id="windowsproducer-align"></span>`const ALIGN: usize`

- <span id="windowsproducer-init"></span>`type Init = T`

- <span id="windowsproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="windowsproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="windowsproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="windowsproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for WindowsProducer<'data, T>`

- <span id="windowsproducer-item"></span>`type Item = &'data [T]`

- <span id="windowsproducer-intoiter"></span>`type IntoIter = Windows<'data, T>`

- <span id="windowsproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md)

- <span id="windowsproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `IterMut<'data, T>`

```rust
struct IterMut<'data, T> {
    slice: &'data mut [T],
}
```

Parallel iterator over mutable items in a slice

#### Trait Implementations

##### `impl<'data, T: fmt::Debug> Debug for IterMut<'data, T>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IterMut<'_, T>`

- <span id="itermut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="itermut-len"></span>`fn len(&self) -> usize`

- <span id="itermut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for IterMut<'data, T>`

##### `impl<T> IntoParallelIterator for IterMut<'data, T>`

- <span id="itermut-iter"></span>`type Iter = T`

- <span id="itermut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Send> ParallelIterator for IterMut<'data, T>`

- <span id="itermut-item"></span>`type Item = &'data mut T`

- <span id="itermut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="itermut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'data, T>`

- <span id="itermut-align"></span>`const ALIGN: usize`

- <span id="itermut-init"></span>`type Init = T`

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

#### Trait Implementations

##### `impl<T> IntoEither for IterMutProducer<'data, T>`

##### `impl<T> Pointable for IterMutProducer<'data, T>`

- <span id="itermutproducer-align"></span>`const ALIGN: usize`

- <span id="itermutproducer-init"></span>`type Init = T`

- <span id="itermutproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermutproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermutproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermutproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Send> Producer for IterMutProducer<'data, T>`

- <span id="itermutproducer-item"></span>`type Item = &'data mut T`

- <span id="itermutproducer-intoiter"></span>`type IntoIter = IterMut<'data, T>`

- <span id="itermutproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md)

- <span id="itermutproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `Split<'data, T, P>`

```rust
struct Split<'data, T, P> {
    slice: &'data [T],
    separator: P,
}
```

Parallel iterator over slices separated by a predicate

#### Trait Implementations

##### `impl<T, P: Clone> Clone for Split<'_, T, P>`

- <span id="split-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Debug, P> Debug for Split<'_, T, P>`

- <span id="split-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Split<'data, T, P>`

##### `impl<T> IntoParallelIterator for Split<'data, T, P>`

- <span id="split-iter"></span>`type Iter = T`

- <span id="split-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="split-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for Split<'data, T, P>`

- <span id="split-item"></span>`type Item = &'data [T]`

- <span id="split-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Split<'data, T, P>`

- <span id="split-align"></span>`const ALIGN: usize`

- <span id="split-init"></span>`type Init = T`

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

Parallel iterator over slices separated by a predicate,
including the matched part as a terminator.

#### Trait Implementations

##### `impl<T, P: Clone> Clone for SplitInclusive<'_, T, P>`

- <span id="splitinclusive-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: Debug, P> Debug for SplitInclusive<'_, T, P>`

- <span id="splitinclusive-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitInclusive<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-iter"></span>`type Iter = T`

- <span id="splitinclusive-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusive-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-item"></span>`type Item = &'data [T]`

- <span id="splitinclusive-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitInclusive<'data, T, P>`

- <span id="splitinclusive-align"></span>`const ALIGN: usize`

- <span id="splitinclusive-init"></span>`type Init = T`

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

Parallel iterator over mutable slices separated by a predicate

#### Trait Implementations

##### `impl<T: Debug, P> Debug for SplitMut<'_, T, P>`

- <span id="splitmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitMut<'data, T, P>`

- <span id="splitmut-iter"></span>`type Iter = T`

- <span id="splitmut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for SplitMut<'data, T, P>`

- <span id="splitmut-item"></span>`type Item = &'data mut [T]`

- <span id="splitmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitMut<'data, T, P>`

- <span id="splitmut-align"></span>`const ALIGN: usize`

- <span id="splitmut-init"></span>`type Init = T`

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

Parallel iterator over mutable slices separated by a predicate,
including the matched part as a terminator.

#### Trait Implementations

##### `impl<T: Debug, P> Debug for SplitInclusiveMut<'_, T, P>`

- <span id="splitinclusivemut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitInclusiveMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-iter"></span>`type Iter = T`

- <span id="splitinclusivemut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="splitinclusivemut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-item"></span>`type Item = &'data mut [T]`

- <span id="splitinclusivemut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitInclusiveMut<'data, T, P>`

- <span id="splitinclusivemut-align"></span>`const ALIGN: usize`

- <span id="splitinclusivemut-init"></span>`type Init = T`

- <span id="splitinclusivemut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="splitinclusivemut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="splitinclusivemut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="splitinclusivemut-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `ParallelSlice<T: Sync>`

```rust
trait ParallelSlice<T: Sync> { ... }
```

Parallel extensions for slices.

#### Required Methods

- `fn as_parallel_slice(&self) -> &[T]`

  Returns a plain slice, which is used to implement the rest of the

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

### `ParallelSliceMut<T: Send>`

```rust
trait ParallelSliceMut<T: Send> { ... }
```

Parallel extensions for mutable slices.

#### Required Methods

- `fn as_parallel_slice_mut(&mut self) -> &mut [T]`

  Returns a plain mutable slice, which is used to implement the rest of

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

