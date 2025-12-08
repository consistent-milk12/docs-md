*[rayon](../../index.md) / [iter](../index.md) / [chain](index.md)*

---

# Module `chain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Chain`](#chain) | struct | `Chain` is an iterator that joins `b` after `a` in one continuous iterator. |
| [`ChainProducer`](#chainproducer) | struct |  |
| [`ChainSeq`](#chainseq) | struct | Wrapper for `Chain` to implement `ExactSizeIterator` |

## Structs

### `Chain<A, B>`

```rust
struct Chain<A, B> {
    a: A,
    b: B,
}
```

`Chain` is an iterator that joins `b` after `a` in one continuous iterator.
This struct is created by the `chain()` method on [`ParallelIterator`](../../prelude/index.md)


#### Implementations

- <span id="chain-new"></span>`fn new(a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Chain<A, B>`

- <span id="chain-clone"></span>`fn clone(&self) -> Chain<A, B>` — [`Chain`](../index.md)

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Chain<A, B>`

- <span id="chain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A, B> IndexedParallelIterator for Chain<A, B>`

- <span id="chain-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="chain-len"></span>`fn len(&self) -> usize`

- <span id="chain-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md)

##### `impl<T> IntoEither for Chain<A, B>`

##### `impl<T> IntoParallelIterator for Chain<A, B>`

- <span id="chain-iter"></span>`type Iter = T`

- <span id="chain-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chain-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Chain<A, B>`

- <span id="chain-item"></span>`type Item = <A as ParallelIterator>::Item`

- <span id="chain-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md)

- <span id="chain-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Chain<A, B>`

- <span id="chain-align"></span>`const ALIGN: usize`

- <span id="chain-init"></span>`type Init = T`

- <span id="chain-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chain-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chain-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chain-drop"></span>`unsafe fn drop(ptr: usize)`

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

- <span id="chainproducer-new"></span>`fn new(a_len: usize, a: A, b: B) -> Self`

#### Trait Implementations

##### `impl<T> IntoEither for ChainProducer<A, B>`

##### `impl<T> Pointable for ChainProducer<A, B>`

- <span id="chainproducer-align"></span>`const ALIGN: usize`

- <span id="chainproducer-init"></span>`type Init = T`

- <span id="chainproducer-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chainproducer-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chainproducer-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chainproducer-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<A, B> Producer for ChainProducer<A, B>`

- <span id="chainproducer-item"></span>`type Item = <A as Producer>::Item`

- <span id="chainproducer-intoiter"></span>`type IntoIter = ChainSeq<<A as Producer>::IntoIter, <B as Producer>::IntoIter>`

- <span id="chainproducer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md)

- <span id="chainproducer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="chainproducer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="chainproducer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="chainproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `ChainSeq<A, B>`

```rust
struct ChainSeq<A, B> {
    chain: iter::Chain<A, B>,
}
```

Wrapper for `Chain` to implement `ExactSizeIterator`

#### Implementations

- <span id="chainseq-new"></span>`fn new(a: A, b: B) -> ChainSeq<A, B>` — [`ChainSeq`](#chainseq)

#### Trait Implementations

##### `impl<A, B> DoubleEndedIterator for ChainSeq<A, B>`

- <span id="chainseq-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<A, B> ExactSizeIterator for ChainSeq<A, B>`

##### `impl<T> IntoEither for ChainSeq<A, B>`

##### `impl<I> IntoIterator for ChainSeq<A, B>`

- <span id="chainseq-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chainseq-intoiter"></span>`type IntoIter = I`

- <span id="chainseq-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A, B> Iterator for ChainSeq<A, B>`

- <span id="chainseq-item"></span>`type Item = <A as Iterator>::Item`

- <span id="chainseq-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="chainseq-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Pointable for ChainSeq<A, B>`

- <span id="chainseq-align"></span>`const ALIGN: usize`

- <span id="chainseq-init"></span>`type Init = T`

- <span id="chainseq-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chainseq-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chainseq-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chainseq-drop"></span>`unsafe fn drop(ptr: usize)`

