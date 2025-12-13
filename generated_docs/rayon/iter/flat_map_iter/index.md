*[rayon](../../index.md) / [iter](../index.md) / [flat_map_iter](index.md)*

---

# Module `flat_map_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlatMapIter`](#flatmapiter) | struct | `FlatMapIter` maps each element to a serial iterator, then flattens these iterators together. |
| [`FlatMapIterConsumer`](#flatmapiterconsumer) | struct |  |
| [`FlatMapIterFolder`](#flatmapiterfolder) | struct |  |

## Structs

### `FlatMapIter<I, F>`

```rust
struct FlatMapIter<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map_iter.rs:12-15`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flat_map_iter.rs#L12-L15)*

`FlatMapIter` maps each element to a serial iterator, then flattens these iterators together.
This struct is created by the `flat_map_iter()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="flatmapiter-new"></span>`fn new(base: I, map_op: F) -> Self`

  Creates a new `FlatMapIter` iterator.

#### Trait Implementations

##### `impl Any for FlatMapIter<I, F>`

- <span id="flatmapiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMapIter<I, F>`

- <span id="flatmapiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMapIter<I, F>`

- <span id="flatmapiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMapIter<I, F>`

- <span id="flatmapiter-clone"></span>`fn clone(&self) -> FlatMapIter<I, F>` — [`FlatMapIter`](#flatmapiter)

##### `impl CloneToUninit for FlatMapIter<I, F>`

- <span id="flatmapiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for FlatMapIter<I, F>`

- <span id="flatmapiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FlatMapIter<I, F>`

- <span id="flatmapiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMapIter<I, F>`

- <span id="flatmapiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMapIter<I, F>`

##### `impl IntoParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatmapiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmapiter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-paralleliterator-type-item"></span>`type Item = <SI as IntoIterator>::Item`

- <span id="flatmapiter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FlatMapIter<I, F>`

- <span id="flatmapiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmapiter-pointable-type-init"></span>`type Init = T`

- <span id="flatmapiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FlatMapIter<I, F>`

- <span id="flatmapiter-toowned-type-owned"></span>`type Owned = T`

- <span id="flatmapiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatmapiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlatMapIter<I, F>`

- <span id="flatmapiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmapiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMapIter<I, F>`

- <span id="flatmapiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmapiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlatMapIterConsumer<'f, C, F>`

```rust
struct FlatMapIterConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map_iter.rs:52-55`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flat_map_iter.rs#L52-L55)*

#### Implementations

- <span id="flatmapiterconsumer-new"></span>`fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl Any for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Consumer for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-consumer-type-folder"></span>`type Folder = FlatMapIterFolder<'f, <C as Consumer>::Folder, F>`

- <span id="flatmapiterconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flatmapiterconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatmapiterconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatmapiterconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatmapiterconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMapIterConsumer<'f, C, F>`

##### `impl Pointable for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmapiterconsumer-pointable-type-init"></span>`type Init = T`

- <span id="flatmapiterconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiterconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiterconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiterconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmapiterconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmapiterconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, F> UnindexedConsumer for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flatmapiterconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FlatMapIterFolder<'f, C, F>`

```rust
struct FlatMapIterFolder<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map_iter.rs:109-112`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flat_map_iter.rs#L109-L112)*

#### Trait Implementations

##### `impl Any for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Folder for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="flatmapiterfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flatmapiterfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="flatmapiterfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="flatmapiterfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMapIterFolder<'f, C, F>`

##### `impl Pointable for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmapiterfolder-pointable-type-init"></span>`type Init = T`

- <span id="flatmapiterfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiterfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiterfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiterfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmapiterfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmapiterfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

