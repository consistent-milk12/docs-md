*[rayon](../../index.md) / [collections](../index.md) / [btree_map](index.md)*

---

# Module `btree_map`

This module contains the parallel iterator types for B-Tree maps
(`BTreeMap<K, V>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Structs

### `IntoIter<K, V>`

```rust
struct IntoIter<K, V> {
    inner: vec::IntoIter<(K, V)>,
}
```

Parallel iterator over a B-Tree map

#### Trait Implementations

##### `impl<K: $crate::fmt::Debug, V: $crate::fmt::Debug> Debug for IntoIter<K, V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for IntoIter<K, V>`

##### `impl<T> IntoParallelIterator for IntoIter<K, V>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<K: Send, V: Send> ParallelIterator for IntoIter<K, V>`

- `type Item = (K, V)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<K, V>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Iter<'a, K, V>`

```rust
struct Iter<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a V)>,
}
```

Parallel iterator over an immutable reference to a B-Tree map

#### Trait Implementations

##### `impl<K, V> Clone for Iter<'_, K, V>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, K: $crate::fmt::Debug, V: $crate::fmt::Debug> Debug for Iter<'a, K, V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Iter<'a, K, V>`

##### `impl<T> IntoParallelIterator for Iter<'a, K, V>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a, K: Sync + 'a, V: Sync + 'a> ParallelIterator for Iter<'a, K, V>`

- `type Item = (&'a K, &'a V)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, K, V>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `IterMut<'a, K, V>`

```rust
struct IterMut<'a, K, V> {
    inner: vec::IntoIter<(&'a K, &'a mut V)>,
}
```

Parallel iterator over a mutable reference to a B-Tree map

#### Trait Implementations

##### `impl<'a, K: $crate::fmt::Debug, V: $crate::fmt::Debug> Debug for IterMut<'a, K, V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for IterMut<'a, K, V>`

##### `impl<T> IntoParallelIterator for IterMut<'a, K, V>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a, K: Sync + 'a, V: Send + 'a> ParallelIterator for IterMut<'a, K, V>`

- `type Item = (&'a K, &'a mut V)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'a, K, V>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

