*[rayon](../index.md) / [option](index.md)*

---

# Module `option`

Parallel iterator types for [`options`](../../textwrap/options/index.md)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Structs

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    opt: Option<T>,
}
```

A parallel iterator over the value in [`Some`](#some) variant of an [`Option`](../../clap_derive/index.md).

The iterator yields one value if the [`Option`](../../clap_derive/index.md) is a [`Some`](#some), otherwise none.

This `struct` is created by the `into_par_iter` function.


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

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    inner: IntoIter<&'a T>,
}
```

A parallel iterator over a reference to the [`Some`](#some) variant of an [`Option`](../../clap_derive/index.md).

The iterator yields one value if the [`Option`](../../clap_derive/index.md) is a [`Some`](#some), otherwise none.

This `struct` is created by the `par_iter` function.


#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a, T: $crate::fmt::Debug> Debug for Iter<'a, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, T: Sync> IndexedParallelIterator for Iter<'a, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Iter<'a, T>`

##### `impl<T> IntoParallelIterator for Iter<'a, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a, T: Sync> ParallelIterator for Iter<'a, T>`

- `type Item = &'a T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Iter<'a, T>`

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

A parallel iterator over a mutable reference to the [`Some`](#some) variant of an [`Option`](../../clap_derive/index.md).

The iterator yields one value if the [`Option`](../../clap_derive/index.md) is a [`Some`](#some), otherwise none.

This `struct` is created by the `par_iter_mut` function.


#### Trait Implementations

##### `impl<'a, T: $crate::fmt::Debug> Debug for IterMut<'a, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a, T: Send> IndexedParallelIterator for IterMut<'a, T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for IterMut<'a, T>`

##### `impl<T> IntoParallelIterator for IterMut<'a, T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a, T: Send> ParallelIterator for IterMut<'a, T>`

- `type Item = &'a mut T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for IterMut<'a, T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `OptionProducer<T: Send>`

```rust
struct OptionProducer<T: Send> {
    opt: Option<T>,
}
```

Private producer for an option

#### Trait Implementations

##### `impl<T> IntoEither for OptionProducer<T>`

##### `impl<T> Pointable for OptionProducer<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Send> Producer for OptionProducer<T>`

- `type Item = T`

- `type IntoIter = IntoIter<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

