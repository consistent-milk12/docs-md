*[rayon](../../index.md) / [iter](../index.md) / [map_with](index.md)*

---

# Module `map_with`

## Structs

### `MapWith<I, T, F>`

```rust
struct MapWith<I, T, F> {
    base: I,
    item: T,
    map_op: F,
}
```

`MapWith` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_with()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, item: T, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, T: $crate::clone::Clone, F: $crate::clone::Clone> Clone for MapWith<I, T, F>`

- `fn clone(self: &Self) -> MapWith<I, T, F>` — [`MapWith`](#mapwith)

##### `impl<I: Debug, T: Debug, F> Debug for MapWith<I, T, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T, F, R> IndexedParallelIterator for MapWith<I, T, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for MapWith<I, T, F>`

##### `impl<T> IntoParallelIterator for MapWith<I, T, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, T, F, R> ParallelIterator for MapWith<I, T, F>`

- `type Item = R`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MapWith<I, T, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MapWithProducer<'f, P, U, F>`

```rust
struct MapWithProducer<'f, P, U, F> {
    base: P,
    item: U,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for MapWithProducer<'f, P, U, F>`

##### `impl<T> Pointable for MapWithProducer<'f, P, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, P, U, F, R> Producer for MapWithProducer<'f, P, U, F>`

- `type Item = R`

- `type IntoIter = MapWithIter<'f, <P as Producer>::IntoIter, U, F>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<G>(self: Self, folder: G) -> G`

### `MapWithIter<'f, I, U, F>`

```rust
struct MapWithIter<'f, I, U, F> {
    base: I,
    item: U,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, I, U, F, R> DoubleEndedIterator for MapWithIter<'f, I, U, F>`

- `fn next_back(self: &mut Self) -> Option<R>`

##### `impl<'f, I, U, F, R> ExactSizeIterator for MapWithIter<'f, I, U, F>`

##### `impl<T> IntoEither for MapWithIter<'f, I, U, F>`

##### `impl<I> IntoIterator for MapWithIter<'f, I, U, F>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'f, I, U, F, R> Iterator for MapWithIter<'f, I, U, F>`

- `type Item = R`

- `fn next(self: &mut Self) -> Option<R>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for MapWithIter<'f, I, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MapWithConsumer<'f, C, U, F>`

```rust
struct MapWithConsumer<'f, C, U, F> {
    base: C,
    item: U,
    map_op: &'f F,
}
```

#### Implementations

- `fn new(base: C, item: U, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, U, R, C, F> Consumer for MapWithConsumer<'f, C, U, F>`

- `type Folder = MapWithFolder<'f, <C as Consumer>::Folder, U, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for MapWithConsumer<'f, C, U, F>`

##### `impl<T> Pointable for MapWithConsumer<'f, C, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, U, R, C, F> UnindexedConsumer for MapWithConsumer<'f, C, U, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `MapWithFolder<'f, C, U, F>`

```rust
struct MapWithFolder<'f, C, U, F> {
    base: C,
    item: U,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, T, U, R, C, F> Folder for MapWithFolder<'f, C, U, F>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for MapWithFolder<'f, C, U, F>`

##### `impl<T> Pointable for MapWithFolder<'f, C, U, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MapInit<I, INIT, F>`

```rust
struct MapInit<I, INIT, F> {
    base: I,
    init: INIT,
    map_op: F,
}
```

`MapInit` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_init()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, init: INIT, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, INIT: $crate::clone::Clone, F: $crate::clone::Clone> Clone for MapInit<I, INIT, F>`

- `fn clone(self: &Self) -> MapInit<I, INIT, F>` — [`MapInit`](#mapinit)

##### `impl<I: Debug, INIT, F> Debug for MapInit<I, INIT, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, INIT, T, F, R> IndexedParallelIterator for MapInit<I, INIT, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for MapInit<I, INIT, F>`

##### `impl<T> IntoParallelIterator for MapInit<I, INIT, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, INIT, T, F, R> ParallelIterator for MapInit<I, INIT, F>`

- `type Item = R`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for MapInit<I, INIT, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MapInitProducer<'f, P, INIT, F>`

```rust
struct MapInitProducer<'f, P, INIT, F> {
    base: P,
    init: &'f INIT,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for MapInitProducer<'f, P, INIT, F>`

##### `impl<T> Pointable for MapInitProducer<'f, P, INIT, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, P, INIT, U, F, R> Producer for MapInitProducer<'f, P, INIT, F>`

- `type Item = R`

- `type IntoIter = MapWithIter<'f, <P as Producer>::IntoIter, U, F>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<G>(self: Self, folder: G) -> G`

### `MapInitConsumer<'f, C, INIT, F>`

```rust
struct MapInitConsumer<'f, C, INIT, F> {
    base: C,
    init: &'f INIT,
    map_op: &'f F,
}
```

#### Implementations

- `fn new(base: C, init: &'f INIT, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, INIT, U, R, C, F> Consumer for MapInitConsumer<'f, C, INIT, F>`

- `type Folder = MapWithFolder<'f, <C as Consumer>::Folder, U, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for MapInitConsumer<'f, C, INIT, F>`

##### `impl<T> Pointable for MapInitConsumer<'f, C, INIT, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, INIT, U, R, C, F> UnindexedConsumer for MapInitConsumer<'f, C, INIT, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

