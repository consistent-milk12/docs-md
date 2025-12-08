*[rayon](../index.md) / [range](index.md)*

---

# Module `range`

Parallel iterator types for [ranges],
the type for values created by `a..b` expressions

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

```rust
use rayon::prelude::*;

let r = (0..100u64).into_par_iter()
                   .sum();

// compare result with sequential calculation
assert_eq!((0..100).sum::<u64>(), r);
```


## Modules

- [`private`](private/index.md) - These traits help drive integer type inference. Without them, an unknown `{integer}` type only

## Structs

### `Iter<T>`

```rust
struct Iter<T> {
    range: std::ops::Range<T>,
}
```

Parallel iterator over a range, implemented for all integer types and `char`.

**Note:** The `zip` operation requires `IndexedParallelIterator`
which is not implemented for `u64`, `i64`, `u128`, or `i128`.

```rust
use rayon::prelude::*;

let p = (0..25usize).into_par_iter()
                  .zip(0..25usize)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum::<usize>();

let s = (0..25usize).zip(0..25)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum();

assert_eq!(p, s);
```

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Iter<T>`

- `fn clone(self: &Self) -> Iter<T>` — [`Iter`](#iter)

##### `impl<T: $crate::fmt::Debug> Debug for Iter<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IndexedParallelIterator for Iter<char>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Iter<T>`

##### `impl<T> IntoParallelIterator for Iter<T>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<T: RangeInteger> ParallelIterator for Iter<T>`

- `type Item = T`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Iter<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `IterProducer<T>`

```rust
struct IterProducer<T> {
    range: std::ops::Range<T>,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for IterProducer<T>`

##### `impl<T> IntoIterator for IterProducer<T>`

- `type Item = <Range<T> as Iterator>::Item`

- `type IntoIter = Range<T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<T> Pointable for IterProducer<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl Producer for IterProducer<i16>`

- `type Item = <Range<i16> as Iterator>::Item`

- `type IntoIter = Range<i16>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md)

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

##### `impl UnindexedProducer for IterProducer<u128>`

- `type Item = u128`

- `fn split(self: Self) -> (Self, Option<Self>)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

## Traits

### `UnindexedRangeLen<L>`

```rust
trait UnindexedRangeLen<L> { ... }
```

#### Required Methods

- `fn unindexed_len(self: &Self) -> L`

## Macros

### `indexed_range_impl!`

### `unindexed_range_impl!`

### `convert_char!`

