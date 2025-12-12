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

#### Trait Implementations

##### `impl<T: Send + 'c> Consumer for CollectConsumer<'c, T>`

- <span id="collectconsumer-consumer-type-folder"></span>`type Folder = CollectResult<'c, T>`

- <span id="collectconsumer-consumer-type-reducer"></span>`type Reducer = CollectReducer`

- <span id="collectconsumer-consumer-type-result"></span>`type Result = CollectResult<'c, T>`

- <span id="collectconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, CollectReducer)` — [`CollectReducer`](#collectreducer)

- <span id="collectconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../../plumbing/index.md#consumer)

- <span id="collectconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for CollectConsumer<'c, T>`

##### `impl<T> Pointable for CollectConsumer<'c, T>`

- <span id="collectconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectconsumer-pointable-type-init"></span>`type Init = T`

- <span id="collectconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Send + 'c> UnindexedConsumer for CollectConsumer<'c, T>`

- <span id="collectconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="collectconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../../plumbing/index.md#consumer)

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

- <span id="collectresult-release-ownership"></span>`fn release_ownership(self) -> usize`

#### Trait Implementations

##### `impl<T> Drop for CollectResult<'c, T>`

- <span id="collectresult-drop"></span>`fn drop(&mut self)`

##### `impl<T: Send + 'c> Folder for CollectResult<'c, T>`

- <span id="collectresult-folder-type-result"></span>`type Result = CollectResult<'c, T>`

- <span id="collectresult-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="collectresult-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../../plumbing/index.md#folder)

- <span id="collectresult-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for CollectResult<'c, T>`

##### `impl<T> Pointable for CollectResult<'c, T>`

- <span id="collectresult-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectresult-pointable-type-init"></span>`type Init = T`

- <span id="collectresult-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectresult-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectresult-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectresult-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for CollectReducer`

- <span id="collectreducer-reduce"></span>`fn reduce(self, left: CollectResult<'c, T>, right: CollectResult<'c, T>) -> CollectResult<'c, T>` — [`CollectResult`](#collectresult)

##### `impl<T> Send for CollectResult<'c, T>`

### `CollectReducer`

```rust
struct CollectReducer;
```

*Defined in [`rayon-1.11.0/src/iter/collect/consumer.rs:166`](../../../../../.source_1765521767/rayon-1.11.0/src/iter/collect/consumer.rs#L166)*

CollectReducer combines adjacent chunks; the result must always
be contiguous so that it is one combined slice.

#### Trait Implementations

##### `impl IntoEither for CollectReducer`

##### `impl Pointable for CollectReducer`

- <span id="collectreducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="collectreducer-pointable-type-init"></span>`type Init = T`

- <span id="collectreducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectreducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectreducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectreducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> Reducer for CollectReducer`

- <span id="collectreducer-reduce"></span>`fn reduce(self, left: CollectResult<'c, T>, right: CollectResult<'c, T>) -> CollectResult<'c, T>` — [`CollectResult`](#collectresult)

