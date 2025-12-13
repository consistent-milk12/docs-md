*[rayon](../../index.md) / [iter](../index.md) / [cloned](index.md)*

---

# Module `cloned`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Cloned`](#cloned) | struct | `Cloned` is an iterator that clones the elements of an underlying iterator. |
| [`ClonedProducer`](#clonedproducer) | struct |  |
| [`ClonedConsumer`](#clonedconsumer) | struct |  |
| [`ClonedFolder`](#clonedfolder) | struct |  |

## Structs

### `Cloned<I>`

```rust
struct Cloned<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:13-15`](../../../../.source_1765521767/rayon-1.11.0/src/iter/cloned.rs#L13-L15)*

`Cloned` is an iterator that clones the elements of an underlying iterator.

This struct is created by the `cloned()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="cloned-new"></span>`fn new(base: I) -> Self`

  Creates a new `Cloned` iterator.

#### Trait Implementations

##### `impl Any for Cloned<I>`

- <span id="cloned-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cloned<I>`

- <span id="cloned-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cloned<I>`

- <span id="cloned-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Cloned<I>`

- <span id="cloned-clone"></span>`fn clone(&self) -> Cloned<I>` — [`Cloned`](#cloned)

##### `impl CloneToUninit for Cloned<I>`

- <span id="cloned-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Cloned<I>`

- <span id="cloned-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Cloned<I>`

- <span id="cloned-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Cloned<I>`

- <span id="cloned-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="cloned-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="cloned-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Cloned<I>`

- <span id="cloned-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Cloned<I>`

##### `impl IntoParallelIterator for Cloned<I>`

- <span id="cloned-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="cloned-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="cloned-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Cloned<I>`

- <span id="cloned-paralleliterator-type-item"></span>`type Item = T`

- <span id="cloned-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="cloned-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Cloned<I>`

- <span id="cloned-pointable-const-align"></span>`const ALIGN: usize`

- <span id="cloned-pointable-type-init"></span>`type Init = T`

- <span id="cloned-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cloned-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cloned-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cloned-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Cloned<I>`

- <span id="cloned-toowned-type-owned"></span>`type Owned = T`

- <span id="cloned-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cloned-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Cloned<I>`

- <span id="cloned-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cloned-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cloned<I>`

- <span id="cloned-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cloned-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClonedProducer<P>`

```rust
struct ClonedProducer<P> {
    base: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:91-93`](../../../../.source_1765521767/rayon-1.11.0/src/iter/cloned.rs#L91-L93)*

#### Trait Implementations

##### `impl Any for ClonedProducer<P>`

- <span id="clonedproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClonedProducer<P>`

- <span id="clonedproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClonedProducer<P>`

- <span id="clonedproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ClonedProducer<P>`

- <span id="clonedproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClonedProducer<P>`

- <span id="clonedproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ClonedProducer<P>`

##### `impl Pointable for ClonedProducer<P>`

- <span id="clonedproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="clonedproducer-pointable-type-init"></span>`type Init = T`

- <span id="clonedproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clonedproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clonedproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clonedproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for ClonedProducer<P>`

- <span id="clonedproducer-producer-type-item"></span>`type Item = T`

- <span id="clonedproducer-producer-type-intoiter"></span>`type IntoIter = Cloned<<P as Producer>::IntoIter>`

- <span id="clonedproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="clonedproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="clonedproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="clonedproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="clonedproducer-producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

##### `impl<U> TryFrom for ClonedProducer<P>`

- <span id="clonedproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="clonedproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClonedProducer<P>`

- <span id="clonedproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="clonedproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ClonedConsumer<C>`

```rust
struct ClonedConsumer<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:134-136`](../../../../.source_1765521767/rayon-1.11.0/src/iter/cloned.rs#L134-L136)*

#### Implementations

- <span id="clonedconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl Any for ClonedConsumer<C>`

- <span id="clonedconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClonedConsumer<C>`

- <span id="clonedconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClonedConsumer<C>`

- <span id="clonedconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for ClonedConsumer<C>`

- <span id="clonedconsumer-consumer-type-folder"></span>`type Folder = ClonedFolder<<C as Consumer>::Folder>`

- <span id="clonedconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="clonedconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="clonedconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="clonedconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="clonedconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ClonedConsumer<C>`

- <span id="clonedconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClonedConsumer<C>`

- <span id="clonedconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ClonedConsumer<C>`

##### `impl Pointable for ClonedConsumer<C>`

- <span id="clonedconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="clonedconsumer-pointable-type-init"></span>`type Init = T`

- <span id="clonedconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clonedconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clonedconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clonedconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ClonedConsumer<C>`

- <span id="clonedconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="clonedconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClonedConsumer<C>`

- <span id="clonedconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="clonedconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for ClonedConsumer<C>`

- <span id="clonedconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="clonedconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `ClonedFolder<F>`

```rust
struct ClonedFolder<F> {
    base: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:187-189`](../../../../.source_1765521767/rayon-1.11.0/src/iter/cloned.rs#L187-L189)*

#### Trait Implementations

##### `impl Any for ClonedFolder<F>`

- <span id="clonedfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClonedFolder<F>`

- <span id="clonedfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClonedFolder<F>`

- <span id="clonedfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, F> Folder for ClonedFolder<F>`

- <span id="clonedfolder-folder-type-result"></span>`type Result = <F as Folder>::Result`

- <span id="clonedfolder-folder-consume"></span>`fn consume(self, item: &'a T) -> Self`

- <span id="clonedfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="clonedfolder-folder-complete"></span>`fn complete(self) -> <F as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="clonedfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ClonedFolder<F>`

- <span id="clonedfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClonedFolder<F>`

- <span id="clonedfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ClonedFolder<F>`

##### `impl Pointable for ClonedFolder<F>`

- <span id="clonedfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="clonedfolder-pointable-type-init"></span>`type Init = T`

- <span id="clonedfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clonedfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clonedfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clonedfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ClonedFolder<F>`

- <span id="clonedfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="clonedfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClonedFolder<F>`

- <span id="clonedfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="clonedfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

