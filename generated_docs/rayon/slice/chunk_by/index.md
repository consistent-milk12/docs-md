*[rayon](../../index.md) / [slice](../index.md) / [chunk_by](index.md)*

---

# Module `chunk_by`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ChunkByProducer`](#chunkbyproducer) | struct |  |
| [`ChunkBy`](#chunkby) | struct | Parallel iterator over slice in (non-overlapping) chunks separated by a predicate. |
| [`ChunkByMut`](#chunkbymut) | struct | Parallel iterator over slice in (non-overlapping) mutable chunks |
| [`ChunkBySlice`](#chunkbyslice) | trait |  |

## Structs

### `ChunkByProducer<'p, T, Slice, Pred>`

```rust
struct ChunkByProducer<'p, T, Slice, Pred> {
    slice: Slice,
    pred: &'p Pred,
    tail: usize,
    marker: std::marker::PhantomData<fn(&T)>,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for ChunkByProducer<'p, T, Slice, Pred>`

##### `impl<T> Pointable for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-align"></span>`const ALIGN: usize`

- <span id="chunkbyproducer-init"></span>`type Init = T`

- <span id="chunkbyproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkbyproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkbyproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkbyproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, Slice, Pred> UnindexedProducer for ChunkByProducer<'_, T, Slice, Pred>`

- <span id="chunkbyproducer-item"></span>`type Item = Slice`

- <span id="chunkbyproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="chunkbyproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `ChunkBy<'data, T, P>`

```rust
struct ChunkBy<'data, T, P> {
    pred: P,
    slice: &'data [T],
}
```

Parallel iterator over slice in (non-overlapping) chunks separated by a predicate.

This struct is created by the `par_chunk_by` method on `&[T]`.


#### Implementations

- <span id="chunkby-new"></span>`fn new(slice: &'data [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T, P: Clone> Clone for ChunkBy<'_, T, P>`

- <span id="chunkby-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug, P> Debug for ChunkBy<'_, T, P>`

- <span id="chunkby-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkBy<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-iter"></span>`type Iter = T`

- <span id="chunkby-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkby-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-item"></span>`type Item = &'data [T]`

- <span id="chunkby-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkBy<'data, T, P>`

- <span id="chunkby-align"></span>`const ALIGN: usize`

- <span id="chunkby-init"></span>`type Init = T`

- <span id="chunkby-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkby-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkby-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkby-drop"></span>`unsafe fn drop(ptr: usize)`

### `ChunkByMut<'data, T, P>`

```rust
struct ChunkByMut<'data, T, P> {
    pred: P,
    slice: &'data mut [T],
}
```

Parallel iterator over slice in (non-overlapping) mutable chunks
separated by a predicate.

This struct is created by the `par_chunk_by_mut` method on `&mut [T]`.


#### Implementations

- <span id="chunkbymut-new"></span>`fn new(slice: &'data mut [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug, P> Debug for ChunkByMut<'_, T, P>`

- <span id="chunkbymut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkByMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-iter"></span>`type Iter = T`

- <span id="chunkbymut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkbymut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-item"></span>`type Item = &'data mut [T]`

- <span id="chunkbymut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-align"></span>`const ALIGN: usize`

- <span id="chunkbymut-init"></span>`type Init = T`

- <span id="chunkbymut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkbymut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkbymut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkbymut-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `ChunkBySlice<T>`

```rust
trait ChunkBySlice<T>: AsRef<[T]> + Default + Send { ... }
```

#### Required Methods

- `fn split(self, index: usize) -> (Self, Self)`

- `fn chunk_by(self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self>`

#### Provided Methods

- `fn find(&self, pred: &impl Fn(&T, &T) -> bool, start: usize, end: usize) -> Option<usize>`

- `fn rfind(&self, pred: &impl Fn(&T, &T) -> bool, end: usize) -> Option<usize>`

#### Implementors

- `&[T]`
- `&mut [T]`

