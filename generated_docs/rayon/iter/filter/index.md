*[rayon](../../index.md) / [iter](../index.md) / [filter](index.md)*

---

# Module `filter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Filter`](#filter) | struct | `Filter` takes a predicate `filter_op` and filters out elements that match. |
| [`FilterConsumer`](#filterconsumer) | struct |  |
| [`FilterFolder`](#filterfolder) | struct |  |

## Structs

### `Filter<I, P>`

```rust
struct Filter<I, P> {
    base: I,
    filter_op: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter.rs:12-15`](../../../../.source_1765633015/rayon-1.11.0/src/iter/filter.rs#L12-L15)*

`Filter` takes a predicate `filter_op` and filters out elements that match.
This struct is created by the `filter()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="filter-new"></span>`fn new(base: I, filter_op: P) -> Self`

  Creates a new `Filter` iterator.

#### Trait Implementations

##### `impl Any for Filter<I, P>`

- <span id="filter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Filter<I, P>`

- <span id="filter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Filter<I, P>`

- <span id="filter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Filter<I, P>`

- <span id="filter-clone"></span>`fn clone(&self) -> Filter<I, P>` — [`Filter`](#filter)

##### `impl CloneToUninit for Filter<I, P>`

- <span id="filter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, P> Debug for Filter<I, P>`

- <span id="filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Filter<I, P>`

- <span id="filter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Filter<I, P>`

- <span id="filter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Filter<I, P>`

##### `impl IntoParallelIterator for Filter<I, P>`

- <span id="filter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="filter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Filter<I, P>`

- <span id="filter-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="filter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for Filter<I, P>`

- <span id="filter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filter-pointable-type-init"></span>`type Init = T`

- <span id="filter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Filter<I, P>`

- <span id="filter-toowned-type-owned"></span>`type Owned = T`

- <span id="filter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Filter<I, P>`

- <span id="filter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Filter<I, P>`

- <span id="filter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FilterConsumer<'p, C, P>`

```rust
struct FilterConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter.rs:49-52`](../../../../.source_1765633015/rayon-1.11.0/src/iter/filter.rs#L49-L52)*

#### Implementations

- <span id="filterconsumer-new"></span>`fn new(base: C, filter_op: &'p P) -> Self`

#### Trait Implementations

##### `impl Any for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, P> Consumer for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-consumer-type-folder"></span>`type Folder = FilterFolder<'p, <C as Consumer>::Folder, P>`

- <span id="filterconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="filterconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="filterconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filterconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filterconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterConsumer<'p, C, P>`

##### `impl Pointable for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filterconsumer-pointable-type-init"></span>`type Init = T`

- <span id="filterconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filterconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filterconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filterconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filterconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filterconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, P> UnindexedConsumer for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="filterconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FilterFolder<'p, C, P>`

```rust
struct FilterFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter.rs:104-107`](../../../../.source_1765633015/rayon-1.11.0/src/iter/filter.rs#L104-L107)*

#### Trait Implementations

##### `impl Any for FilterFolder<'p, C, P>`

- <span id="filterfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterFolder<'p, C, P>`

- <span id="filterfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterFolder<'p, C, P>`

- <span id="filterfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<C, P, T> Folder for FilterFolder<'p, C, P>`

- <span id="filterfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="filterfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="filterfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="filterfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FilterFolder<'p, C, P>`

- <span id="filterfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FilterFolder<'p, C, P>`

- <span id="filterfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterFolder<'p, C, P>`

##### `impl Pointable for FilterFolder<'p, C, P>`

- <span id="filterfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filterfolder-pointable-type-init"></span>`type Init = T`

- <span id="filterfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filterfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filterfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filterfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FilterFolder<'p, C, P>`

- <span id="filterfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filterfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterFolder<'p, C, P>`

- <span id="filterfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filterfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

