*[rayon](../index.md) / [vec](index.md)*

---

# Module `vec`

Parallel iterator types for [vectors](#vectors) (`Vec<T>`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    vec: Vec<T>,
}
```

Parallel iterator that moves out of a vector.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for IntoIter<T>`

- `fn clone(self: &Self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

##### `impl<T: $crate::fmt::Debug> Debug for IntoIter<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: Send> IndexedParallelIterator for IntoIter<T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for IntoIter<T>`

##### `impl<T> IntoParallelIterator for IntoIter<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Drain<'data, T: Send>`

```rust
struct Drain<'data, T: Send> {
    vec: &'data mut Vec<T>,
    range: std::ops::Range<usize>,
    orig_len: usize,
}
```

Draining parallel iterator that moves a range out of a vector, but keeps the total capacity.

#### Trait Implementations

##### `impl<'data, T: $crate::fmt::Debug + Send> Debug for Drain<'data, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'data, T: Send> Drop for Drain<'data, T>`

- `fn drop(self: &mut Self)`

##### `impl<'data, T: Send> IndexedParallelIterator for Drain<'data, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Drain<'data, T>`

##### `impl<T> IntoParallelIterator for Drain<'data, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'data, T: Send> ParallelIterator for Drain<'data, T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Drain<'data, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

