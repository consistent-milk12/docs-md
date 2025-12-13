*[rayon](../../index.md) / [iter](../index.md) / [copied](index.md)*

---

# Module `copied`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Copied`](#copied) | struct | `Copied` is an iterator that copies the elements of an underlying iterator. |
| [`CopiedProducer`](#copiedproducer) | struct |  |
| [`CopiedConsumer`](#copiedconsumer) | struct |  |
| [`CopiedFolder`](#copiedfolder) | struct |  |

## Structs

### `Copied<I>`

```rust
struct Copied<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/copied.rs:13-15`](../../../../.source_1765633015/rayon-1.11.0/src/iter/copied.rs#L13-L15)*

`Copied` is an iterator that copies the elements of an underlying iterator.

This struct is created by the `copied()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="copied-new"></span>`fn new(base: I) -> Self`

  Creates a new `Copied` iterator.

#### Trait Implementations

##### `impl Any for Copied<I>`

- <span id="copied-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Copied<I>`

- <span id="copied-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Copied<I>`

- <span id="copied-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Copied<I>`

- <span id="copied-clone"></span>`fn clone(&self) -> Copied<I>` — [`Copied`](#copied)

##### `impl CloneToUninit for Copied<I>`

- <span id="copied-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Copied<I>`

- <span id="copied-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Copied<I>`

- <span id="copied-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Copied<I>`

- <span id="copied-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="copied-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="copied-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Copied<I>`

- <span id="copied-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Copied<I>`

##### `impl IntoParallelIterator for Copied<I>`

- <span id="copied-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="copied-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="copied-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Copied<I>`

- <span id="copied-paralleliterator-type-item"></span>`type Item = T`

- <span id="copied-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="copied-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Copied<I>`

- <span id="copied-pointable-const-align"></span>`const ALIGN: usize`

- <span id="copied-pointable-type-init"></span>`type Init = T`

- <span id="copied-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copied-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copied-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copied-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Copied<I>`

- <span id="copied-toowned-type-owned"></span>`type Owned = T`

- <span id="copied-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="copied-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Copied<I>`

- <span id="copied-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="copied-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Copied<I>`

- <span id="copied-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="copied-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CopiedProducer<P>`

```rust
struct CopiedProducer<P> {
    base: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/copied.rs:91-93`](../../../../.source_1765633015/rayon-1.11.0/src/iter/copied.rs#L91-L93)*

#### Trait Implementations

##### `impl Any for CopiedProducer<P>`

- <span id="copiedproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CopiedProducer<P>`

- <span id="copiedproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CopiedProducer<P>`

- <span id="copiedproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for CopiedProducer<P>`

- <span id="copiedproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CopiedProducer<P>`

- <span id="copiedproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CopiedProducer<P>`

##### `impl Pointable for CopiedProducer<P>`

- <span id="copiedproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="copiedproducer-pointable-type-init"></span>`type Init = T`

- <span id="copiedproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copiedproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copiedproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copiedproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for CopiedProducer<P>`

- <span id="copiedproducer-producer-type-item"></span>`type Item = T`

- <span id="copiedproducer-producer-type-intoiter"></span>`type IntoIter = Copied<<P as Producer>::IntoIter>`

- <span id="copiedproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="copiedproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="copiedproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="copiedproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="copiedproducer-producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

##### `impl<U> TryFrom for CopiedProducer<P>`

- <span id="copiedproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="copiedproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CopiedProducer<P>`

- <span id="copiedproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="copiedproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CopiedConsumer<C>`

```rust
struct CopiedConsumer<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/copied.rs:134-136`](../../../../.source_1765633015/rayon-1.11.0/src/iter/copied.rs#L134-L136)*

#### Implementations

- <span id="copiedconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl Any for CopiedConsumer<C>`

- <span id="copiedconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CopiedConsumer<C>`

- <span id="copiedconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CopiedConsumer<C>`

- <span id="copiedconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for CopiedConsumer<C>`

- <span id="copiedconsumer-consumer-type-folder"></span>`type Folder = CopiedFolder<<C as Consumer>::Folder>`

- <span id="copiedconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="copiedconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="copiedconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="copiedconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="copiedconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for CopiedConsumer<C>`

- <span id="copiedconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CopiedConsumer<C>`

- <span id="copiedconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CopiedConsumer<C>`

##### `impl Pointable for CopiedConsumer<C>`

- <span id="copiedconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="copiedconsumer-pointable-type-init"></span>`type Init = T`

- <span id="copiedconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copiedconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copiedconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copiedconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CopiedConsumer<C>`

- <span id="copiedconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="copiedconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CopiedConsumer<C>`

- <span id="copiedconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="copiedconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for CopiedConsumer<C>`

- <span id="copiedconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="copiedconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `CopiedFolder<F>`

```rust
struct CopiedFolder<F> {
    base: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/copied.rs:187-189`](../../../../.source_1765633015/rayon-1.11.0/src/iter/copied.rs#L187-L189)*

#### Trait Implementations

##### `impl Any for CopiedFolder<F>`

- <span id="copiedfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CopiedFolder<F>`

- <span id="copiedfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CopiedFolder<F>`

- <span id="copiedfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, F> Folder for CopiedFolder<F>`

- <span id="copiedfolder-folder-type-result"></span>`type Result = <F as Folder>::Result`

- <span id="copiedfolder-folder-consume"></span>`fn consume(self, item: &'a T) -> Self`

- <span id="copiedfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="copiedfolder-folder-complete"></span>`fn complete(self) -> <F as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="copiedfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for CopiedFolder<F>`

- <span id="copiedfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CopiedFolder<F>`

- <span id="copiedfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CopiedFolder<F>`

##### `impl Pointable for CopiedFolder<F>`

- <span id="copiedfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="copiedfolder-pointable-type-init"></span>`type Init = T`

- <span id="copiedfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copiedfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copiedfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copiedfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for CopiedFolder<F>`

- <span id="copiedfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="copiedfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CopiedFolder<F>`

- <span id="copiedfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="copiedfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

