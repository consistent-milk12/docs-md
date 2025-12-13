*[rayon](../../index.md) / [iter](../index.md) / [skip](index.md)*

---

# Module `skip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Skip`](#skip) | struct | `Skip` is an iterator that skips over the first `n` elements. |

## Structs

### `Skip<I>`

```rust
struct Skip<I> {
    base: I,
    n: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip.rs:11-14`](../../../../.source_1765633015/rayon-1.11.0/src/iter/skip.rs#L11-L14)*

`Skip` is an iterator that skips over the first `n` elements.
This struct is created by the `skip()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="skip-new"></span>`fn new(base: I, n: usize) -> Self`

  Creates a new `Skip` iterator.

#### Trait Implementations

##### `impl Any for Skip<I>`

- <span id="skip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Skip<I>`

- <span id="skip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Skip<I>`

- <span id="skip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Skip<I>`

- <span id="skip-clone"></span>`fn clone(&self) -> Skip<I>` — [`Skip`](#skip)

##### `impl CloneToUninit for Skip<I>`

- <span id="skip-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Skip<I>`

- <span id="skip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Skip<I>`

- <span id="skip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Skip<I>`

- <span id="skip-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="skip-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skip-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Skip<I>`

- <span id="skip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Skip<I>`

##### `impl IntoParallelIterator for Skip<I>`

- <span id="skip-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="skip-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skip-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Skip<I>`

- <span id="skip-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skip-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="skip-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Skip<I>`

- <span id="skip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="skip-pointable-type-init"></span>`type Init = T`

- <span id="skip-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skip-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skip-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skip-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Skip<I>`

- <span id="skip-toowned-type-owned"></span>`type Owned = T`

- <span id="skip-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="skip-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Skip<I>`

- <span id="skip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="skip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Skip<I>`

- <span id="skip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="skip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

