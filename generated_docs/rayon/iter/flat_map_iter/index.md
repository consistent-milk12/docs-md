*[rayon](../../index.md) / [iter](../index.md) / [flat_map_iter](index.md)*

---

# Module `flat_map_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlatMapIter`](#flatmapiter) | struct | `FlatMapIter` maps each element to a serial iterator, then flattens these iterators together. |
| [`FlatMapIterConsumer`](#flatmapiterconsumer) | struct |  |
| [`FlatMapIterFolder`](#flatmapiterfolder) | struct |  |

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

- <span id="flatmapiter-new"></span>`fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMapIter<I, F>`

- <span id="flatmapiter-clone"></span>`fn clone(&self) -> FlatMapIter<I, F>` — [`FlatMapIter`](../index.md)

##### `impl<I: Debug, F> Debug for FlatMapIter<I, F>`

- <span id="flatmapiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMapIter<I, F>`

##### `impl<T> IntoParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-iter"></span>`type Iter = T`

- <span id="flatmapiter-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmapiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F, SI> ParallelIterator for FlatMapIter<I, F>`

- <span id="flatmapiter-item"></span>`type Item = <SI as IntoIterator>::Item`

- <span id="flatmapiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FlatMapIter<I, F>`

- <span id="flatmapiter-align"></span>`const ALIGN: usize`

- <span id="flatmapiter-init"></span>`type Init = T`

- <span id="flatmapiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `FlatMapIterConsumer<'f, C, F>`

```rust
struct FlatMapIterConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Implementations

- <span id="flatmapiterconsumer-new"></span>`fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, U, C, F> Consumer for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-folder"></span>`type Folder = FlatMapIterFolder<'f, <C as Consumer>::Folder, F>`

- <span id="flatmapiterconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flatmapiterconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatmapiterconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="flatmapiterconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="flatmapiterconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlatMapIterConsumer<'f, C, F>`

##### `impl<T> Pointable for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-align"></span>`const ALIGN: usize`

- <span id="flatmapiterconsumer-init"></span>`type Init = T`

- <span id="flatmapiterconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiterconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiterconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiterconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, T, U, C, F> UnindexedConsumer for FlatMapIterConsumer<'f, C, F>`

- <span id="flatmapiterconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flatmapiterconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FlatMapIterFolder<'f, C, F>`

```rust
struct FlatMapIterFolder<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

#### Trait Implementations

##### `impl<'f, T, U, C, F> Folder for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="flatmapiterfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flatmapiterfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="flatmapiterfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="flatmapiterfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlatMapIterFolder<'f, C, F>`

##### `impl<T> Pointable for FlatMapIterFolder<'f, C, F>`

- <span id="flatmapiterfolder-align"></span>`const ALIGN: usize`

- <span id="flatmapiterfolder-init"></span>`type Init = T`

- <span id="flatmapiterfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapiterfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapiterfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapiterfolder-drop"></span>`unsafe fn drop(ptr: usize)`

