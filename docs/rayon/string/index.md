*[rayon](../index.md) / [string](index.md)*

---

# Module `string`

This module contains the parallel iterator types for owned strings
(`String`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Structs

### `Drain<'a>`

```rust
struct Drain<'a> {
    string: &'a mut String,
    range: std::ops::Range<usize>,
}
```

Draining parallel iterator that moves a range of characters out of a string,
but keeps the total capacity.

#### Trait Implementations

##### `impl<'a> Debug for Drain<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> Drop for Drain<'a>`

- `fn drop(self: &mut Self)`

##### `impl<T> IntoEither for Drain<'a>`

##### `impl<T> IntoParallelIterator for Drain<'a>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<'a> ParallelIterator for Drain<'a>`

- `type Item = char`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Drain<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

