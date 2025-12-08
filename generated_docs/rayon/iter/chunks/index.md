*[rayon](../../index.md) / [iter](../index.md) / [chunks](index.md)*

---

# Module `chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Chunks`](#chunks) | struct | `Chunks` is an iterator that groups elements of an underlying iterator. |
| [`ChunkProducer`](#chunkproducer) | struct |  |
| [`ChunkSeq`](#chunkseq) | struct |  |

## Structs

### `Chunks<I>`

```rust
struct Chunks<I> {
    size: usize,
    i: I,
}
```

`Chunks` is an iterator that groups elements of an underlying iterator.

This struct is created by the `chunks()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="chunks-new"></span>`fn new(i: I, size: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Chunks<I>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Chunks<I>` — [`Chunks`](../index.md)

##### `impl<I: fmt::Debug> Debug for Chunks<I>`

- <span id="chunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Chunks<I>`

- <span id="chunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="chunks-len"></span>`fn len(&self) -> usize`

- <span id="chunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Chunks<I>`

##### `impl<T> IntoParallelIterator for Chunks<I>`

- <span id="chunks-iter"></span>`type Iter = T`

- <span id="chunks-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Chunks<I>`

- <span id="chunks-item"></span>`type Item = Vec<<I as ParallelIterator>::Item>`

- <span id="chunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="chunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chunks<I>`

- <span id="chunks-align"></span>`const ALIGN: usize`

- <span id="chunks-init"></span>`type Init = T`

- <span id="chunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunks-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="chunkproducer-new"></span>`fn new(chunk_size: usize, len: usize, base: P, map: F) -> Self`

#### Trait Implementations

##### `impl<T> IntoEither for ChunkProducer<P, F>`

##### `impl<T> Pointable for ChunkProducer<P, F>`

- <span id="chunkproducer-align"></span>`const ALIGN: usize`

- <span id="chunkproducer-init"></span>`type Init = T`

- <span id="chunkproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, F, T> Producer for ChunkProducer<P, F>`

- <span id="chunkproducer-item"></span>`type Item = T`

- <span id="chunkproducer-intoiter"></span>`type IntoIter = Map<ChunkSeq<P>, F>`

- <span id="chunkproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="chunkproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="chunkproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="chunkproducer-max-len"></span>`fn max_len(&self) -> usize`

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

- <span id="chunkseq-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<P> ExactSizeIterator for ChunkSeq<P>`

- <span id="chunkseq-len"></span>`fn len(&self) -> usize`

##### `impl<T> IntoEither for ChunkSeq<P>`

##### `impl<I> IntoIterator for ChunkSeq<P>`

- <span id="chunkseq-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunkseq-intoiter"></span>`type IntoIter = I`

- <span id="chunkseq-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<P> Iterator for ChunkSeq<P>`

- <span id="chunkseq-item"></span>`type Item = <P as Producer>::IntoIter`

- <span id="chunkseq-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="chunkseq-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for ChunkSeq<P>`

- <span id="chunkseq-align"></span>`const ALIGN: usize`

- <span id="chunkseq-init"></span>`type Init = T`

- <span id="chunkseq-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkseq-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkseq-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkseq-drop"></span>`unsafe fn drop(ptr: usize)`

