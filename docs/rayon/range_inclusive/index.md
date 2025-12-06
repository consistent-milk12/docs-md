*[rayon](../index.md) / [range_inclusive](index.md)*

---

# Module `range_inclusive`

Parallel iterator types for [inclusive ranges],
the type for values created by `a..=b` expressions

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

```rust
use rayon::prelude::*;

let r = (0..=100u64).into_par_iter()
                    .sum();

// compare result with sequential calculation
assert_eq!((0..=100).sum::<u64>(), r);
```


## Structs

### `Iter<T>`

```rust
struct Iter<T> {
    range: std::ops::RangeInclusive<T>,
}
```

Parallel iterator over an inclusive range, implemented for all integer types and `char`.

**Note:** The `zip` operation requires `IndexedParallelIterator`
which is only implemented for `u8`, `i8`, `u16`, `i16`, and `char`.

```rust
use rayon::prelude::*;

let p = (0..=25u16).into_par_iter()
                  .zip(0..=25u16)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum::<u16>();

let s = (0..=25u16).zip(0..=25u16)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum();

assert_eq!(p, s);
```

#### Implementations

- `fn bounds(self: &Self) -> Option<(T, T)>`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Iter<T>`

- `fn clone(self: &Self) -> Iter<T>` — [`Iter`](#iter)

##### `impl<T: $crate::fmt::Debug> Debug for Iter<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: IndexedRangeInteger> IndexedParallelIterator for Iter<T>`

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

