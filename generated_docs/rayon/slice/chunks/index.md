*[rayon](../../index.md) / [slice](../index.md) / [chunks](index.md)*

---

# Module `chunks`

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

- `fn new(chunk_size: usize, slice: &'data [T]) -> Self`

#### Trait Implementations

##### `impl<T> Clone for Chunks<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'data, T: $crate::fmt::Debug> Debug for Chunks<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Chunks<'_, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for Chunks<'data, T>`

##### `impl<T> IntoParallelIterator for Chunks<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for Chunks<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for ChunksProducer<'data, T>`

- `type Item = &'data [T]`

- `type IntoIter = Chunks<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

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

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExact<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExact<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Sync> ParallelIterator for ChunksExact<'data, T>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExact<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Sync> Producer for ChunksExactProducer<'data, T>`

- `type Item = &'data [T]`

- `type IntoIter = ChunksExact<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

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

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ChunksMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Send> Producer for ChunksMutProducer<'data, T>`

- `type Item = &'data mut [T]`

- `type IntoIter = ChunksMut<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

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

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl<T> IntoEither for ChunksExactMut<'data, T>`

##### `impl<T> IntoParallelIterator for ChunksExactMut<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for ChunksExactMut<'data, T>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ChunksExactMut<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'data, T: 'data + Send> Producer for ChunksExactMutProducer<'data, T>`

- `type Item = &'data mut [T]`

- `type IntoIter = ChunksExactMut<'data, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

