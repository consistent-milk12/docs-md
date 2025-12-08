*[rayon](../../index.md) / [iter](../index.md) / [step_by](index.md)*

---

# Module `step_by`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StepBy`](#stepby) | struct | `StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step. |
| [`StepByProducer`](#stepbyproducer) | struct |  |

## Structs

### `StepBy<I>`

```rust
struct StepBy<I> {
    base: I,
    step: usize,
}
```

`StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
This struct is created by the `step_by()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="stepby-new"></span>`fn new(base: I, step: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for StepBy<I>`

- <span id="stepby-clone"></span>`fn clone(&self) -> StepBy<I>` — [`StepBy`](../index.md)

##### `impl<I: fmt::Debug> Debug for StepBy<I>`

- <span id="stepby-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for StepBy<I>`

- <span id="stepby-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="stepby-len"></span>`fn len(&self) -> usize`

- <span id="stepby-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for StepBy<I>`

##### `impl<T> IntoParallelIterator for StepBy<I>`

- <span id="stepby-iter"></span>`type Iter = T`

- <span id="stepby-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="stepby-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for StepBy<I>`

- <span id="stepby-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="stepby-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="stepby-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for StepBy<I>`

- <span id="stepby-align"></span>`const ALIGN: usize`

- <span id="stepby-init"></span>`type Init = T`

- <span id="stepby-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stepby-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stepby-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stepby-drop"></span>`unsafe fn drop(ptr: usize)`

### `StepByProducer<P>`

```rust
struct StepByProducer<P> {
    base: P,
    step: usize,
    len: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for StepByProducer<P>`

##### `impl<T> Pointable for StepByProducer<P>`

- <span id="stepbyproducer-align"></span>`const ALIGN: usize`

- <span id="stepbyproducer-init"></span>`type Init = T`

- <span id="stepbyproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stepbyproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stepbyproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stepbyproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for StepByProducer<P>`

- <span id="stepbyproducer-item"></span>`type Item = <P as Producer>::Item`

- <span id="stepbyproducer-intoiter"></span>`type IntoIter = StepBy<<P as Producer>::IntoIter>`

- <span id="stepbyproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="stepbyproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="stepbyproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="stepbyproducer-max-len"></span>`fn max_len(&self) -> usize`

