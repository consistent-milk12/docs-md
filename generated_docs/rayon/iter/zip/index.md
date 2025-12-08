*[rayon](../../index.md) / [iter](../index.md) / [zip](index.md)*

---

# Module `zip`

## Structs

### `Zip<A, B>`

```rust
struct Zip<A, B> {
    a: A,
    b: B,
}
```

`Zip` is an iterator that zips up `a` and `b` into a single iterator
of pairs. This struct is created by the `zip()` method on
[`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- `fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for Zip<A, B>`

- `fn clone(self: &Self) -> Zip<A, B>` — [`Zip`](../index.md)

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for Zip<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Zip<A, B>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Zip<A, B>`

##### `impl<T> IntoParallelIterator for Zip<A, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B> ParallelIterator for Zip<A, B>`

- `type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Zip<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ZipProducer<A: Producer, B: Producer>`

```rust
struct ZipProducer<A: Producer, B: Producer> {
    a: A,
    b: B,
}
```

#### Trait Implementations

##### `impl<T> IntoEither for ZipProducer<A, B>`

##### `impl<T> Pointable for ZipProducer<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<A: Producer, B: Producer> Producer for ZipProducer<A, B>`

- `type Item = (<A as Producer>::Item, <B as Producer>::Item)`

- `type IntoIter = Zip<<A as Producer>::IntoIter, <B as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

