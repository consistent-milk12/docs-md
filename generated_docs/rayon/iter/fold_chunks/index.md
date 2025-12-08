*[rayon](../../index.md) / [iter](../index.md) / [fold_chunks](index.md)*

---

# Module `fold_chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FoldChunks`](#foldchunks) | struct | `FoldChunks` is an iterator that groups elements of an underlying iterator and applies a |

## Structs

### `FoldChunks<I, ID, F>`

```rust
struct FoldChunks<I, ID, F> {
    base: I,
    chunk_size: usize,
    fold_op: F,
    identity: ID,
}
```

`FoldChunks` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="foldchunks-new"></span>`fn new(base: I, chunk_size: usize, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for FoldChunks<I, ID, F>`

- <span id="foldchunks-clone"></span>`fn clone(&self) -> FoldChunks<I, ID, F>` — [`FoldChunks`](../index.md)

##### `impl<I: Debug, ID, F> Debug for FoldChunks<I, ID, F>`

- <span id="foldchunks-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, ID, U, F> IndexedParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-len"></span>`fn len(&self) -> usize`

- <span id="foldchunks-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="foldchunks-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for FoldChunks<I, ID, F>`

##### `impl<T> IntoParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-iter"></span>`type Iter = T`

- <span id="foldchunks-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunks-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, ID, U, F> ParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-item"></span>`type Item = U`

- <span id="foldchunks-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="foldchunks-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for FoldChunks<I, ID, F>`

- <span id="foldchunks-align"></span>`const ALIGN: usize`

- <span id="foldchunks-init"></span>`type Init = T`

- <span id="foldchunks-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunks-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunks-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunks-drop"></span>`unsafe fn drop(ptr: usize)`

