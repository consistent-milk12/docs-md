*[rayon](../../index.md) / [iter](../index.md) / [fold_chunks](index.md)*

---

# Module `fold_chunks`

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

This struct is created by the `fold_chunks()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, chunk_size: usize, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, ID: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FoldChunks<I, ID, F>`

- `fn clone(self: &Self) -> FoldChunks<I, ID, F>` — [`FoldChunks`](#foldchunks)

##### `impl<I: Debug, ID, F> Debug for FoldChunks<I, ID, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, ID, U, F> IndexedParallelIterator for FoldChunks<I, ID, F>`

- `fn len(self: &Self) -> usize`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for FoldChunks<I, ID, F>`

##### `impl<T> IntoParallelIterator for FoldChunks<I, ID, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, ID, U, F> ParallelIterator for FoldChunks<I, ID, F>`

- `type Item = U`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for FoldChunks<I, ID, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

