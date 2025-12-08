*[rayon](../../index.md) / [iter](../index.md) / [skip](index.md)*

---

# Module `skip`

## Structs

### `Skip<I>`

```rust
struct Skip<I> {
    base: I,
    n: usize,
}
```

`Skip` is an iterator that skips over the first `n` elements.
This struct is created by the `skip()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I, n: usize) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Skip<I>`

- `fn clone(self: &Self) -> Skip<I>` — [`Skip`](#skip)

##### `impl<I: $crate::fmt::Debug> Debug for Skip<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Skip<I>`

- `fn len(self: &Self) -> usize`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Skip<I>`

##### `impl<T> IntoParallelIterator for Skip<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Skip<I>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Skip<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

