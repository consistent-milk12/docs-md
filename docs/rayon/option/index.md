*[rayon](../index.md) / [option](index.md)*

---

# Module `option`

Parallel iterator types for [options](#options)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.

[options](#options): std::option

## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    opt: Option<T>,
}
```

A parallel iterator over the value in [`Some`](#some) variant of an [`Option`](#option).

The iterator yields one value if the [`Option`](#option) is a [`Some`](#some), otherwise none.

This `struct` is created by the [`into_par_iter`](#into-par-iter) function.


#### Trait Implementations

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> IntoIter<T>` — [`IntoIter`](../../option/index.md)

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<T: Send>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

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
    inner: IntoIter<&'a T>,
}
```

A parallel iterator over a reference to the [`Some`](#some) variant of an [`Option`](#option).

The iterator yields one value if the [`Option`](#option) is a [`Some`](#some), otherwise none.

This `struct` is created by the [`par_iter`](#par-iter) function.


#### Trait Implementations

##### `impl Clone<T>`

- `fn clone(self: &Self) -> Self`

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<'a, T: Sync>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'a, T: Sync>`

- `type Item = &'a T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

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
    inner: IntoIter<&'a mut T>,
}
```

A parallel iterator over a mutable reference to the [`Some`](#some) variant of an [`Option`](#option).

The iterator yields one value if the [`Option`](#option) is a [`Some`](#some), otherwise none.

This `struct` is created by the [`par_iter_mut`](#par-iter-mut) function.


#### Trait Implementations

##### `impl Debug<'a, T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator<'a, T: Send>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../../iter/plumbing/index.md)

##### `impl IntoEither<T>`

##### `impl IntoParallelIterator<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl ParallelIterator<'a, T: Send>`

- `type Item = &'a mut T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

