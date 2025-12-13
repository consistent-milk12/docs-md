*[rayon](../../index.md) / [iter](../index.md) / [update](index.md)*

---

# Module `update`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Update`](#update) | struct | `Update` is an iterator that mutates the elements of an underlying iterator before they are yielded. |
| [`UpdateProducer`](#updateproducer) | struct |  |
| [`UpdateConsumer`](#updateconsumer) | struct |  |
| [`UpdateFolder`](#updatefolder) | struct |  |
| [`UpdateSeq`](#updateseq) | struct | Standard Update adaptor, based on `itertools::adaptors::Update` |
| [`apply`](#apply) | fn |  |

## Structs

### `Update<I, F>`

```rust
struct Update<I, F> {
    base: I,
    update_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:14-17`](../../../../.source_1765633015/rayon-1.11.0/src/iter/update.rs#L14-L17)*

`Update` is an iterator that mutates the elements of an
underlying iterator before they are yielded.

This struct is created by the `update()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="update-new"></span>`fn new(base: I, update_op: F) -> Self`

  Creates a new `Update` iterator.

#### Trait Implementations

##### `impl Any for Update<I, F>`

- <span id="update-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Update<I, F>`

- <span id="update-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Update<I, F>`

- <span id="update-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Update<I, F>`

- <span id="update-clone"></span>`fn clone(&self) -> Update<I, F>` — [`Update`](#update)

##### `impl CloneToUninit for Update<I, F>`

- <span id="update-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for Update<I, F>`

- <span id="update-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Update<I, F>`

- <span id="update-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> IndexedParallelIterator for Update<I, F>`

- <span id="update-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="update-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="update-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Update<I, F>`

- <span id="update-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Update<I, F>`

##### `impl IntoParallelIterator for Update<I, F>`

- <span id="update-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="update-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="update-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Update<I, F>`

- <span id="update-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="update-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="update-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Update<I, F>`

- <span id="update-pointable-const-align"></span>`const ALIGN: usize`

- <span id="update-pointable-type-init"></span>`type Init = T`

- <span id="update-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="update-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="update-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="update-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Update<I, F>`

- <span id="update-toowned-type-owned"></span>`type Owned = T`

- <span id="update-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="update-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Update<I, F>`

- <span id="update-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="update-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Update<I, F>`

- <span id="update-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="update-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UpdateProducer<'f, P, F>`

```rust
struct UpdateProducer<'f, P, F> {
    base: P,
    update_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:106-109`](../../../../.source_1765633015/rayon-1.11.0/src/iter/update.rs#L106-L109)*

#### Trait Implementations

##### `impl Any for UpdateProducer<'f, P, F>`

- <span id="updateproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UpdateProducer<'f, P, F>`

- <span id="updateproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UpdateProducer<'f, P, F>`

- <span id="updateproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for UpdateProducer<'f, P, F>`

- <span id="updateproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UpdateProducer<'f, P, F>`

- <span id="updateproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UpdateProducer<'f, P, F>`

##### `impl Pointable for UpdateProducer<'f, P, F>`

- <span id="updateproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="updateproducer-pointable-type-init"></span>`type Init = T`

- <span id="updateproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updateproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updateproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updateproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, F> Producer for UpdateProducer<'f, P, F>`

- <span id="updateproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="updateproducer-producer-type-intoiter"></span>`type IntoIter = UpdateSeq<<P as Producer>::IntoIter, &'f F>`

- <span id="updateproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="updateproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="updateproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="updateproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="updateproducer-producer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

##### `impl<U> TryFrom for UpdateProducer<'f, P, F>`

- <span id="updateproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="updateproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UpdateProducer<'f, P, F>`

- <span id="updateproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="updateproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UpdateConsumer<'f, C, F>`

```rust
struct UpdateConsumer<'f, C, F> {
    base: C,
    update_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:162-165`](../../../../.source_1765633015/rayon-1.11.0/src/iter/update.rs#L162-L165)*

#### Implementations

- <span id="updateconsumer-new"></span>`fn new(base: C, update_op: &'f F) -> Self`

#### Trait Implementations

##### `impl Any for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Consumer for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-consumer-type-folder"></span>`type Folder = UpdateFolder<'f, <C as Consumer>::Folder, F>`

- <span id="updateconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="updateconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="updateconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="updateconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="updateconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UpdateConsumer<'f, C, F>`

##### `impl Pointable for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="updateconsumer-pointable-type-init"></span>`type Init = T`

- <span id="updateconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updateconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updateconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updateconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="updateconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="updateconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, F> UnindexedConsumer for UpdateConsumer<'f, C, F>`

- <span id="updateconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="updateconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `UpdateFolder<'f, C, F>`

```rust
struct UpdateFolder<'f, C, F> {
    base: C,
    update_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:217-220`](../../../../.source_1765633015/rayon-1.11.0/src/iter/update.rs#L217-L220)*

#### Trait Implementations

##### `impl Any for UpdateFolder<'f, C, F>`

- <span id="updatefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UpdateFolder<'f, C, F>`

- <span id="updatefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UpdateFolder<'f, C, F>`

- <span id="updatefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Folder for UpdateFolder<'f, C, F>`

- <span id="updatefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="updatefolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="updatefolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="updatefolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="updatefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for UpdateFolder<'f, C, F>`

- <span id="updatefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UpdateFolder<'f, C, F>`

- <span id="updatefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UpdateFolder<'f, C, F>`

##### `impl Pointable for UpdateFolder<'f, C, F>`

- <span id="updatefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="updatefolder-pointable-type-init"></span>`type Init = T`

- <span id="updatefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updatefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updatefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updatefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for UpdateFolder<'f, C, F>`

- <span id="updatefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="updatefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UpdateFolder<'f, C, F>`

- <span id="updatefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="updatefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UpdateSeq<I, F>`

```rust
struct UpdateSeq<I, F> {
    base: I,
    update_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:268-271`](../../../../.source_1765633015/rayon-1.11.0/src/iter/update.rs#L268-L271)*

Standard Update adaptor, based on `itertools::adaptors::Update`

#### Trait Implementations

##### `impl Any for UpdateSeq<I, F>`

- <span id="updateseq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UpdateSeq<I, F>`

- <span id="updateseq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UpdateSeq<I, F>`

- <span id="updateseq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for UpdateSeq<I, F>`

- <span id="updateseq-clone"></span>`fn clone(&self) -> UpdateSeq<I, F>` — [`UpdateSeq`](#updateseq)

##### `impl CloneToUninit for UpdateSeq<I, F>`

- <span id="updateseq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, F: fmt::Debug> Debug for UpdateSeq<I, F>`

- <span id="updateseq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> DoubleEndedIterator for UpdateSeq<I, F>`

- <span id="updateseq-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I, F> ExactSizeIterator for UpdateSeq<I, F>`

##### `impl<T> From for UpdateSeq<I, F>`

- <span id="updateseq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for UpdateSeq<I, F>`

- <span id="updateseq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UpdateSeq<I, F>`

##### `impl<I> IntoIterator for UpdateSeq<I, F>`

- <span id="updateseq-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="updateseq-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="updateseq-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for UpdateSeq<I, F>`

- <span id="updateseq-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="updateseq-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="updateseq-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="updateseq-iterator-fold"></span>`fn fold<Acc, G>(self, init: Acc, g: G) -> Acc`

- <span id="updateseq-iterator-collect"></span>`fn collect<C>(self) -> C`

##### `impl Pointable for UpdateSeq<I, F>`

- <span id="updateseq-pointable-const-align"></span>`const ALIGN: usize`

- <span id="updateseq-pointable-type-init"></span>`type Init = T`

- <span id="updateseq-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="updateseq-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="updateseq-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="updateseq-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for UpdateSeq<I, F>`

- <span id="updateseq-toowned-type-owned"></span>`type Owned = T`

- <span id="updateseq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="updateseq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UpdateSeq<I, F>`

- <span id="updateseq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="updateseq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UpdateSeq<I, F>`

- <span id="updateseq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="updateseq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `apply`

```rust
fn apply<T>(update_op: impl Fn(&mut T)) -> impl Fn(T) -> T
```

*Defined in [`rayon-1.11.0/src/iter/update.rs:222-227`](../../../../.source_1765633015/rayon-1.11.0/src/iter/update.rs#L222-L227)*

