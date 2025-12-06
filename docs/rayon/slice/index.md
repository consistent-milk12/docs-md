*[rayon](../index.md) / [slice](index.md)*

---

# Module `slice`

Parallel iterator types for [slices](#slices)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Structs

### `ChunkBy<'data, T, P>`

```rust
struct ChunkBy<'data, T, P> {
    pred: P,
    slice: &'data [T],
}
```

Parallel iterator over slice in (non-overlapping) chunks separated by a predicate.

This struct is created by the [`par_chunk_by`](#par-chunk-by) method on `&[T]`.


#### Implementations

- `fn new(slice: &'data [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T, P: Clone> Clone for ChunkBy<'_, T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: fmt::Debug, P> Debug for ChunkBy<'_, T, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkBy<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkBy<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkBy<'data, T, P>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkBy<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ChunkByMut<'data, T, P>`

```rust
struct ChunkByMut<'data, T, P> {
    pred: P,
    slice: &'data mut [T],
}
```

Parallel iterator over slice in (non-overlapping) mutable chunks
separated by a predicate.

This struct is created by the [`par_chunk_by_mut`](#par-chunk-by-mut) method on `&mut [T]`.


#### Implementations

- `fn new(slice: &'data mut [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug, P> Debug for ChunkByMut<'_, T, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkByMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkByMut<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkByMut<'data, T, P>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkByMut<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Chunks<'data, T>`

```rust
struct Chunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

Parallel iterator over immutable non-overlapping chunks of a slice

#### Implementations

- `fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for Chunks<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'data, T: $crate::fmt::Debug> Debug for Chunks<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Chunks<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Chunks<'data, T>`

##### `impl<T> IntoParallelIterator for Chunks<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Chunks<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- `fn remainder(self: &Self) -> &'data [T]`

#### Trait Implementations

##### `impl<T> Clone for ChunksExact<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'data, T: $crate::fmt::Debug> Debug for ChunksExact<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for ChunksExact<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExact<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for ChunksExact<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExact<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- `fn into_remainder(self: Self) -> &'data mut [T]`

- `fn remainder(self: &mut Self) -> &mut [T]`

- `fn take_remainder(self: &mut Self) -> &'data mut [T]`

#### Trait Implementations

##### `impl<'data, T: $crate::fmt::Debug> Debug for ChunksExactMut<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for ChunksExactMut<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExactMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksExactMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExactMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ChunksMut<'data, T>`

```rust
struct ChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

Parallel iterator over mutable non-overlapping chunks of a slice

#### Implementations

- `fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<'data, T: $crate::fmt::Debug> Debug for ChunksMut<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for ChunksMut<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ChunksMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RChunks<'data, T>`

```rust
struct RChunks<'data, T> {
    chunk_size: usize,
    slice: &'data [T],
}
```

Parallel iterator over immutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- `fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for RChunks<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'data, T: $crate::fmt::Debug> Debug for RChunks<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for RChunks<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunks<'data, T>`

##### `impl<T> IntoParallelIterator for RChunks<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for RChunks<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunks<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(chunk_size: usize, slice: &'data [T]) -> Self`

- `fn remainder(self: &Self) -> &'data [T]`

#### Trait Implementations

##### `impl<T> Clone for RChunksExact<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'data, T: $crate::fmt::Debug> Debug for RChunksExact<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for RChunksExact<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExact<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for RChunksExact<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExact<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

- `fn into_remainder(self: Self) -> &'data mut [T]`

- `fn remainder(self: &mut Self) -> &mut [T]`

- `fn take_remainder(self: &mut Self) -> &'data mut [T]`

#### Trait Implementations

##### `impl<'data, T: $crate::fmt::Debug + Send> Debug for RChunksExactMut<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, T: Send + 'data> IndexedParallelIterator for RChunksExactMut<'data, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExactMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send + 'data> ParallelIterator for RChunksExactMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExactMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RChunksMut<'data, T>`

```rust
struct RChunksMut<'data, T> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

Parallel iterator over mutable non-overlapping chunks of a slice, starting at the end.

#### Implementations

- `fn new(chunk_size: usize, slice: &'data mut [T]) -> Self`

#### Trait Implementations

##### `impl<'data, T: $crate::fmt::Debug> Debug for RChunksMut<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for RChunksMut<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for RChunksMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunksMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Iter<'data, T>`

```rust
struct Iter<'data, T> {
    slice: &'data [T],
}
```

Parallel iterator over immutable items in a slice

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'data, T: $crate::fmt::Debug> Debug for Iter<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Iter<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Iter<'data, T>`

##### `impl<T> IntoParallelIterator for Iter<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Iter<'data, T>`

- `type Item = &'data T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn clone(self: &Self) -> Self`

##### `impl<'data, T: $crate::fmt::Debug> Debug for Windows<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Windows<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Windows<'data, T>`

##### `impl<T> IntoParallelIterator for Windows<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Windows<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Windows<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `IterMut<'data, T>`

```rust
struct IterMut<'data, T> {
    slice: &'data mut [T],
}
```

Parallel iterator over mutable items in a slice

#### Trait Implementations

##### `impl<'data, T: $crate::fmt::Debug> Debug for IterMut<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IterMut<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for IterMut<'data, T>`

##### `impl<T> IntoParallelIterator for IterMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for IterMut<'data, T>`

- `type Item = &'data mut T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn clone(self: &Self) -> Self`

##### `impl<T: Debug, P> Debug for Split<'_, T, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Split<'data, T, P>`

##### `impl<T> IntoParallelIterator for Split<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for Split<'data, T, P>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Split<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn clone(self: &Self) -> Self`

##### `impl<T: Debug, P> Debug for SplitInclusive<'_, T, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitInclusive<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusive<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for SplitInclusive<'data, T, P>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitInclusive<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitMut<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for SplitMut<'data, T, P>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitMut<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for SplitInclusiveMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for SplitInclusiveMut<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for SplitInclusiveMut<'data, T, P>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for SplitInclusiveMut<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `ParallelSlice<T: Sync>`

```rust
trait ParallelSlice<T: Sync> { ... }
```

Parallel extensions for slices.

#### Required Methods

- `fn as_parallel_slice(self: &Self) -> &[T]`

  Returns a plain slice, which is used to implement the rest of the

- `fn par_split<P>(self: &Self, separator: P) -> Split<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that

- `fn par_split_inclusive<P>(self: &Self, separator: P) -> SplitInclusive<'_, T, P>`

  Returns a parallel iterator over subslices separated by elements that

- `fn par_windows(self: &Self, window_size: usize) -> Windows<'_, T>`

  Returns a parallel iterator over all contiguous windows of length

- `fn par_chunks(self: &Self, chunk_size: usize) -> Chunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of

- `fn par_chunks_exact(self: &Self, chunk_size: usize) -> ChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of

- `fn par_rchunks(self: &Self, chunk_size: usize) -> RChunks<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- `fn par_rchunks_exact(self: &Self, chunk_size: usize) -> RChunksExact<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- `fn par_chunk_by<F>(self: &Self, pred: F) -> ChunkBy<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping runs

### `ParallelSliceMut<T: Send>`

```rust
trait ParallelSliceMut<T: Send> { ... }
```

Parallel extensions for mutable slices.

#### Required Methods

- `fn as_parallel_slice_mut(self: &mut Self) -> &mut [T]`

  Returns a plain mutable slice, which is used to implement the rest of

- `fn par_split_mut<P>(self: &mut Self, separator: P) -> SplitMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by

- `fn par_split_inclusive_mut<P>(self: &mut Self, separator: P) -> SplitInclusiveMut<'_, T, P>`

  Returns a parallel iterator over mutable subslices separated by elements

- `fn par_chunks_mut(self: &mut Self, chunk_size: usize) -> ChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of

- `fn par_chunks_exact_mut(self: &mut Self, chunk_size: usize) -> ChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of

- `fn par_rchunks_mut(self: &mut Self, chunk_size: usize) -> RChunksMut<'_, T>`

  Returns a parallel iterator over at most `chunk_size` elements of `self` at a time,

- `fn par_rchunks_exact_mut(self: &mut Self, chunk_size: usize) -> RChunksExactMut<'_, T>`

  Returns a parallel iterator over `chunk_size` elements of `self` at a time,

- `fn par_sort(self: &mut Self)`

  Sorts the slice in parallel.

- `fn par_sort_by<F>(self: &mut Self, compare: F)`

  Sorts the slice in parallel with a comparator function.

- `fn par_sort_by_key<K, F>(self: &mut Self, f: F)`

  Sorts the slice in parallel with a key extraction function.

- `fn par_sort_by_cached_key<K, F>(self: &mut Self, f: F)`

  Sorts the slice in parallel with a key extraction function.

- `fn par_sort_unstable(self: &mut Self)`

  Sorts the slice in parallel, but might not preserve the order of equal elements.

- `fn par_sort_unstable_by<F>(self: &mut Self, compare: F)`

  Sorts the slice in parallel with a comparator function, but might not preserve the order of

- `fn par_sort_unstable_by_key<K, F>(self: &mut Self, f: F)`

  Sorts the slice in parallel with a key extraction function, but might not preserve the order

- `fn par_chunk_by_mut<F>(self: &mut Self, pred: F) -> ChunkByMut<'_, T, F>`

  Returns a parallel iterator over the slice producing non-overlapping mutable

