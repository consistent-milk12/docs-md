*[rayon](../../index.md) / [iter](../index.md) / [chain](index.md)*

---

# Module `chain`

## Structs

### `Chain<A, B>`

```rust
struct Chain<A, B> {
    a: A,
    b: B,
}
```

`Chain` is an iterator that joins `b` after `a` in one continuous iterator.
This struct is created by the `chain()` method on [`ParallelIterator`](../index.md)


#### Implementations

- `fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: $crate::clone::Clone, B: $crate::clone::Clone> Clone for Chain<A, B>`

- `fn clone(self: &Self) -> Chain<A, B>` — [`Chain`](#chain)

##### `impl<A: $crate::fmt::Debug, B: $crate::fmt::Debug> Debug for Chain<A, B>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Chain<A, B>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Chain<A, B>`

##### `impl<T> IntoParallelIterator for Chain<A, B>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<A, B> ParallelIterator for Chain<A, B>`

- `type Item = <A as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Chain<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `ChainProducer<A, B>`

```rust
struct ChainProducer<A, B>
where
    A: Producer,
    B: Producer<Item = <A as >::Item> {
    a_len: usize,
    a: A,
    b: B,
}
```

#### Implementations

- `fn new(a_len: usize, a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<T> IntoEither for ChainProducer<A, B>`

##### `impl<T> Pointable for ChainProducer<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<A, B> Producer for ChainProducer<A, B>`

- `type Item = <A as Producer>::Item`

- `type IntoIter = ChainSeq<<A as Producer>::IntoIter, <B as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

- `fn fold_with<F>(self: Self, folder: F) -> F`

### `ChainSeq<A, B>`

```rust
struct ChainSeq<A, B> {
    chain: iter::Chain<A, B>,
}
```

Wrapper for `Chain` to implement `ExactSizeIterator`

#### Implementations

- `fn new(a: A, b: B) -> ChainSeq<A, B>` — [`ChainSeq`](#chainseq)

#### Trait Implementations

##### `impl<A, B> DoubleEndedIterator for ChainSeq<A, B>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<A, B> ExactSizeIterator for ChainSeq<A, B>`

##### `impl<T> IntoEither for ChainSeq<A, B>`

##### `impl<I> IntoIterator for ChainSeq<A, B>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<A, B> Iterator for ChainSeq<A, B>`

- `type Item = <A as Iterator>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for ChainSeq<A, B>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

