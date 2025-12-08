*[rayon](../../index.md) / [iter](../index.md) / [copied](index.md)*

---

# Module `copied`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Copied`](#copied) | struct | `Copied` is an iterator that copies the elements of an underlying iterator. |
| [`CopiedProducer`](#copiedproducer) | struct |  |
| [`CopiedConsumer`](#copiedconsumer) | struct |  |
| [`CopiedFolder`](#copiedfolder) | struct |  |

## Structs

### `Copied<I>`

```rust
struct Copied<I> {
    base: I,
}
```

`Copied` is an iterator that copies the elements of an underlying iterator.

This struct is created by the `copied()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="copied-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Copied<I>`

- <span id="copied-clone"></span>`fn clone(&self) -> Copied<I>` — [`Copied`](../index.md)

##### `impl<I: fmt::Debug> Debug for Copied<I>`

- <span id="copied-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T, I> IndexedParallelIterator for Copied<I>`

- <span id="copied-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="copied-len"></span>`fn len(&self) -> usize`

- <span id="copied-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Copied<I>`

##### `impl<T> IntoParallelIterator for Copied<I>`

- <span id="copied-iter"></span>`type Iter = T`

- <span id="copied-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="copied-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, T, I> ParallelIterator for Copied<I>`

- <span id="copied-item"></span>`type Item = T`

- <span id="copied-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="copied-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Copied<I>`

- <span id="copied-align"></span>`const ALIGN: usize`

- <span id="copied-init"></span>`type Init = T`

- <span id="copied-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copied-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copied-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copied-drop"></span>`unsafe fn drop(ptr: usize)`

### `CopiedProducer<P>`

```rust
struct CopiedProducer<P> {
    base: P,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for CopiedProducer<P>`

##### `impl<T> Pointable for CopiedProducer<P>`

- <span id="copiedproducer-align"></span>`const ALIGN: usize`

- <span id="copiedproducer-init"></span>`type Init = T`

- <span id="copiedproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copiedproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copiedproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copiedproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, T, P> Producer for CopiedProducer<P>`

- <span id="copiedproducer-item"></span>`type Item = T`

- <span id="copiedproducer-intoiter"></span>`type IntoIter = Copied<<P as Producer>::IntoIter>`

- <span id="copiedproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="copiedproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="copiedproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="copiedproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="copiedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `CopiedConsumer<C>`

```rust
struct CopiedConsumer<C> {
    base: C,
}
```

#### Implementations

- <span id="copiedconsumer-new"></span>`fn new(base: C) -> Self`

#### Trait Implementations

##### `impl<'a, T, C> Consumer for CopiedConsumer<C>`

- <span id="copiedconsumer-folder"></span>`type Folder = CopiedFolder<<C as Consumer>::Folder>`

- <span id="copiedconsumer-reducer"></span>`type Reducer = <C as Consumer>::Reducer`

- <span id="copiedconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="copiedconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="copiedconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="copiedconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for CopiedConsumer<C>`

##### `impl<T> Pointable for CopiedConsumer<C>`

- <span id="copiedconsumer-align"></span>`const ALIGN: usize`

- <span id="copiedconsumer-init"></span>`type Init = T`

- <span id="copiedconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copiedconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copiedconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copiedconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, T, C> UnindexedConsumer for CopiedConsumer<C>`

- <span id="copiedconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="copiedconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `CopiedFolder<F>`

```rust
struct CopiedFolder<F> {
    base: F,
}
```

#### Trait Implementations

##### `impl<'a, T, F> Folder for CopiedFolder<F>`

- <span id="copiedfolder-result"></span>`type Result = <F as Folder>::Result`

- <span id="copiedfolder-consume"></span>`fn consume(self, item: &'a T) -> Self`

- <span id="copiedfolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="copiedfolder-complete"></span>`fn complete(self) -> <F as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="copiedfolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for CopiedFolder<F>`

##### `impl<T> Pointable for CopiedFolder<F>`

- <span id="copiedfolder-align"></span>`const ALIGN: usize`

- <span id="copiedfolder-init"></span>`type Init = T`

- <span id="copiedfolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="copiedfolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="copiedfolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="copiedfolder-drop"></span>`unsafe fn drop(ptr: usize)`

