*[rayon](../../index.md) / [iter](../index.md) / [repeat](index.md)*

---

# Module `repeat`

## Structs

### `Repeat<T>`

```rust
struct Repeat<T> {
    element: T,
}
```

Iterator adaptor for [the `repeat()` function].


#### Implementations

- `fn take(self: Self, n: usize) -> RepeatN<T>` — [`RepeatN`](../index.md)

- `fn zip<Z>(self: Self, zip_op: Z) -> Zip<RepeatN<T>, <Z as >::Iter>` — [`Zip`](../index.md), [`RepeatN`](../index.md), [`IntoParallelIterator`](../../prelude/index.md)

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Repeat<T>`

- `fn clone(self: &Self) -> Repeat<T>` — [`Repeat`](../index.md)

##### `impl<T: $crate::fmt::Debug> Debug for Repeat<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> IntoEither for Repeat<T>`

##### `impl<T> IntoParallelIterator for Repeat<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T> ParallelIterator for Repeat<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Repeat<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `RepeatProducer<T: Clone + Send>`

```rust
struct RepeatProducer<T: Clone + Send> {
    element: T,
}
```

Unindexed producer for `Repeat`.

#### Trait Implementations

##### `impl<T> IntoEither for RepeatProducer<T>`

##### `impl<T> Pointable for RepeatProducer<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Clone + Send> UnindexedProducer for RepeatProducer<T>`

- `type Item = T`

- `fn split(self: Self) -> (Self, Option<Self>)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

### `RepeatN<T>`

```rust
struct RepeatN<T> {
    inner: RepeatNProducer<T>,
}
```

Iterator adaptor for [the `repeat_n()` function].


#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RepeatN<T>`

- `fn clone(self: &Self) -> RepeatN<T>` — [`RepeatN`](../index.md)

##### `impl<T: fmt::Debug> Debug for RepeatN<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IndexedParallelIterator for RepeatN<T>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

##### `impl<T> IntoEither for RepeatN<T>`

##### `impl<T> IntoParallelIterator for RepeatN<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T> ParallelIterator for RepeatN<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for RepeatN<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Enums

### `RepeatNProducer<T>`

```rust
enum RepeatNProducer<T> {
    Repeats(T, std::num::NonZeroUsize),
    Empty,
}
```

Producer for `RepeatN`.

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for RepeatNProducer<T>`

- `fn clone(self: &Self) -> RepeatNProducer<T>` — [`RepeatNProducer`](#repeatnproducer)

##### `impl<T: Clone> DoubleEndedIterator for RepeatNProducer<T>`

- `fn next_back(self: &mut Self) -> Option<T>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<T>`

##### `impl<T: Clone> ExactSizeIterator for RepeatNProducer<T>`

- `fn len(self: &Self) -> usize`

##### `impl<T> IntoEither for RepeatNProducer<T>`

##### `impl<I> IntoIterator for RepeatNProducer<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T: Clone> Iterator for RepeatNProducer<T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn nth(self: &mut Self, n: usize) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for RepeatNProducer<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T: Clone + Send> Producer for RepeatNProducer<T>`

- `type Item = T`

- `type IntoIter = RepeatNProducer<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

## Functions

### `repeat`

```rust
fn repeat<T: Clone + Send>(element: T) -> Repeat<T>
```

Creates a parallel iterator that endlessly repeats `element` (by
cloning it). Note that this iterator has "infinite" length, so
typically you would want to use `zip` or `take` or some other
means to shorten it, or consider using
[the `repeat_n()` function] instead.

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::repeat;
let x: Vec<(i32, i32)> = repeat(22).zip(0..3).collect();
assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
```

### `repeat_n`

```rust
fn repeat_n<T: Clone + Send>(element: T, n: usize) -> RepeatN<T>
```

Creates a parallel iterator that produces `n` repeats of `element`
(by cloning it).

# Examples

```rust
use rayon::prelude::*;
use rayon::iter::repeat_n;
let x: Vec<(i32, i32)> = repeat_n(22, 3).zip(0..3).collect();
assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);
```

### `repeatn`

```rust
fn repeatn<T: Clone + Send>(element: T, n: usize) -> RepeatN<T>
```

Creates a parallel iterator that produces `n` repeats of `element`
(by cloning it).

Deprecated in favor of [`repeat_n`](../index.md) for consistency with the standard library.

