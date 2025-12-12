*[rayon](../../index.md) / [iter](../index.md) / [flatten_iter](index.md)*

---

# Module `flatten_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlattenIter`](#flatteniter) | struct | `FlattenIter` turns each element to a serial iterator, then flattens these iterators together. |
| [`FlattenIterConsumer`](#flatteniterconsumer) | struct |  |
| [`FlattenIterFolder`](#flatteniterfolder) | struct |  |

## Structs

### `FlattenIter<I>`

```rust
struct FlattenIter<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:10-12`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flatten_iter.rs#L10-L12)*

`FlattenIter` turns each element to a serial iterator, then flattens these iterators
together. This struct is created by the `flatten_iter()` method on [`ParallelIterator`](../index.md).


#### Implementations

- <span id="flatteniter-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for FlattenIter<I>`

- <span id="flatteniter-clone"></span>`fn clone(&self) -> FlattenIter<I>` — [`FlattenIter`](#flatteniter)

##### `impl<I: fmt::Debug> Debug for FlattenIter<I>`

- <span id="flatteniter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for FlattenIter<I>`

##### `impl IntoParallelIterator for FlattenIter<I>`

- <span id="flatteniter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="flatteniter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatteniter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for FlattenIter<I>`

- <span id="flatteniter-paralleliterator-type-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoIterator>::Item`

- <span id="flatteniter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for FlattenIter<I>`

- <span id="flatteniter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatteniter-pointable-type-init"></span>`type Init = T`

- <span id="flatteniter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniter-drop"></span>`unsafe fn drop(ptr: usize)`

### `FlattenIterConsumer<C>`

```rust
struct FlattenIterConsumer<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:39-41`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flatten_iter.rs#L39-L41)*

#### Implementations

- <span id="flatteniterconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<T, C> Consumer for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-consumer-type-folder"></span>`type Folder = FlattenIterFolder<<C as Consumer>::Folder>`

- <span id="flatteniterconsumer-consumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flatteniterconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatteniterconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatteniterconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="flatteniterconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FlattenIterConsumer<C>`

##### `impl Pointable for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatteniterconsumer-pointable-type-init"></span>`type Init = T`

- <span id="flatteniterconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniterconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniterconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniterconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C> UnindexedConsumer for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flatteniterconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `FlattenIterFolder<C>`

```rust
struct FlattenIterFolder<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/flatten_iter.rs:92-94`](../../../../.source_1765210505/rayon-1.11.0/src/iter/flatten_iter.rs#L92-L94)*

#### Trait Implementations

##### `impl<T, C> Folder for FlattenIterFolder<C>`

- <span id="flatteniterfolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="flatteniterfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flatteniterfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="flatteniterfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="flatteniterfolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for FlattenIterFolder<C>`

##### `impl Pointable for FlattenIterFolder<C>`

- <span id="flatteniterfolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatteniterfolder-pointable-type-init"></span>`type Init = T`

- <span id="flatteniterfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniterfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniterfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniterfolder-drop"></span>`unsafe fn drop(ptr: usize)`

