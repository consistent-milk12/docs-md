*[rayon](../index.md) / [array](index.md)*

---

# Module `array`

Parallel iterator types for [arrays] (`[T; N]`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator that moves out of an array. |

## Structs

### `IntoIter<T, const N: usize>`

```rust
struct IntoIter<T, const N: usize> {
    array: [T; N],
}
```

*Defined in [`rayon-1.11.0/src/array.rs:43-45`](../../../.source_1765210505/rayon-1.11.0/src/array.rs#L43-L45)*

Parallel iterator that moves out of an array.

#### Trait Implementations

##### `impl<T: clone::Clone, const N: usize> Clone for IntoIter<T, N>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T, N>` — [`IntoIter`](#intoiter)

##### `impl<T: fmt::Debug, const N: usize> Debug for IntoIter<T, N>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Send, const N: usize> IndexedParallelIterator for IntoIter<T, N>`

- <span id="intoiter-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-len"></span>`fn len(&self) -> usize`

- <span id="intoiter-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T> IntoEither for IntoIter<T, N>`

##### `impl<T> IntoParallelIterator for IntoIter<T, N>`

- <span id="intoiter-type-iter"></span>`type Iter = T`

- <span id="intoiter-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send, const N: usize> ParallelIterator for IntoIter<T, N>`

- <span id="intoiter-type-item"></span>`type Item = T`

- <span id="intoiter-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T, N>`

- <span id="intoiter-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-type-init"></span>`type Init = T`

- <span id="intoiter-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-drop"></span>`unsafe fn drop(ptr: usize)`

