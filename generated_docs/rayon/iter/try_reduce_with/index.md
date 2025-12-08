*[rayon](../../index.md) / [iter](../index.md) / [try_reduce_with](index.md)*

---

# Module `try_reduce_with`

## Structs

### `TryReduceWithConsumer<'r, R>`

```rust
struct TryReduceWithConsumer<'r, R> {
    reduce_op: &'r R,
    full: &'r std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'r, R> Clone for TryReduceWithConsumer<'r, R>`

- `fn clone(self: &Self) -> Self`

##### `impl<'r, R, T> Consumer for TryReduceWithConsumer<'r, R>`

- `type Folder = TryReduceWithFolder<'r, R, T>`

- `type Reducer = TryReduceWithConsumer<'r, R>`

- `type Result = Option<T>`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, Self)`

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<'r, R> Copy for TryReduceWithConsumer<'r, R>`

##### `impl<T> IntoEither for TryReduceWithConsumer<'r, R>`

##### `impl<T> Pointable for TryReduceWithConsumer<'r, R>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'r, R, T> Reducer for TryReduceWithConsumer<'r, R>`

- `fn reduce(self: Self, left: Option<T>, right: Option<T>) -> Option<T>`

##### `impl<'r, R, T> UnindexedConsumer for TryReduceWithConsumer<'r, R>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `TryReduceWithFolder<'r, R, T: Try>`

```rust
struct TryReduceWithFolder<'r, R, T: Try> {
    reduce_op: &'r R,
    opt_control: Option<std::ops::ControlFlow<<T as >::Residual, <T as >::Output>>,
    full: &'r std::sync::atomic::AtomicBool,
}
```

#### Trait Implementations

##### `impl<'r, R, T> Folder for TryReduceWithFolder<'r, R, T>`

- `type Result = Option<T>`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> Option<T>`

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for TryReduceWithFolder<'r, R, T>`

##### `impl<T> Pointable for TryReduceWithFolder<'r, R, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `try_reduce_with`

```rust
fn try_reduce_with<PI, R, T>(pi: PI, reduce_op: R) -> Option<T>
where
    PI: ParallelIterator<Item = T>,
    R: Fn(<T as >::Output, <T as >::Output) -> T + Sync,
    T: Try + Send
```

