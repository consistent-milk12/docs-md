*[rayon](../../index.md) / [iter](../index.md) / [reduce](index.md)*

---

# Module `reduce`

## Structs

### `ReduceConsumer<'r, R, ID>`

```rust
struct ReduceConsumer<'r, R, ID> {
    identity: &'r ID,
    reduce_op: &'r R,
}
```

#### Trait Implementations

##### `impl<'r, R, ID> Clone for ReduceConsumer<'r, R, ID>`

- `fn clone(self: &Self) -> Self`

##### `impl<'r, R, ID, T> Consumer for ReduceConsumer<'r, R, ID>`

- `type Folder = ReduceFolder<'r, R, T>`

- `type Reducer = ReduceConsumer<'r, R, ID>`

- `type Result = T`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, Self)`

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<'r, R, ID> Copy for ReduceConsumer<'r, R, ID>`

##### `impl<T> IntoEither for ReduceConsumer<'r, R, ID>`

##### `impl<T> Pointable for ReduceConsumer<'r, R, ID>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'r, R, ID, T> Reducer for ReduceConsumer<'r, R, ID>`

- `fn reduce(self: Self, left: T, right: T) -> T`

##### `impl<'r, R, ID, T> UnindexedConsumer for ReduceConsumer<'r, R, ID>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `ReduceFolder<'r, R, T>`

```rust
struct ReduceFolder<'r, R, T> {
    reduce_op: &'r R,
    item: T,
}
```

#### Trait Implementations

##### `impl<'r, R, T> Folder for ReduceFolder<'r, R, T>`

- `type Result = T`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> T`

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for ReduceFolder<'r, R, T>`

##### `impl<T> Pointable for ReduceFolder<'r, R, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

