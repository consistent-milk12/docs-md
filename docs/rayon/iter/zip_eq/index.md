*[rayon](../../index.md) / [iter](../index.md) / [zip_eq](index.md)*

---

# Module `zip_eq`

## Structs

### `ZipEq<A, B>`

```rust
struct ZipEq<A, B> {
    zip: Zip<A, B>,
}
```

An [`IndexedParallelIterator`](../index.md) that iterates over two parallel iterators of equal
length simultaneously.

This struct is created by the [`zip_eq`](#zip-eq) method on [`IndexedParallelIterator`](../index.md),
see its documentation for more information.


#### Implementations

- `fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for ZipEq<A, B>`

- `fn clone(self: &Self) -> ZipEq<A, B>` — [`ZipEq`](#zipeq)

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for ZipEq<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B> IndexedParallelIterator for ZipEq<A, B>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for ZipEq<A, B>`

##### `impl<T> IntoParallelIterator for ZipEq<A, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B> ParallelIterator for ZipEq<A, B>`

- `type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for ZipEq<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

