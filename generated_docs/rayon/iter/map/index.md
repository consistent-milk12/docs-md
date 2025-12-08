*[rayon](../../index.md) / [iter](../index.md) / [map](index.md)*

---

# Module `map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Map`](#map) | struct | `Map` is an iterator that transforms the elements of an underlying iterator. |
| [`MapProducer`](#mapproducer) | struct |  |
| [`MapConsumer`](#mapconsumer) | struct |  |
| [`MapFolder`](#mapfolder) | struct |  |

## Structs

### `Map<I, F>`

```rust
struct Map<I, F> {
    base: I,
    map_op: F,
}
```

`Map` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="map-new"></span>`fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Map<I, F>`

- <span id="map-clone"></span>`fn clone(&self) -> Map<I, F>` — [`Map`](../index.md)

##### `impl<I: Debug, F> Debug for Map<I, F>`

- <span id="map-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F, R> IndexedParallelIterator for Map<I, F>`

- <span id="map-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="map-len"></span>`fn len(&self) -> usize`

- <span id="map-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Map<I, F>`

##### `impl<T> IntoParallelIterator for Map<I, F>`

- <span id="map-iter"></span>`type Iter = T`

- <span id="map-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="map-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F, R> ParallelIterator for Map<I, F>`

- <span id="map-item"></span>`type Item = <F as FnOnce>::Output`

- <span id="map-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="map-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Map<I, F>`

- <span id="map-align"></span>`const ALIGN: usize`

- <span id="map-init"></span>`type Init = T`

- <span id="map-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="map-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="map-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="map-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="mapproducer-align"></span>`const ALIGN: usize`

- <span id="mapproducer-init"></span>`type Init = T`

- <span id="mapproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, P, F, R> Producer for MapProducer<'f, P, F>`

- <span id="mapproducer-item"></span>`type Item = <F as FnOnce>::Output`

- <span id="mapproducer-intoiter"></span>`type IntoIter = Map<<P as Producer>::IntoIter, &'f F>`

- <span id="mapproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="mapproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="mapproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="mapproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="mapproducer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

### `MapConsumer<'f, C, F>`

```rust
struct MapConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Implementations

- <span id="mapconsumer-new"></span>`fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, R, C, F> Consumer for MapConsumer<'f, C, F>`

- <span id="mapconsumer-folder"></span>`type Folder = MapFolder<'f, <C as Consumer>::Folder, F>`

- <span id="mapconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="mapconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="mapconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="mapconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="mapconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for MapConsumer<'f, C, F>`

##### `impl<T> Pointable for MapConsumer<'f, C, F>`

- <span id="mapconsumer-align"></span>`const ALIGN: usize`

- <span id="mapconsumer-init"></span>`type Init = T`

- <span id="mapconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, T, R, C, F> UnindexedConsumer for MapConsumer<'f, C, F>`

- <span id="mapconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="mapconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `MapFolder<'f, C, F>`

```rust
struct MapFolder<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, T, R, C, F> Folder for MapFolder<'f, C, F>`

- <span id="mapfolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="mapfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="mapfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="mapfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="mapfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for MapFolder<'f, C, F>`

##### `impl<T> Pointable for MapFolder<'f, C, F>`

- <span id="mapfolder-align"></span>`const ALIGN: usize`

- <span id="mapfolder-init"></span>`type Init = T`

- <span id="mapfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapfolder-drop"></span>`unsafe fn drop(ptr: usize)`

