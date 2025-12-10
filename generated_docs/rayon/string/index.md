*[rayon](../index.md) / [string](index.md)*

---

# Module `string`

This module contains the parallel iterator types for owned strings
(`String`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | Draining parallel iterator that moves a range of characters out of a string, but keeps the total capacity. |

## Structs

### `Drain<'a>`

```rust
struct Drain<'a> {
    string: &'a mut String,
    range: std::ops::Range<usize>,
}
```

*Defined in [`rayon-1.11.0/src/string.rs:25-28`](../../../.source_1765210505/rayon-1.11.0/src/string.rs#L25-L28)*

Draining parallel iterator that moves a range of characters out of a string,
but keeps the total capacity.

#### Trait Implementations

##### `impl Debug for Drain<'a>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Drain<'a>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl IntoEither for Drain<'a>`

##### `impl IntoParallelIterator for Drain<'a>`

- <span id="drain-type-iter"></span>`type Iter = T`

- <span id="drain-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Drain<'a>`

- <span id="drain-type-item"></span>`type Item = char`

- <span id="drain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Drain<'a>`

- <span id="drain-const-align"></span>`const ALIGN: usize`

- <span id="drain-type-init"></span>`type Init = T`

- <span id="drain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-drop"></span>`unsafe fn drop(ptr: usize)`

