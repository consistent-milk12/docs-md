*[rayon](../../index.md) / [iter](../index.md) / [filter_map](index.md)*

---

# Module `filter_map`

## Structs

### `FilterMap<I, P>`

```rust
struct FilterMap<I, P> {
    base: I,
    filter_op: P,
}
```

`FilterMap` creates an iterator that uses `filter_op` to both filter and map elements.
This struct is created by the `filter_map()` method on [`ParallelIterator`](../../prelude/index.md).


#### Implementations

- `fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, P: $crate::clone::Clone> Clone for FilterMap<I, P>`

- `fn clone(self: &Self) -> FilterMap<I, P>` — [`FilterMap`](../index.md)

##### `impl<I: Debug, P> Debug for FilterMap<I, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FilterMap<I, P>`

##### `impl<T> IntoParallelIterator for FilterMap<I, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, P, R> ParallelIterator for FilterMap<I, P>`

- `type Item = R`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FilterMap<I, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `FilterMapConsumer<'p, C, P>`

```rust
struct FilterMapConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

#### Implementations

- `fn new(base: C, filter_op: &'p P) -> Self`

#### Trait Implementations

##### `impl<'p, T, U, C, P> Consumer for FilterMapConsumer<'p, C, P>`

- `type Folder = FilterMapFolder<'p, <C as Consumer>::Folder, P>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FilterMapConsumer<'p, C, P>`

##### `impl<T> Pointable for FilterMapConsumer<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'p, T, U, C, P> UnindexedConsumer for FilterMapConsumer<'p, C, P>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FilterMapFolder<'p, C, P>`

```rust
struct FilterMapFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

#### Trait Implementations

##### `impl<'p, T, U, C, P> Folder for FilterMapFolder<'p, C, P>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for FilterMapFolder<'p, C, P>`

##### `impl<T> Pointable for FilterMapFolder<'p, C, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

