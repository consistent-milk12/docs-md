*[rayon](../../index.md) / [iter](../index.md) / [filter](index.md)*

---

# Module `filter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Filter`](#filter) | struct | `Filter` takes a predicate `filter_op` and filters out elements that match. |
| [`FilterConsumer`](#filterconsumer) | struct |  |
| [`FilterFolder`](#filterfolder) | struct |  |

## Structs

### `Filter<I, P>`

```rust
struct Filter<I, P> {
    base: I,
    filter_op: P,
}
```

`Filter` takes a predicate `filter_op` and filters out elements that match.
This struct is created by the `filter()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="filter-new"></span>`fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Filter<I, P>`

- <span id="filter-clone"></span>`fn clone(&self) -> Filter<I, P>` — [`Filter`](../index.md)

##### `impl<I: Debug, P> Debug for Filter<I, P>`

- <span id="filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Filter<I, P>`

##### `impl<T> IntoParallelIterator for Filter<I, P>`

- <span id="filter-iter"></span>`type Iter = T`

- <span id="filter-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Filter<I, P>`

- <span id="filter-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="filter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Filter<I, P>`

- <span id="filter-align"></span>`const ALIGN: usize`

- <span id="filter-init"></span>`type Init = T`

- <span id="filter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filter-drop"></span>`unsafe fn drop(ptr: usize)`

### `FilterConsumer<'p, C, P>`

```rust
struct FilterConsumer<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

#### Implementations

- <span id="filterconsumer-new"></span>`fn new(base: C, filter_op: &'p P) -> Self`

#### Trait Implementations

##### `impl<'p, T, C, P> Consumer for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-folder"></span>`type Folder = FilterFolder<'p, <C as Consumer>::Folder, P>`

- <span id="filterconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="filterconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="filterconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="filterconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="filterconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FilterConsumer<'p, C, P>`

##### `impl<T> Pointable for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-align"></span>`const ALIGN: usize`

- <span id="filterconsumer-init"></span>`type Init = T`

- <span id="filterconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filterconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filterconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filterconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'p, T, C, P> UnindexedConsumer for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="filterconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FilterFolder<'p, C, P>`

```rust
struct FilterFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

#### Trait Implementations

##### `impl<'p, C, P, T> Folder for FilterFolder<'p, C, P>`

- <span id="filterfolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="filterfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="filterfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="filterfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FilterFolder<'p, C, P>`

##### `impl<T> Pointable for FilterFolder<'p, C, P>`

- <span id="filterfolder-align"></span>`const ALIGN: usize`

- <span id="filterfolder-init"></span>`type Init = T`

- <span id="filterfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filterfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filterfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filterfolder-drop"></span>`unsafe fn drop(ptr: usize)`

