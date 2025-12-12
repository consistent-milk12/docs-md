*[rayon](../index.md) / [result](index.md)*

---

# Module `result`

Parallel iterator types for [results]

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a result |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a result |
| [`IterMut`](#itermut) | struct | Parallel iterator over a mutable reference to a result |

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    inner: option::IntoIter<T>,
}
```

*Defined in [`rayon-1.11.0/src/result.rs:16-18`](../../../.source_1765521767/rayon-1.11.0/src/result.rs#L16-L18)*

Parallel iterator over a result

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

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: option::IntoIter<&'a T>,
}
```

*Defined in [`rayon-1.11.0/src/result.rs:38-40`](../../../.source_1765521767/rayon-1.11.0/src/result.rs#L38-L40)*

Parallel iterator over an immutable reference to a result

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug> Debug for Iter<'a, T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Sync> IndexedParallelIterator for Iter<'a, T>`

- <span id="iter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-len"></span>`fn len(&self) -> usize`

- <span id="iter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for Iter<'a, T>`

##### `impl<T> IntoParallelIterator for Iter<'a, T>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Sync> ParallelIterator for Iter<'a, T>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = &'a T`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, T>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    inner: option::IntoIter<&'a mut T>,
}
```

*Defined in [`rayon-1.11.0/src/result.rs:68-70`](../../../.source_1765521767/rayon-1.11.0/src/result.rs#L68-L70)*

Parallel iterator over a mutable reference to a result

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for IterMut<'a, T>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IterMut<'a, T>`

- <span id="itermut-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-len"></span>`fn len(&self) -> usize`

- <span id="itermut-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for IterMut<'a, T>`

##### `impl<T> IntoParallelIterator for IterMut<'a, T>`

- <span id="itermut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="itermut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IterMut<'a, T>`

- <span id="itermut-paralleliterator-type-item"></span>`type Item = &'a mut T`

- <span id="itermut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="itermut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'a, T>`

- <span id="itermut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermut-pointable-type-init"></span>`type Init = T`

- <span id="itermut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-drop"></span>`unsafe fn drop(ptr: usize)`

