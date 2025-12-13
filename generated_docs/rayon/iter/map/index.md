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

*Defined in [`rayon-1.11.0/src/iter/map.rs:14-17`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map.rs#L14-L17)*

`Map` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="map-new"></span>`fn new(base: I, map_op: F) -> Self`

  Creates a new `Map` iterator.

#### Trait Implementations

##### `impl Any for Map<I, F>`

- <span id="map-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Map<I, F>`

- <span id="map-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Map<I, F>`

- <span id="map-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Map<I, F>`

- <span id="map-clone"></span>`fn clone(&self) -> Map<I, F>` — [`Map`](#map)

##### `impl CloneToUninit for Map<I, F>`

- <span id="map-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, F> Debug for Map<I, F>`

- <span id="map-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Map<I, F>`

- <span id="map-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> IndexedParallelIterator for Map<I, F>`

- <span id="map-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="map-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="map-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Map<I, F>`

- <span id="map-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Map<I, F>`

##### `impl IntoParallelIterator for Map<I, F>`

- <span id="map-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="map-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="map-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F> ParallelIterator for Map<I, F>`

- <span id="map-paralleliterator-type-item"></span>`type Item = <F as FnOnce>::Output`

- <span id="map-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="map-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Map<I, F>`

- <span id="map-pointable-const-align"></span>`const ALIGN: usize`

- <span id="map-pointable-type-init"></span>`type Init = T`

- <span id="map-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="map-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="map-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="map-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Map<I, F>`

- <span id="map-toowned-type-owned"></span>`type Owned = T`

- <span id="map-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="map-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Map<I, F>`

- <span id="map-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="map-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Map<I, F>`

- <span id="map-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="map-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapProducer<'f, P, F>`

```rust
struct MapProducer<'f, P, F> {
    base: P,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map.rs:109-112`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map.rs#L109-L112)*

#### Trait Implementations

##### `impl Any for MapProducer<'f, P, F>`

- <span id="mapproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapProducer<'f, P, F>`

- <span id="mapproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapProducer<'f, P, F>`

- <span id="mapproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MapProducer<'f, P, F>`

- <span id="mapproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapProducer<'f, P, F>`

- <span id="mapproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapProducer<'f, P, F>`

##### `impl Pointable for MapProducer<'f, P, F>`

- <span id="mapproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapproducer-pointable-type-init"></span>`type Init = T`

- <span id="mapproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, F> Producer for MapProducer<'f, P, F>`

- <span id="mapproducer-producer-type-item"></span>`type Item = <F as FnOnce>::Output`

- <span id="mapproducer-producer-type-intoiter"></span>`type IntoIter = Map<<P as Producer>::IntoIter, &'f F>`

- <span id="mapproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="mapproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="mapproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="mapproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="mapproducer-producer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

##### `impl<U> TryFrom for MapProducer<'f, P, F>`

- <span id="mapproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapProducer<'f, P, F>`

- <span id="mapproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapConsumer<'f, C, F>`

```rust
struct MapConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map.rs:163-166`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map.rs#L163-L166)*

#### Implementations

- <span id="mapconsumer-new"></span>`fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl Any for MapConsumer<'f, C, F>`

- <span id="mapconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapConsumer<'f, C, F>`

- <span id="mapconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapConsumer<'f, C, F>`

- <span id="mapconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Consumer for MapConsumer<'f, C, F>`

- <span id="mapconsumer-consumer-type-folder"></span>`type Folder = MapFolder<'f, <C as Consumer>::Folder, F>`

- <span id="mapconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="mapconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="mapconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for MapConsumer<'f, C, F>`

- <span id="mapconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapConsumer<'f, C, F>`

- <span id="mapconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapConsumer<'f, C, F>`

##### `impl Pointable for MapConsumer<'f, C, F>`

- <span id="mapconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapconsumer-pointable-type-init"></span>`type Init = T`

- <span id="mapconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MapConsumer<'f, C, F>`

- <span id="mapconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapConsumer<'f, C, F>`

- <span id="mapconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, C, F> UnindexedConsumer for MapConsumer<'f, C, F>`

- <span id="mapconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="mapconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `MapFolder<'f, C, F>`

```rust
struct MapFolder<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map.rs:220-223`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map.rs#L220-L223)*

#### Trait Implementations

##### `impl Any for MapFolder<'f, C, F>`

- <span id="mapfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapFolder<'f, C, F>`

- <span id="mapfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapFolder<'f, C, F>`

- <span id="mapfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, C, F> Folder for MapFolder<'f, C, F>`

- <span id="mapfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="mapfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="mapfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="mapfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="mapfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for MapFolder<'f, C, F>`

- <span id="mapfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapFolder<'f, C, F>`

- <span id="mapfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapFolder<'f, C, F>`

##### `impl Pointable for MapFolder<'f, C, F>`

- <span id="mapfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapfolder-pointable-type-init"></span>`type Init = T`

- <span id="mapfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MapFolder<'f, C, F>`

- <span id="mapfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapFolder<'f, C, F>`

- <span id="mapfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

