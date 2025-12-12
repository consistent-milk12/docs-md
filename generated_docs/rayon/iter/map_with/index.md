*[rayon](../../index.md) / [iter](../index.md) / [map_with](index.md)*

---

# Module `map_with`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MapWith`](#mapwith) | struct | `MapWith` is an iterator that transforms the elements of an underlying iterator. |
| [`MapWithProducer`](#mapwithproducer) | struct |  |
| [`MapWithIter`](#mapwithiter) | struct |  |
| [`MapWithConsumer`](#mapwithconsumer) | struct |  |
| [`MapWithFolder`](#mapwithfolder) | struct |  |
| [`MapInit`](#mapinit) | struct | `MapInit` is an iterator that transforms the elements of an underlying iterator. |
| [`MapInitProducer`](#mapinitproducer) | struct |  |
| [`MapInitConsumer`](#mapinitconsumer) | struct |  |

## Structs

### `MapWith<I, T, F>`

```rust
struct MapWith<I, T, F> {
    base: I,
    item: T,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:13-17`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L13-L17)*

`MapWith` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_with()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="mapwith-new"></span>`fn new(base: I, item: T, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, T: clone::Clone, F: clone::Clone> Clone for MapWith<I, T, F>`

- <span id="mapwith-clone"></span>`fn clone(&self) -> MapWith<I, T, F>` — [`MapWith`](#mapwith)

##### `impl<I: Debug, T: Debug, F> Debug for MapWith<I, T, F>`

- <span id="mapwith-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T, F> IndexedParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwith-len"></span>`fn len(&self) -> usize`

- <span id="mapwith-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for MapWith<I, T, F>`

##### `impl<T> IntoParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="mapwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapwith-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, T, F> ParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-paralleliterator-type-item"></span>`type Item = R`

- <span id="mapwith-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwith-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MapWith<I, T, F>`

- <span id="mapwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwith-pointable-type-init"></span>`type Init = T`

- <span id="mapwith-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwith-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwith-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwith-drop"></span>`unsafe fn drop(ptr: usize)`

### `MapWithProducer<'f, P, U, F>`

```rust
struct MapWithProducer<'f, P, U, F> {
    base: P,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:118-122`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L118-L122)*

#### Trait Implementations

##### `impl IntoEither for MapWithProducer<'f, P, U, F>`

##### `impl Pointable for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithproducer-pointable-type-init"></span>`type Init = T`

- <span id="mapwithproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, U, F> Producer for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-producer-type-item"></span>`type Item = R`

- <span id="mapwithproducer-producer-type-intoiter"></span>`type IntoIter = MapWithIter<'f, <P as Producer>::IntoIter, U, F>`

- <span id="mapwithproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="mapwithproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="mapwithproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="mapwithproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="mapwithproducer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

### `MapWithIter<'f, I, U, F>`

```rust
struct MapWithIter<'f, I, U, F> {
    base: I,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:178-182`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L178-L182)*

#### Trait Implementations

##### `impl<I, U, F> DoubleEndedIterator for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-next-back"></span>`fn next_back(&mut self) -> Option<R>`

##### `impl<I, U, F> ExactSizeIterator for MapWithIter<'f, I, U, F>`

##### `impl IntoEither for MapWithIter<'f, I, U, F>`

##### `impl<I> IntoIterator for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="mapwithiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="mapwithiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, U, F> Iterator for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-iterator-type-item"></span>`type Item = R`

- <span id="mapwithiter-next"></span>`fn next(&mut self) -> Option<R>`

- <span id="mapwithiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithiter-pointable-type-init"></span>`type Init = T`

- <span id="mapwithiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `MapWithConsumer<'f, C, U, F>`

```rust
struct MapWithConsumer<'f, C, U, F> {
    base: C,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:225-229`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L225-L229)*

#### Implementations

- <span id="mapwithconsumer-new"></span>`fn new(base: C, item: U, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<T, U, C, F> Consumer for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-consumer-type-folder"></span>`type Folder = MapWithFolder<'f, <C as Consumer>::Folder, U, F>`

- <span id="mapwithconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="mapwithconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="mapwithconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwithconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwithconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for MapWithConsumer<'f, C, U, F>`

##### `impl Pointable for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithconsumer-pointable-type-init"></span>`type Init = T`

- <span id="mapwithconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U, C, F> UnindexedConsumer for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="mapwithconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `MapWithFolder<'f, C, U, F>`

```rust
struct MapWithFolder<'f, C, U, F> {
    base: C,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:286-290`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L286-L290)*

#### Trait Implementations

##### `impl<T, U, C, F> Folder for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="mapwithfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="mapwithfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="mapwithfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="mapwithfolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for MapWithFolder<'f, C, U, F>`

##### `impl Pointable for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithfolder-pointable-type-init"></span>`type Init = T`

- <span id="mapwithfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithfolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `MapInit<I, INIT, F>`

```rust
struct MapInit<I, INIT, F> {
    base: I,
    init: INIT,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:341-345`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L341-L345)*

`MapInit` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_init()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="mapinit-new"></span>`fn new(base: I, init: INIT, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, INIT: clone::Clone, F: clone::Clone> Clone for MapInit<I, INIT, F>`

- <span id="mapinit-clone"></span>`fn clone(&self) -> MapInit<I, INIT, F>` — [`MapInit`](#mapinit)

##### `impl<I: Debug, INIT, F> Debug for MapInit<I, INIT, F>`

- <span id="mapinit-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, INIT, F> IndexedParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinit-len"></span>`fn len(&self) -> usize`

- <span id="mapinit-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl IntoEither for MapInit<I, INIT, F>`

##### `impl IntoParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="mapinit-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapinit-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, INIT, F> ParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-paralleliterator-type-item"></span>`type Item = R`

- <span id="mapinit-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinit-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for MapInit<I, INIT, F>`

- <span id="mapinit-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapinit-pointable-type-init"></span>`type Init = T`

- <span id="mapinit-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinit-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinit-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinit-drop"></span>`unsafe fn drop(ptr: usize)`

### `MapInitProducer<'f, P, INIT, F>`

```rust
struct MapInitProducer<'f, P, INIT, F> {
    base: P,
    init: &'f INIT,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:443-447`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L443-L447)*

#### Trait Implementations

##### `impl IntoEither for MapInitProducer<'f, P, INIT, F>`

##### `impl Pointable for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapinitproducer-pointable-type-init"></span>`type Init = T`

- <span id="mapinitproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinitproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinitproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinitproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, INIT, F> Producer for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-producer-type-item"></span>`type Item = R`

- <span id="mapinitproducer-producer-type-intoiter"></span>`type IntoIter = MapWithIter<'f, <P as Producer>::IntoIter, U, F>`

- <span id="mapinitproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="mapinitproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="mapinitproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="mapinitproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="mapinitproducer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

### `MapInitConsumer<'f, C, INIT, F>`

```rust
struct MapInitConsumer<'f, C, INIT, F> {
    base: C,
    init: &'f INIT,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:506-510`](../../../../.source_1765521767/rayon-1.11.0/src/iter/map_with.rs#L506-L510)*

#### Implementations

- <span id="mapinitconsumer-new"></span>`fn new(base: C, init: &'f INIT, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<T, INIT, C, F> Consumer for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-consumer-type-folder"></span>`type Folder = MapWithFolder<'f, <C as Consumer>::Folder, U, F>`

- <span id="mapinitconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="mapinitconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="mapinitconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinitconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinitconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for MapInitConsumer<'f, C, INIT, F>`

##### `impl Pointable for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapinitconsumer-pointable-type-init"></span>`type Init = T`

- <span id="mapinitconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinitconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinitconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinitconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, INIT, C, F> UnindexedConsumer for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="mapinitconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

