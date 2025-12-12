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

*Defined in [`rayon-1.11.0/src/iter/filter.rs:12-15`](../../../../.source_1765521767/rayon-1.11.0/src/iter/filter.rs#L12-L15)*

`Filter` takes a predicate `filter_op` and filters out elements that match.
This struct is created by the `filter()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="filter-new"></span>`fn new(base: I, filter_op: P) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, P: clone::Clone> Clone for Filter<I, P>`

- <span id="filter-clone"></span>`fn clone(&self) -> Filter<I, P>` — [`Filter`](#filter)

##### `impl<I: Debug, P> Debug for Filter<I, P>`

- <span id="filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for Filter<I, P>`

##### `impl IntoParallelIterator for Filter<I, P>`

- <span id="filter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="filter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="filter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, P> ParallelIterator for Filter<I, P>`

- <span id="filter-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="filter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for Filter<I, P>`

- <span id="filter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filter-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/filter.rs:49-52`](../../../../.source_1765521767/rayon-1.11.0/src/iter/filter.rs#L49-L52)*

#### Implementations

- <span id="filterconsumer-new"></span>`fn new(base: C, filter_op: &'p P) -> Self`

#### Trait Implementations

##### `impl<T, C, P> Consumer for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-consumer-type-folder"></span>`type Folder = FilterFolder<'p, <C as Consumer>::Folder, P>`

- <span id="filterconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="filterconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="filterconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filterconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="filterconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FilterConsumer<'p, C, P>`

##### `impl Pointable for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filterconsumer-pointable-type-init"></span>`type Init = T`

- <span id="filterconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filterconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filterconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filterconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C, P> UnindexedConsumer for FilterConsumer<'p, C, P>`

- <span id="filterconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="filterconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FilterFolder<'p, C, P>`

```rust
struct FilterFolder<'p, C, P> {
    base: C,
    filter_op: &'p P,
}
```

*Defined in [`rayon-1.11.0/src/iter/filter.rs:104-107`](../../../../.source_1765521767/rayon-1.11.0/src/iter/filter.rs#L104-L107)*

#### Trait Implementations

##### `impl<C, P, T> Folder for FilterFolder<'p, C, P>`

- <span id="filterfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="filterfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="filterfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="filterfolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FilterFolder<'p, C, P>`

##### `impl Pointable for FilterFolder<'p, C, P>`

- <span id="filterfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="filterfolder-pointable-type-init"></span>`type Init = T`

- <span id="filterfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="filterfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="filterfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="filterfolder-drop"></span>`unsafe fn drop(ptr: usize)`

