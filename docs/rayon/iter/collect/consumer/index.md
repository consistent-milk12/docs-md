*[rayon](../../../index.md) / [iter](../../index.md) / [collect](../index.md) / [consumer](index.md)*

---

# Module `consumer`

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

- `unsafe fn new(start: *mut T, len: usize) -> Self`

#### Trait Implementations

##### `impl<'c, T: Send + 'c> Consumer for CollectConsumer<'c, T>`

- `type Folder = CollectResult<'c, T>`

- `type Reducer = CollectReducer`

- `type Result = CollectResult<'c, T>`

- `fn split_at(self: Self, index: usize) -> (Self, Self, CollectReducer)` — [`CollectReducer`](#collectreducer)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for CollectConsumer<'c, T>`

##### `impl<T> Pointable for CollectConsumer<'c, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'c, T: Send + 'c> UnindexedConsumer for CollectConsumer<'c, T>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../../plumbing/index.md)

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

- `fn len(self: &Self) -> usize`

- `fn release_ownership(self: Self) -> usize`

#### Trait Implementations

##### `impl<'c, T> Drop for CollectResult<'c, T>`

- `fn drop(self: &mut Self)`

##### `impl<'c, T: Send + 'c> Folder for CollectResult<'c, T>`

- `type Result = CollectResult<'c, T>`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for CollectResult<'c, T>`

##### `impl<T> Pointable for CollectResult<'c, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'c, T> Reducer for CollectReducer`

- `fn reduce(self: Self, left: CollectResult<'c, T>, right: CollectResult<'c, T>) -> CollectResult<'c, T>` — [`CollectResult`](#collectresult)

