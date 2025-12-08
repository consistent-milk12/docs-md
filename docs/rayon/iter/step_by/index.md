*[rayon](../../index.md) / [iter](../index.md) / [step_by](index.md)*

---

# Module `step_by`

## Structs

### `StepBy<I>`

```rust
struct StepBy<I> {
    base: I,
    step: usize,
}
```

`StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step.
This struct is created by the `step_by()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, step: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for StepBy<I>`

- `fn clone(self: &Self) -> StepBy<I>` — [`StepBy`](#stepby)

##### `impl<I: $crate::fmt::Debug> Debug for StepBy<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for StepBy<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for StepBy<I>`

##### `impl<T> IntoParallelIterator for StepBy<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for StepBy<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for StepBy<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

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

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for StepByProducer<P>`

- `type Item = <P as Producer>::Item`

- `type IntoIter = StepBy<<P as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

