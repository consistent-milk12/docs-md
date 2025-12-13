*[rayon](../../../index.md) / [iter](../../index.md) / [collect](../index.md) / [consumer](index.md)*

---

# Module `consumer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CollectConsumer`](#collectconsumer) | struct |  |
| [`CollectResult`](#collectresult) | struct | CollectResult represents an initialized part of the target slice. |
| [`CollectReducer`](#collectreducer) | struct | CollectReducer combines adjacent chunks; the result must always be contiguous so that it is one combined slice. |

## Structs

### `CollectConsumer<'c, T: Send>`

```rust
struct CollectConsumer<'c, T: Send> {
    start: crate::SendPtr<T>,
    len: usize,
    marker: std::marker::PhantomData<&'c mut T>,
}
```

*Defined in [`rayon-1.11.0/src/iter/collect/consumer.rs:7-12`](../../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/consumer.rs#L7-L12)*

#### Fields

- **`start`**: `crate::SendPtr<T>`

  See `CollectResult` for explanation of why this is not a slice

#### Implementations

- <span id="collectconsumer-appender"></span>`fn appender(vec: &mut Vec<T>, len: usize) -> CollectConsumer<'_, T>` — [`CollectConsumer`](#collectconsumer)

  Create a collector for `len` items in the unused capacity of the vector.

#### Trait Implementations

##### `impl<T> Any for CollectConsumer<'c, T>`

- <span id="collectconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CollectConsumer<'c, T>`

- <span id="collectconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CollectConsumer<'c, T>`

- <span id="collectconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Send + 'c> Consumer for CollectConsumer<'c, T>`

- <span id="collectconsumer-consumer-type-folder"></span>`type Folder = CollectResult<'c, T>`

- <span id="collectconsumer-consumer-type-reducer"></span>`type Reducer = CollectReducer`

- <span id="collectconsumer-consumer-type-result"></span>`type Result = CollectResult<'c, T>`

- <span id="collectconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, CollectReducer)` — [`CollectReducer`](#collectreducer)

- <span id="collectconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../../plumbing/index.md#consumer)

- <span id="collectconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for CollectConsumer<'c, T>`

- <span id="collectconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for CollectConsumer<'c, T>`

- <span id="collectconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for CollectConsumer<'c, T>`

##### `impl<T> Pointable for CollectConsumer<'c, T>`

- <span id="collectconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectconsumer-pointable-type-init"></span>`type Init = T`

- <span id="collectconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for CollectConsumer<'c, T>`

- <span id="collectconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collectconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CollectConsumer<'c, T>`

- <span id="collectconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collectconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T: Send + 'c> UnindexedConsumer for CollectConsumer<'c, T>`

- <span id="collectconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="collectconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../../plumbing/index.md#consumer)

### `CollectResult<'c, T>`

```rust
struct CollectResult<'c, T> {
    start: crate::SendPtr<T>,
    total_len: usize,
    initialized_len: usize,
    invariant_lifetime: std::marker::PhantomData<&'c mut &'c mut [T]>,
}
```

*Defined in [`rayon-1.11.0/src/iter/collect/consumer.rs:44-55`](../../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/consumer.rs#L44-L55)*

CollectResult represents an initialized part of the target slice.

This is a proxy owner of the elements in the slice; when it drops,
the elements will be dropped, unless its ownership is released before then.

#### Fields

- **`start`**: `crate::SendPtr<T>`

  This pointer and length has the same representation as a slice,
  but retains the provenance of the entire array so that we can merge
  these regions together in `CollectReducer`.

- **`initialized_len`**: `usize`

  The current initialized length after `start`

- **`invariant_lifetime`**: `std::marker::PhantomData<&'c mut &'c mut [T]>`

  Lifetime invariance guarantees that the data flows from consumer to result,
  especially for the `scope_fn` callback in `Collect::with_consumer`.

#### Implementations

- <span id="collectresult-len"></span>`fn len(&self) -> usize`

  The current length of the collect result

- <span id="collectresult-release-ownership"></span>`fn release_ownership(self) -> usize`

  Release ownership of the slice of elements, and return the length

#### Trait Implementations

##### `impl<T> Any for CollectResult<'c, T>`

- <span id="collectresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CollectResult<'c, T>`

- <span id="collectresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CollectResult<'c, T>`

- <span id="collectresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Drop for CollectResult<'c, T>`

- <span id="collectresult-drop"></span>`fn drop(&mut self)`

##### `impl<T: Send + 'c> Folder for CollectResult<'c, T>`

- <span id="collectresult-folder-type-result"></span>`type Result = CollectResult<'c, T>`

- <span id="collectresult-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="collectresult-folder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../../plumbing/index.md#folder)

- <span id="collectresult-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for CollectResult<'c, T>`

- <span id="collectresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for CollectResult<'c, T>`

- <span id="collectresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for CollectResult<'c, T>`

##### `impl<T> Pointable for CollectResult<'c, T>`

- <span id="collectresult-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectresult-pointable-type-init"></span>`type Init = T`

- <span id="collectresult-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectresult-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectresult-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectresult-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for CollectReducer`

- <span id="collectreducer-reducer-reduce"></span>`fn reduce(self, left: CollectResult<'c, T>, right: CollectResult<'c, T>) -> CollectResult<'c, T>` — [`CollectResult`](#collectresult)

##### `impl<T> Send for CollectResult<'c, T>`

##### `impl<T, U> TryFrom for CollectResult<'c, T>`

- <span id="collectresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collectresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CollectResult<'c, T>`

- <span id="collectresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collectresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CollectReducer`

```rust
struct CollectReducer;
```

*Defined in [`rayon-1.11.0/src/iter/collect/consumer.rs:166`](../../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/consumer.rs#L166)*

CollectReducer combines adjacent chunks; the result must always
be contiguous so that it is one combined slice.

#### Trait Implementations

##### `impl Any for CollectReducer`

- <span id="collectreducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CollectReducer`

- <span id="collectreducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CollectReducer`

- <span id="collectreducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for CollectReducer`

- <span id="collectreducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CollectReducer`

- <span id="collectreducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CollectReducer`

##### `impl Pointable for CollectReducer`

- <span id="collectreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectreducer-pointable-type-init"></span>`type Init = T`

- <span id="collectreducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectreducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectreducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectreducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for CollectReducer`

- <span id="collectreducer-reducer-reduce"></span>`fn reduce(self, left: CollectResult<'c, T>, right: CollectResult<'c, T>) -> CollectResult<'c, T>` — [`CollectResult`](#collectresult)

##### `impl<U> TryFrom for CollectReducer`

- <span id="collectreducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="collectreducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CollectReducer`

- <span id="collectreducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="collectreducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

