*[rayon](../../index.md) / [iter](../index.md) / [repeat](index.md)*

---

# Module `repeat`

## Contents

- [Structs](#structs)
  - [`Repeat`](#repeat)
  - [`RepeatProducer`](#repeatproducer)
  - [`RepeatN`](#repeatn)
- [Enums](#enums)
  - [`RepeatNProducer`](#repeatnproducer)
- [Functions](#functions)
  - [`repeat`](#repeat)
  - [`repeat_n`](#repeat_n)
  - [`repeatn`](#repeatn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Repeat`](#repeat) | struct | Iterator adaptor for [the `repeat()` function]. |
| [`RepeatProducer`](#repeatproducer) | struct | Unindexed producer for `Repeat`. |
| [`RepeatN`](#repeatn) | struct | Iterator adaptor for [the `repeat_n()` function]. |
| [`RepeatNProducer`](#repeatnproducer) | enum | Producer for `RepeatN`. |
| [`repeat`](#repeat) | fn | Creates a parallel iterator that endlessly repeats `element` (by |
| [`repeat_n`](#repeat_n) | fn | Creates a parallel iterator that produces `n` repeats of `element` |
| [`repeatn`](#repeatn) | fn | Creates a parallel iterator that produces `n` repeats of `element` |

## Structs

### `Repeat<T>`

```rust
struct Repeat<T> {
    element: T,
}
```

Iterator adaptor for [the `repeat()` function].


#### Implementations

- <span id="repeat-take"></span>`fn take(self, n: usize) -> RepeatN<T>` — [`RepeatN`](../index.md)

- <span id="repeat-zip"></span>`fn zip<Z>(self, zip_op: Z) -> Zip<RepeatN<T>, <Z as >::Iter>` — [`Zip`](../index.md), [`RepeatN`](../index.md), [`IntoParallelIterator`](../../prelude/index.md)

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Repeat<T>`

- <span id="repeat-clone"></span>`fn clone(&self) -> Repeat<T>` — [`Repeat`](../index.md)

##### `impl<T: fmt::Debug> Debug for Repeat<T>`

- <span id="repeat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IntoEither for Repeat<T>`

##### `impl<T> IntoParallelIterator for Repeat<T>`

- <span id="repeat-iter"></span>`type Iter = T`

- <span id="repeat-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeat-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for Repeat<T>`

- <span id="repeat-item"></span>`type Item = T`

- <span id="repeat-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

##### `impl<T> Pointable for Repeat<T>`

- <span id="repeat-align"></span>`const ALIGN: usize`

- <span id="repeat-init"></span>`type Init = T`

- <span id="repeat-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeat-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeat-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeat-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="repeatproducer-align"></span>`const ALIGN: usize`

- <span id="repeatproducer-init"></span>`type Init = T`

- <span id="repeatproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Clone + Send> UnindexedProducer for RepeatProducer<T>`

- <span id="repeatproducer-item"></span>`type Item = T`

- <span id="repeatproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="repeatproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `RepeatN<T>`

```rust
struct RepeatN<T> {
    inner: RepeatNProducer<T>,
}
```

Iterator adaptor for [the `repeat_n()` function].


#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RepeatN<T>`

- <span id="repeatn-clone"></span>`fn clone(&self) -> RepeatN<T>` — [`RepeatN`](../index.md)

##### `impl<T: fmt::Debug> Debug for RepeatN<T>`

- <span id="repeatn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> IndexedParallelIterator for RepeatN<T>`

- <span id="repeatn-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="repeatn-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

- <span id="repeatn-len"></span>`fn len(&self) -> usize`

##### `impl<T> IntoEither for RepeatN<T>`

##### `impl<T> IntoParallelIterator for RepeatN<T>`

- <span id="repeatn-iter"></span>`type Iter = T`

- <span id="repeatn-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="repeatn-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T> ParallelIterator for RepeatN<T>`

- <span id="repeatn-item"></span>`type Item = T`

- <span id="repeatn-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="repeatn-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for RepeatN<T>`

- <span id="repeatn-align"></span>`const ALIGN: usize`

- <span id="repeatn-init"></span>`type Init = T`

- <span id="repeatn-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatn-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatn-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatn-drop"></span>`unsafe fn drop(ptr: usize)`

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

##### `impl<T: clone::Clone> Clone for RepeatNProducer<T>`

- <span id="repeatnproducer-clone"></span>`fn clone(&self) -> RepeatNProducer<T>` — [`RepeatNProducer`](#repeatnproducer)

##### `impl<T: Clone> DoubleEndedIterator for RepeatNProducer<T>`

- <span id="repeatnproducer-next-back"></span>`fn next_back(&mut self) -> Option<T>`

- <span id="repeatnproducer-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<T>`

##### `impl<T: Clone> ExactSizeIterator for RepeatNProducer<T>`

- <span id="repeatnproducer-len"></span>`fn len(&self) -> usize`

##### `impl<T> IntoEither for RepeatNProducer<T>`

##### `impl<I> IntoIterator for RepeatNProducer<T>`

- <span id="repeatnproducer-item"></span>`type Item = <I as Iterator>::Item`

- <span id="repeatnproducer-intoiter"></span>`type IntoIter = I`

- <span id="repeatnproducer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Clone> Iterator for RepeatNProducer<T>`

- <span id="repeatnproducer-item"></span>`type Item = T`

- <span id="repeatnproducer-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="repeatnproducer-nth"></span>`fn nth(&mut self, n: usize) -> Option<T>`

- <span id="repeatnproducer-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for RepeatNProducer<T>`

- <span id="repeatnproducer-align"></span>`const ALIGN: usize`

- <span id="repeatnproducer-init"></span>`type Init = T`

- <span id="repeatnproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="repeatnproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="repeatnproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="repeatnproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T: Clone + Send> Producer for RepeatNProducer<T>`

- <span id="repeatnproducer-item"></span>`type Item = T`

- <span id="repeatnproducer-intoiter"></span>`type IntoIter = RepeatNProducer<T>`

- <span id="repeatnproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="repeatnproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

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

