*[rayon](../../index.md) / [iter](../index.md) / [flatten_iter](index.md)*

---

# Module `flatten_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlattenIter`](#flatteniter) | struct | `FlattenIter` turns each element to a serial iterator, then flattens these iterators together. |
| [`FlattenIterConsumer`](#flatteniterconsumer) | struct |  |
| [`FlattenIterFolder`](#flatteniterfolder) | struct |  |

## Structs

### `FlattenIter<I>`

```rust
struct FlattenIter<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:10-12`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flatten_iter.rs#L10-L12)*

`FlattenIter` turns each element to a serial iterator, then flattens these iterators
together. This struct is created by the `flatten_iter()` method on [`ParallelIterator`](../index.md).


#### Implementations

- <span id="flatteniter-new"></span>`fn new(base: I) -> Self`

  Creates a new `FlattenIter` iterator.

#### Trait Implementations

##### `impl Any for FlattenIter<I>`

- <span id="flatteniter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlattenIter<I>`

- <span id="flatteniter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlattenIter<I>`

- <span id="flatteniter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for FlattenIter<I>`

- <span id="flatteniter-clone"></span>`fn clone(&self) -> FlattenIter<I>` — [`FlattenIter`](#flatteniter)

##### `impl CloneToUninit for FlattenIter<I>`

- <span id="flatteniter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for FlattenIter<I>`

- <span id="flatteniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FlattenIter<I>`

- <span id="flatteniter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlattenIter<I>`

- <span id="flatteniter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlattenIter<I>`

##### `impl IntoParallelIterator for FlattenIter<I>`

- <span id="flatteniter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatteniter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatteniter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for FlattenIter<I>`

- <span id="flatteniter-paralleliterator-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoIterator>::Item`

- <span id="flatteniter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FlattenIter<I>`

- <span id="flatteniter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatteniter-pointable-type-init"></span>`type Init = T`

- <span id="flatteniter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FlattenIter<I>`

- <span id="flatteniter-toowned-type-owned"></span>`type Owned = T`

- <span id="flatteniter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatteniter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlattenIter<I>`

- <span id="flatteniter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatteniter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlattenIter<I>`

- <span id="flatteniter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatteniter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlattenIterConsumer<C>`

```rust
struct FlattenIterConsumer<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:39-41`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flatten_iter.rs#L39-L41)*

#### Implementations

- <span id="flatteniterconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl Any for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-consumer-type-folder"></span>`type Folder = FlattenIterFolder<<C as Consumer>::Folder>`

- <span id="flatteniterconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flatteniterconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatteniterconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatteniterconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatteniterconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlattenIterConsumer<C>`

##### `impl Pointable for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatteniterconsumer-pointable-type-init"></span>`type Init = T`

- <span id="flatteniterconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniterconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniterconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniterconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatteniterconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatteniterconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flatteniterconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FlattenIterFolder<C>`

```rust
struct FlattenIterFolder<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:92-94`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flatten_iter.rs#L92-L94)*

#### Trait Implementations

##### `impl Any for FlattenIterFolder<C>`

- <span id="flatteniterfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlattenIterFolder<C>`

- <span id="flatteniterfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlattenIterFolder<C>`

- <span id="flatteniterfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Folder for FlattenIterFolder<C>`

- <span id="flatteniterfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="flatteniterfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flatteniterfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="flatteniterfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="flatteniterfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlattenIterFolder<C>`

- <span id="flatteniterfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlattenIterFolder<C>`

- <span id="flatteniterfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlattenIterFolder<C>`

##### `impl Pointable for FlattenIterFolder<C>`

- <span id="flatteniterfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatteniterfolder-pointable-type-init"></span>`type Init = T`

- <span id="flatteniterfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniterfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniterfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniterfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlattenIterFolder<C>`

- <span id="flatteniterfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatteniterfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlattenIterFolder<C>`

- <span id="flatteniterfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatteniterfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

