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


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | These traits help drive integer type inference. |
| [`Iter`](#iter) | struct | Parallel iterator over an inclusive range, implemented for all integer types and `char`. |
| [`convert!`](#convert) | macro |  |
| [`parallel_range_impl!`](#parallel_range_impl) | macro |  |
| [`indexed_range_impl!`](#indexed_range_impl) | macro |  |
| [`convert_char!`](#convert_char) | macro |  |

## Modules

- [`private`](private/index.md) — These traits help drive integer type inference. Without them, an unknown `{integer}` type only

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

- <span id="iter-bounds"></span>`fn bounds(&self) -> Option<(T, T)>`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Iter<T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<T>` — [`Iter`](#iter)

##### `impl<T: fmt::Debug> Debug for Iter<T>`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: IndexedRangeInteger> IndexedParallelIterator for Iter<T>`

- <span id="iter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="iter-len"></span>`fn len(&self) -> usize`

- <span id="iter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md)

##### `impl<T> IntoEither for Iter<T>`

##### `impl<T> IntoParallelIterator for Iter<T>`

- <span id="iter-iter"></span>`type Iter = T`

- <span id="iter-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Iter<char>`

- <span id="iter-item"></span>`type Item = char`

- <span id="iter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

- <span id="iter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<T>`

- <span id="iter-align"></span>`const ALIGN: usize`

- <span id="iter-init"></span>`type Init = T`

- <span id="iter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-drop"></span>`unsafe fn drop(ptr: usize)`

## Macros

### `convert!`

### `parallel_range_impl!`

### `indexed_range_impl!`

### `convert_char!`

