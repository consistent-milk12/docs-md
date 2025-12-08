*[rayon](../../index.md) / [iter](../index.md) / [chunks](index.md)*

---

# Module `chunks`

## Structs

### `Chunks<I>`

```rust
struct Chunks<I> {
    size: usize,
    i: I,
}
```

`Chunks` is an iterator that groups elements of an underlying iterator.

This struct is created by the `chunks()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- `fn new(i: I, size: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Chunks<I>`

- `fn clone(self: &Self) -> Chunks<I>` — [`Chunks`](#chunks)

##### `impl<I: $crate::fmt::Debug> Debug for Chunks<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Chunks<I>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Chunks<I>`

##### `impl<T> IntoParallelIterator for Chunks<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Chunks<I>`

- `type Item = Vec<<I as ParallelIterator>::Item>`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ChunkProducer<P, F>`

```rust
struct ChunkProducer<P, F> {
    chunk_size: usize,
    len: usize,
    base: P,
    map: F,
}
```

#### Implementations

- `fn new(chunk_size: usize, len: usize, base: P, map: F) -> Self`

#### Trait Implementations

##### `impl<T> IntoEither for ChunkProducer<P, F>`

##### `impl<T> Pointable for ChunkProducer<P, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P, F, T> Producer for ChunkProducer<P, F>`

- `type Item = T`

- `type IntoIter = Map<ChunkSeq<P>, F>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

### `ChunkSeq<P>`

```rust
struct ChunkSeq<P> {
    chunk_size: usize,
    len: usize,
    inner: Option<P>,
}
```

#### Trait Implementations

##### `impl<P> DoubleEndedIterator for ChunkSeq<P>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<P> ExactSizeIterator for ChunkSeq<P>`

- `fn len(self: &Self) -> usize`

##### `impl<T> IntoEither for ChunkSeq<P>`

##### `impl<I> IntoIterator for ChunkSeq<P>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<P> Iterator for ChunkSeq<P>`

- `type Item = <P as Producer>::IntoIter`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for ChunkSeq<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

