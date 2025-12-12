*[rayon](../../index.md) / [collections](../index.md) / [binary_heap](index.md)*

---

# Module `binary_heap`

This module contains the parallel iterator types for heaps
(`BinaryHeap<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a binary heap |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a binary heap |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves out of a binary heap, but keeps the total capacity. |

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/binary_heap.rs:15-17`](../../../../.source_1765521767/rayon-1.11.0/src/collections/binary_heap.rs#L15-L17)*

Parallel iterator over a binary heap

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T: fmt::Debug> Debug for IntoIter<T>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IntoIter<T>`

- <span id="intoiter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

- <span id="intoiter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for IntoIter<T>`

##### `impl<T> IntoParallelIterator for IntoIter<T>`

- <span id="intoiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intoiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T>`

- <span id="intoiter-paralleliterator-type-item"></span>`type Item = T`

- <span id="intoiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="intoiter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

- <span id="intoiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: slice::Iter<'a, T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/binary_heap.rs:37-39`](../../../../.source_1765521767/rayon-1.11.0/src/collections/binary_heap.rs#L37-L39)*

Parallel iterator over an immutable reference to a binary heap

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Iter<'a, T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync + 'a> IndexedParallelIterator for Iter<'a, T>`

- <span id="iter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="iter-len"></span>`fn len(&self) -> usize`

- <span id="iter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Iter<'a, T>`

##### `impl<T> IntoParallelIterator for Iter<'a, T>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync + 'a> ParallelIterator for Iter<'a, T>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = &'a T`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, T>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    heap: &'a mut std::collections::BinaryHeap<T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/binary_heap.rs:70-72`](../../../../.source_1765521767/rayon-1.11.0/src/collections/binary_heap.rs#L70-L72)*

Draining parallel iterator that moves out of a binary heap,
but keeps the total capacity.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Drain<'a, T>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Drain<'_, T>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T: Ord + Send> IndexedParallelIterator for Drain<'_, T>`

- <span id="drain-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="drain-len"></span>`fn len(&self) -> usize`

- <span id="drain-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Drain<'a, T>`

##### `impl<T> IntoParallelIterator for Drain<'a, T>`

- <span id="drain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="drain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Ord + Send> ParallelIterator for Drain<'_, T>`

- <span id="drain-paralleliterator-type-item"></span>`type Item = T`

- <span id="drain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="drain-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Drain<'a, T>`

- <span id="drain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drain-pointable-type-init"></span>`type Init = T`

- <span id="drain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-drop"></span>`unsafe fn drop(ptr: usize)`

