*[rayon](../../index.md) / [iter](../index.md) / [flat_map_iter](index.md)*

---

# Module `flat_map_iter`

## Structs

### `FlatMapIter<I, F>`

```rust
struct FlatMapIter<I, F> {
    base: I,
    map_op: F,
}
```

`FlatMapIter` maps each element to a serial iterator, then flattens these iterators together.
This struct is created by the `flat_map_iter()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for FlatMapIter<I, F>`

- `fn clone(self: &Self) -> FlatMapIter<I, F>` — [`FlatMapIter`](../index.md)

##### `impl<I: Debug, F> Debug for FlatMapIter<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMapIter<I, F>`

##### `impl<T> IntoParallelIterator for FlatMapIter<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F, SI> ParallelIterator for FlatMapIter<I, F>`

- `type Item = <SI as IntoIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FlatMapIter<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FlatMapIterConsumer<'f, C, F>`

```rust
struct FlatMapIterConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Implementations

- `fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, U, C, F> Consumer for FlatMapIterConsumer<'f, C, F>`

- `type Folder = FlatMapIterFolder<'f, <C as Consumer>::Folder, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlatMapIterConsumer<'f, C, F>`

##### `impl<T> Pointable for FlatMapIterConsumer<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, U, C, F> UnindexedConsumer for FlatMapIterConsumer<'f, C, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FlatMapIterFolder<'f, C, F>`

```rust
struct FlatMapIterFolder<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, T, U, C, F> Folder for FlatMapIterFolder<'f, C, F>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FlatMapIterFolder<'f, C, F>`

##### `impl<T> Pointable for FlatMapIterFolder<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

