*[rayon](../../index.md) / [iter](../index.md) / [flatten](index.md)*

---

# Module `flatten`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Flatten`](#flatten) | struct | `Flatten` turns each element to a parallel iterator, then flattens these iterators together. |
| [`FlattenConsumer`](#flattenconsumer) | struct |  |
| [`FlattenFolder`](#flattenfolder) | struct |  |

## Structs

### `Flatten<I>`

```rust
struct Flatten<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:10-12`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flatten.rs#L10-L12)*

`Flatten` turns each element to a parallel iterator, then flattens these iterators
together. This struct is created by the `flatten()` method on [`ParallelIterator`](../index.md).


#### Implementations

- <span id="flatten-new"></span>`fn new(base: I) -> Self`

  Creates a new `Flatten` iterator.

#### Trait Implementations

##### `impl Any for Flatten<I>`

- <span id="flatten-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Flatten<I>`

- <span id="flatten-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Flatten<I>`

- <span id="flatten-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Flatten<I>`

- <span id="flatten-clone"></span>`fn clone(&self) -> Flatten<I>` — [`Flatten`](#flatten)

##### `impl CloneToUninit for Flatten<I>`

- <span id="flatten-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Flatten<I>`

- <span id="flatten-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Flatten<I>`

- <span id="flatten-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Flatten<I>`

- <span id="flatten-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Flatten<I>`

##### `impl IntoParallelIterator for Flatten<I>`

- <span id="flatten-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatten-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatten-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Flatten<I>`

- <span id="flatten-paralleliterator-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoParallelIterator>::Item`

- <span id="flatten-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for Flatten<I>`

- <span id="flatten-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatten-pointable-type-init"></span>`type Init = T`

- <span id="flatten-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatten-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatten-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatten-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Flatten<I>`

- <span id="flatten-toowned-type-owned"></span>`type Owned = T`

- <span id="flatten-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flatten-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Flatten<I>`

- <span id="flatten-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatten-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Flatten<I>`

- <span id="flatten-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatten-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FlattenConsumer<C>`

```rust
struct FlattenConsumer<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:39-41`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flatten.rs#L39-L41)*

#### Implementations

- <span id="flattenconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl Any for FlattenConsumer<C>`

- <span id="flattenconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlattenConsumer<C>`

- <span id="flattenconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlattenConsumer<C>`

- <span id="flattenconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Consumer for FlattenConsumer<C>`

- <span id="flattenconsumer-consumer-type-folder"></span>`type Folder = FlattenFolder<C, <C as Consumer>::Result>`

- <span id="flattenconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flattenconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flattenconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flattenconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flattenconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlattenConsumer<C>`

- <span id="flattenconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlattenConsumer<C>`

- <span id="flattenconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlattenConsumer<C>`

##### `impl Pointable for FlattenConsumer<C>`

- <span id="flattenconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flattenconsumer-pointable-type-init"></span>`type Init = T`

- <span id="flattenconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flattenconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flattenconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flattenconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlattenConsumer<C>`

- <span id="flattenconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flattenconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlattenConsumer<C>`

- <span id="flattenconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flattenconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C> UnindexedConsumer for FlattenConsumer<C>`

- <span id="flattenconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flattenconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FlattenFolder<C, R>`

```rust
struct FlattenFolder<C, R> {
    base: C,
    previous: Option<R>,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten.rs:93-96`](../../../../.source_1765633015/rayon-1.11.0/src/iter/flatten.rs#L93-L96)*

#### Trait Implementations

##### `impl Any for FlattenFolder<C, R>`

- <span id="flattenfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlattenFolder<C, R>`

- <span id="flattenfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlattenFolder<C, R>`

- <span id="flattenfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C> Folder for FlattenFolder<C, <C as >::Result>`

- <span id="flattenfolder-folder-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flattenfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flattenfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="flattenfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FlattenFolder<C, R>`

- <span id="flattenfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FlattenFolder<C, R>`

- <span id="flattenfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlattenFolder<C, R>`

##### `impl Pointable for FlattenFolder<C, R>`

- <span id="flattenfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flattenfolder-pointable-type-init"></span>`type Init = T`

- <span id="flattenfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flattenfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flattenfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flattenfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlattenFolder<C, R>`

- <span id="flattenfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flattenfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlattenFolder<C, R>`

- <span id="flattenfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flattenfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

