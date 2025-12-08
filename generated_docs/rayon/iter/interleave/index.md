*[rayon](../../index.md) / [iter](../index.md) / [interleave](index.md)*

---

# Module `interleave`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Interleave`](#interleave) | struct | `Interleave` is an iterator that interleaves elements of iterators |
| [`InterleaveProducer`](#interleaveproducer) | struct |  |
| [`InterleaveSeq`](#interleaveseq) | struct | Wrapper for Interleave to implement DoubleEndedIterator and |

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
the `interleave()` method on [`IndexedParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="interleave-new"></span>`fn new(i: I, j: J) -> Self`

#### Trait Implementations

##### `impl<I: clone::Clone, J: clone::Clone> Clone for Interleave<I, J>`

- <span id="interleave-clone"></span>`fn clone(&self) -> Interleave<I, J>` — [`Interleave`](../index.md)

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for Interleave<I, J>`

- <span id="interleave-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, J> IndexedParallelIterator for Interleave<I, J>`

- <span id="interleave-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="interleave-len"></span>`fn len(&self) -> usize`

- <span id="interleave-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Interleave<I, J>`

##### `impl<T> IntoParallelIterator for Interleave<I, J>`

- <span id="interleave-iter"></span>`type Iter = T`

- <span id="interleave-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="interleave-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, J> ParallelIterator for Interleave<I, J>`

- <span id="interleave-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="interleave-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="interleave-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Interleave<I, J>`

- <span id="interleave-align"></span>`const ALIGN: usize`

- <span id="interleave-init"></span>`type Init = T`

- <span id="interleave-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleave-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleave-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleave-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="interleaveproducer-new"></span>`fn new(i: I, j: J, i_len: usize, j_len: usize, i_next: bool) -> InterleaveProducer<I, J>` — [`InterleaveProducer`](#interleaveproducer)

#### Trait Implementations

##### `impl<T> IntoEither for InterleaveProducer<I, J>`

##### `impl<T> Pointable for InterleaveProducer<I, J>`

- <span id="interleaveproducer-align"></span>`const ALIGN: usize`

- <span id="interleaveproducer-init"></span>`type Init = T`

- <span id="interleaveproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleaveproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleaveproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleaveproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<I, J> Producer for InterleaveProducer<I, J>`

- <span id="interleaveproducer-item"></span>`type Item = <I as Producer>::Item`

- <span id="interleaveproducer-intoiter"></span>`type IntoIter = InterleaveSeq<<I as Producer>::IntoIter, <J as Producer>::IntoIter>`

- <span id="interleaveproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="interleaveproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="interleaveproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="interleaveproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

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

- <span id="interleaveseq-next-back"></span>`fn next_back(&mut self) -> Option<<I as >::Item>`

##### `impl<I, J> ExactSizeIterator for InterleaveSeq<I, J>`

- <span id="interleaveseq-len"></span>`fn len(&self) -> usize`

##### `impl<T> IntoEither for InterleaveSeq<I, J>`

##### `impl<I> IntoIterator for InterleaveSeq<I, J>`

- <span id="interleaveseq-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleaveseq-intoiter"></span>`type IntoIter = I`

- <span id="interleaveseq-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, J> Iterator for InterleaveSeq<I, J>`

- <span id="interleaveseq-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleaveseq-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="interleaveseq-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for InterleaveSeq<I, J>`

- <span id="interleaveseq-align"></span>`const ALIGN: usize`

- <span id="interleaveseq-init"></span>`type Init = T`

- <span id="interleaveseq-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleaveseq-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleaveseq-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleaveseq-drop"></span>`unsafe fn drop(ptr: usize)`

