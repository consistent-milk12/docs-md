*[rayon](../../index.md) / [iter](../index.md) / [map](index.md)*

---

# Module `map`

## Structs

### `Map<I, F>`

```rust
struct Map<I, F> {
    base: I,
    map_op: F,
}
```

`Map` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, F: $crate::clone::Clone> Clone for Map<I, F>`

- `fn clone(self: &Self) -> Map<I, F>` — [`Map`](#map)

##### `impl<I: Debug, F> Debug for Map<I, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F, R> IndexedParallelIterator for Map<I, F>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Map<I, F>`

##### `impl<T> IntoParallelIterator for Map<I, F>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, F, R> ParallelIterator for Map<I, F>`

- `type Item = <F as FnOnce>::Output`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Map<I, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `MapProducer<'f, P, F>`

```rust
struct MapProducer<'f, P, F> {
    base: P,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for MapProducer<'f, P, F>`

##### `impl<T> Pointable for MapProducer<'f, P, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, P, F, R> Producer for MapProducer<'f, P, F>`

- `type Item = <F as FnOnce>::Output`

- `type IntoIter = Map<<P as Producer>::IntoIter, &'f F>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<G>(self: Self, folder: G) -> G`

### `MapConsumer<'f, C, F>`

```rust
struct MapConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Implementations

- `fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, R, C, F> Consumer for MapConsumer<'f, C, F>`

- `type Folder = MapFolder<'f, <C as Consumer>::Folder, F>`

- `type Reducer = <C as Consumer>::Reducer`

- `type Result = <C as Consumer>::Result`

- `fn split_at(self: Self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- `fn into_folder(self: Self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for MapConsumer<'f, C, F>`

##### `impl<T> Pointable for MapConsumer<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<'f, T, R, C, F> UnindexedConsumer for MapConsumer<'f, C, F>`

- `fn split_off_left(self: &Self) -> Self`

- `fn to_reducer(self: &Self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `MapFolder<'f, C, F>`

```rust
struct MapFolder<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, T, R, C, F> Folder for MapFolder<'f, C, F>`

- `type Result = <C as Folder>::Result`

- `fn consume(self: Self, item: T) -> Self`

- `fn consume_iter<I>(self: Self, iter: I) -> Self`

- `fn complete(self: Self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- `fn full(self: &Self) -> bool`

##### `impl<T> IntoEither for MapFolder<'f, C, F>`

##### `impl<T> Pointable for MapFolder<'f, C, F>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

