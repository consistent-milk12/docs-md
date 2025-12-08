*[rayon](../../index.md) / [iter](../index.md) / [panic_fuse](index.md)*

---

# Module `panic_fuse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PanicFuse`](#panicfuse) | struct | `PanicFuse` is an adaptor that wraps an iterator with a fuse in case |
| [`Fuse`](#fuse) | struct | Helper that sets a bool to `true` if dropped while unwinding. |
| [`PanicFuseProducer`](#panicfuseproducer) | struct |  |
| [`PanicFuseIter`](#panicfuseiter) | struct |  |
| [`PanicFuseConsumer`](#panicfuseconsumer) | struct |  |
| [`PanicFuseFolder`](#panicfusefolder) | struct |  |
| [`PanicFuseReducer`](#panicfusereducer) | struct |  |

## Structs

### `PanicFuse<I>`

```rust
struct PanicFuse<I> {
    base: I,
}
```

`PanicFuse` is an adaptor that wraps an iterator with a fuse in case
of panics, to halt all threads as soon as possible.

This struct is created by the `panic_fuse()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="panicfuse-new"></span>`fn new(base: I) -> PanicFuse<I>` — [`PanicFuse`](../index.md)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for PanicFuse<I>`

- <span id="panicfuse-clone"></span>`fn clone(&self) -> PanicFuse<I>` — [`PanicFuse`](../index.md)

##### `impl<I: fmt::Debug> Debug for PanicFuse<I>`

- <span id="panicfuse-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for PanicFuse<I>`

- <span id="panicfuse-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="panicfuse-len"></span>`fn len(&self) -> usize`

- <span id="panicfuse-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for PanicFuse<I>`

##### `impl<T> IntoParallelIterator for PanicFuse<I>`

- <span id="panicfuse-iter"></span>`type Iter = T`

- <span id="panicfuse-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="panicfuse-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for PanicFuse<I>`

- <span id="panicfuse-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="panicfuse-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="panicfuse-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for PanicFuse<I>`

- <span id="panicfuse-align"></span>`const ALIGN: usize`

- <span id="panicfuse-init"></span>`type Init = T`

- <span id="panicfuse-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuse-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuse-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuse-drop"></span>`unsafe fn drop(ptr: usize)`

### `Fuse<'a>`

```rust
struct Fuse<'a>(&'a std::sync::atomic::AtomicBool);
```

Helper that sets a bool to `true` if dropped while unwinding.

#### Implementations

- <span id="fuse-panicked"></span>`fn panicked(&self) -> bool`

#### Trait Implementations

##### `impl<'a> Clone for Fuse<'a>`

- <span id="fuse-clone"></span>`fn clone(&self) -> Fuse<'a>` — [`Fuse`](#fuse)

##### `impl<'a> Drop for Fuse<'a>`

- <span id="fuse-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for Fuse<'a>`

##### `impl<T> Pointable for Fuse<'a>`

- <span id="fuse-align"></span>`const ALIGN: usize`

- <span id="fuse-init"></span>`type Init = T`

- <span id="fuse-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="fuse-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="fuse-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="fuse-drop"></span>`unsafe fn drop(ptr: usize)`

### `PanicFuseProducer<'a, P>`

```rust
struct PanicFuseProducer<'a, P> {
    base: P,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for PanicFuseProducer<'a, P>`

##### `impl<T> Pointable for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-align"></span>`const ALIGN: usize`

- <span id="panicfuseproducer-init"></span>`type Init = T`

- <span id="panicfuseproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, P> Producer for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-item"></span>`type Item = <P as Producer>::Item`

- <span id="panicfuseproducer-intoiter"></span>`type IntoIter = PanicFuseIter<'a, <P as Producer>::IntoIter>`

- <span id="panicfuseproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="panicfuseproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="panicfuseproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="panicfuseproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="panicfuseproducer-fold-with"></span>`fn fold_with<G>(self, folder: G) -> G`

### `PanicFuseIter<'a, I>`

```rust
struct PanicFuseIter<'a, I> {
    base: I,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<'a, I> DoubleEndedIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<'a, I> ExactSizeIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-len"></span>`fn len(&self) -> usize`

##### `impl<T> IntoEither for PanicFuseIter<'a, I>`

##### `impl<I> IntoIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="panicfuseiter-intoiter"></span>`type IntoIter = I`

- <span id="panicfuseiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, I> Iterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="panicfuseiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="panicfuseiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-align"></span>`const ALIGN: usize`

- <span id="panicfuseiter-init"></span>`type Init = T`

- <span id="panicfuseiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `PanicFuseConsumer<'a, C>`

```rust
struct PanicFuseConsumer<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<'a, T, C> Consumer for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-folder"></span>`type Folder = PanicFuseFolder<'a, <C as Consumer>::Folder>`

- <span id="panicfuseconsumer-reducer"></span>`type Reducer = PanicFuseReducer<'a, <C as Consumer>::Reducer>`

- <span id="panicfuseconsumer-result"></span>`type Result = <C as Consumer>::Result`

- <span id="panicfuseconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md)

- <span id="panicfuseconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md)

- <span id="panicfuseconsumer-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for PanicFuseConsumer<'a, C>`

##### `impl<T> Pointable for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-align"></span>`const ALIGN: usize`

- <span id="panicfuseconsumer-init"></span>`type Init = T`

- <span id="panicfuseconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, T, C> UnindexedConsumer for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="panicfuseconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md)

### `PanicFuseFolder<'a, C>`

```rust
struct PanicFuseFolder<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<'a, T, C> Folder for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-result"></span>`type Result = <C as Folder>::Result`

- <span id="panicfusefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="panicfusefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="panicfusefolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md)

- <span id="panicfusefolder-full"></span>`fn full(&self) -> bool`

##### `impl<T> IntoEither for PanicFuseFolder<'a, C>`

##### `impl<T> Pointable for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-align"></span>`const ALIGN: usize`

- <span id="panicfusefolder-init"></span>`type Init = T`

- <span id="panicfusefolder-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfusefolder-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfusefolder-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfusefolder-drop"></span>`unsafe fn drop(ptr: usize)`

### `PanicFuseReducer<'a, C>`

```rust
struct PanicFuseReducer<'a, C> {
    base: C,
    _fuse: Fuse<'a>,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for PanicFuseReducer<'a, C>`

##### `impl<T> Pointable for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-align"></span>`const ALIGN: usize`

- <span id="panicfusereducer-init"></span>`type Init = T`

- <span id="panicfusereducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfusereducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfusereducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfusereducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<'a, T, C> Reducer for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

