*[rayon](../../index.md) / [collections](../index.md) / [hash_set](index.md)*

---

# Module `hash_set`

This module contains the parallel iterator types for hash sets
(`HashSet<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a hash set |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a hash set |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves out of a hash set, but keeps the total capacity. |

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_set.rs:15-17`](../../../../.source_1765210505/rayon-1.11.0/src/collections/hash_set.rs#L15-L17)*

Parallel iterator over a hash set

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for IntoIter<T>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for IntoIter<T>`

##### `impl<T> IntoParallelIterator for IntoIter<T>`

- <span id="intoiter-type-iter"></span>`type Iter = T`

- <span id="intoiter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T>`

- <span id="intoiter-type-item"></span>`type Item = T`

- <span id="intoiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

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
    inner: vec::IntoIter<&'a T>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_set.rs:31-33`](../../../../.source_1765210505/rayon-1.11.0/src/collections/hash_set.rs#L31-L33)*

Parallel iterator over an immutable reference to a hash set

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, T: fmt::Debug> Debug for Iter<'a, T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Iter<'a, T>`

##### `impl<T> IntoParallelIterator for Iter<'a, T>`

- <span id="iter-type-iter"></span>`type Iter = T`

- <span id="iter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, T: Sync> ParallelIterator for Iter<'a, T>`

- <span id="iter-type-item"></span>`type Item = &'a T`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, T>`

- <span id="iter-const-align"></span>`const ALIGN: usize`

- <span id="iter-type-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    inner: vec::IntoIter<T>,
    marker: std::marker::PhantomData<&'a mut std::collections::HashSet<T>>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_set.rs:58-61`](../../../../.source_1765210505/rayon-1.11.0/src/collections/hash_set.rs#L58-L61)*

Draining parallel iterator that moves out of a hash set,
but keeps the total capacity.

#### Trait Implementations

##### `impl<'a, T: fmt::Debug> Debug for Drain<'a, T>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Drain<'a, T>`

##### `impl<T> IntoParallelIterator for Drain<'a, T>`

- <span id="drain-type-iter"></span>`type Iter = T`

- <span id="drain-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for Drain<'_, T>`

- <span id="drain-type-item"></span>`type Item = T`

- <span id="drain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="drain-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Drain<'a, T>`

- <span id="drain-const-align"></span>`const ALIGN: usize`

- <span id="drain-type-init"></span>`type Init = T`

- <span id="drain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-drop"></span>`unsafe fn drop(ptr: usize)`

