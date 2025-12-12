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

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:6-9`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L6-L9)*

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunks-new"></span>`fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for RChunks<'_, T>`

- <span id="rchunks-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for RChunks<'data, T>`

- <span id="rchunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for RChunks<'_, T>`

- <span id="rchunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunks-len"></span>`fn len(&self) -> usize`

- <span id="rchunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunks<'data, T>`

##### `impl<T> IntoParallelIterator for RChunks<'data, T>`

- <span id="rchunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for RChunks<'data, T>`

- <span id="rchunks-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="rchunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunks<'data, T>`

- <span id="rchunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunks-pointable-type-init"></span>`type Init = T`

- <span id="rchunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunks-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunksProducer<'data, T: Sync>`

```rust
struct RChunksProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:61-64`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L61-L64)*

#### Trait Implementations

##### `impl<T> IntoEither for RChunksProducer<'data, T>`

##### `impl<T> Pointable for RChunksProducer<'data, T>`

- <span id="rchunksproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for RChunksProducer<'data, T>`

- <span id="rchunksproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="rchunksproducer-producer-type-intoiter"></span>`type IntoIter = RChunks<'data, T>`

- <span id="rchunksproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `RChunksExact<'data, T>`

```rust
struct RChunksExact<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
    rem: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:92-96`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L92-L96)*

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

- <span id="rchunksexact-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexact-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexact-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksexact-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexact-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for RChunksExact<'data, T>`

- <span id="rchunksexact-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="rchunksexact-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexact-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExact<'data, T>`

- <span id="rchunksexact-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexact-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexact-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexact-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexact-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexact-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunksExactProducer<'data, T: Sync>`

```rust
struct RChunksExactProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:161-164`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L161-L164)*

#### Trait Implementations

##### `impl<T> IntoEither for RChunksExactProducer<'data, T>`

##### `impl<T> Pointable for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexactproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexactproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexactproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexactproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexactproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Sync> Producer for RChunksExactProducer<'data, T>`

- <span id="rchunksexactproducer-producer-type-item"></span>`type Item = &'data [T]`

- <span id="rchunksexactproducer-producer-type-intoiter"></span>`type IntoIter = RChunksExact<'data, T>`

- <span id="rchunksexactproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksexactproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `RChunksMut<'data, T>`

```rust
struct RChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:192-195`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L192-L195)*

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- <span id="rchunksmut-new"></span>`fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for RChunksMut<'data, T>`

- <span id="rchunksmut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for RChunksMut<'_, T>`

- <span id="rchunksmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksmut-len"></span>`fn len(&self) -> usize`

- <span id="rchunksmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for RChunksMut<'data, T>`

- <span id="rchunksmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksMut<'data, T>`

- <span id="rchunksmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksmut-pointable-type-init"></span>`type Init = T`

- <span id="rchunksmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunksMutProducer<'data, T: Send>`

```rust
struct RChunksMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:241-244`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L241-L244)*

#### Trait Implementations

##### `impl<T> IntoEither for RChunksMutProducer<'data, T>`

##### `impl<T> Pointable for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksmutproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksmutproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksmutproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksmutproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksmutproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for RChunksMutProducer<'data, T>`

- <span id="rchunksmutproducer-producer-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksmutproducer-producer-type-intoiter"></span>`type IntoIter = RChunksMut<'data, T>`

- <span id="rchunksmutproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksmutproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `RChunksExactMut<'data, T: Send>`

```rust
struct RChunksExactMut<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
    rem: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:272-276`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L272-L276)*

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

- <span id="rchunksexactmut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexactmut-len"></span>`fn len(&self) -> usize`

- <span id="rchunksexactmut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for RChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rchunksexactmut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rchunksexactmut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send + 'data> ParallelIterator for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksexactmut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="rchunksexactmut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExactMut<'data, T>`

- <span id="rchunksexactmut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexactmut-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexactmut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexactmut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexactmut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexactmut-drop"></span>`unsafe fn drop(ptr: usize)`

### `RChunksExactMutProducer<'data, T: Send>`

```rust
struct RChunksExactMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/rchunks.rs:358-361`](../../../../.source_1765521767/rayon-1.11.0/src/slice/rchunks.rs#L358-L361)*

#### Trait Implementations

##### `impl<T> IntoEither for RChunksExactMutProducer<'data, T>`

##### `impl<T> Pointable for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rchunksexactmutproducer-pointable-type-init"></span>`type Init = T`

- <span id="rchunksexactmutproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rchunksexactmutproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rchunksexactmutproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rchunksexactmutproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for RChunksExactMutProducer<'data, T>`

- <span id="rchunksexactmutproducer-producer-type-item"></span>`type Item = &'data mut [T]`

- <span id="rchunksexactmutproducer-producer-type-intoiter"></span>`type IntoIter = RChunksExactMut<'data, T>`

- <span id="rchunksexactmutproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md#producer)

- <span id="rchunksexactmutproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

