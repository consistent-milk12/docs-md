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

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:13-17`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L13-L17)*

`MapWith` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_with()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="mapwith-new"></span>`fn new(base: I, item: T, map_op: F) -> Self`

  Creates a new `MapWith` iterator.

#### Trait Implementations

##### `impl<T> Any for MapWith<I, T, F>`

- <span id="mapwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapWith<I, T, F>`

- <span id="mapwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapWith<I, T, F>`

- <span id="mapwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, T: clone::Clone, F: clone::Clone> Clone for MapWith<I, T, F>`

- <span id="mapwith-clone"></span>`fn clone(&self) -> MapWith<I, T, F>` — [`MapWith`](#mapwith)

##### `impl<T> CloneToUninit for MapWith<I, T, F>`

- <span id="mapwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, T: Debug, F> Debug for MapWith<I, T, F>`

- <span id="mapwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MapWith<I, T, F>`

- <span id="mapwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T, F> IndexedParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwith-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="mapwith-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T, U> Into for MapWith<I, T, F>`

- <span id="mapwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for MapWith<I, T, F>`

##### `impl<T> IntoParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="mapwith-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapwith-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, T, F> ParallelIterator for MapWith<I, T, F>`

- <span id="mapwith-paralleliterator-type-item"></span>`type Item = R`

- <span id="mapwith-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwith-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for MapWith<I, T, F>`

- <span id="mapwith-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwith-pointable-type-init"></span>`type Init = T`

- <span id="mapwith-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwith-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwith-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwith-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for MapWith<I, T, F>`

- <span id="mapwith-toowned-type-owned"></span>`type Owned = T`

- <span id="mapwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for MapWith<I, T, F>`

- <span id="mapwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for MapWith<I, T, F>`

- <span id="mapwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapWithProducer<'f, P, U, F>`

```rust
struct MapWithProducer<'f, P, U, F> {
    base: P,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:118-122`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L118-L122)*

#### Trait Implementations

##### `impl Any for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapWithProducer<'f, P, U, F>`

##### `impl Pointable for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithproducer-pointable-type-init"></span>`type Init = T`

- <span id="mapwithproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, U, F> Producer for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-producer-type-item"></span>`type Item = R`

- <span id="mapwithproducer-producer-type-intoiter"></span>`type IntoIter = MapWithIter<'f, <P as Producer>::IntoIter, U, F>`

- <span id="mapwithproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="mapwithproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="mapwithproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="mapwithproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="mapwithproducer-producer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

##### `impl<U> TryFrom for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapwithproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapWithProducer<'f, P, U, F>`

- <span id="mapwithproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapwithproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapWithIter<'f, I, U, F>`

```rust
struct MapWithIter<'f, I, U, F> {
    base: I,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:178-182`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L178-L182)*

#### Trait Implementations

##### `impl Any for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, U, F> DoubleEndedIterator for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<R>`

##### `impl<I, U, F> ExactSizeIterator for MapWithIter<'f, I, U, F>`

##### `impl<T> From for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapWithIter<'f, I, U, F>`

##### `impl<I> IntoIterator for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="mapwithiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="mapwithiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, U, F> Iterator for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-iterator-type-item"></span>`type Item = R`

- <span id="mapwithiter-iterator-next"></span>`fn next(&mut self) -> Option<R>`

- <span id="mapwithiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithiter-pointable-type-init"></span>`type Init = T`

- <span id="mapwithiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapwithiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapWithIter<'f, I, U, F>`

- <span id="mapwithiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapwithiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapWithConsumer<'f, C, U, F>`

```rust
struct MapWithConsumer<'f, C, U, F> {
    base: C,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:225-229`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L225-L229)*

#### Implementations

- <span id="mapwithconsumer-new"></span>`fn new(base: C, item: U, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl Any for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, U, C, F> Consumer for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-consumer-type-folder"></span>`type Folder = MapWithFolder<'f, <C as Consumer>::Folder, U, F>`

- <span id="mapwithconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="mapwithconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="mapwithconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwithconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapwithconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapWithConsumer<'f, C, U, F>`

##### `impl Pointable for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithconsumer-pointable-type-init"></span>`type Init = T`

- <span id="mapwithconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapwithconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapwithconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, U, C, F> UnindexedConsumer for MapWithConsumer<'f, C, U, F>`

- <span id="mapwithconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="mapwithconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `MapWithFolder<'f, C, U, F>`

```rust
struct MapWithFolder<'f, C, U, F> {
    base: C,
    item: U,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:286-290`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L286-L290)*

#### Trait Implementations

