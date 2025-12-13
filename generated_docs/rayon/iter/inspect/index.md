*[rayon](../../index.md) / [iter](../index.md) / [inspect](index.md)*

---

# Module `inspect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Inspect`](#inspect) | struct | `Inspect` is an iterator that calls a function with a reference to each element before yielding it. |
| [`InspectProducer`](#inspectproducer) | struct |  |
| [`InspectConsumer`](#inspectconsumer) | struct |  |
| [`InspectFolder`](#inspectfolder) | struct |  |

## Structs

### `Inspect<I, F>`

```rust
struct Inspect<I, F> {
    base: I,
    inspect_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:15-18`](../../../../.source_1765521767/rayon-1.11.0/src/iter/inspect.rs#L15-L18)*

`Inspect` is an iterator that calls a function with a reference to each
element before yielding it.

This struct is created by the `inspect()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="inspect-new"></span>`fn new(base: I, inspect_op: F) -> Self`

  Creates a new `Inspect` iterator.

#### Trait Implementations

##### `impl Any for Inspect<I, F>`

- <span id="inspect-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Inspect<I, F>`

- <span id="inspect-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Inspect<I, F>`

- <span id="inspect-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Inspect<I, F>`

- <span id="inspect-clone"></span>`fn clone(&self) -> Inspect<I, F>` — [`Inspect`](#inspect)

##### `impl CloneToUninit for Inspect<I, F>`

- <span id="inspect-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for Inspect<I, F>`

- <span id="inspect-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Inspect<I, F>`

- <span id="inspect-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> IndexedParallelIterator for Inspect<I, F>`

- <span id="inspect-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="inspect-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="inspect-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Inspect<I, F>`

- <span id="inspect-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Inspect<I, F>`

##### `impl IntoParallelIterator for Inspect<I, F>`

- <span id="inspect-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="inspect-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="inspect-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Inspect<I, F>`

- <span id="inspect-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="inspect-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="inspect-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Inspect<I, F>`

- <span id="inspect-pointable-const-align"></span>`const ALIGN: usize`

- <span id="inspect-pointable-type-init"></span>`type Init = T`

- <span id="inspect-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspect-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspect-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspect-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Inspect<I, F>`

- <span id="inspect-toowned-type-owned"></span>`type Owned = T`

- <span id="inspect-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="inspect-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Inspect<I, F>`

- <span id="inspect-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inspect-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Inspect<I, F>`

- <span id="inspect-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inspect-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InspectProducer<'f, P, F>`

```rust
struct InspectProducer<'f, P, F> {
    base: P,
    inspect_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:107-110`](../../../../.source_1765521767/rayon-1.11.0/src/iter/inspect.rs#L107-L110)*

#### Trait Implementations

##### `impl Any for InspectProducer<'f, P, F>`

- <span id="inspectproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InspectProducer<'f, P, F>`

- <span id="inspectproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InspectProducer<'f, P, F>`

- <span id="inspectproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for InspectProducer<'f, P, F>`

- <span id="inspectproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InspectProducer<'f, P, F>`

- <span id="inspectproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for InspectProducer<'f, P, F>`

##### `impl Pointable for InspectProducer<'f, P, F>`

- <span id="inspectproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="inspectproducer-pointable-type-init"></span>`type Init = T`

- <span id="inspectproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspectproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspectproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspectproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, F> Producer for InspectProducer<'f, P, F>`

- <span id="inspectproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="inspectproducer-producer-type-intoiter"></span>`type IntoIter = Inspect<<P as Producer>::IntoIter, &'f F>`

- <span id="inspectproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="inspectproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="inspectproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="inspectproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="inspectproducer-producer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

##### `impl<U> TryFrom for InspectProducer<'f, P, F>`

- <span id="inspectproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inspectproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InspectProducer<'f, P, F>`

- <span id="inspectproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inspectproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InspectConsumer<'f, C, F>`

```rust
struct InspectConsumer<'f, C, F> {
    base: C,
    inspect_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:161-164`](../../../../.source_1765521767/rayon-1.11.0/src/iter/inspect.rs#L161-L164)*

#### Implementations

- <span id="inspectconsumer-new"></span>`fn new(base: C, inspect_op: &'f F) -> Self`

#### Trait Implementations

##### `impl Any for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Consumer for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-consumer-type-folder"></span>`type Folder = InspectFolder<'f, <C as Consumer>::Folder, F>`

- <span id="inspectconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="inspectconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="inspectconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="inspectconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="inspectconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for InspectConsumer<'f, C, F>`

##### `impl Pointable for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="inspectconsumer-pointable-type-init"></span>`type Init = T`

- <span id="inspectconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspectconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspectconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspectconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inspectconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inspectconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, F> UnindexedConsumer for InspectConsumer<'f, C, F>`

- <span id="inspectconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="inspectconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `InspectFolder<'f, C, F>`

```rust
struct InspectFolder<'f, C, F> {
    base: C,
    inspect_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/inspect.rs:216-219`](../../../../.source_1765521767/rayon-1.11.0/src/iter/inspect.rs#L216-L219)*

#### Trait Implementations

##### `impl Any for InspectFolder<'f, C, F>`

- <span id="inspectfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InspectFolder<'f, C, F>`

- <span id="inspectfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InspectFolder<'f, C, F>`

- <span id="inspectfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Folder for InspectFolder<'f, C, F>`

- <span id="inspectfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="inspectfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="inspectfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="inspectfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="inspectfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for InspectFolder<'f, C, F>`

- <span id="inspectfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InspectFolder<'f, C, F>`

- <span id="inspectfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for InspectFolder<'f, C, F>`

##### `impl Pointable for InspectFolder<'f, C, F>`

- <span id="inspectfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="inspectfolder-pointable-type-init"></span>`type Init = T`

- <span id="inspectfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="inspectfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="inspectfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="inspectfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for InspectFolder<'f, C, F>`

- <span id="inspectfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inspectfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InspectFolder<'f, C, F>`

- <span id="inspectfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inspectfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

