*[rayon](../../index.md) / [iter](../index.md) / [par_bridge](index.md)*

---

# Module `par_bridge`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IterBridge`](#iterbridge) | struct | `IterBridge` is a parallel iterator that wraps a sequential iterator. |
| [`IterParallelProducer`](#iterparallelproducer) | struct |  |
| [`ParallelBridge`](#parallelbridge) | trait | Conversion trait to convert an `Iterator` to a `ParallelIterator`. |

## Structs

### `IterBridge<Iter>`

```rust
struct IterBridge<Iter> {
    iter: Iter,
}
```

`IterBridge` is a parallel iterator that wraps a sequential iterator.

This type is created when using the `par_bridge` method on `ParallelBridge`. See the
[`ParallelBridge`](../../prelude/index.md) documentation for details.

#### Trait Implementations

##### `impl<Iter: clone::Clone> Clone for IterBridge<Iter>`

- <span id="iterbridge-clone"></span>`fn clone(&self) -> IterBridge<Iter>` — [`IterBridge`](../index.md)

##### `impl<Iter: fmt::Debug> Debug for IterBridge<Iter>`

- <span id="iterbridge-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for IterBridge<Iter>`

##### `impl<T> IntoParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-iter"></span>`type Iter = T`

- <span id="iterbridge-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iterbridge-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<Iter> ParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-item"></span>`type Item = <Iter as Iterator>::Item`

- <span id="iterbridge-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for IterBridge<Iter>`

- <span id="iterbridge-align"></span>`const ALIGN: usize`

- <span id="iterbridge-init"></span>`type Init = T`

- <span id="iterbridge-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterbridge-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterbridge-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterbridge-drop"></span>`unsafe fn drop(ptr: usize)`

### `IterParallelProducer<'a, Iter>`

```rust
struct IterParallelProducer<'a, Iter> {
    split_count: std::sync::atomic::AtomicUsize,
    iter: std::sync::Mutex<std::iter::Fuse<Iter>>,
    threads_started: &'a [std::sync::atomic::AtomicBool],
}
```

#### Trait Implementations

##### `impl<T> IntoEither for IterParallelProducer<'a, Iter>`

##### `impl<T> Pointable for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-align"></span>`const ALIGN: usize`

- <span id="iterparallelproducer-init"></span>`type Init = T`

- <span id="iterparallelproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterparallelproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterparallelproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterparallelproducer-drop"></span>`unsafe fn drop(ptr: usize)`

## Traits

### `ParallelBridge`

```rust
trait ParallelBridge: Sized { ... }
```

Conversion trait to convert an `Iterator` to a `ParallelIterator`.

This creates a "bridge" from a sequential iterator to a parallel one, by distributing its items
across the Rayon thread pool. This has the advantage of being able to parallelize just about
anything, but the resulting `ParallelIterator` can be less efficient than if you started with
`par_iter` instead. However, it can still be useful for iterators that are difficult to
parallelize by other means, like channels or file or network I/O.

Iterator items are pulled by `next()` one at a time, synchronized from each thread that is
ready for work, so this may become a bottleneck if the serial iterator can't keep up with the
parallel demand. The items are not buffered by `IterBridge`, so it's fine to use this with
large or even unbounded iterators.

The resulting iterator is not guaranteed to keep the order of the original iterator.

# Examples

To use this trait, take an existing `Iterator` and call `par_bridge` on it. After that, you can
use any of the `ParallelIterator` methods:

```rust
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;

let rx = {
    let (tx, rx) = channel();

    tx.send("one!");
    tx.send("two!");
    tx.send("three!");

    rx
};

let mut output: Vec<&'static str> = rx.into_iter().par_bridge().collect();
output.sort_unstable();

assert_eq!(&*output, &["one!", "three!", "two!"]);
```

#### Required Methods

- `fn par_bridge(self) -> IterBridge<Self>`

  Creates a bridge from this type to a `ParallelIterator`.

#### Implementors

- `T`

