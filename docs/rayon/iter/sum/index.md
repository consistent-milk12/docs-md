*[rayon](../../index.md) / [iter](../index.md) / [sum](index.md)*

---

# Module `sum`

## Structs

### `SumConsumer<S: Send>`

```rust
struct SumConsumer<S: Send> {
    _marker: std::marker::PhantomData<*const S>,
}
```

#### Implementations

- `fn new() -> SumConsumer<S>` — [`SumConsumer`](#sumconsumer)

#### Trait Implementations

##### `impl<S, T> Consumer for SumConsumer<S>`

- `type Folder = SumFolder<S>`

- `type Reducer = SumConsumer<S>`

- `type Result = S`

- `fn split_at(self: Self, _index: usize) -> (Self, Self, Self)`

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for SumConsumer<S>`

##### `impl<T> Pointable for SumConsumer<S>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<S> Reducer for SumConsumer<S>`

- `fn reduce(self: Self, left: S, right: S) -> S`

##### `impl<S: Send> Send for SumConsumer<S>`

##### `impl<S, T> UnindexedConsumer for SumConsumer<S>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `SumFolder<S>`

```rust
struct SumFolder<S> {
    sum: S,
}
```

#### Trait Implementations

##### `impl<S, T> Folder for SumFolder<S>`

- `type Result = S`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> S`

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for SumFolder<S>`

##### `impl<T> Pointable for SumFolder<S>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `sum`

```rust
fn sum<PI, S>(pi: PI) -> S
where
    PI: ParallelIterator,
    S: Send + Sum<<PI as >::Item> + Sum
```

### `add`

```rust
fn add<T: Sum>(left: T, right: T) -> T
```

