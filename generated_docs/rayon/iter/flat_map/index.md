*[rayon](../../index.md) / [iter](../index.md) / [flat_map](index.md)*

---

# Module `flat_map`

## Structs

### `FlatMap<I, F>`

```rust
struct FlatMap<I, F> {
    base: I,
    map_op: F,
}
```

`FlatMap` maps each element to a parallel iterator, then flattens these iterators together.
This struct is created by the `flat_map()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FlatMap<I, F>`

- `fn clone(self: &Self) -> FlatMap<I, F>` — [`FlatMap`](../index.md)

##### `impl<I: Debug, F> Debug for FlatMap<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMap<I, F>`

##### `impl<T> IntoParallelIterator for FlatMap<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F, PI> ParallelIterator for FlatMap<I, F>`

- `type Item = <PI as IntoParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FlatMap<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FlatMapConsumer<'f, C, F>`

```rust
struct FlatMapConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Implementations

- `fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, U, C, F> Consumer for FlatMapConsumer<'f, C, F>`

- `type Folder = FlatMapFolder<'f, C, F, <C as Consumer>::Result>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlatMapConsumer<'f, C, F>`

##### `impl<T> Pointable for FlatMapConsumer<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, U, C, F> UnindexedConsumer for FlatMapConsumer<'f, C, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FlatMapFolder<'f, C, F, R>`

```rust
struct FlatMapFolder<'f, C, F, R> {
    base: C,
    map_op: &'f F,
    previous: Option<R>,
}
```

#### Trait Implementations

##### `impl<'f, T, U, C, F> Folder for FlatMapFolder<'f, C, F, <C as >::Result>`

- `type Result = <C as Consumer>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlatMapFolder<'f, C, F, R>`

##### `impl<T> Pointable for FlatMapFolder<'f, C, F, R>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

