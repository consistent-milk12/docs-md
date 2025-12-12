*[rayon](../../index.md) / [iter](../index.md) / [filter_map](index.md)*

---

# Module `filter_map`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FilterMap`](#filtermap) | struct | `FilterMap` creates an iterator that uses `filter_op` to both filter and map elements. |
| [`FilterMapConsumer`](#filtermapconsumer) | struct |  |
| [`FilterMapFolder`](#filtermapfolder) | struct |  |

## Structs

### `FilterMap<I, P>`

```rust
struct FilterMap<I, P> {
    base: I,
    filter_op: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:12-15`](../../../../.source_1765521767/rayon-1.11.0/src/iter/filter_map.rs#L12-L15)*

`FilterMap` creates an iterator that uses `filter_op` to both filter and map elements.
This struct is created by the `filter_map()` method on [`ParallelIterator`](../index.md).


#### Implementations

- <span id="filtermap-new"></span>`fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for FilterMap<I, P>`

- <span id="filtermap-clone"></span>`fn clone(&self) -> FilterMap<I, P>` — [`FilterMap`](#filtermap)

##### `impl<I: Debug, P> Debug for FilterMap<I, P>`

- <span id="filtermap-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for FilterMap<I, P>`

##### `impl IntoParallelIterator for FilterMap<I, P>`

- <span id="filtermap-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="filtermap-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filtermap-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for FilterMap<I, P>`

- <span id="filtermap-paralleliterator-type-item"></span>`type Item = R`

- <span id="filtermap-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FilterMap<I, P>`

- <span id="filtermap-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filtermap-pointable-type-init"></span>`type Init = T`

- <span id="filtermap-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermap-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermap-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermap-drop"></span>`unsafe fn drop(ptr: usize)`

### `FilterMapConsumer<'p, C, P>`

```rust
struct FilterMapConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:52-55`](../../../../.source_1765521767/rayon-1.11.0/src/iter/filter_map.rs#L52-L55)*

#### Implementations

- <span id="filtermapconsumer-new"></span>`fn new(base: C, filter_op: &'p P) -> Self`

#### Trait Implementations

##### `impl<T, C, P> Consumer for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-consumer-type-folder"></span>`type Folder = FilterMapFolder<'p, <C as Consumer>::Folder, P>`

- <span id="filtermapconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="filtermapconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="filtermapconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filtermapconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filtermapconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FilterMapConsumer<'p, C, P>`

##### `impl Pointable for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filtermapconsumer-pointable-type-init"></span>`type Init = T`

- <span id="filtermapconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermapconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermapconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermapconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C, P> UnindexedConsumer for FilterMapConsumer<'p, C, P>`

- <span id="filtermapconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="filtermapconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FilterMapFolder<'p, C, P>`

```rust
struct FilterMapFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter_map.rs:108-111`](../../../../.source_1765521767/rayon-1.11.0/src/iter/filter_map.rs#L108-L111)*

#### Trait Implementations

##### `impl<T, C, P> Folder for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="filtermapfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="filtermapfolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="filtermapfolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FilterMapFolder<'p, C, P>`

##### `impl Pointable for FilterMapFolder<'p, C, P>`

- <span id="filtermapfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filtermapfolder-pointable-type-init"></span>`type Init = T`

- <span id="filtermapfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filtermapfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filtermapfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filtermapfolder-drop"></span>`unsafe fn drop(ptr: usize)`

