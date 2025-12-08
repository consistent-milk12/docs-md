*[rayon](../../index.md) / [iter](../index.md) / [take](index.md)*

---

# Module `take`

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

- `fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Take<I>`

- `fn clone(self: &Self) -> Take<I>` — [`Take`](../index.md)

##### `impl<I: $crate::fmt::Debug> Debug for Take<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Take<I>`

- `fn len(self: &Self) -> usize`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Take<I>`

##### `impl<T> IntoParallelIterator for Take<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Take<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Take<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

