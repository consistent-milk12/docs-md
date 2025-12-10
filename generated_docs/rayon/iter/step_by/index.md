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

*Defined in [`rayon-1.11.0/src/iter/step_by.rs:11-14`](../../../../.source_1765210505/rayon-1.11.0/src/iter/step_by.rs#L11-L14)*

`StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
This struct is created by the `step_by()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="stepby-new"></span>`fn new(base: I, step: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for StepBy<I>`

- <span id="stepby-clone"></span>`fn clone(&self) -> StepBy<I>` — [`StepBy`](#stepby)

##### `impl<I: fmt::Debug> Debug for StepBy<I>`

- <span id="stepby-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for StepBy<I>`

- <span id="stepby-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="stepby-len"></span>`fn len(&self) -> usize`

- <span id="stepby-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for StepBy<I>`

##### `impl<T> IntoParallelIterator for StepBy<I>`

- <span id="stepby-type-iter"></span>`type Iter = T`

- <span id="stepby-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="stepby-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for StepBy<I>`

- <span id="stepby-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="stepby-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="stepby-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for StepBy<I>`

- <span id="stepby-const-align"></span>`const ALIGN: usize`

- <span id="stepby-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/iter/step_by.rs:93-97`](../../../../.source_1765210505/rayon-1.11.0/src/iter/step_by.rs#L93-L97)*

#### Trait Implementations

##### `impl<T> IntoEither for StepByProducer<P>`

##### `impl<T> Pointable for StepByProducer<P>`

- <span id="stepbyproducer-const-align"></span>`const ALIGN: usize`

- <span id="stepbyproducer-type-init"></span>`type Init = T`

- <span id="stepbyproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="stepbyproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="stepbyproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="stepbyproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for StepByProducer<P>`

- <span id="stepbyproducer-type-item"></span>`type Item = <P as Producer>::Item`

- <span id="stepbyproducer-type-intoiter"></span>`type IntoIter = StepBy<<P as Producer>::IntoIter>`

- <span id="stepbyproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="stepbyproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="stepbyproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="stepbyproducer-max-len"></span>`fn max_len(&self) -> usize`

