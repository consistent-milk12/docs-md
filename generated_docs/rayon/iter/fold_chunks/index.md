*[rayon](../../index.md) / [iter](../index.md) / [fold_chunks](index.md)*

---

# Module `fold_chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FoldChunks`](#foldchunks) | struct | `FoldChunks` is an iterator that groups elements of an underlying iterator and applies a function over them, producing a single value for each group. |

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

*Defined in [`rayon-1.11.0/src/iter/fold_chunks.rs:15-20`](../../../../.source_1765633015/rayon-1.11.0/src/iter/fold_chunks.rs#L15-L20)*

`FoldChunks` is an iterator that groups elements of an underlying iterator and applies a
function over them, producing a single value for each group.

This struct is created by the `fold_chunks()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="foldchunks-new"></span>`fn new(base: I, chunk_size: usize, identity: ID, fold_op: F) -> Self`

  Creates a new `FoldChunks` iterator

#### Trait Implementations

##### `impl Any for FoldChunks<I, ID, F>`

- <span id="foldchunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldChunks<I, ID, F>`

- <span id="foldchunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldChunks<I, ID, F>`

- <span id="foldchunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for FoldChunks<I, ID, F>`

- <span id="foldchunks-clone"></span>`fn clone(&self) -> FoldChunks<I, ID, F>` — [`FoldChunks`](#foldchunks)

##### `impl CloneToUninit for FoldChunks<I, ID, F>`

- <span id="foldchunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, ID, F> Debug for FoldChunks<I, ID, F>`

- <span id="foldchunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FoldChunks<I, ID, F>`

- <span id="foldchunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, ID, F> IndexedParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="foldchunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldchunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for FoldChunks<I, ID, F>`

- <span id="foldchunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldChunks<I, ID, F>`

##### `impl IntoParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldchunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldchunks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, ID, F> ParallelIterator for FoldChunks<I, ID, F>`

- <span id="foldchunks-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldchunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldchunks-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for FoldChunks<I, ID, F>`

- <span id="foldchunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldchunks-pointable-type-init"></span>`type Init = T`

- <span id="foldchunks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldchunks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldchunks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldchunks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FoldChunks<I, ID, F>`

- <span id="foldchunks-toowned-type-owned"></span>`type Owned = T`

- <span id="foldchunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foldchunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FoldChunks<I, ID, F>`

- <span id="foldchunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldchunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldChunks<I, ID, F>`

- <span id="foldchunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldchunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

