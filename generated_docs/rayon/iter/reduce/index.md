*[rayon](../../index.md) / [iter](../index.md) / [reduce](index.md)*

---

# Module `reduce`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReduceConsumer`](#reduceconsumer) | struct |  |
| [`ReduceFolder`](#reducefolder) | struct |  |
| [`reduce`](#reduce) | fn |  |

## Structs

### `ReduceConsumer<'r, R, ID>`

```rust
struct ReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
}
```

*Defined in [`rayon-1.11.0/src/iter/reduce.rs:18-21`](../../../../.source_1765521767/rayon-1.11.0/src/iter/reduce.rs#L18-L21)*

#### Trait Implementations

##### `impl Any for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, ID> Clone for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<R, ID, T> Consumer for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-consumer-type-folder"></span>`type Folder = ReduceFolder<'r, R, T>`

- <span id="reduceconsumer-consumer-type-reducer"></span>`type Reducer = ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-consumer-type-result"></span>`type Result = T`

- <span id="reduceconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, Self)`

- <span id="reduceconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="reduceconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<R, ID> Copy for ReduceConsumer<'r, R, ID>`

##### `impl<T> From for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ReduceConsumer<'r, R, ID>`

##### `impl Pointable for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="reduceconsumer-pointable-type-init"></span>`type Init = T`

- <span id="reduceconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="reduceconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="reduceconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="reduceconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<R, ID, T> Reducer for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-reducer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

##### `impl ToOwned for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-toowned-type-owned"></span>`type Owned = T`

- <span id="reduceconsumer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="reduceconsumer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reduceconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reduceconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<R, ID, T> UnindexedConsumer for ReduceConsumer<'r, R, ID>`

- <span id="reduceconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="reduceconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `ReduceFolder<'r, R, T>`

```rust
struct ReduceFolder<'r, R, T> {
    reduce_op: &'r R,
    item: T,
}
```

*Defined in [`rayon-1.11.0/src/iter/reduce.rs:81-84`](../../../../.source_1765521767/rayon-1.11.0/src/iter/reduce.rs#L81-L84)*

#### Trait Implementations

##### `impl<T> Any for ReduceFolder<'r, R, T>`

- <span id="reducefolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReduceFolder<'r, R, T>`

- <span id="reducefolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReduceFolder<'r, R, T>`

- <span id="reducefolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R, T> Folder for ReduceFolder<'r, R, T>`

- <span id="reducefolder-folder-type-result"></span>`type Result = T`

- <span id="reducefolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="reducefolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="reducefolder-folder-complete"></span>`fn complete(self) -> T`

- <span id="reducefolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for ReduceFolder<'r, R, T>`

- <span id="reducefolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ReduceFolder<'r, R, T>`

- <span id="reducefolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ReduceFolder<'r, R, T>`

##### `impl<T> Pointable for ReduceFolder<'r, R, T>`

- <span id="reducefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="reducefolder-pointable-type-init"></span>`type Init = T`

- <span id="reducefolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="reducefolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="reducefolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="reducefolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ReduceFolder<'r, R, T>`

- <span id="reducefolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reducefolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ReduceFolder<'r, R, T>`

- <span id="reducefolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reducefolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `reduce`

```rust
fn reduce<PI, R, ID, T>(pi: PI, identity: ID, reduce_op: R) -> T
where
    PI: ParallelIterator<Item = T>,
    R: Fn(T, T) -> T + Sync,
    ID: Fn() -> T + Sync,
    T: Send
```

*Defined in [`rayon-1.11.0/src/iter/reduce.rs:4-16`](../../../../.source_1765521767/rayon-1.11.0/src/iter/reduce.rs#L4-L16)*

