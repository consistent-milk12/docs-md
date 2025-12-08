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

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- <span id="chunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for Chunks<'_, T>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<'data, T: fmt::Debug> Debug for Chunks<'data, T>`

- <span id="chunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Chunks<'_, T>`

- <span id="chunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunks-len"></span>`fn len(&self) -> usize`

- <span id="chunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for Chunks<'data, T>`

##### `impl<T> IntoParallelIterator for Chunks<'data, T>`

- <span id="chunks-iter"></span>`type Iter = T`

- <span id="chunks-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Chunks<'data, T>`

- <span id="chunks-item"></span>`type Item = &'data [T]`

- <span id="chunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<'data, T>`

- <span id="chunks-align"></span>`const ALIGN: usize`

- <span id="chunks-init"></span>`type Init = T`

- <span id="chunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunks-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunksProducer<'data, T: Sync>`

```rust
struct ChunksProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for ChunksProducer<'data, T>`

##### `impl<T> Pointable for ChunksProducer<'data, T>`

- <span id="chunksproducer-align"></span>`const ALIGN: usize`

- <span id="chunksproducer-init"></span>`type Init = T`

- <span id="chunksproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for ChunksProducer<'data, T>`

- <span id="chunksproducer-item"></span>`type Item = &'data [T]`

- <span id="chunksproducer-intoiter"></span>`type IntoIter = Chunks<'data, T>`

- <span id="chunksproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- <span id="chunksproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

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

- <span id="chunksexact-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunksexact-len"></span>`fn len(&self) -> usize`

- <span id="chunksexact-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-iter"></span>`type Iter = T`

- <span id="chunksexact-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexact-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for ChunksExact<'data, T>`

- <span id="chunksexact-item"></span>`type Item = &'data [T]`

- <span id="chunksexact-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunksexact-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExact<'data, T>`

- <span id="chunksexact-align"></span>`const ALIGN: usize`

- <span id="chunksexact-init"></span>`type Init = T`

- <span id="chunksexact-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexact-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexact-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexact-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunksExactProducer<'data, T: Sync>`

```rust
struct ChunksExactProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for ChunksExactProducer<'data, T>`

##### `impl<T> Pointable for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-align"></span>`const ALIGN: usize`

- <span id="chunksexactproducer-init"></span>`type Init = T`

- <span id="chunksexactproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexactproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexactproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexactproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for ChunksExactProducer<'data, T>`

- <span id="chunksexactproducer-item"></span>`type Item = &'data [T]`

- <span id="chunksexactproducer-intoiter"></span>`type IntoIter = ChunksExact<'data, T>`

- <span id="chunksexactproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- <span id="chunksexactproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

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

- <span id="chunksmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunksmut-len"></span>`fn len(&self) -> usize`

- <span id="chunksmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-iter"></span>`type Iter = T`

- <span id="chunksmut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksMut<'data, T>`

- <span id="chunksmut-item"></span>`type Item = &'data mut [T]`

- <span id="chunksmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunksmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksMut<'data, T>`

- <span id="chunksmut-align"></span>`const ALIGN: usize`

- <span id="chunksmut-init"></span>`type Init = T`

- <span id="chunksmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunksMutProducer<'data, T: Send>`

```rust
struct ChunksMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for ChunksMutProducer<'data, T>`

##### `impl<T> Pointable for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-align"></span>`const ALIGN: usize`

- <span id="chunksmutproducer-init"></span>`type Init = T`

- <span id="chunksmutproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksmutproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksmutproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksmutproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Send> Producer for ChunksMutProducer<'data, T>`

- <span id="chunksmutproducer-item"></span>`type Item = &'data mut [T]`

- <span id="chunksmutproducer-intoiter"></span>`type IntoIter = ChunksMut<'data, T>`

- <span id="chunksmutproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- <span id="chunksmutproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

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

- <span id="chunksexactmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunksexactmut-len"></span>`fn len(&self) -> usize`

- <span id="chunksexactmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-iter"></span>`type Iter = T`

- <span id="chunksexactmut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunksexactmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-item"></span>`type Item = &'data mut [T]`

- <span id="chunksexactmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="chunksexactmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExactMut<'data, T>`

- <span id="chunksexactmut-align"></span>`const ALIGN: usize`

- <span id="chunksexactmut-init"></span>`type Init = T`

- <span id="chunksexactmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexactmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexactmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexactmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunksExactMutProducer<'data, T: Send>`

```rust
struct ChunksExactMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for ChunksExactMutProducer<'data, T>`

##### `impl<T> Pointable for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-align"></span>`const ALIGN: usize`

- <span id="chunksexactmutproducer-init"></span>`type Init = T`

- <span id="chunksexactmutproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunksexactmutproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunksexactmutproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunksexactmutproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Send> Producer for ChunksExactMutProducer<'data, T>`

- <span id="chunksexactmutproducer-item"></span>`type Item = &'data mut [T]`

- <span id="chunksexactmutproducer-intoiter"></span>`type IntoIter = ChunksExactMut<'data, T>`

- <span id="chunksexactmutproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- <span id="chunksexactmutproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

