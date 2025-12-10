*[rayon](../../index.md) / [iter](../index.md) / [zip_eq](index.md)*

---

# Module `zip_eq`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ZipEq`](#zipeq) | struct | An [`IndexedParallelIterator`] that iterates over two parallel iterators of equal length simultaneously. |

## Structs

### `ZipEq<A, B>`

```rust
struct ZipEq<A, B> {
    zip: Zip<A, B>,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip_eq.rs:13-15`](../../../../.source_1765210505/rayon-1.11.0/src/iter/zip_eq.rs#L13-L15)*

An [`IndexedParallelIterator`](../index.md) that iterates over two parallel iterators of equal
length simultaneously.

This struct is created by the [`zip_eq`](#zip-eq) method on [`IndexedParallelIterator`](../index.md),
see its documentation for more information.


#### Implementations

- <span id="zipeq-new"></span>`fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for ZipEq<A, B>`

- <span id="zipeq-clone"></span>`fn clone(&self) -> ZipEq<A, B>` — [`ZipEq`](#zipeq)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for ZipEq<A, B>`

- <span id="zipeq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> IndexedParallelIterator for ZipEq<A, B>`

- <span id="zipeq-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zipeq-len"></span>`fn len(&self) -> usize`

- <span id="zipeq-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<T> IntoEither for ZipEq<A, B>`

##### `impl<T> IntoParallelIterator for ZipEq<A, B>`

- <span id="zipeq-type-iter"></span>`type Iter = T`

- <span id="zipeq-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zipeq-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for ZipEq<A, B>`

- <span id="zipeq-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zipeq-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zipeq-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for ZipEq<A, B>`

- <span id="zipeq-const-align"></span>`const ALIGN: usize`

- <span id="zipeq-type-init"></span>`type Init = T`

- <span id="zipeq-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zipeq-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zipeq-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zipeq-drop"></span>`unsafe fn drop(ptr: usize)`

