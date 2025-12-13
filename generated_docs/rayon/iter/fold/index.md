*[rayon](../../index.md) / [iter](../index.md) / [fold](index.md)*

---

# Module `fold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Fold`](#fold) | struct | `Fold` is an iterator that applies a function over an iterator producing a single value. |
| [`FoldConsumer`](#foldconsumer) | struct |  |
| [`FoldFolder`](#foldfolder) | struct |  |
| [`FoldWith`](#foldwith) | struct | `FoldWith` is an iterator that applies a function over an iterator producing a single value. |
| [`FoldWithConsumer`](#foldwithconsumer) | struct |  |

## Structs

### `Fold<I, ID, F>`

```rust
struct Fold<I, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:22-26`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L22-L26)*

`Fold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="fold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl Any for Fold<I, ID, F>`

- <span id="fold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Fold<I, ID, F>`

- <span id="fold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Fold<I, ID, F>`

- <span id="fold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for Fold<I, ID, F>`

- <span id="fold-clone"></span>`fn clone(&self) -> Fold<I, ID, F>` — [`Fold`](#fold)

##### `impl CloneToUninit for Fold<I, ID, F>`

- <span id="fold-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, ID, F> Debug for Fold<I, ID, F>`

- <span id="fold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Fold<I, ID, F>`

- <span id="fold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Fold<I, ID, F>`

- <span id="fold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Fold<I, ID, F>`

##### `impl IntoParallelIterator for Fold<I, ID, F>`

- <span id="fold-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="fold-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="fold-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, ID, F> ParallelIterator for Fold<I, ID, F>`

- <span id="fold-paralleliterator-type-item"></span>`type Item = U`

- <span id="fold-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for Fold<I, ID, F>`

- <span id="fold-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fold-pointable-type-init"></span>`type Init = T`

- <span id="fold-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fold-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fold-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fold-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Fold<I, ID, F>`

- <span id="fold-toowned-type-owned"></span>`type Owned = T`

- <span id="fold-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fold-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Fold<I, ID, F>`

- <span id="fold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Fold<I, ID, F>`

- <span id="fold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FoldConsumer<'c, C, ID, F>`

```rust
struct FoldConsumer<'c, C, ID, F> {
    base: C,
    fold_op: &'c F,
    identity: &'c ID,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:56-60`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L56-L60)*

#### Trait Implementations

##### `impl Any for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, ID, F> Consumer for FoldConsumer<'r, C, ID, F>`

- <span id="foldconsumer-consumer-type-folder"></span>`type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="foldconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="foldconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="foldconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldConsumer<'c, C, ID, F>`

##### `impl Pointable for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldconsumer-pointable-type-init"></span>`type Init = T`

- <span id="foldconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldConsumer<'c, C, ID, F>`

- <span id="foldconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, ID, F> UnindexedConsumer for FoldConsumer<'r, C, ID, F>`

- <span id="foldconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foldconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FoldFolder<'r, C, ID, F>`

```rust
struct FoldFolder<'r, C, ID, F> {
    base: C,
    fold_op: &'r F,
    item: ID,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:117-121`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L117-L121)*

#### Trait Implementations

##### `impl Any for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<C, ID, F, T> Folder for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="foldfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="foldfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="foldfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="foldfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldFolder<'r, C, ID, F>`

##### `impl Pointable for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldfolder-pointable-type-init"></span>`type Init = T`

- <span id="foldfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldFolder<'r, C, ID, F>`

- <span id="foldfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FoldWith<I, U, F>`

```rust
struct FoldWith<I, U, F> {
    base: I,
    item: U,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:191-195`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L191-L195)*

`FoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `fold_with()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="foldwith-new"></span>`fn new(base: I, item: U, fold_op: F) -> Self`

#### Trait Implementations

##### `impl Any for FoldWith<I, U, F>`

- <span id="foldwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldWith<I, U, F>`

- <span id="foldwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldWith<I, U, F>`

- <span id="foldwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone, F: clone::Clone> Clone for FoldWith<I, U, F>`

- <span id="foldwith-clone"></span>`fn clone(&self) -> FoldWith<I, U, F>` — [`FoldWith`](#foldwith)

##### `impl CloneToUninit for FoldWith<I, U, F>`

- <span id="foldwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, U: Debug, F> Debug for FoldWith<I, U, F>`

- <span id="foldwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FoldWith<I, U, F>`

- <span id="foldwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FoldWith<I, U, F>`

- <span id="foldwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldWith<I, U, F>`

##### `impl IntoParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="foldwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="foldwith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for FoldWith<I, U, F>`

- <span id="foldwith-paralleliterator-type-item"></span>`type Item = U`

- <span id="foldwith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FoldWith<I, U, F>`

- <span id="foldwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldwith-pointable-type-init"></span>`type Init = T`

- <span id="foldwith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldwith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldwith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldwith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for FoldWith<I, U, F>`

- <span id="foldwith-toowned-type-owned"></span>`type Owned = T`

- <span id="foldwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foldwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FoldWith<I, U, F>`

- <span id="foldwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldWith<I, U, F>`

- <span id="foldwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FoldWithConsumer<'c, C, U, F>`

```rust
struct FoldWithConsumer<'c, C, U, F> {
    base: C,
    item: U,
    fold_op: &'c F,
}
```

*Defined in [`rayon-1.11.0/src/iter/fold.rs:227-231`](../../../../.source_1765521767/rayon-1.11.0/src/iter/fold.rs#L227-L231)*

#### Trait Implementations

##### `impl Any for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<U, T, C, F> Consumer for FoldWithConsumer<'r, C, U, F>`

- <span id="foldwithconsumer-consumer-type-folder"></span>`type Folder = FoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="foldwithconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="foldwithconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="foldwithconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldwithconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="foldwithconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FoldWithConsumer<'c, C, U, F>`

##### `impl Pointable for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="foldwithconsumer-pointable-type-init"></span>`type Init = T`

- <span id="foldwithconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="foldwithconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="foldwithconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="foldwithconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldwithconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldWithConsumer<'c, C, U, F>`

- <span id="foldwithconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldwithconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<U, T, C, F> UnindexedConsumer for FoldWithConsumer<'r, C, U, F>`

- <span id="foldwithconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="foldwithconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

