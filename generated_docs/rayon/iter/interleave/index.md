*[rayon](../../index.md) / [iter](../index.md) / [interleave](index.md)*

---

# Module `interleave`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Interleave`](#interleave) | struct | `Interleave` is an iterator that interleaves elements of iterators `i` and `j` in one continuous iterator. |
| [`InterleaveProducer`](#interleaveproducer) | struct |  |
| [`InterleaveSeq`](#interleaveseq) | struct | Wrapper for Interleave to implement DoubleEndedIterator and ExactSizeIterator. |

## Structs

### `Interleave<I, J>`

```rust
struct Interleave<I, J> {
    i: I,
    j: J,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave.rs:12-15`](../../../../.source_1765521767/rayon-1.11.0/src/iter/interleave.rs#L12-L15)*

`Interleave` is an iterator that interleaves elements of iterators
`i` and `j` in one continuous iterator. This struct is created by
the `interleave()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="interleave-new"></span>`fn new(i: I, j: J) -> Self`

  Creates a new `Interleave` iterator

#### Trait Implementations

##### `impl Any for Interleave<I, J>`

- <span id="interleave-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Interleave<I, J>`

- <span id="interleave-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Interleave<I, J>`

- <span id="interleave-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, J: clone::Clone> Clone for Interleave<I, J>`

- <span id="interleave-clone"></span>`fn clone(&self) -> Interleave<I, J>` — [`Interleave`](#interleave)

##### `impl CloneToUninit for Interleave<I, J>`

- <span id="interleave-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for Interleave<I, J>`

- <span id="interleave-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Interleave<I, J>`

- <span id="interleave-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> IndexedParallelIterator for Interleave<I, J>`

- <span id="interleave-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="interleave-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="interleave-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Interleave<I, J>`

- <span id="interleave-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Interleave<I, J>`

##### `impl IntoParallelIterator for Interleave<I, J>`

- <span id="interleave-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="interleave-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="interleave-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I, J> ParallelIterator for Interleave<I, J>`

- <span id="interleave-paralleliterator-type-item"></span>`type Item = <I as ParallelIterator>::Item`

- <span id="interleave-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="interleave-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Interleave<I, J>`

- <span id="interleave-pointable-const-align"></span>`const ALIGN: usize`

- <span id="interleave-pointable-type-init"></span>`type Init = T`

- <span id="interleave-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleave-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleave-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleave-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Interleave<I, J>`

- <span id="interleave-toowned-type-owned"></span>`type Owned = T`

- <span id="interleave-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interleave-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Interleave<I, J>`

- <span id="interleave-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interleave-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Interleave<I, J>`

- <span id="interleave-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interleave-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`rayon-1.11.0/src/iter/interleave.rs:133-143`](../../../../.source_1765521767/rayon-1.11.0/src/iter/interleave.rs#L133-L143)*

#### Implementations

- <span id="interleaveproducer-new"></span>`fn new(i: I, j: J, i_len: usize, j_len: usize, i_next: bool) -> InterleaveProducer<I, J>` — [`InterleaveProducer`](#interleaveproducer)

#### Trait Implementations

##### `impl Any for InterleaveProducer<I, J>`

- <span id="interleaveproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InterleaveProducer<I, J>`

- <span id="interleaveproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InterleaveProducer<I, J>`

- <span id="interleaveproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for InterleaveProducer<I, J>`

- <span id="interleaveproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InterleaveProducer<I, J>`

- <span id="interleaveproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for InterleaveProducer<I, J>`

##### `impl Pointable for InterleaveProducer<I, J>`

- <span id="interleaveproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="interleaveproducer-pointable-type-init"></span>`type Init = T`

- <span id="interleaveproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleaveproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleaveproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleaveproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<I, J> Producer for InterleaveProducer<I, J>`

- <span id="interleaveproducer-producer-type-item"></span>`type Item = <I as Producer>::Item`

- <span id="interleaveproducer-producer-type-intoiter"></span>`type IntoIter = InterleaveSeq<<I as Producer>::IntoIter, <J as Producer>::IntoIter>`

- <span id="interleaveproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="interleaveproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="interleaveproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="interleaveproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

  We know 0 < index <= self.i_len + self.j_len

  

  Find a, b satisfying:

  

   (1) 0 < a <= self.i_len

   (2) 0 < b <= self.j_len

   (3) a + b == index

  

  For even splits, set a = b = index/2.

  For odd splits, set a = (index/2)+1, b = index/2, if `i`

  should yield the next element, otherwise, if `j` should yield

  the next element, set a = index/2 and b = (index/2)+1

##### `impl<U> TryFrom for InterleaveProducer<I, J>`

- <span id="interleaveproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interleaveproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InterleaveProducer<I, J>`

- <span id="interleaveproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interleaveproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InterleaveSeq<I, J>`

```rust
struct InterleaveSeq<I, J> {
    i: std::iter::Fuse<I>,
    j: std::iter::Fuse<J>,
    i_next: bool,
}
```

*Defined in [`rayon-1.11.0/src/iter/interleave.rs:243-251`](../../../../.source_1765521767/rayon-1.11.0/src/iter/interleave.rs#L243-L251)*

Wrapper for Interleave to implement DoubleEndedIterator and
ExactSizeIterator.

This iterator is fused.

#### Fields

- **`i_next`**: `bool`

  Flag to control which iterator should provide the next element. When
  `false` then `i` produces the next element, otherwise `j` produces the
  next element.

#### Trait Implementations

##### `impl Any for InterleaveSeq<I, J>`

- <span id="interleaveseq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InterleaveSeq<I, J>`

- <span id="interleaveseq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InterleaveSeq<I, J>`

- <span id="interleaveseq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, J> DoubleEndedIterator for InterleaveSeq<I, J>`

- <span id="interleaveseq-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<I as >::Item>`

##### `impl<I, J> ExactSizeIterator for InterleaveSeq<I, J>`

- <span id="interleaveseq-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for InterleaveSeq<I, J>`

- <span id="interleaveseq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for InterleaveSeq<I, J>`

- <span id="interleaveseq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for InterleaveSeq<I, J>`

##### `impl<I> IntoIterator for InterleaveSeq<I, J>`

- <span id="interleaveseq-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleaveseq-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="interleaveseq-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, J> Iterator for InterleaveSeq<I, J>`

- <span id="interleaveseq-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleaveseq-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="interleaveseq-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for InterleaveSeq<I, J>`

- <span id="interleaveseq-pointable-const-align"></span>`const ALIGN: usize`

- <span id="interleaveseq-pointable-type-init"></span>`type Init = T`

- <span id="interleaveseq-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="interleaveseq-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="interleaveseq-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="interleaveseq-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for InterleaveSeq<I, J>`

- <span id="interleaveseq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interleaveseq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InterleaveSeq<I, J>`

- <span id="interleaveseq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interleaveseq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

