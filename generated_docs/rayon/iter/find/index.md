*[rayon](../../index.md) / [iter](../index.md) / [find](index.md)*

---

# Module `find`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FindConsumer`](#findconsumer) | struct |  |
| [`FindFolder`](#findfolder) | struct |  |
| [`FindReducer`](#findreducer) | struct |  |
| [`find`](#find) | fn |  |

## Structs

### `FindConsumer<'p, P>`

```rust
struct FindConsumer<'p, P> {
    find_op: &'p P,
    found: &'p std::sync::atomic::AtomicBool,
}
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:15-18`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L15-L18)*

#### Implementations

- <span id="findconsumer-new"></span>`fn new(find_op: &'p P, found: &'p AtomicBool) -> Self`

#### Trait Implementations

##### `impl Any for FindConsumer<'p, P>`

- <span id="findconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindConsumer<'p, P>`

- <span id="findconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindConsumer<'p, P>`

- <span id="findconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P> Consumer for FindConsumer<'p, P>`

- <span id="findconsumer-consumer-type-folder"></span>`type Folder = FindFolder<'p, T, P>`

- <span id="findconsumer-consumer-type-reducer"></span>`type Reducer = FindReducer`

- <span id="findconsumer-consumer-type-result"></span>`type Result = Option<T>`

- <span id="findconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="findconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="findconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FindConsumer<'p, P>`

- <span id="findconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindConsumer<'p, P>`

- <span id="findconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FindConsumer<'p, P>`

##### `impl Pointable for FindConsumer<'p, P>`

- <span id="findconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findconsumer-pointable-type-init"></span>`type Init = T`

- <span id="findconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FindConsumer<'p, P>`

- <span id="findconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindConsumer<'p, P>`

- <span id="findconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, P> UnindexedConsumer for FindConsumer<'p, P>`

- <span id="findconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="findconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FindFolder<'p, T, P>`

```rust
struct FindFolder<'p, T, P> {
    find_op: &'p P,
    found: &'p std::sync::atomic::AtomicBool,
    item: Option<T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:66-70`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L66-L70)*

#### Trait Implementations

##### `impl<T> Any for FindFolder<'p, T, P>`

- <span id="findfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindFolder<'p, T, P>`

- <span id="findfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindFolder<'p, T, P>`

- <span id="findfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P> Folder for FindFolder<'p, T, P>`

- <span id="findfolder-folder-type-result"></span>`type Result = Option<T>`

- <span id="findfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="findfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="findfolder-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="findfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for FindFolder<'p, T, P>`

- <span id="findfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for FindFolder<'p, T, P>`

- <span id="findfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for FindFolder<'p, T, P>`

##### `impl<T> Pointable for FindFolder<'p, T, P>`

- <span id="findfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findfolder-pointable-type-init"></span>`type Init = T`

- <span id="findfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for FindFolder<'p, T, P>`

- <span id="findfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FindFolder<'p, T, P>`

- <span id="findfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FindReducer`

```rust
struct FindReducer;
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:114`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L114)*

#### Trait Implementations

##### `impl Any for FindReducer`

- <span id="findreducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindReducer`

- <span id="findreducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindReducer`

- <span id="findreducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FindReducer`

- <span id="findreducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindReducer`

- <span id="findreducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FindReducer`

##### `impl Pointable for FindReducer`

- <span id="findreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="findreducer-pointable-type-init"></span>`type Init = T`

- <span id="findreducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="findreducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="findreducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="findreducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for FindReducer`

- <span id="findreducer-reducer-reduce"></span>`fn reduce(self, left: Option<T>, right: Option<T>) -> Option<T>`

##### `impl<U> TryFrom for FindReducer`

- <span id="findreducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findreducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindReducer`

- <span id="findreducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findreducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `find`

```rust
fn find<I, P>(pi: I, find_op: P) -> Option<<I as >::Item>
where
    I: ParallelIterator,
    P: Fn(&<I as >::Item) -> bool + Sync
```

*Defined in [`rayon-1.11.0/src/iter/find.rs:5-13`](../../../../.source_1765521767/rayon-1.11.0/src/iter/find.rs#L5-L13)*

