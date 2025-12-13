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

*Defined in [`rayon-1.11.0/src/iter/fold_chunks_with.rs:15-20`](../../../../.source_1765633015/rayon-1.11.0/src/iter/fold_chunks_with.rs#L15-L20)*

`FoldChunksWith` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks_with()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="foldchunkswith-new"></span>`fn new(base: I, chunk_size: usize, item: U, fold_op: F) -> Self`

  Creates a new `FoldChunksWith` iterator

#### Trait Implementations

##### `impl Any for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-clone"></span>`fn clone(&self) -> FoldChunksWith<I, U, F>` — [`FoldChunksWith`](#foldchunkswith)

##### `impl CloneToUninit for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, U: Debug, F> Debug for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, U, F> IndexedParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="foldchunkswith-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldchunkswith-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldChunksWith<I, U, F>`

##### `impl IntoParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldchunkswith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunkswith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, U, F> ParallelIterator for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldchunkswith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldchunkswith-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldchunkswith-pointable-type-init"></span>`type Init = T`

- <span id="foldchunkswith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunkswith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunkswith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunkswith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-toowned-type-owned"></span>`type Owned = T`

- <span id="foldchunkswith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foldchunkswith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldchunkswith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldChunksWith<I, U, F>`

- <span id="foldchunkswith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldchunkswith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

