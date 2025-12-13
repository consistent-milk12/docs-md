*[rayon](../index.md) / [option](index.md)*

---

# Module `option`

Parallel iterator types for [`options`](../../textwrap/options/index.md)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | A parallel iterator over the value in [`Some`] variant of an [`Option`]. |
| [`Iter`](#iter) | struct | A parallel iterator over a reference to the [`Some`] variant of an [`Option`]. |
| [`IterMut`](#itermut) | struct | A parallel iterator over a mutable reference to the [`Some`] variant of an [`Option`]. |
| [`OptionProducer`](#optionproducer) | struct | Private producer for an option |

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    opt: Option<T>,
}
```

*Defined in [`rayon-1.11.0/src/option.rs:20-22`](../../../.source_1765521767/rayon-1.11.0/src/option.rs#L20-L22)*

A parallel iterator over the value in [`Some`](#some) variant of an [`Option`](../../clap_derive/index.md).

The iterator yields one value if the [`Option`](../../clap_derive/index.md) is a [`Some`](#some), otherwise none.

This `struct` is created by the `into_par_iter` function.


#### Trait Implementations

##### `impl<T> Any for IntoIter<T>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<T>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<T>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T> CloneToUninit for IntoIter<T>`

- <span id="intoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for IntoIter<T>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntoIter<T>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for IntoIter<T>`

- <span id="intoiter-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="intoiter-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for IntoIter<T>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IntoIter<T>`

##### `impl<T> IntoParallelIterator for IntoIter<T>`

- <span id="intoiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intoiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T>`

- <span id="intoiter-paralleliterator-type-item"></span>`type Item = T`

- <span id="intoiter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

- <span id="intoiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for IntoIter<T>`

- <span id="intoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="intoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IntoIter<T>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntoIter<T>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: IntoIter<&'a T>,
}
```

*Defined in [`rayon-1.11.0/src/option.rs:83-85`](../../../.source_1765521767/rayon-1.11.0/src/option.rs#L83-L85)*

A parallel iterator over a reference to the [`Some`](#some) variant of an [`Option`](../../clap_derive/index.md).

The iterator yields one value if the [`Option`](../../clap_derive/index.md) is a [`Some`](#some), otherwise none.

This `struct` is created by the `par_iter` function.


#### Trait Implementations

##### `impl<T> Any for Iter<'a, T>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<'a, T>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<'a, T>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for Iter<'a, T>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Iter<'a, T>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Iter<'a, T>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Sync> IndexedParallelIterator for Iter<'a, T>`

- <span id="iter-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="iter-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for Iter<'a, T>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Iter<'a, T>`

##### `impl<T> IntoParallelIterator for Iter<'a, T>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Iter<'a, T>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = &'a T`

- <span id="iter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, T>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Iter<'a, T>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Iter<'a, T>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Iter<'a, T>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    inner: IntoIter<&'a mut T>,
}
```

*Defined in [`rayon-1.11.0/src/option.rs:119-121`](../../../.source_1765521767/rayon-1.11.0/src/option.rs#L119-L121)*

A parallel iterator over a mutable reference to the [`Some`](#some) variant of an [`Option`](../../clap_derive/index.md).

The iterator yields one value if the [`Option`](../../clap_derive/index.md) is a [`Some`](#some), otherwise none.

This `struct` is created by the `par_iter_mut` function.


#### Trait Implementations

##### `impl<T> Any for IterMut<'a, T>`

- <span id="itermut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterMut<'a, T>`

- <span id="itermut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterMut<'a, T>`

- <span id="itermut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug> Debug for IterMut<'a, T>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IterMut<'a, T>`

- <span id="itermut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for IterMut<'a, T>`

- <span id="itermut-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="itermut-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for IterMut<'a, T>`

- <span id="itermut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IterMut<'a, T>`

##### `impl<T> IntoParallelIterator for IterMut<'a, T>`

- <span id="itermut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="itermut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IterMut<'a, T>`

- <span id="itermut-paralleliterator-type-item"></span>`type Item = &'a mut T`

- <span id="itermut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'a, T>`

- <span id="itermut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermut-pointable-type-init"></span>`type Init = T`

- <span id="itermut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for IterMut<'a, T>`

- <span id="itermut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="itermut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterMut<'a, T>`

- <span id="itermut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="itermut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OptionProducer<T: Send>`

```rust
struct OptionProducer<T: Send> {
    opt: Option<T>,
}
```

*Defined in [`rayon-1.11.0/src/option.rs:140-142`](../../../.source_1765521767/rayon-1.11.0/src/option.rs#L140-L142)*

Private producer for an option

#### Trait Implementations

##### `impl<T> Any for OptionProducer<T>`

- <span id="optionproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OptionProducer<T>`

- <span id="optionproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OptionProducer<T>`

- <span id="optionproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for OptionProducer<T>`

- <span id="optionproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for OptionProducer<T>`

- <span id="optionproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for OptionProducer<T>`

##### `impl<T> Pointable for OptionProducer<T>`

- <span id="optionproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="optionproducer-pointable-type-init"></span>`type Init = T`

- <span id="optionproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="optionproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="optionproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="optionproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Producer for OptionProducer<T>`

- <span id="optionproducer-producer-type-item"></span>`type Item = T`

- <span id="optionproducer-producer-type-intoiter"></span>`type IntoIter = IntoIter<T>`

- <span id="optionproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="optionproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for OptionProducer<T>`

- <span id="optionproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="optionproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for OptionProducer<T>`

- <span id="optionproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="optionproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

