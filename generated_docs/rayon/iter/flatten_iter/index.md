*[rayon](../../index.md) / [iter](../index.md) / [flatten_iter](index.md)*

---

# Module `flatten_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlattenIter`](#flatteniter) | struct | `FlattenIter` turns each element to a serial iterator, then flattens these iterators |
| [`FlattenIterConsumer`](#flatteniterconsumer) | struct |  |
| [`FlattenIterFolder`](#flatteniterfolder) | struct |  |

## Structs

### `FlattenIter<I>`

```rust
struct FlattenIter<I> {
    base: I,
}
```

`FlattenIter` turns each element to a serial iterator, then flattens these iterators
together. This struct is created by the `flatten_iter()` method on [`ParallelIterator`](../../prelude/index.md).


#### Implementations

- <span id="flatteniter-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for FlattenIter<I>`

- <span id="flatteniter-clone"></span>`fn clone(&self) -> FlattenIter<I>` — [`FlattenIter`](../index.md)

##### `impl<I: fmt::Debug> Debug for FlattenIter<I>`

- <span id="flatteniter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for FlattenIter<I>`

##### `impl<T> IntoParallelIterator for FlattenIter<I>`

- <span id="flatteniter-iter"></span>`type Iter = T`

- <span id="flatteniter-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="flatteniter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for FlattenIter<I>`

- <span id="flatteniter-item"></span>`type Item = <<I as ParallelIterator>::Item as IntoIterator>::Item`

- <span id="flatteniter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for FlattenIter<I>`

- <span id="flatteniter-align"></span>`const ALIGN: usize`

- <span id="flatteniter-init"></span>`type Init = T`

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

#### Implementations

- <span id="flatteniterconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<T, C> Consumer for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-folder"></span>`type Folder = FlattenIterFolder<<C as Consumer>::Folder>`

- <span id="flatteniterconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="flatteniterconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="flatteniterconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <C as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="flatteniterconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="flatteniterconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlattenIterConsumer<C>`

##### `impl<T> Pointable for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-align"></span>`const ALIGN: usize`

- <span id="flatteniterconsumer-init"></span>`type Init = T`

- <span id="flatteniterconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniterconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniterconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniterconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C> UnindexedConsumer for FlattenIterConsumer<C>`

- <span id="flatteniterconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="flatteniterconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `FlattenIterFolder<C>`

```rust
struct FlattenIterFolder<C> {
    base: C,
}
```

#### Trait Implementations

##### `impl<T, C> Folder for FlattenIterFolder<C>`

- <span id="flatteniterfolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="flatteniterfolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="flatteniterfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="flatteniterfolder-complete"></span>`fn complete(self) -> <Self as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="flatteniterfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for FlattenIterFolder<C>`

##### `impl<T> Pointable for FlattenIterFolder<C>`

- <span id="flatteniterfolder-align"></span>`const ALIGN: usize`

- <span id="flatteniterfolder-init"></span>`type Init = T`

- <span id="flatteniterfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatteniterfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatteniterfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatteniterfolder-drop"></span>`unsafe fn drop(ptr: usize)`

