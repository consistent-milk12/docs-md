*[rayon](../../index.md) / [collections](../index.md) / [vec_deque](index.md)*

---

# Module `vec_deque`

This module contains the parallel iterator types for double-ended queues
(`VecDeque<T>`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Structs

### `IntoIter<T: Send>`

```rust
struct IntoIter<T: Send> {
    inner: vec::IntoIter<T>,
}
```

Parallel iterator over a double-ended queue

#### Trait Implementations

##### `impl Clone<T: $crate::clone::Clone + Send>`

- `fn clone(self: &Self) -> IntoIter<T>` — [`IntoIter`](../../../collections/vec_deque/index.md)

##### `impl Debug<T: $crate::fmt::Debug + Send>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<T: Send>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: Chain<slice::Iter<'a, T>, slice::Iter<'a, T>>,
}
```

Parallel iterator over an immutable reference to a double-ended queue

#### Trait Implementations

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<'a, T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'a, T: Sync>`

- `type Item = &'a T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    inner: Chain<slice::IterMut<'a, T>, slice::IterMut<'a, T>>,
}
```

Parallel iterator over a mutable reference to a double-ended queue

#### Trait Implementations

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<'a, T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'a, T: Send>`

- `type Item = &'a mut T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    deque: &'a mut std::collections::VecDeque<T>,
    range: std::ops::Range<usize>,
    orig_len: usize,
}
```

Draining parallel iterator that moves a range out of a double-ended queue,
but keeps the total capacity.

#### Trait Implementations

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Drop<T>`

- `fn drop(self: &mut Self)`

##### `impl IndexedParallelIterator<T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<T: Send>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

