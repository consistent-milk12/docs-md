*[rayon](../../index.md) / [iter](../index.md) / [flat_map](index.md)*

---

# Module `flat_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlatMap`](#flatmap) | struct | `FlatMap` maps each element to a parallel iterator, then flattens these iterators together. |
| [`FlatMapConsumer`](#flatmapconsumer) | struct |  |
| [`FlatMapFolder`](#flatmapfolder) | struct |  |

## Structs

### `FlatMap<I, F>`

```rust
struct FlatMap<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:12-15`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flat_map.rs#L12-L15)*

`FlatMap` maps each element to a parallel iterator, then flattens these iterators together.
This struct is created by the `flat_map()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="flatmap-new"></span>`fn new(base: I, map_op: F) -> Self`

  Creates a new `FlatMap` iterator.

#### Trait Implementations

##### `impl Any for FlatMap<I, F>`

- <span id="flatmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMap<I, F>`

- <span id="flatmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMap<I, F>`

- <span id="flatmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMap<I, F>`

- <span id="flatmap-clone"></span>`fn clone(&self) -> FlatMap<I, F>` — [`FlatMap`](#flatmap)

##### `impl CloneToUninit for FlatMap<I, F>`

- <span id="flatmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for FlatMap<I, F>`

- <span id="flatmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FlatMap<I, F>`

- <span id="flatmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMap<I, F>`

- <span id="flatmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMap<I, F>`

##### `impl IntoParallelIterator for FlatMap<I, F>`

- <span id="flatmap-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatmap-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmap-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for FlatMap<I, F>`

- <span id="flatmap-paralleliterator-type-item"></span>`type Item = <PI as IntoParallelIterator>::Item`

- <span id="flatmap-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FlatMap<I, F>`

- <span id="flatmap-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmap-pointable-type-init"></span>`type Init = T`

- <span id="flatmap-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmap-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmap-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmap-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FlatMap<I, F>`

- <span id="flatmap-toowned-type-owned"></span>`type Owned = T`

- <span id="flatmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FlatMap<I, F>`

- <span id="flatmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMap<I, F>`

- <span id="flatmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlatMapConsumer<'f, C, F>`

```rust
struct FlatMapConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:50-53`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flat_map.rs#L50-L53)*

#### Implementations

- <span id="flatmapconsumer-new"></span>`fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl Any for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Consumer for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-consumer-type-folder"></span>`type Folder = FlatMapFolder<'f, C, F, <C as Consumer>::Result>`

- <span id="flatmapconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flatmapconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatmapconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatmapconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatmapconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMapConsumer<'f, C, F>`

##### `impl Pointable for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmapconsumer-pointable-type-init"></span>`type Init = T`

- <span id="flatmapconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmapconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmapconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, F> UnindexedConsumer for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flatmapconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FlatMapFolder<'f, C, F, R>`

```rust
struct FlatMapFolder<'f, C, F, R> {
    base: C,
    map_op: &'f F,
    previous: Option<R>,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:108-112`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flat_map.rs#L108-L112)*

#### Trait Implementations

##### `impl Any for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Folder for FlatMapFolder<'f, C, F, <C as >::Result>`

- <span id="flatmapfolder-folder-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatmapfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flatmapfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="flatmapfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatMapFolder<'f, C, F, R>`

##### `impl Pointable for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatmapfolder-pointable-type-init"></span>`type Init = T`

- <span id="flatmapfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatmapfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatmapfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

