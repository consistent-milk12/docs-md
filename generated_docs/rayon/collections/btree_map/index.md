*[rayon](../../index.md) / [collections](../index.md) / [btree_map](index.md)*

---

# Module `btree_map`

This module contains the parallel iterator types for B-Tree maps
(`BTreeMap<K, V>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a B-Tree map |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a B-Tree map |
| [`IterMut`](#itermut) | struct | Parallel iterator over a mutable reference to a B-Tree map |

## Structs

### `IntoIter<K, V>`

```rust
struct IntoIter<K, V> {
    inner: vec::IntoIter<(K, V)>,
}
```

Parallel iterator over a B-Tree map

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IntoIter<K, V>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for IntoIter<K, V>`

##### `impl<T> IntoParallelIterator for IntoIter<K, V>`

- <span id="intoiter-iter"></span>`type Iter = T`

- <span id="intoiter-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Send, V: Send> ParallelIterator for IntoIter<K, V>`

- <span id="intoiter-item"></span>`type Item = (K, V)`

- <span id="intoiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="intoiter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<K, V>`

- <span id="intoiter-align"></span>`const ALIGN: usize`

- <span id="intoiter-init"></span>`type Init = T`

- <span id="intoiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-drop"></span>`unsafe fn drop(ptr: usize)`

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a V)>,
}
```

Parallel iterator over an immutable reference to a B-Tree map

#### Trait Implementations

##### `impl<K, V> Clone for Iter<'_, K, V>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a, K: fmt::Debug, V: fmt::Debug> Debug for Iter<'a, K, V>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Iter<'a, K, V>`

##### `impl<T> IntoParallelIterator for Iter<'a, K, V>`

- <span id="iter-iter"></span>`type Iter = T`

- <span id="iter-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, K: Sync + 'a, V: Sync + 'a> ParallelIterator for Iter<'a, K, V>`

- <span id="iter-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, K, V>`

- <span id="iter-align"></span>`const ALIGN: usize`

- <span id="iter-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a mut V)>,
}
```

Parallel iterator over a mutable reference to a B-Tree map

#### Trait Implementations

##### `impl<'a, K: fmt::Debug, V: fmt::Debug> Debug for IterMut<'a, K, V>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for IterMut<'a, K, V>`

##### `impl<T> IntoParallelIterator for IterMut<'a, K, V>`

- <span id="itermut-iter"></span>`type Iter = T`

- <span id="itermut-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a, K: Sync + 'a, V: Send + 'a> ParallelIterator for IterMut<'a, K, V>`

- <span id="itermut-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- <span id="itermut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'a, K, V>`

- <span id="itermut-align"></span>`const ALIGN: usize`

- <span id="itermut-init"></span>`type Init = T`

- <span id="itermut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-drop"></span>`unsafe fn drop(ptr: usize)`

