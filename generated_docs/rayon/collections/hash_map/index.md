*[rayon](../../index.md) / [collections](../index.md) / [hash_map](index.md)*

---

# Module `hash_map`

This module contains the parallel iterator types for hash maps
(`HashMap<K, V>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator over a hash map |
| [`Iter`](#iter) | struct | Parallel iterator over an immutable reference to a hash map |
| [`IterMut`](#itermut) | struct | Parallel iterator over a mutable reference to a hash map |
| [`Drain`](#drain) | struct | Draining parallel iterator that moves out of a hash map, but keeps the total capacity. |

## Structs

### `IntoIter<K, V>`

```rust
struct IntoIter<K, V> {
    inner: vec::IntoIter<(K, V)>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:15-17`](../../../../.source_1765210505/rayon-1.11.0/src/collections/hash_map.rs#L15-L17)*

Parallel iterator over a hash map

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IntoIter<K, V>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for IntoIter<K, V>`

##### `impl IntoParallelIterator for IntoIter<K, V>`

- <span id="intoiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intoiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Send, V: Send> ParallelIterator for IntoIter<K, V>`

- <span id="intoiter-paralleliterator-type-item"></span>`type Item = (K, V)`

- <span id="intoiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="intoiter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for IntoIter<K, V>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:31-33`](../../../../.source_1765210505/rayon-1.11.0/src/collections/hash_map.rs#L31-L33)*

Parallel iterator over an immutable reference to a hash map

#### Trait Implementations

##### `impl<K, V> Clone for Iter<'_, K, V>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Iter<'a, K, V>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for Iter<'a, K, V>`

##### `impl IntoParallelIterator for Iter<'a, K, V>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Sync, V: Sync> ParallelIterator for Iter<'a, K, V>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Iter<'a, K, V>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

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

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:55-57`](../../../../.source_1765210505/rayon-1.11.0/src/collections/hash_map.rs#L55-L57)*

Parallel iterator over a mutable reference to a hash map

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for IterMut<'a, K, V>`

- <span id="itermut-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for IterMut<'a, K, V>`

##### `impl IntoParallelIterator for IterMut<'a, K, V>`

- <span id="itermut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="itermut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="itermut-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Sync, V: Send> ParallelIterator for IterMut<'a, K, V>`

- <span id="itermut-paralleliterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="itermut-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for IterMut<'a, K, V>`

- <span id="itermut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="itermut-pointable-type-init"></span>`type Init = T`

- <span id="itermut-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="itermut-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="itermut-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="itermut-drop"></span>`unsafe fn drop(ptr: usize)`

### `Drain<'a, K, V>`

```rust
struct Drain<'a, K, V> {
    inner: vec::IntoIter<(K, V)>,
    marker: std::marker::PhantomData<&'a mut std::collections::HashMap<K, V>>,
}
```

*Defined in [`rayon-1.11.0/src/collections/hash_map.rs:72-75`](../../../../.source_1765210505/rayon-1.11.0/src/collections/hash_map.rs#L72-L75)*

Draining parallel iterator that moves out of a hash map,
but keeps the total capacity.

#### Trait Implementations

##### `impl<K: fmt::Debug, V: fmt::Debug> Debug for Drain<'a, K, V>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoEither for Drain<'a, K, V>`

##### `impl IntoParallelIterator for Drain<'a, K, V>`

- <span id="drain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="drain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<K: Send, V: Send> ParallelIterator for Drain<'_, K, V>`

- <span id="drain-paralleliterator-type-item"></span>`type Item = (K, V)`

- <span id="drain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

- <span id="drain-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Drain<'a, K, V>`

- <span id="drain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drain-pointable-type-init"></span>`type Init = T`

- <span id="drain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-drop"></span>`unsafe fn drop(ptr: usize)`

