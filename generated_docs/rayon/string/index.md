*[rayon](../index.md) / [string](index.md)*

---

# Module `string`

This module contains the parallel iterator types for owned strings
(`String`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | Draining parallel iterator that moves a range of characters out of a string |

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

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a> Drop for Drain<'a>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T> IntoEither for Drain<'a>`

##### `impl<T> IntoParallelIterator for Drain<'a>`

- <span id="drain-iter"></span>`type Iter = T`

- <span id="drain-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<'a> ParallelIterator for Drain<'a>`

- <span id="drain-item"></span>`type Item = char`

- <span id="drain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md)

##### `impl<T> Pointable for Drain<'a>`

- <span id="drain-align"></span>`const ALIGN: usize`

- <span id="drain-init"></span>`type Init = T`

- <span id="drain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-drop"></span>`unsafe fn drop(ptr: usize)`

