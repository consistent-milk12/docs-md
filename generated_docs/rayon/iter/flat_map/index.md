*[rayon](../../index.md) / [iter](../index.md) / [flat_map](index.md)*

---

# Module `flat_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlatMap`](#flatmap) | struct | `FlatMap` maps each element to a parallel iterator, then flattens these iterators together. |
| [`FlatMapConsumer`](#flatmapconsumer) | struct |  |
| [`FlatMapFolder`](#flatmapfolder) | struct |  |

## Structs

### `FlatMap<I, F>`

```rust
struct FlatMap<I, F> {
    base: I,
    map_op: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:12-15`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flat_map.rs#L12-L15)*

`FlatMap` maps each element to a parallel iterator, then flattens these iterators together.
This struct is created by the `flat_map()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="flatmap-new"></span>`fn new(base: I, map_op: F) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FlatMap<I, F>`

- <span id="flatmap-clone"></span>`fn clone(&self) -> FlatMap<I, F>` — [`FlatMap`](#flatmap)

##### `impl<I: Debug, F> Debug for FlatMap<I, F>`

- <span id="flatmap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlatMap<I, F>`

##### `impl<T> IntoParallelIterator for FlatMap<I, F>`

- <span id="flatmap-type-iter"></span>`type Iter = T`

- <span id="flatmap-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatmap-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, F, PI> ParallelIterator for FlatMap<I, F>`

- <span id="flatmap-type-item"></span>`type Item = <PI as IntoParallelIterator>::Item`

- <span id="flatmap-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FlatMap<I, F>`

- <span id="flatmap-const-align"></span>`const ALIGN: usize`

- <span id="flatmap-type-init"></span>`type Init = T`

- <span id="flatmap-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmap-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmap-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmap-drop"></span>`unsafe fn drop(ptr: usize)`

### `FlatMapConsumer<'f, C, F>`

```rust
struct FlatMapConsumer<'f, C, F> {
    base: C,
    map_op: &'f F,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:50-53`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flat_map.rs#L50-L53)*

#### Implementations

- <span id="flatmapconsumer-new"></span>`fn new(base: C, map_op: &'f F) -> Self`

#### Trait Implementations

##### `impl<'f, T, U, C, F> Consumer for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-type-folder"></span>`type Folder = FlatMapFolder<'f, C, F, <C as Consumer>::Result>`

- <span id="flatmapconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flatmapconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatmapconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="flatmapconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="flatmapconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlatMapConsumer<'f, C, F>`

##### `impl<T> Pointable for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-const-align"></span>`const ALIGN: usize`

- <span id="flatmapconsumer-type-init"></span>`type Init = T`

- <span id="flatmapconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'f, T, U, C, F> UnindexedConsumer for FlatMapConsumer<'f, C, F>`

- <span id="flatmapconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flatmapconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FlatMapFolder<'f, C, F, R>`

```rust
struct FlatMapFolder<'f, C, F, R> {
    base: C,
    map_op: &'f F,
    previous: Option<R>,
}
```

*Defined in [`rayon-1.11.0/src/iter/flat_map.rs:108-112`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flat_map.rs#L108-L112)*

#### Trait Implementations

##### `impl<'f, T, U, C, F> Folder for FlatMapFolder<'f, C, F, <C as >::Result>`

- <span id="flatmapfolder-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatmapfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flatmapfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="flatmapfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlatMapFolder<'f, C, F, R>`

##### `impl<T> Pointable for FlatMapFolder<'f, C, F, R>`

- <span id="flatmapfolder-const-align"></span>`const ALIGN: usize`

- <span id="flatmapfolder-type-init"></span>`type Init = T`

- <span id="flatmapfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatmapfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatmapfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatmapfolder-drop"></span>`unsafe fn drop(ptr: usize)`

