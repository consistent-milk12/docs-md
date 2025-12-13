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

*Defined in [`rayon-1.11.0/src/iter/chain.rs:12-15`](../../../../.source_1765521767/rayon-1.11.0/src/iter/chain.rs#L12-L15)*

`Chain` is an iterator that joins `b` after `a` in one continuous iterator.
This struct is created by the `chain()` method on [`ParallelIterator`](../index.md)


#### Implementations

- <span id="chain-new"></span>`fn new(a: A, b: B) -> Self`

  Creates a new `Chain` iterator.

#### Trait Implementations

##### `impl Any for Chain<A, B>`

- <span id="chain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chain<A, B>`

- <span id="chain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chain<A, B>`

- <span id="chain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Chain<A, B>`

- <span id="chain-clone"></span>`fn clone(&self) -> Chain<A, B>` — [`Chain`](#chain)

##### `impl CloneToUninit for Chain<A, B>`

- <span id="chain-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Chain<A, B>`

- <span id="chain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Chain<A, B>`

- <span id="chain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, B> IndexedParallelIterator for Chain<A, B>`

- <span id="chain-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="chain-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chain-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Chain<A, B>`

- <span id="chain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chain<A, B>`

##### `impl IntoParallelIterator for Chain<A, B>`

- <span id="chain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chain-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Chain<A, B>`

- <span id="chain-paralleliterator-type-item"></span>`type Item = <A as ParallelIterator>::Item`

- <span id="chain-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="chain-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Chain<A, B>`

- <span id="chain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chain-pointable-type-init"></span>`type Init = T`

- <span id="chain-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chain-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chain-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chain-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Chain<A, B>`

- <span id="chain-toowned-type-owned"></span>`type Owned = T`

- <span id="chain-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chain-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chain<A, B>`

- <span id="chain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chain<A, B>`

- <span id="chain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-1.11.0/src/iter/chain.rs:138-146`](../../../../.source_1765521767/rayon-1.11.0/src/iter/chain.rs#L138-L146)*

#### Implementations

- <span id="chainproducer-new"></span>`fn new(a_len: usize, a: A, b: B) -> Self`

#### Trait Implementations

##### `impl Any for ChainProducer<A, B>`

- <span id="chainproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChainProducer<A, B>`

- <span id="chainproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChainProducer<A, B>`

- <span id="chainproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChainProducer<A, B>`

- <span id="chainproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChainProducer<A, B>`

- <span id="chainproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ChainProducer<A, B>`

##### `impl Pointable for ChainProducer<A, B>`

- <span id="chainproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chainproducer-pointable-type-init"></span>`type Init = T`

- <span id="chainproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chainproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chainproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chainproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<A, B> Producer for ChainProducer<A, B>`

- <span id="chainproducer-producer-type-item"></span>`type Item = <A as Producer>::Item`

- <span id="chainproducer-producer-type-intoiter"></span>`type IntoIter = ChainSeq<<A as Producer>::IntoIter, <B as Producer>::IntoIter>`

- <span id="chainproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="chainproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="chainproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="chainproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="chainproducer-producer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

##### `impl<U> TryFrom for ChainProducer<A, B>`

- <span id="chainproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chainproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChainProducer<A, B>`

- <span id="chainproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chainproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChainSeq<A, B>`

```rust
struct ChainSeq<A, B> {
    chain: iter::Chain<A, B>,
}
```

*Defined in [`rayon-1.11.0/src/iter/chain.rs:213-215`](../../../../.source_1765521767/rayon-1.11.0/src/iter/chain.rs#L213-L215)*

Wrapper for `Chain` to implement `ExactSizeIterator`

#### Implementations

- <span id="chainseq-new"></span>`fn new(a: A, b: B) -> ChainSeq<A, B>` — [`ChainSeq`](#chainseq)

#### Trait Implementations

##### `impl Any for ChainSeq<A, B>`

- <span id="chainseq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChainSeq<A, B>`

- <span id="chainseq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChainSeq<A, B>`

- <span id="chainseq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A, B> DoubleEndedIterator for ChainSeq<A, B>`

- <span id="chainseq-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<A, B> ExactSizeIterator for ChainSeq<A, B>`

##### `impl<T> From for ChainSeq<A, B>`

- <span id="chainseq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChainSeq<A, B>`

- <span id="chainseq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ChainSeq<A, B>`

##### `impl IntoIterator for ChainSeq<A, B>`

- <span id="chainseq-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chainseq-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chainseq-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A, B> Iterator for ChainSeq<A, B>`

- <span id="chainseq-iterator-type-item"></span>`type Item = <A as Iterator>::Item`

- <span id="chainseq-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="chainseq-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for ChainSeq<A, B>`

- <span id="chainseq-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chainseq-pointable-type-init"></span>`type Init = T`

- <span id="chainseq-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chainseq-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chainseq-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chainseq-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ChainSeq<A, B>`

- <span id="chainseq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chainseq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChainSeq<A, B>`

- <span id="chainseq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chainseq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

