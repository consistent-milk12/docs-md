*[rayon](../../index.md) / [iter](../index.md) / [empty](index.md)*

---

# Module `empty`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Empty`](#empty) | struct | Iterator adaptor for [the `empty()` function]. |
| [`EmptyProducer`](#emptyproducer) | struct | Private empty producer |
| [`empty`](#empty) | fn | Creates a parallel iterator that produces nothing. |

## Structs

### `Empty<T>`

```rust
struct Empty<T> {
    marker: std::marker::PhantomData<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:33-35`](../../../../.source_1765633015/rayon-1.11.0/src/iter/empty.rs#L33-L35)*

Iterator adaptor for [the `empty()` function].


#### Trait Implementations

##### `impl<T> Any for Empty<T>`

- <span id="empty-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Empty<T>`

- <span id="empty-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Empty<T>`

- <span id="empty-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Empty<T>`

- <span id="empty-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Empty<T>`

- <span id="empty-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: Send> Debug for Empty<T>`

- <span id="empty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Empty<T>`

- <span id="empty-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for Empty<T>`

- <span id="empty-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="empty-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="empty-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T, U> Into for Empty<T>`

- <span id="empty-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Empty<T>`

##### `impl<T> IntoParallelIterator for Empty<T>`

- <span id="empty-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="empty-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="empty-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Empty<T>`

- <span id="empty-paralleliterator-type-item"></span>`type Item = T`

- <span id="empty-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="empty-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Empty<T>`

- <span id="empty-pointable-const-align"></span>`const ALIGN: usize`

- <span id="empty-pointable-type-init"></span>`type Init = T`

- <span id="empty-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="empty-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="empty-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="empty-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Empty<T>`

- <span id="empty-toowned-type-owned"></span>`type Owned = T`

- <span id="empty-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="empty-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Empty<T>`

- <span id="empty-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="empty-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Empty<T>`

- <span id="empty-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="empty-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EmptyProducer<T: Send>`

```rust
struct EmptyProducer<T: Send>(std::marker::PhantomData<T>);
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:87`](../../../../.source_1765633015/rayon-1.11.0/src/iter/empty.rs#L87)*

Private empty producer

#### Trait Implementations

##### `impl<T> Any for EmptyProducer<T>`

- <span id="emptyproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EmptyProducer<T>`

- <span id="emptyproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EmptyProducer<T>`

- <span id="emptyproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for EmptyProducer<T>`

- <span id="emptyproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for EmptyProducer<T>`

- <span id="emptyproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for EmptyProducer<T>`

##### `impl<T> Pointable for EmptyProducer<T>`

- <span id="emptyproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="emptyproducer-pointable-type-init"></span>`type Init = T`

- <span id="emptyproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="emptyproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="emptyproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="emptyproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Producer for EmptyProducer<T>`

- <span id="emptyproducer-producer-type-item"></span>`type Item = T`

- <span id="emptyproducer-producer-type-intoiter"></span>`type IntoIter = Empty<T>`

- <span id="emptyproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="emptyproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="emptyproducer-producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

##### `impl<T, U> TryFrom for EmptyProducer<T>`

- <span id="emptyproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="emptyproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for EmptyProducer<T>`

- <span id="emptyproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="emptyproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `empty`

```rust
fn empty<T: Send>() -> Empty<T>
```

*Defined in [`rayon-1.11.0/src/iter/empty.rs:24-28`](../../../../.source_1765633015/rayon-1.11.0/src/iter/empty.rs#L24-L28)*

Creates a parallel iterator that produces nothing.

This admits no parallelism on its own, but it could be used for code that
deals with generic parallel iterators.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::empty;

let pi = (0..1234).into_par_iter()
    .chain(empty())
    .chain(1234..10_000);

assert_eq!(pi.count(), 10_000);
```

