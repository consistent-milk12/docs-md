*[rayon](../../index.md) / [iter](../index.md) / [try_reduce](index.md)*

---

# Module `try_reduce`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TryReduceConsumer`](#tryreduceconsumer) | struct |  |
| [`TryReduceFolder`](#tryreducefolder) | struct |  |
| [`try_reduce`](#try-reduce) | fn |  |

## Structs

### `TryReduceConsumer<'r, R, ID>`

```rust
struct TryReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
    full: &'r std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce.rs:24-28`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_reduce.rs#L24-L28)*

#### Trait Implementations

##### `impl Any for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, ID> Clone for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, ID, T> Consumer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-consumer-type-folder"></span>`type Folder = TryReduceFolder<'r, R, T>`

- <span id="tryreduceconsumer-consumer-type-reducer"></span>`type Reducer = TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-consumer-type-result"></span>`type Result = T`

- <span id="tryreduceconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="tryreduceconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="tryreduceconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<R, ID> Copy for TryReduceConsumer<'r, R, ID>`

##### `impl<T> From for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TryReduceConsumer<'r, R, ID>`

##### `impl Pointable for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryreduceconsumer-pointable-type-init"></span>`type Init = T`

- <span id="tryreduceconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreduceconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreduceconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreduceconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<R, ID, T> Reducer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-reducer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

##### `impl ToOwned for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-toowned-type-owned"></span>`type Owned = T`

- <span id="tryreduceconsumer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryreduceconsumer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryreduceconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryreduceconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<R, ID, T> UnindexedConsumer for TryReduceConsumer<'r, R, ID>`

- <span id="tryreduceconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="tryreduceconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `TryReduceFolder<'r, R, T: Try>`

```rust
struct TryReduceFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    control: std::ops::ControlFlow<<T as >::Residual, <T as >::Output>,
    full: &'r std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce.rs:93-97`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_reduce.rs#L93-L97)*

#### Trait Implementations

##### `impl<T> Any for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, T> Folder for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-folder-type-result"></span>`type Result = T`

- <span id="tryreducefolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="tryreducefolder-folder-complete"></span>`fn complete(self) -> T`

- <span id="tryreducefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for TryReduceFolder<'r, R, T>`

##### `impl<T> Pointable for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tryreducefolder-pointable-type-init"></span>`type Init = T`

- <span id="tryreducefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tryreducefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tryreducefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tryreducefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryreducefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TryReduceFolder<'r, R, T>`

- <span id="tryreducefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryreducefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `try_reduce`

```rust
fn try_reduce<PI, R, ID, T>(pi: PI, identity: ID, reduce_op: R) -> T
where
    PI: ParallelIterator<Item = T>,
    R: Fn(<T as >::Output, <T as >::Output) -> T + Sync,
    ID: Fn() -> <T as >::Output + Sync,
    T: Try + Send
```

*Defined in [`rayon-1.11.0/src/iter/try_reduce.rs:8-22`](../../../../.source_1765633015/rayon-1.11.0/src/iter/try_reduce.rs#L8-L22)*

