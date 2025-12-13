*[rayon](../../index.md) / [iter](../index.md) / [take](index.md)*

---

# Module `take`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Take`](#take) | struct | `Take` is an iterator that iterates over the first `n` elements. |

## Structs

### `Take<I>`

```rust
struct Take<I> {
    base: I,
    n: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/take.rs:10-13`](../../../../.source_1765633015/rayon-1.11.0/src/iter/take.rs#L10-L13)*

`Take` is an iterator that iterates over the first `n` elements.
This struct is created by the `take()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="take-new"></span>`fn new(base: I, n: usize) -> Self`

  Creates a new `Take` iterator.

#### Trait Implementations

##### `impl Any for Take<I>`

- <span id="take-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Take<I>`

- <span id="take-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Take<I>`

- <span id="take-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Take<I>`

- <span id="take-clone"></span>`fn clone(&self) -> Take<I>` — [`Take`](#take)

##### `impl CloneToUninit for Take<I>`

- <span id="take-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Take<I>`

- <span id="take-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Take<I>`

- <span id="take-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Take<I>`

- <span id="take-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="take-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="take-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Take<I>`

- <span id="take-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Take<I>`

##### `impl IntoParallelIterator for Take<I>`

- <span id="take-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="take-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="take-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Take<I>`

- <span id="take-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="take-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="take-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Take<I>`

- <span id="take-pointable-const-align"></span>`const ALIGN: usize`

- <span id="take-pointable-type-init"></span>`type Init = T`

- <span id="take-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="take-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="take-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="take-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Take<I>`

- <span id="take-toowned-type-owned"></span>`type Owned = T`

- <span id="take-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="take-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Take<I>`

- <span id="take-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="take-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Take<I>`

- <span id="take-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="take-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

