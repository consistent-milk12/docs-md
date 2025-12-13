*[rayon](../../index.md) / [iter](../index.md) / [filter_map](index.md)*

---

# Module `filter_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FilterMap`](#filtermap) | struct | `FilterMap` creates an iterator that uses `filter_op` to both filter and map elements. |
| [`FilterMapConsumer`](#filtermapconsumer) | struct |  |
| [`FilterMapFolder`](#filtermapfolder) | struct |  |

## Structs

### `FilterMap<I, P>`

```rust
struct FilterMap<I, P> {
    base: I,
    filter_op: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:12-15`](../../../../.source_1765633015/rayon-1.11.0/src/iter/filter_map.rs#L12-L15)*

`FilterMap` creates an iterator that uses `filter_op` to both filter and map elements.
This struct is created by the `filter_map()` method on [`ParallelIterator`](../index.md).


#### Implementations

- <span id="filtermap-new"></span>`fn new(base: I, filter_op: P) -> Self`

  Creates a new `FilterMap` iterator.

#### Trait Implementations

##### `impl Any for FilterMap<I, P>`

- <span id="filtermap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterMap<I, P>`

- <span id="filtermap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterMap<I, P>`

- <span id="filtermap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for FilterMap<I, P>`

- <span id="filtermap-clone"></span>`fn clone(&self) -> FilterMap<I, P>` — [`FilterMap`](#filtermap)

##### `impl CloneToUninit for FilterMap<I, P>`

- <span id="filtermap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, P> Debug for FilterMap<I, P>`

- <span id="filtermap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FilterMap<I, P>`

- <span id="filtermap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FilterMap<I, P>`

- <span id="filtermap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterMap<I, P>`

##### `impl IntoParallelIterator for FilterMap<I, P>`

- <span id="filtermap-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="filtermap-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filtermap-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for FilterMap<I, P>`

- <span id="filtermap-paralleliterator-type-item"></span>`type Item = R`

- <span id="filtermap-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FilterMap<I, P>`

- <span id="filtermap-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filtermap-pointable-type-init"></span>`type Init = T`

- <span id="filtermap-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermap-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermap-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermap-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FilterMap<I, P>`

- <span id="filtermap-toowned-type-owned"></span>`type Owned = T`

- <span id="filtermap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filtermap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FilterMap<I, P>`

- <span id="filtermap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filtermap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterMap<I, P>`

- <span id="filtermap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filtermap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FilterMapConsumer<'p, C, P>`

```rust
struct FilterMapConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:52-55`](../../../../.source_1765633015/rayon-1.11.0/src/iter/filter_map.rs#L52-L55)*

#### Implementations

- <span id="filtermapconsumer-new"></span>`fn new(base: C, filter_op: &'p P) -> Self`

#### Trait Implementations

##### `impl Any for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Consumer for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-consumer-type-folder"></span>`type Folder = FilterMapFolder<'p, <C as Consumer>::Folder, P>`

- <span id="filtermapconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="filtermapconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="filtermapconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filtermapconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filtermapconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterMapConsumer<'p, C, P>`

##### `impl Pointable for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filtermapconsumer-pointable-type-init"></span>`type Init = T`

- <span id="filtermapconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermapconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermapconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermapconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filtermapconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filtermapconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, P> UnindexedConsumer for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="filtermapconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FilterMapFolder<'p, C, P>`

```rust
struct FilterMapFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:108-111`](../../../../.source_1765633015/rayon-1.11.0/src/iter/filter_map.rs#L108-L111)*

#### Trait Implementations

##### `impl Any for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Folder for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="filtermapfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="filtermapfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="filtermapfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterMapFolder<'p, C, P>`

##### `impl Pointable for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filtermapfolder-pointable-type-init"></span>`type Init = T`

- <span id="filtermapfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermapfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermapfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermapfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filtermapfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filtermapfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

