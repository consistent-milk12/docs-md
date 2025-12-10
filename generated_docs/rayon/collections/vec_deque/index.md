*[rayon](../../index.md) / [collections](../index.md) / [vec_deque](index.md)*

---

# Module `vec_deque`

This module contains the parallel iterator types for double-ended queues
(`VecDeque<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a double-ended queue |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a double-ended queue |
| [`IterMut`](#itermut) | struct | Parallel iterator over a mutable reference to a double-ended queue |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves a range out of a double-ended queue, but keeps the total capacity. |

## Structs

### `IntoIter<T: Send>`

```rust
struct IntoIter<T: Send> {
    inner: vec::IntoIter<T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/vec_deque.rs:17-19`](../../../../.source_1765210505/rayon-1.11.0/src/collections/vec_deque.rs#L17-L19)*

Parallel iterator over a double-ended queue

#### Trait Implementations

##### `impl<T: clone::Clone + Send> Clone for IntoIter<T>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T: fmt::Debug + Send> Debug for IntoIter<T>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IntoIter<T>`

- <span id="intoiter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

- <span id="intoiter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for IntoIter<T>`

##### `impl<T> IntoParallelIterator for IntoIter<T>`

- <span id="intoiter-type-iter"></span>`type Iter = T`

- <span id="intoiter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T>`

- <span id="intoiter-type-item"></span>`type Item = T`

- <span id="intoiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="intoiter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T>`

- <span id="intoiter-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-type-init"></span>`type Init = T`

- <span id="intoiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: Chain<slice::Iter<'a, T>, slice::Iter<'a, T>>,
}
```

*Defined in [`rayon-1.11.0/src/collections/vec_deque.rs:39-41`](../../../../.source_1765210505/rayon-1.11.0/src/collections/vec_deque.rs#L39-L41)*

Parallel iterator over an immutable reference to a double-ended queue

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, T: fmt::Debug> Debug for Iter<'a, T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: Sync> IndexedParallelIterator for Iter<'a, T>`

- <span id="iter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="iter-len"></span>`fn len(&self) -> usize`

- <span id="iter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Iter<'a, T>`

##### `impl<T> IntoParallelIterator for Iter<'a, T>`

- <span id="iter-type-iter"></span>`type Iter = T`

- <span id="iter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, T: Sync> ParallelIterator for Iter<'a, T>`

- <span id="iter-type-item"></span>`type Item = &'a T`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, T>`

- <span id="iter-const-align"></span>`const ALIGN: usize`

- <span id="iter-type-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    inner: Chain<slice::IterMut<'a, T>, slice::IterMut<'a, T>>,
}
```

*Defined in [`rayon-1.11.0/src/collections/vec_deque.rs:70-72`](../../../../.source_1765210505/rayon-1.11.0/src/collections/vec_deque.rs#L70-L72)*

Parallel iterator over a mutable reference to a double-ended queue

#### Trait Implementations

##### `impl<'a, T: fmt::Debug> Debug for IterMut<'a, T>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: Send> IndexedParallelIterator for IterMut<'a, T>`

- <span id="itermut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="itermut-len"></span>`fn len(&self) -> usize`

- <span id="itermut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for IterMut<'a, T>`

##### `impl<T> IntoParallelIterator for IterMut<'a, T>`

- <span id="itermut-type-iter"></span>`type Iter = T`

- <span id="itermut-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, T: Send> ParallelIterator for IterMut<'a, T>`

- <span id="itermut-type-item"></span>`type Item = &'a mut T`

- <span id="itermut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="itermut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'a, T>`

- <span id="itermut-const-align"></span>`const ALIGN: usize`

- <span id="itermut-type-init"></span>`type Init = T`

- <span id="itermut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-drop"></span>`unsafe fn drop(ptr: usize)`

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    deque: &'a mut std::collections::VecDeque<T>,
    range: std::ops::Range<usize>,
    orig_len: usize,
}
```

*Defined in [`rayon-1.11.0/src/collections/vec_deque.rs:94-98`](../../../../.source_1765210505/rayon-1.11.0/src/collections/vec_deque.rs#L94-L98)*

Draining parallel iterator that moves a range out of a double-ended queue,
but keeps the total capacity.

#### Trait Implementations

##### `impl<'a, T: fmt::Debug> Debug for Drain<'a, T>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Drop for Drain<'_, T>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T: Send> IndexedParallelIterator for Drain<'_, T>`

- <span id="drain-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="drain-len"></span>`fn len(&self) -> usize`

- <span id="drain-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Drain<'a, T>`

##### `impl<T> IntoParallelIterator for Drain<'a, T>`

- <span id="drain-type-iter"></span>`type Iter = T`

- <span id="drain-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Drain<'_, T>`

- <span id="drain-type-item"></span>`type Item = T`

- <span id="drain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="drain-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Drain<'a, T>`

- <span id="drain-const-align"></span>`const ALIGN: usize`

- <span id="drain-type-init"></span>`type Init = T`

- <span id="drain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-drop"></span>`unsafe fn drop(ptr: usize)`

