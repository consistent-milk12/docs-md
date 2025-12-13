*[rayon](../../index.md) / [iter](../index.md) / [step_by](index.md)*

---

# Module `step_by`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StepBy`](#stepby) | struct | `StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step. |
| [`StepByProducer`](#stepbyproducer) | struct |  |

## Structs

### `StepBy<I>`

```rust
struct StepBy<I> {
    base: I,
    step: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/step_by.rs:11-14`](../../../../.source_1765633015/rayon-1.11.0/src/iter/step_by.rs#L11-L14)*

`StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
This struct is created by the `step_by()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="stepby-new"></span>`fn new(base: I, step: usize) -> Self`

  Creates a new `StepBy` iterator.

#### Trait Implementations

##### `impl Any for StepBy<I>`

- <span id="stepby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StepBy<I>`

- <span id="stepby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StepBy<I>`

- <span id="stepby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for StepBy<I>`

- <span id="stepby-clone"></span>`fn clone(&self) -> StepBy<I>` — [`StepBy`](#stepby)

##### `impl CloneToUninit for StepBy<I>`

- <span id="stepby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for StepBy<I>`

- <span id="stepby-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StepBy<I>`

- <span id="stepby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for StepBy<I>`

- <span id="stepby-indexedparalleliterator-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="stepby-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="stepby-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for StepBy<I>`

- <span id="stepby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for StepBy<I>`

##### `impl IntoParallelIterator for StepBy<I>`

- <span id="stepby-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="stepby-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="stepby-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for StepBy<I>`

- <span id="stepby-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="stepby-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="stepby-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for StepBy<I>`

- <span id="stepby-pointable-const-align"></span>`const ALIGN: usize`

- <span id="stepby-pointable-type-init"></span>`type Init = T`

- <span id="stepby-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stepby-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stepby-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stepby-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for StepBy<I>`

- <span id="stepby-toowned-type-owned"></span>`type Owned = T`

- <span id="stepby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stepby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StepBy<I>`

- <span id="stepby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stepby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StepBy<I>`

- <span id="stepby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stepby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StepByProducer<P>`

```rust
struct StepByProducer<P> {
    base: P,
    step: usize,
    len: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/step_by.rs:93-97`](../../../../.source_1765633015/rayon-1.11.0/src/iter/step_by.rs#L93-L97)*

#### Trait Implementations

##### `impl Any for StepByProducer<P>`

- <span id="stepbyproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StepByProducer<P>`

- <span id="stepbyproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StepByProducer<P>`

- <span id="stepbyproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for StepByProducer<P>`

- <span id="stepbyproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StepByProducer<P>`

- <span id="stepbyproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for StepByProducer<P>`

##### `impl Pointable for StepByProducer<P>`

- <span id="stepbyproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="stepbyproducer-pointable-type-init"></span>`type Init = T`

- <span id="stepbyproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stepbyproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stepbyproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stepbyproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for StepByProducer<P>`

- <span id="stepbyproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="stepbyproducer-producer-type-intoiter"></span>`type IntoIter = StepBy<<P as Producer>::IntoIter>`

- <span id="stepbyproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="stepbyproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="stepbyproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="stepbyproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

##### `impl<U> TryFrom for StepByProducer<P>`

- <span id="stepbyproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stepbyproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StepByProducer<P>`

- <span id="stepbyproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stepbyproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

