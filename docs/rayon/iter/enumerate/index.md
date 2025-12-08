*[rayon](../../index.md) / [iter](../index.md) / [enumerate](index.md)*

---

# Module `enumerate`

## Structs

### `Enumerate<I>`

```rust
struct Enumerate<I> {
    base: I,
}
```

`Enumerate` is an iterator that returns the current count along with the element.
This struct is created by the `enumerate()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- `fn new(base: I) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for Enumerate<I>`

- `fn clone(self: &Self) -> Enumerate<I>` — [`Enumerate`](#enumerate)

##### `impl<I: $crate::fmt::Debug> Debug for Enumerate<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IndexedParallelIterator for Enumerate<I>`

- `fn drive<C: Consumer<<Self as >::Item>>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Enumerate<I>`

##### `impl<T> IntoParallelIterator for Enumerate<I>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I> ParallelIterator for Enumerate<I>`

- `type Item = (usize, <I as ParallelIterator>::Item)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Enumerate<I>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `EnumerateProducer<P>`

```rust
struct EnumerateProducer<P> {
    base: P,
    offset: usize,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for EnumerateProducer<P>`

##### `impl<T> Pointable for EnumerateProducer<P>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<P> Producer for EnumerateProducer<P>`

- `type Item = (usize, <P as Producer>::Item)`

- `type IntoIter = Zip<Range<usize>, <P as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

