*[rayon](../../index.md) / [iter](../index.md) / [noop](index.md)*

---

# Module `noop`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoopConsumer`](#noopconsumer) | struct |  |
| [`NoopReducer`](#noopreducer) | struct |  |

## Structs

### `NoopConsumer`

```rust
struct NoopConsumer;
```

*Defined in [`rayon-1.11.0/src/iter/noop.rs:3`](../../../../.source_1765633015/rayon-1.11.0/src/iter/noop.rs#L3)*

#### Trait Implementations

##### `impl Any for NoopConsumer`

- <span id="noopconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoopConsumer`

- <span id="noopconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoopConsumer`

- <span id="noopconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Consumer for NoopConsumer`

- <span id="noopconsumer-consumer-type-folder"></span>`type Folder = NoopConsumer`

- <span id="noopconsumer-consumer-type-reducer"></span>`type Reducer = NoopReducer`

- <span id="noopconsumer-consumer-type-result"></span>`type Result = ()`

- <span id="noopconsumer-consumer-split-at"></span>`fn split_at(self, _index: usize) -> (Self, Self, NoopReducer)` — [`NoopReducer`](#noopreducer)

- <span id="noopconsumer-consumer-into-folder"></span>`fn into_folder(self) -> Self`

- <span id="noopconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> Folder for NoopConsumer`

- <span id="noopconsumer-folder-type-result"></span>`type Result = ()`

- <span id="noopconsumer-folder-consume"></span>`fn consume(self, _item: T) -> Self`

- <span id="noopconsumer-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="noopconsumer-folder-complete"></span>`fn complete(self)`

- <span id="noopconsumer-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for NoopConsumer`

- <span id="noopconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoopConsumer`

- <span id="noopconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for NoopConsumer`

##### `impl Pointable for NoopConsumer`

- <span id="noopconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="noopconsumer-pointable-type-init"></span>`type Init = T`

- <span id="noopconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="noopconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="noopconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="noopconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for NoopConsumer`

- <span id="noopconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="noopconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoopConsumer`

- <span id="noopconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="noopconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> UnindexedConsumer for NoopConsumer`

- <span id="noopconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="noopconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> NoopReducer` — [`NoopReducer`](#noopreducer)

### `NoopReducer`

```rust
struct NoopReducer;
```

*Defined in [`rayon-1.11.0/src/iter/noop.rs:55`](../../../../.source_1765633015/rayon-1.11.0/src/iter/noop.rs#L55)*

#### Trait Implementations

##### `impl Any for NoopReducer`

- <span id="noopreducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoopReducer`

- <span id="noopreducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoopReducer`

- <span id="noopreducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for NoopReducer`

- <span id="noopreducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoopReducer`

- <span id="noopreducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for NoopReducer`

##### `impl Pointable for NoopReducer`

- <span id="noopreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="noopreducer-pointable-type-init"></span>`type Init = T`

- <span id="noopreducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="noopreducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="noopreducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="noopreducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Reducer for NoopReducer`

- <span id="noopreducer-reducer-reduce"></span>`fn reduce(self, _left: (), _right: ())`

##### `impl<U> TryFrom for NoopReducer`

- <span id="noopreducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="noopreducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoopReducer`

- <span id="noopreducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="noopreducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

