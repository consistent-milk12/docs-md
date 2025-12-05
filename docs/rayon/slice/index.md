*[rayon](../index.md) / [slice](index.md)*

---

# Module `slice`

Parallel iterator types for [slices](#slices)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[slices](#slices): std::slice

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

##### `impl Clone<T, P: Clone>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<T: fmt::Debug, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T, P>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Debug<T: fmt::Debug, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T, P>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Sync>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Sync>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Send>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Send>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Sync>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Sync>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Debug<'data, T: $crate::fmt::Debug + Send>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<'data, T: Send + 'data>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Send + 'data>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Send>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Sync>`

- `type Item = &'data T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Sync>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Debug<'data, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T: Send>`

- `type Item = &'data mut T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

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

##### `impl Clone<T, P: Clone>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<T: Debug, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T, P>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Clone<T, P: Clone>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<T: Debug, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T, P>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Debug<T: Debug, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T, P>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

##### `impl Debug<T: Debug, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'data, T, P>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl Pointable<T>`

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

