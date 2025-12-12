*[rayon](../index.md) / [vec](index.md)*

---

# Module `vec`

Parallel iterator types for [vectors] (`Vec<T>`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator that moves out of a vector. |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves a range out of a vector, but keeps the total capacity. |
| [`DrainProducer`](#drainproducer) | struct |  |
| [`SliceDrain`](#slicedrain) | struct |  |

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    vec: Vec<T>,
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:38-40`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L38-L40)*

Parallel iterator that moves out of a vector.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T: fmt::Debug> Debug for IntoIter<T>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IntoIter<T>`

- <span id="intoiter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

- <span id="intoiter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for IntoIter<T>`

##### `impl<T> IntoParallelIterator for IntoIter<T>`

- <span id="intoiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intoiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T>`

- <span id="intoiter-paralleliterator-type-item"></span>`type Item = T`

- <span id="intoiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

- <span id="intoiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Drain<'data, T: Send>`

```rust
struct Drain<'data, T: Send> {
    vec: &'data mut Vec<T>,
    range: std::ops::Range<usize>,
    orig_len: usize,
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:111-115`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L111-L115)*

Draining parallel iterator that moves a range out of a vector, but keeps the total capacity.

#### Trait Implementations

##### `impl<T: fmt::Debug + Send> Debug for Drain<'data, T>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> Drop for Drain<'data, T>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T: Send> IndexedParallelIterator for Drain<'data, T>`

- <span id="drain-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="drain-len"></span>`fn len(&self) -> usize`

- <span id="drain-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Drain<'data, T>`

##### `impl<T> IntoParallelIterator for Drain<'data, T>`

- <span id="drain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="drain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Drain<'data, T>`

- <span id="drain-paralleliterator-type-item"></span>`type Item = T`

- <span id="drain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="drain-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Drain<'data, T>`

- <span id="drain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drain-pointable-type-init"></span>`type Init = T`

- <span id="drain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-drop"></span>`unsafe fn drop(ptr: usize)`

### `DrainProducer<'data, T: Send>`

```rust
struct DrainProducer<'data, T: Send> {
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:188-190`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L188-L190)*

#### Implementations

- <span id="drainproducer-new"></span>`unsafe fn new(slice: &mut [T]) -> DrainProducer<'_, T>` — [`DrainProducer`](#drainproducer)

- <span id="drainproducer-from-vec"></span>`unsafe fn from_vec(vec: &mut Vec<T>, len: usize) -> DrainProducer<'_, T>` — [`DrainProducer`](#drainproducer)

#### Trait Implementations

##### `impl<T: 'data + Send> Drop for DrainProducer<'data, T>`

- <span id="drainproducer-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for DrainProducer<'data, T>`

##### `impl<T> Pointable for DrainProducer<'data, T>`

- <span id="drainproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drainproducer-pointable-type-init"></span>`type Init = T`

- <span id="drainproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drainproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drainproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drainproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: 'data + Send> Producer for DrainProducer<'data, T>`

- <span id="drainproducer-producer-type-item"></span>`type Item = T`

- <span id="drainproducer-producer-type-intoiter"></span>`type IntoIter = SliceDrain<'data, T>`

- <span id="drainproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="drainproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

### `SliceDrain<'data, T>`

```rust
struct SliceDrain<'data, T> {
    iter: slice::IterMut<'data, T>,
}
```

*Defined in [`rayon-1.11.0/src/vec.rs:246-248`](../../../.source_1765521767/rayon-1.11.0/src/vec.rs#L246-L248)*

#### Trait Implementations

##### `impl<T: 'data> DoubleEndedIterator for SliceDrain<'data, T>`

- <span id="slicedrain-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: 'data> Drop for SliceDrain<'data, T>`

- <span id="slicedrain-drop"></span>`fn drop(&mut self)`

##### `impl<T: 'data> ExactSizeIterator for SliceDrain<'data, T>`

- <span id="slicedrain-len"></span>`fn len(&self) -> usize`

##### `impl<T: 'data> FusedIterator for SliceDrain<'data, T>`

##### `impl<T> IntoEither for SliceDrain<'data, T>`

##### `impl IntoIterator for SliceDrain<'data, T>`

- <span id="slicedrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="slicedrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="slicedrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'data> Iterator for SliceDrain<'data, T>`

- <span id="slicedrain-iterator-type-item"></span>`type Item = T`

- <span id="slicedrain-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="slicedrain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="slicedrain-count"></span>`fn count(self) -> usize`

##### `impl<T> Pointable for SliceDrain<'data, T>`

- <span id="slicedrain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="slicedrain-pointable-type-init"></span>`type Init = T`

- <span id="slicedrain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="slicedrain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="slicedrain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="slicedrain-drop"></span>`unsafe fn drop(ptr: usize)`

