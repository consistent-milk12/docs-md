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

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:72-74`](../../../../.source_1765521767/rayon-1.11.0/src/iter/par_bridge.rs#L72-L74)*

`IterBridge` is a parallel iterator that wraps a sequential iterator.

This type is created when using the `par_bridge` method on `ParallelBridge`. See the
[`ParallelBridge`](#parallelbridge) documentation for details.

#### Trait Implementations

##### `impl Any for IterBridge<Iter>`

- <span id="iterbridge-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterBridge<Iter>`

- <span id="iterbridge-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterBridge<Iter>`

- <span id="iterbridge-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Iter: clone::Clone> Clone for IterBridge<Iter>`

- <span id="iterbridge-clone"></span>`fn clone(&self) -> IterBridge<Iter>` — [`IterBridge`](#iterbridge)

##### `impl CloneToUninit for IterBridge<Iter>`

- <span id="iterbridge-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Iter: fmt::Debug> Debug for IterBridge<Iter>`

- <span id="iterbridge-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IterBridge<Iter>`

- <span id="iterbridge-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterBridge<Iter>`

- <span id="iterbridge-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IterBridge<Iter>`

##### `impl IntoParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iterbridge-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iterbridge-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<Iter> ParallelIterator for IterBridge<Iter>`

- <span id="iterbridge-paralleliterator-type-item"></span>`type Item = <Iter as Iterator>::Item`

- <span id="iterbridge-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

##### `impl Pointable for IterBridge<Iter>`

- <span id="iterbridge-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iterbridge-pointable-type-init"></span>`type Init = T`

- <span id="iterbridge-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterbridge-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterbridge-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterbridge-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for IterBridge<Iter>`

- <span id="iterbridge-toowned-type-owned"></span>`type Owned = T`

- <span id="iterbridge-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iterbridge-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IterBridge<Iter>`

- <span id="iterbridge-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterbridge-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterBridge<Iter>`

- <span id="iterbridge-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterbridge-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IterParallelProducer<'a, Iter>`

```rust
struct IterParallelProducer<'a, Iter> {
    split_count: std::sync::atomic::AtomicUsize,
    iter: std::sync::Mutex<std::iter::Fuse<Iter>>,
    threads_started: &'a [std::sync::atomic::AtomicBool],
}
```

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:100-104`](../../../../.source_1765521767/rayon-1.11.0/src/iter/par_bridge.rs#L100-L104)*

#### Trait Implementations

##### `impl Any for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IterParallelProducer<'a, Iter>`

##### `impl Pointable for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iterparallelproducer-pointable-type-init"></span>`type Init = T`

- <span id="iterparallelproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterparallelproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterparallelproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterparallelproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterparallelproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IterParallelProducer<'a, Iter>`

- <span id="iterparallelproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterparallelproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<Iter: Iterator + Send> UnindexedProducer for &IterParallelProducer<'_, Iter>`

- <span id="iterparallelproducer-unindexedproducer-type-item"></span>`type Item = <Iter as Iterator>::Item`

- <span id="iterparallelproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="iterparallelproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Traits

### `ParallelBridge`

```rust
trait ParallelBridge: Sized { ... }
```

*Defined in [`rayon-1.11.0/src/iter/par_bridge.rs:53-56`](../../../../.source_1765521767/rayon-1.11.0/src/iter/par_bridge.rs#L53-L56)*

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

