*[rayon](../../index.md) / [iter](../index.md) / [skip](index.md)*

---

# Module `skip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Skip`](#skip) | struct | `Skip` is an iterator that skips over the first `n` elements. |

## Structs

### `Skip<I>`

```rust
struct Skip<I> {
    base: I,
    n: usize,
}
```

*Defined in [`rayon-1.11.0/src/iter/skip.rs:11-14`](../../../../.source_1765210505/rayon-1.11.0/src/iter/skip.rs#L11-L14)*

`Skip` is an iterator that skips over the first `n` elements.
This struct is created by the `skip()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="skip-new"></span>`fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for Skip<I>`

- <span id="skip-clone"></span>`fn clone(&self) -> Skip<I>` — [`Skip`](#skip)

##### `impl<I: fmt::Debug> Debug for Skip<I>`

- <span id="skip-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IndexedParallelIterator for Skip<I>`

- <span id="skip-len"></span>`fn len(&self) -> usize`

- <span id="skip-drive"></span>`fn drive<C: Consumer<<Self as >::Item>>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="skip-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Skip<I>`

##### `impl<T> IntoParallelIterator for Skip<I>`

- <span id="skip-type-iter"></span>`type Iter = T`

- <span id="skip-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="skip-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Skip<I>`

- <span id="skip-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="skip-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="skip-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Skip<I>`

- <span id="skip-const-align"></span>`const ALIGN: usize`

- <span id="skip-type-init"></span>`type Init = T`

- <span id="skip-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="skip-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="skip-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="skip-drop"></span>`unsafe fn drop(ptr: usize)`

