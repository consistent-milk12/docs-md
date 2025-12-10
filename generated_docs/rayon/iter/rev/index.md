*[rayon](../../index.md) / [iter](../index.md) / [rev](index.md)*

---

# Module `rev`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Rev`](#rev) | struct | `Rev` is an iterator that produces elements in reverse order. |
| [`RevProducer`](#revproducer) | struct |  |

## Structs

### `Rev<I>`

```rust
struct Rev<I> {
    base: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/rev.rs:11-13`](../../../../.source_1765210505/rayon-1.11.0/src/iter/rev.rs#L11-L13)*

`Rev` is an iterator that produces elements in reverse order. This struct
is created by the `rev()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="rev-new"></span>`fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Rev<I>`

- <span id="rev-clone"></span>`fn clone(&self) -> Rev<I>` — [`Rev`](#rev)

##### `impl<I: fmt::Debug> Debug for Rev<I>`

- <span id="rev-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Rev<I>`

- <span id="rev-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="rev-len"></span>`fn len(&self) -> usize`

- <span id="rev-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Rev<I>`

##### `impl<T> IntoParallelIterator for Rev<I>`

- <span id="rev-type-iter"></span>`type Iter = T`

- <span id="rev-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="rev-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Rev<I>`

- <span id="rev-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="rev-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="rev-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Rev<I>`

- <span id="rev-const-align"></span>`const ALIGN: usize`

- <span id="rev-type-init"></span>`type Init = T`

- <span id="rev-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="rev-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="rev-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="rev-drop"></span>`unsafe fn drop(ptr: usize)`

### `RevProducer<P>`

```rust
struct RevProducer<P> {
    base: P,
    len: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/rev.rs:83-86`](../../../../.source_1765210505/rayon-1.11.0/src/iter/rev.rs#L83-L86)*

#### Trait Implementations

##### `impl<T> IntoEither for RevProducer<P>`

##### `impl<T> Pointable for RevProducer<P>`

- <span id="revproducer-const-align"></span>`const ALIGN: usize`

- <span id="revproducer-type-init"></span>`type Init = T`

- <span id="revproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="revproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="revproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="revproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for RevProducer<P>`

- <span id="revproducer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="revproducer-type-intoiter"></span>`type IntoIter = Rev<<P as Producer>::IntoIter>`

- <span id="revproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="revproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="revproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="revproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

