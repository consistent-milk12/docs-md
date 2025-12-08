*[rayon](../../index.md) / [slice](../index.md) / [rchunks](index.md)*

---

# Module `rchunks`

## Structs

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

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunks<'data, T>`

##### `impl<T> IntoParallelIterator for RChunks<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for RChunks<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunks<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RChunksProducer<'data, T: Sync>`

```rust
struct RChunksProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for RChunksProducer<'data, T>`

##### `impl<T> Pointable for RChunksProducer<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for RChunksProducer<'data, T>`

- `type Item = &'data [T]`

- `type IntoIter = RChunks<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

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

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExact<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for RChunksExact<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExact<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RChunksExactProducer<'data, T: Sync>`

```rust
struct RChunksExactProducer<'data, T: Sync> {
    chunk_size: usize,
    slice: &'data [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for RChunksExactProducer<'data, T>`

##### `impl<T> Pointable for RChunksExactProducer<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for RChunksExactProducer<'data, T>`

- `type Item = &'data [T]`

- `type IntoIter = RChunksExact<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

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

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for RChunksMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunksMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RChunksMutProducer<'data, T: Send>`

```rust
struct RChunksMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for RChunksMutProducer<'data, T>`

##### `impl<T> Pointable for RChunksMutProducer<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Send> Producer for RChunksMutProducer<'data, T>`

- `type Item = &'data mut [T]`

- `type IntoIter = RChunksMut<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

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

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for RChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for RChunksExactMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send + 'data> ParallelIterator for RChunksExactMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RChunksExactMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RChunksExactMutProducer<'data, T: Send>`

```rust
struct RChunksExactMutProducer<'data, T: Send> {
    chunk_size: usize,
    slice: &'data mut [T],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for RChunksExactMutProducer<'data, T>`

##### `impl<T> Pointable for RChunksExactMutProducer<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Send> Producer for RChunksExactMutProducer<'data, T>`

- `type Item = &'data mut [T]`

- `type IntoIter = RChunksExactMut<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

