*[rayon](../../index.md) / [iter](../index.md) / [cloned](index.md)*

---

# Module `cloned`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Cloned`](#cloned) | struct | `Cloned` is an iterator that clones the elements of an underlying iterator. |
| [`ClonedProducer`](#clonedproducer) | struct |  |
| [`ClonedConsumer`](#clonedconsumer) | struct |  |
| [`ClonedFolder`](#clonedfolder) | struct |  |

## Structs

### `Cloned<I>`

```rust
struct Cloned<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:13-15`](../../../../.source_1765210505/rayon-1.11.0/src/iter/cloned.rs#L13-L15)*

`Cloned` is an iterator that clones the elements of an underlying iterator.

This struct is created by the `cloned()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="cloned-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Cloned<I>`

- <span id="cloned-clone"></span>`fn clone(&self) -> Cloned<I>` — [`Cloned`](#cloned)

##### `impl<I: fmt::Debug> Debug for Cloned<I>`

- <span id="cloned-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T, I> IndexedParallelIterator for Cloned<I>`

- <span id="cloned-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="cloned-len"></span>`fn len(&self) -> usize`

- <span id="cloned-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Cloned<I>`

##### `impl<T> IntoParallelIterator for Cloned<I>`

- <span id="cloned-type-iter"></span>`type Iter = T`

- <span id="cloned-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="cloned-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, T, I> ParallelIterator for Cloned<I>`

- <span id="cloned-type-item"></span>`type Item = T`

- <span id="cloned-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="cloned-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Cloned<I>`

- <span id="cloned-const-align"></span>`const ALIGN: usize`

- <span id="cloned-type-init"></span>`type Init = T`

- <span id="cloned-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="cloned-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="cloned-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="cloned-drop"></span>`unsafe fn drop(ptr: usize)`

### `ClonedProducer<P>`

```rust
struct ClonedProducer<P> {
    base: P,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:91-93`](../../../../.source_1765210505/rayon-1.11.0/src/iter/cloned.rs#L91-L93)*

#### Trait Implementations

##### `impl<T> IntoEither for ClonedProducer<P>`

##### `impl<T> Pointable for ClonedProducer<P>`

- <span id="clonedproducer-const-align"></span>`const ALIGN: usize`

- <span id="clonedproducer-type-init"></span>`type Init = T`

- <span id="clonedproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clonedproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clonedproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clonedproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, T, P> Producer for ClonedProducer<P>`

- <span id="clonedproducer-type-item"></span>`type Item = T`

- <span id="clonedproducer-type-intoiter"></span>`type IntoIter = Cloned<<P as Producer>::IntoIter>`

- <span id="clonedproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="clonedproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="clonedproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="clonedproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="clonedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `ClonedConsumer<C>`

```rust
struct ClonedConsumer<C> {
    base: C,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:134-136`](../../../../.source_1765210505/rayon-1.11.0/src/iter/cloned.rs#L134-L136)*

#### Implementations

- <span id="clonedconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<'a, T, C> Consumer for ClonedConsumer<C>`

- <span id="clonedconsumer-type-folder"></span>`type Folder = ClonedFolder<<C as Consumer>::Folder>`

- <span id="clonedconsumer-type-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="clonedconsumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="clonedconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="clonedconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="clonedconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ClonedConsumer<C>`

##### `impl<T> Pointable for ClonedConsumer<C>`

- <span id="clonedconsumer-const-align"></span>`const ALIGN: usize`

- <span id="clonedconsumer-type-init"></span>`type Init = T`

- <span id="clonedconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clonedconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clonedconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clonedconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, T, C> UnindexedConsumer for ClonedConsumer<C>`

- <span id="clonedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="clonedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `ClonedFolder<F>`

```rust
struct ClonedFolder<F> {
    base: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/cloned.rs:187-189`](../../../../.source_1765210505/rayon-1.11.0/src/iter/cloned.rs#L187-L189)*

#### Trait Implementations

##### `impl<'a, T, F> Folder for ClonedFolder<F>`

- <span id="clonedfolder-type-result"></span>`type Result = <F as Folder>::Result`

- <span id="clonedfolder-consume"></span>`fn consume(self, item: &'a T) -> Self`

- <span id="clonedfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="clonedfolder-complete"></span>`fn complete(self) -> <F as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="clonedfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for ClonedFolder<F>`

##### `impl<T> Pointable for ClonedFolder<F>`

- <span id="clonedfolder-const-align"></span>`const ALIGN: usize`

- <span id="clonedfolder-type-init"></span>`type Init = T`

- <span id="clonedfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="clonedfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="clonedfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="clonedfolder-drop"></span>`unsafe fn drop(ptr: usize)`

