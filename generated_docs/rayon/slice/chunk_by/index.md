*[rayon](../../index.md) / [slice](../index.md) / [chunk_by](index.md)*

---

# Module `chunk_by`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T, Slice, Pred> UnindexedProducer for ChunkByProducer<'_, T, Slice, Pred>`

- `type Item = Slice`

- `fn split(self: Self) -> (Self, Option<Self>)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

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

- `fn new(slice: &'data [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T, P: Clone> Clone for ChunkBy<'_, T, P>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: fmt::Debug, P> Debug for ChunkBy<'_, T, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkBy<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkBy<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkBy<'data, T, P>`

- `type Item = &'data [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkBy<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `fn new(slice: &'data mut [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug, P> Debug for ChunkByMut<'_, T, P>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for ChunkByMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkByMut<'data, T, P>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T, P> ParallelIterator for ChunkByMut<'data, T, P>`

- `type Item = &'data mut [T]`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

##### `impl<T> Pointable for ChunkByMut<'data, T, P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Traits

### `ChunkBySlice<T>`

```rust
trait ChunkBySlice<T>: AsRef<[T]> + Default + Send { ... }
```

#### Required Methods

- `fn split(self: Self, index: usize) -> (Self, Self)`

- `fn chunk_by(self: Self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self>`

- `fn find(self: &Self, pred: &impl Fn(&T, &T) -> bool, start: usize, end: usize) -> Option<usize>`

- `fn rfind(self: &Self, pred: &impl Fn(&T, &T) -> bool, end: usize) -> Option<usize>`

