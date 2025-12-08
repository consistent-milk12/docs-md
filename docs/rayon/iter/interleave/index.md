*[rayon](../../index.md) / [iter](../index.md) / [interleave](index.md)*

---

# Module `interleave`

## Structs

### `Interleave<I, J>`

```rust
struct Interleave<I, J> {
    i: I,
    j: J,
}
```

`Interleave` is an iterator that interleaves elements of iterators
`i` and `j` in one continuous iterator. This struct is created by
the `interleave()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- `fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone, J: $crate::clone::Clone> Clone for Interleave<I, J>`

- `fn clone(self: &Self) -> Interleave<I, J>` — [`Interleave`](#interleave)

##### `impl<I: $crate::fmt::Debug, J: $crate::fmt::Debug> Debug for Interleave<I, J>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I, J> IndexedParallelIterator for Interleave<I, J>`

- `fn drive<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn len(self: &Self) -> usize`

- `fn with_producer<CB>(self: Self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Interleave<I, J>`

##### `impl<T> IntoParallelIterator for Interleave<I, J>`

- `type Iter = T`

- `type Item = <T as ParallelIterator>::Item`

- `fn into_par_iter(self: Self) -> T`

##### `impl<I, J> ParallelIterator for Interleave<I, J>`

- `type Item = <I as ParallelIterator>::Item`

- `fn drive_unindexed<C>(self: Self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- `fn opt_len(self: &Self) -> Option<usize>`

##### `impl<T> Pointable for Interleave<I, J>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

### `InterleaveProducer<I, J>`

```rust
struct InterleaveProducer<I, J>
where
    I: Producer,
    J: Producer<Item = <I as >::Item> {
    i: I,
    j: J,
    i_len: usize,
    j_len: usize,
    i_next: bool,
}
```

#### Implementations

- `fn new(i: I, j: J, i_len: usize, j_len: usize, i_next: bool) -> InterleaveProducer<I, J>` — [`InterleaveProducer`](#interleaveproducer)

#### Trait Implementations

##### `impl<T> IntoEither for InterleaveProducer<I, J>`

##### `impl<T> Pointable for InterleaveProducer<I, J>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<I, J> Producer for InterleaveProducer<I, J>`

- `type Item = <I as Producer>::Item`

- `type IntoIter = InterleaveSeq<<I as Producer>::IntoIter, <J as Producer>::IntoIter>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- `fn min_len(self: &Self) -> usize`

- `fn max_len(self: &Self) -> usize`

- `fn split_at(self: Self, index: usize) -> (Self, Self)`

### `InterleaveSeq<I, J>`

```rust
struct InterleaveSeq<I, J> {
    i: std::iter::Fuse<I>,
    j: std::iter::Fuse<J>,
    i_next: bool,
}
```

Wrapper for Interleave to implement DoubleEndedIterator and
ExactSizeIterator.

This iterator is fused.

#### Fields

- **`i_next`**: `bool`

  Flag to control which iterator should provide the next element. When
  `false` then `i` produces the next element, otherwise `j` produces the
  next element.

#### Trait Implementations

##### `impl<I, J> DoubleEndedIterator for InterleaveSeq<I, J>`

- `fn next_back(self: &mut Self) -> Option<<I as >::Item>`

##### `impl<I, J> ExactSizeIterator for InterleaveSeq<I, J>`

- `fn len(self: &Self) -> usize`

##### `impl<T> IntoEither for InterleaveSeq<I, J>`

##### `impl<I> IntoIterator for InterleaveSeq<I, J>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<I, J> Iterator for InterleaveSeq<I, J>`

- `type Item = <I as Iterator>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for InterleaveSeq<I, J>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

