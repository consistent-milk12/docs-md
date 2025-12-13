*[rayon](../../index.md) / [iter](../index.md) / [try_fold](index.md)*

---

# Module `try_fold`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryFold`](#tryfold) | struct | `TryFold` is an iterator that applies a function over an iterator producing a single value. |
| [`TryFoldConsumer`](#tryfoldconsumer) | struct |  |
| [`TryFoldFolder`](#tryfoldfolder) | struct |  |
| [`TryFoldWith`](#tryfoldwith) | struct | `TryFoldWith` is an iterator that applies a function over an iterator producing a single value. |
| [`TryFoldWithConsumer`](#tryfoldwithconsumer) | struct |  |

## Structs

### `TryFold<I, U, ID, F>`

```rust
struct TryFold<I, U, ID, F> {
    base: I,
    identity: ID,
    fold_op: F,
    marker: std::marker::PhantomData<U>,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:26-31`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_fold.rs#L26-L31)*

`TryFold` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="tryfold-new"></span>`fn new(base: I, identity: ID, fold_op: F) -> Self`

#### Trait Implementations

##### `impl Any for TryFold<I, U, ID, F>`

- <span id="tryfold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFold<I, U, ID, F>`

- <span id="tryfold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFold<I, U, ID, F>`

- <span id="tryfold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone, ID: clone::Clone, F: clone::Clone> Clone for TryFold<I, U, ID, F>`

- <span id="tryfold-clone"></span>`fn clone(&self) -> TryFold<I, U, ID, F>` — [`TryFold`](#tryfold)

##### `impl CloneToUninit for TryFold<I, U, ID, F>`

- <span id="tryfold-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<U, I: ParallelIterator + Debug, ID, F> Debug for TryFold<I, U, ID, F>`

- <span id="tryfold-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryFold<I, U, ID, F>`

- <span id="tryfold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFold<I, U, ID, F>`

- <span id="tryfold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryFold<I, U, ID, F>`

##### `impl IntoParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="tryfold-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfold-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, ID, F> ParallelIterator for TryFold<I, U, ID, F>`

- <span id="tryfold-paralleliterator-type-item"></span>`type Item = U`

- <span id="tryfold-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for TryFold<I, U, ID, F>`

- <span id="tryfold-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryfold-pointable-type-init"></span>`type Init = T`

- <span id="tryfold-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfold-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfold-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfold-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TryFold<I, U, ID, F>`

- <span id="tryfold-toowned-type-owned"></span>`type Owned = T`

- <span id="tryfold-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryfold-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryFold<I, U, ID, F>`

- <span id="tryfold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFold<I, U, ID, F>`

- <span id="tryfold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryFoldConsumer<'c, U, C, ID, F>`

```rust
struct TryFoldConsumer<'c, U, C, ID, F> {
    base: C,
    identity: &'c ID,
    fold_op: &'c F,
    marker: std::marker::PhantomData<U>,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:62-67`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_fold.rs#L62-L67)*

#### Trait Implementations

##### `impl Any for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<U, T, C, ID, F> Consumer for TryFoldConsumer<'r, U, C, ID, F>`

- <span id="tryfoldconsumer-consumer-type-folder"></span>`type Folder = TryFoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="tryfoldconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="tryfoldconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="tryfoldconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="tryfoldconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="tryfoldconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryFoldConsumer<'c, U, C, ID, F>`

##### `impl Pointable for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryfoldconsumer-pointable-type-init"></span>`type Init = T`

- <span id="tryfoldconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfoldconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFoldConsumer<'c, U, C, ID, F>`

- <span id="tryfoldconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfoldconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<U, T, C, ID, F> UnindexedConsumer for TryFoldConsumer<'r, U, C, ID, F>`

- <span id="tryfoldconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryfoldconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `TryFoldFolder<'r, C, U: Try, F>`

```rust
struct TryFoldFolder<'r, C, U: Try, F> {
    base: C,
    fold_op: &'r F,
    control: std::ops::ControlFlow<<U as >::Residual, <U as >::Output>,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:124-128`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_fold.rs#L124-L128)*

#### Trait Implementations

##### `impl Any for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<C, U, F, T> Folder for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="tryfoldfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="tryfoldfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="tryfoldfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryFoldFolder<'r, C, U, F>`

##### `impl Pointable for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryfoldfolder-pointable-type-init"></span>`type Init = T`

- <span id="tryfoldfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfoldfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFoldFolder<'r, C, U, F>`

- <span id="tryfoldfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfoldfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryFoldWith<I, U: Try, F>`

```rust
struct TryFoldWith<I, U: Try, F> {
    base: I,
    item: <U as >::Output,
    fold_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:180-184`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_fold.rs#L180-L184)*

`TryFoldWith` is an iterator that applies a function over an iterator producing a single value.
This struct is created by the `try_fold_with()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="tryfoldwith-new"></span>`fn new(base: I, item: <U as >::Output, fold_op: F) -> Self` — [`Try`](../private/index.md#try)

#### Trait Implementations

##### `impl Any for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, U: clone::Clone + Try, F: clone::Clone> Clone for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-clone"></span>`fn clone(&self) -> TryFoldWith<I, U, F>` — [`TryFoldWith`](#tryfoldwith)

##### `impl CloneToUninit for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, U, F> Debug for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryFoldWith<I, U, F>`

##### `impl IntoParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="tryfoldwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="tryfoldwith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<U, I, F> ParallelIterator for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-paralleliterator-type-item"></span>`type Item = U`

- <span id="tryfoldwith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryfoldwith-pointable-type-init"></span>`type Init = T`

- <span id="tryfoldwith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldwith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldwith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldwith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-toowned-type-owned"></span>`type Owned = T`

- <span id="tryfoldwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryfoldwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfoldwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFoldWith<I, U, F>`

- <span id="tryfoldwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfoldwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryFoldWithConsumer<'c, C, U: Try, F>`

```rust
struct TryFoldWithConsumer<'c, C, U: Try, F> {
    base: C,
    item: <U as >::Output,
    fold_op: &'c F,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_fold.rs:220-224`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_fold.rs#L220-L224)*

#### Trait Implementations

##### `impl Any for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<U, T, C, F> Consumer for TryFoldWithConsumer<'r, C, U, F>`

- <span id="tryfoldwithconsumer-consumer-type-folder"></span>`type Folder = TryFoldFolder<'r, <C as Consumer>::Folder, U, F>`

- <span id="tryfoldwithconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="tryfoldwithconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="tryfoldwithconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="tryfoldwithconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="tryfoldwithconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryFoldWithConsumer<'c, C, U, F>`

##### `impl Pointable for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryfoldwithconsumer-pointable-type-init"></span>`type Init = T`

- <span id="tryfoldwithconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryfoldwithconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryfoldwithconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryfoldwithconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfoldwithconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFoldWithConsumer<'c, C, U, F>`

- <span id="tryfoldwithconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfoldwithconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<U, T, C, F> UnindexedConsumer for TryFoldWithConsumer<'r, C, U, F>`

- <span id="tryfoldwithconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryfoldwithconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

