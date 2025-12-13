*[rayon](../../index.md) / [iter](../index.md) / [rev](index.md)*

---

# Module `rev`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Rev`](#rev) | struct | `Rev` is an iterator that produces elements in reverse order. |
| [`RevProducer`](#revproducer) | struct |  |

## Structs

### `Rev<I>`

```rust
struct Rev<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/rev.rs:11-13`](../../../../.source_1765521767/rayon-1.11.0/src/iter/rev.rs#L11-L13)*

`Rev` is an iterator that produces elements in reverse order. This struct
is created by the `rev()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="rev-new"></span>`fn new(base: I) -> Self`

  Creates a new `Rev` iterator.

#### Trait Implementations

##### `impl Any for Rev<I>`

- <span id="rev-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Rev<I>`

- <span id="rev-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Rev<I>`

- <span id="rev-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Rev<I>`

- <span id="rev-clone"></span>`fn clone(&self) -> Rev<I>` — [`Rev`](#rev)

##### `impl CloneToUninit for Rev<I>`

- <span id="rev-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Rev<I>`

- <span id="rev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Rev<I>`

- <span id="rev-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Rev<I>`

- <span id="rev-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="rev-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="rev-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Rev<I>`

- <span id="rev-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Rev<I>`

##### `impl IntoParallelIterator for Rev<I>`

- <span id="rev-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="rev-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rev-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Rev<I>`

- <span id="rev-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="rev-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="rev-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Rev<I>`

- <span id="rev-pointable-const-align"></span>`const ALIGN: usize`

- <span id="rev-pointable-type-init"></span>`type Init = T`

- <span id="rev-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rev-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rev-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rev-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Rev<I>`

- <span id="rev-toowned-type-owned"></span>`type Owned = T`

- <span id="rev-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rev-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Rev<I>`

- <span id="rev-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rev-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Rev<I>`

- <span id="rev-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rev-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RevProducer<P>`

```rust
struct RevProducer<P> {
    base: P,
    len: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/rev.rs:83-86`](../../../../.source_1765521767/rayon-1.11.0/src/iter/rev.rs#L83-L86)*

#### Trait Implementations

##### `impl Any for RevProducer<P>`

- <span id="revproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RevProducer<P>`

- <span id="revproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RevProducer<P>`

- <span id="revproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for RevProducer<P>`

- <span id="revproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RevProducer<P>`

- <span id="revproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RevProducer<P>`

##### `impl Pointable for RevProducer<P>`

- <span id="revproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="revproducer-pointable-type-init"></span>`type Init = T`

- <span id="revproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="revproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="revproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="revproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for RevProducer<P>`

- <span id="revproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="revproducer-producer-type-intoiter"></span>`type IntoIter = Rev<<P as Producer>::IntoIter>`

- <span id="revproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="revproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="revproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="revproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<U> TryFrom for RevProducer<P>`

- <span id="revproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="revproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RevProducer<P>`

- <span id="revproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="revproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

