*[rayon](../../index.md) / [iter](../index.md) / [try_reduce](index.md)*

---

# Module `try_reduce`

## Structs

### `TryReduceConsumer<'r, R, ID>`

```rust
struct TryReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
    full: &'r std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'r, R, ID> Clone for TryReduceConsumer<'r, R, ID>`

- `fn clone(self: &Self) -> Self`

##### `impl<'r, R, ID, T> Consumer for TryReduceConsumer<'r, R, ID>`

- `type Folder = TryReduceFolder<'r, R, T>`

- `type Reducer = TryReduceConsumer<'r, R, ID>`

- `type Result = T`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, Self)`

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<'r, R, ID> Copy for TryReduceConsumer<'r, R, ID>`

##### `impl<T> IntoEither for TryReduceConsumer<'r, R, ID>`

##### `impl<T> Pointable for TryReduceConsumer<'r, R, ID>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'r, R, ID, T> Reducer for TryReduceConsumer<'r, R, ID>`

- `fn reduce(self: Self, left: T, right: T) -> T`

##### `impl<'r, R, ID, T> UnindexedConsumer for TryReduceConsumer<'r, R, ID>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `TryReduceFolder<'r, R, T: Try>`

```rust
struct TryReduceFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    control: std::ops::ControlFlow<<T as >::Residual, <T as >::Output>,
    full: &'r std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'r, R, T> Folder for TryReduceFolder<'r, R, T>`

- `type Result = T`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> T`

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for TryReduceFolder<'r, R, T>`

##### `impl<T> Pointable for TryReduceFolder<'r, R, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

