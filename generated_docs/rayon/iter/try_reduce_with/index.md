*[rayon](../../index.md) / [iter](../index.md) / [try_reduce_with](index.md)*

---

# Module `try_reduce_with`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReduceWithConsumer`](#tryreducewithconsumer) | struct |  |
| [`TryReduceWithFolder`](#tryreducewithfolder) | struct |  |
| [`try_reduce_with`](#try-reduce-with) | fn |  |

## Structs

### `TryReduceWithConsumer<'r, R>`

```rust
struct TryReduceWithConsumer<'r, R> {
    reduce_op: &'r R,
    full: &'r std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce_with.rs:22-25`](../../../../.source_1765521767/rayon-1.11.0/src/iter/try_reduce_with.rs#L22-L25)*

#### Trait Implementations

##### `impl Any for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R> Clone for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, T> Consumer for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-consumer-type-folder"></span>`type Folder = TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithconsumer-consumer-type-reducer"></span>`type Reducer = TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-consumer-type-result"></span>`type Result = Option<T>`

- <span id="tryreducewithconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="tryreducewithconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="tryreducewithconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<R> Copy for TryReduceWithConsumer<'r, R>`

##### `impl<T> From for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryReduceWithConsumer<'r, R>`

##### `impl Pointable for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryreducewithconsumer-pointable-type-init"></span>`type Init = T`

- <span id="tryreducewithconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreducewithconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreducewithconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreducewithconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<R, T> Reducer for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-reducer-reduce"></span>`fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T>`

##### `impl ToOwned for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-toowned-type-owned"></span>`type Owned = T`

- <span id="tryreducewithconsumer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryreducewithconsumer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryreducewithconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryreducewithconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<R, T> UnindexedConsumer for TryReduceWithConsumer<'r, R>`

- <span id="tryreducewithconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryreducewithconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `TryReduceWithFolder<'r, R, T: Try>`

```rust
struct TryReduceWithFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    opt_control: Option<std::ops::ControlFlow<<T as >::Residual, <T as >::Output>>,
    full: &'r std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce_with.rs:92-96`](../../../../.source_1765521767/rayon-1.11.0/src/iter/try_reduce_with.rs#L92-L96)*

#### Trait Implementations

##### `impl<T> Any for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, T> Folder for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-folder-type-result"></span>`type Result = Option<T>`

- <span id="tryreducewithfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="tryreducewithfolder-folder-complete"></span>`fn complete(self) -> Option<T>`

- <span id="tryreducewithfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for TryReduceWithFolder<'r, R, T>`

##### `impl<T> Pointable for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryreducewithfolder-pointable-type-init"></span>`type Init = T`

- <span id="tryreducewithfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreducewithfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreducewithfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreducewithfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryreducewithfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TryReduceWithFolder<'r, R, T>`

- <span id="tryreducewithfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryreducewithfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `try_reduce_with`

```rust
fn try_reduce_with<PI, R, T>(pi: PI, reduce_op: R) -> Option<T>
where
    PI: ParallelIterator<Item = T>,
    R: Fn(<T as >::Output, <T as >::Output) -> T + Sync,
    T: Try + Send
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce_with.rs:8-20`](../../../../.source_1765521767/rayon-1.11.0/src/iter/try_reduce_with.rs#L8-L20)*

