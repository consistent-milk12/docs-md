*[rayon](../../../index.md) / [iter](../../index.md) / [collect](../index.md) / [consumer](index.md)*

---

# Module `consumer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CollectConsumer`](#collectconsumer) | struct |  |
| [`CollectResult`](#collectresult) | struct | CollectResult represents an initialized part of the target slice. |
| [`CollectReducer`](#collectreducer) | struct | CollectReducer combines adjacent chunks; the result must always |

## Structs

### `CollectConsumer<'c, T: Send>`

```rust
struct CollectConsumer<'c, T: Send> {
    start: crate::SendPtr<T>,
    len: usize,
    marker: std::marker::PhantomData<&'c mut T>,
}
```

#### Fields

- **`start`**: `crate::SendPtr<T>`

  See `CollectResult` for explanation of why this is not a slice

#### Implementations

- <span id="collectconsumer-new"></span>`unsafe fn new(start: *mut T, len: usize) -> Self`

#### Trait Implementations

##### `impl<'c, T: Send + 'c> Consumer for CollectConsumer<'c, T>`

- <span id="collectconsumer-folder"></span>`type Folder = CollectResult<'c, T>`

- <span id="collectconsumer-reducer"></span>`type Reducer = CollectReducer`

- <span id="collectconsumer-result"></span>`type Result = CollectResult<'c, T>`

- <span id="collectconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, CollectReducer)` — [`CollectReducer`](#collectreducer)

- <span id="collectconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../../plumbing/index.md)

- <span id="collectconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for CollectConsumer<'c, T>`

##### `impl<T> Pointable for CollectConsumer<'c, T>`

- <span id="collectconsumer-align"></span>`const ALIGN: usize`

- <span id="collectconsumer-init"></span>`type Init = T`

- <span id="collectconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'c, T: Send + 'c> UnindexedConsumer for CollectConsumer<'c, T>`

- <span id="collectconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="collectconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../../plumbing/index.md)

### `CollectResult<'c, T>`

```rust
struct CollectResult<'c, T> {
    start: crate::SendPtr<T>,
    total_len: usize,
    initialized_len: usize,
    invariant_lifetime: std::marker::PhantomData<&'c mut &'c mut [T]>,
}
```

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

##### `impl<'c, T> Drop for CollectResult<'c, T>`

- <span id="collectresult-drop"></span>`fn drop(&mut self)`

##### `impl<'c, T: Send + 'c> Folder for CollectResult<'c, T>`

- <span id="collectresult-result"></span>`type Result = CollectResult<'c, T>`

- <span id="collectresult-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="collectresult-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../../plumbing/index.md)

- <span id="collectresult-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for CollectResult<'c, T>`

##### `impl<T> Pointable for CollectResult<'c, T>`

- <span id="collectresult-align"></span>`const ALIGN: usize`

- <span id="collectresult-init"></span>`type Init = T`

- <span id="collectresult-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectresult-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectresult-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectresult-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'c, T> Send for CollectResult<'c, T>`

### `CollectReducer`

```rust
struct CollectReducer;
```

CollectReducer combines adjacent chunks; the result must always
be contiguous so that it is one combined slice.

#### Trait Implementations

##### `impl<T> IntoEither for CollectReducer`

##### `impl<T> Pointable for CollectReducer`

- <span id="collectreducer-align"></span>`const ALIGN: usize`

- <span id="collectreducer-init"></span>`type Init = T`

- <span id="collectreducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="collectreducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="collectreducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="collectreducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'c, T> Reducer for CollectReducer`

- <span id="collectreducer-reduce"></span>`fn reduce(self, left: CollectResult<'c, T>, right: CollectResult<'c, T>) -> CollectResult<'c, T>` — [`CollectResult`](#collectresult)

