*[rayon](../../index.md) / [iter](../index.md) / [take](index.md)*

---

# Module `take`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Take`](#take) | struct | `Take` is an iterator that iterates over the first `n` elements. |

## Structs

### `Take<I>`

```rust
struct Take<I> {
    base: I,
    n: usize,
}
```

`Take` is an iterator that iterates over the first `n` elements.
This struct is created by the `take()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="take-new"></span>`fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Take<I>`

- <span id="take-clone"></span>`fn clone(&self) -> Take<I>` — [`Take`](../index.md)

##### `impl<I: fmt::Debug> Debug for Take<I>`

- <span id="take-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Take<I>`

- <span id="take-len"></span>`fn len(&self) -> usize`

- <span id="take-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="take-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Take<I>`

##### `impl<T> IntoParallelIterator for Take<I>`

- <span id="take-iter"></span>`type Iter = T`

- <span id="take-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="take-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Take<I>`

- <span id="take-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="take-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="take-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Take<I>`

- <span id="take-align"></span>`const ALIGN: usize`

- <span id="take-init"></span>`type Init = T`

- <span id="take-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="take-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="take-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="take-drop"></span>`unsafe fn drop(ptr: usize)`

