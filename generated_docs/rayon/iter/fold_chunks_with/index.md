*[rayon](../../index.md) / [iter](../index.md) / [fold_chunks_with](index.md)*

---

# Module `fold_chunks_with`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FoldChunksWith`](#foldchunkswith) | struct | `FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a function over them, producing a single value for each group. |

## Structs

### `FoldChunksWith<I, U, F>`

```rust
struct FoldChunksWith<I, U, F> {
    base: I,
    chunk_size: usize,
    item: U,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold_chunks_with.rs:15-20`](../../../../.source_1765210505/rayon-1.11.0/src/iter/fold_chunks_with.rs#L15-L20)*

`FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks_with()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="foldchunkswith-new"></span>`fn new(base: I, chunk_size: usize, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-clone"></span>`fn clone(&self) -> FoldChunksWith<I, U, F>` — [`FoldChunksWith`](#foldchunkswith)

##### `impl<I: Debug, U: Debug, F> Debug for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, U, F> IndexedParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-len"></span>`fn len(&self) -> usize`

- <span id="foldchunkswith-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldchunkswith-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl IntoEither for FoldChunksWith<I, U, F>`

##### `impl IntoParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldchunkswith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunkswith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, U, F> ParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldchunkswith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldchunkswith-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldchunkswith-pointable-type-init"></span>`type Init = T`

- <span id="foldchunkswith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunkswith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunkswith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunkswith-drop"></span>`unsafe fn drop(ptr: usize)`

