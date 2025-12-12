*[rayon](../../index.md) / [iter](../index.md) / [panic_fuse](index.md)*

---

# Module `panic_fuse`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PanicFuse`](#panicfuse) | struct | `PanicFuse` is an adaptor that wraps an iterator with a fuse in case of panics, to halt all threads as soon as possible. |
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

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:14-16`](../../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L14-L16)*

`PanicFuse` is an adaptor that wraps an iterator with a fuse in case
of panics, to halt all threads as soon as possible.

This struct is created by the `panic_fuse()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="panicfuse-new"></span>`fn new(base: I) -> PanicFuse<I>` — [`PanicFuse`](#panicfuse)

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for PanicFuse<I>`

- <span id="panicfuse-clone"></span>`fn clone(&self) -> PanicFuse<I>` — [`PanicFuse`](#panicfuse)

##### `impl<I: fmt::Debug> Debug for PanicFuse<I>`

- <span id="panicfuse-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for PanicFuse<I>`

- <span id="panicfuse-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuse-len"></span>`fn len(&self) -> usize`

- <span id="panicfuse-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl IntoEither for PanicFuse<I>`

##### `impl IntoParallelIterator for PanicFuse<I>`

- <span id="panicfuse-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="panicfuse-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="panicfuse-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for PanicFuse<I>`

- <span id="panicfuse-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="panicfuse-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuse-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for PanicFuse<I>`

- <span id="panicfuse-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuse-pointable-type-init"></span>`type Init = T`

- <span id="panicfuse-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuse-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuse-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuse-drop"></span>`unsafe fn drop(ptr: usize)`

### `Fuse<'a>`

```rust
struct Fuse<'a>(&'a std::sync::atomic::AtomicBool);
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:20`](../../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L20)*

Helper that sets a bool to `true` if dropped while unwinding.

#### Implementations

- <span id="fuse-panicked"></span>`fn panicked(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Fuse<'a>`

- <span id="fuse-clone"></span>`fn clone(&self) -> Fuse<'a>` — [`Fuse`](#fuse)

##### `impl Drop for Fuse<'a>`

- <span id="fuse-drop"></span>`fn drop(&mut self)`

##### `impl IntoEither for Fuse<'a>`

##### `impl Pointable for Fuse<'a>`

- <span id="fuse-pointable-const-align"></span>`const ALIGN: usize`

- <span id="fuse-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:122-125`](../../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L122-L125)*

#### Trait Implementations

##### `impl IntoEither for PanicFuseProducer<'a, P>`

##### `impl Pointable for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuseproducer-pointable-type-init"></span>`type Init = T`

- <span id="panicfuseproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for PanicFuseProducer<'a, P>`

- <span id="panicfuseproducer-producer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="panicfuseproducer-producer-type-intoiter"></span>`type IntoIter = PanicFuseIter<'a, <P as Producer>::IntoIter>`

- <span id="panicfuseproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

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

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:174-177`](../../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L174-L177)*

#### Trait Implementations

##### `impl<I> DoubleEndedIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I> ExactSizeIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-len"></span>`fn len(&self) -> usize`

##### `impl IntoEither for PanicFuseIter<'a, I>`

##### `impl<I> IntoIterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="panicfuseiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="panicfuseiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="panicfuseiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="panicfuseiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for PanicFuseIter<'a, I>`

- <span id="panicfuseiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuseiter-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:223-226`](../../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L223-L226)*

#### Trait Implementations

##### `impl<T, C> Consumer for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-consumer-type-folder"></span>`type Folder = PanicFuseFolder<'a, <C as Consumer>::Folder>`

- <span id="panicfuseconsumer-consumer-type-reducer"></span>`type Reducer = PanicFuseReducer<'a, <C as Consumer>::Reducer>`

- <span id="panicfuseconsumer-consumer-type-result"></span>`type Result = <C as Consumer>::Result`

- <span id="panicfuseconsumer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self, <Self as >::Reducer)` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuseconsumer-into-folder"></span>`fn into_folder(self) -> <Self as >::Folder` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="panicfuseconsumer-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for PanicFuseConsumer<'a, C>`

##### `impl Pointable for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfuseconsumer-pointable-type-init"></span>`type Init = T`

- <span id="panicfuseconsumer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfuseconsumer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfuseconsumer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfuseconsumer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C> UnindexedConsumer for PanicFuseConsumer<'a, C>`

- <span id="panicfuseconsumer-split-off-left"></span>`fn split_off_left(&self) -> Self`

- <span id="panicfuseconsumer-to-reducer"></span>`fn to_reducer(&self) -> <Self as >::Reducer` — [`Consumer`](../plumbing/index.md#consumer)

### `PanicFuseFolder<'a, C>`

```rust
struct PanicFuseFolder<'a, C> {
    base: C,
    fuse: Fuse<'a>,
}
```

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:285-288`](../../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L285-L288)*

#### Trait Implementations

##### `impl<T, C> Folder for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-folder-type-result"></span>`type Result = <C as Folder>::Result`

- <span id="panicfusefolder-consume"></span>`fn consume(self, item: T) -> Self`

- <span id="panicfusefolder-consume-iter"></span>`fn consume_iter<I>(self, iter: I) -> Self`

- <span id="panicfusefolder-complete"></span>`fn complete(self) -> <C as >::Result` — [`Folder`](../plumbing/index.md#folder)

- <span id="panicfusefolder-full"></span>`fn full(&self) -> bool`

##### `impl IntoEither for PanicFuseFolder<'a, C>`

##### `impl Pointable for PanicFuseFolder<'a, C>`

- <span id="panicfusefolder-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfusefolder-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/panic_fuse.rs:326-329`](../../../../.source_1765210505/rayon-1.11.0/src/iter/panic_fuse.rs#L326-L329)*

#### Trait Implementations

##### `impl IntoEither for PanicFuseReducer<'a, C>`

##### `impl Pointable for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="panicfusereducer-pointable-type-init"></span>`type Init = T`

- <span id="panicfusereducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="panicfusereducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="panicfusereducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="panicfusereducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, C> Reducer for PanicFuseReducer<'a, C>`

- <span id="panicfusereducer-reduce"></span>`fn reduce(self, left: T, right: T) -> T`