##### `impl Any for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, U, C, F> Folder for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="mapwithfolder-folder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="mapwithfolder-folder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="mapwithfolder-folder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="mapwithfolder-folder-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapWithFolder<'f, C, U, F>`

##### `impl Pointable for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapwithfolder-pointable-type-init"></span>`type Init = T`

- <span id="mapwithfolder-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapwithfolder-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapwithfolder-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapwithfolder-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapwithfolder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapWithFolder<'f, C, U, F>`

- <span id="mapwithfolder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapwithfolder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapInit<I, INIT, F>`

```rust
struct MapInit<I, INIT, F> {
    base: I,
    init: INIT,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:341-345`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L341-L345)*

`MapInit` is an iterator that transforms the elements of an underlying iterator.

This struct is created by the `map_init()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="mapinit-new"></span>`fn new(base: I, init: INIT, map_op: F) -> Self`

  Creates a new `MapInit` iterator.

#### Trait Implementations

##### `impl Any for MapInit<I, INIT, F>`

- <span id="mapinit-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapInit<I, INIT, F>`

- <span id="mapinit-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapInit<I, INIT, F>`

- <span id="mapinit-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, INIT: clone::Clone, F: clone::Clone> Clone for MapInit<I, INIT, F>`

- <span id="mapinit-clone"></span>`fn clone(&self) -> MapInit<I, INIT, F>` — [`MapInit`](#mapinit)

##### `impl CloneToUninit for MapInit<I, INIT, F>`

- <span id="mapinit-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: Debug, INIT, F> Debug for MapInit<I, INIT, F>`

- <span id="mapinit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MapInit<I, INIT, F>`

- <span id="mapinit-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, INIT, F> IndexedParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinit-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="mapinit-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for MapInit<I, INIT, F>`

- <span id="mapinit-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapInit<I, INIT, F>`

##### `impl IntoParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="mapinit-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="mapinit-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, INIT, F> ParallelIterator for MapInit<I, INIT, F>`

- <span id="mapinit-paralleliterator-type-item"></span>`type Item = R`

- <span id="mapinit-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinit-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for MapInit<I, INIT, F>`

- <span id="mapinit-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapinit-pointable-type-init"></span>`type Init = T`

- <span id="mapinit-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinit-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinit-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinit-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for MapInit<I, INIT, F>`

- <span id="mapinit-toowned-type-owned"></span>`type Owned = T`

- <span id="mapinit-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mapinit-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MapInit<I, INIT, F>`

- <span id="mapinit-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapinit-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapInit<I, INIT, F>`

- <span id="mapinit-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapinit-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapInitProducer<'f, P, INIT, F>`

```rust
struct MapInitProducer<'f, P, INIT, F> {
    base: P,
    init: &'f INIT,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:443-447`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L443-L447)*

#### Trait Implementations

##### `impl Any for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapInitProducer<'f, P, INIT, F>`

##### `impl Pointable for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapinitproducer-pointable-type-init"></span>`type Init = T`

- <span id="mapinitproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinitproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinitproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinitproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, INIT, F> Producer for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-producer-type-item"></span>`type Item = R`

- <span id="mapinitproducer-producer-type-intoiter"></span>`type IntoIter = MapWithIter<'f, <P as Producer>::IntoIter, U, F>`

- <span id="mapinitproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="mapinitproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="mapinitproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="mapinitproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="mapinitproducer-producer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

##### `impl<U> TryFrom for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapinitproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapInitProducer<'f, P, INIT, F>`

- <span id="mapinitproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapinitproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MapInitConsumer<'f, C, INIT, F>`

```rust
struct MapInitConsumer<'f, C, INIT, F> {
    base: C,
    init: &'f INIT,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/map_with.rs:506-510`](../../../../.source_1765633015/rayon-1.11.0/src/iter/map_with.rs#L506-L510)*

#### Implementations

- <span id="mapinitconsumer-new"></span>`fn new(base: C, init: &'f INIT, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl Any for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, INIT, C, F> Consumer for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-consumer-type-folder"></span>`type Folder = MapWithFolder<'f, <C as Consumer>::Folder, U, F>`

- <span id="mapinitconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="mapinitconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="mapinitconsumer-consumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinitconsumer-consumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="mapinitconsumer-consumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> From for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MapInitConsumer<'f, C, INIT, F>`

##### `impl Pointable for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="mapinitconsumer-pointable-type-init"></span>`type Init = T`

- <span id="mapinitconsumer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="mapinitconsumer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="mapinitconsumer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="mapinitconsumer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mapinitconsumer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mapinitconsumer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, INIT, C, F> UnindexedConsumer for MapInitConsumer<'f, C, INIT, F>`

- <span id="mapinitconsumer-unindexedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="mapinitconsumer-unindexedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

