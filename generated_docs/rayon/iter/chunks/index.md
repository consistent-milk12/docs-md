*[rayon](../../index.md) / [iter](../index.md) / [chunks](index.md)*

---

# Module `chunks`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Chunks`](#chunks) | struct | `Chunks` is an iterator that groups elements of an underlying iterator. |
| [`ChunkProducer`](#chunkproducer) | struct |  |
| [`ChunkSeq`](#chunkseq) | struct |  |

## Structs

### `Chunks<I>`

```rust
struct Chunks<I> {
    size: usize,
    i: I,
}
```

*Defined in [`rayon-1.11.0/src/iter/chunks.rs:11-14`](../../../../.source_1765633015/rayon-1.11.0/src/iter/chunks.rs#L11-L14)*

`Chunks` is an iterator that groups elements of an underlying iterator.

This struct is created by the `chunks()` method on [`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="chunks-new"></span>`fn new(i: I, size: usize) -> Self`

  Creates a new `Chunks` iterator

#### Trait Implementations

##### `impl Any for Chunks<I>`

- <span id="chunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chunks<I>`

- <span id="chunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chunks<I>`

- <span id="chunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Chunks<I>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Chunks<I>` — [`Chunks`](#chunks)

##### `impl CloneToUninit for Chunks<I>`

- <span id="chunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Chunks<I>`

- <span id="chunks-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Chunks<I>`

- <span id="chunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> IndexedParallelIterator for Chunks<I>`

- <span id="chunks-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="chunks-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="chunks-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Chunks<I>`

- <span id="chunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chunks<I>`

##### `impl IntoParallelIterator for Chunks<I>`

- <span id="chunks-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunks-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunks-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<I> ParallelIterator for Chunks<I>`

- <span id="chunks-paralleliterator-type-item"></span>`type Item = Vec<<I as ParallelIterator>::Item>`

- <span id="chunks-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="chunks-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Chunks<I>`

- <span id="chunks-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunks-pointable-type-init"></span>`type Init = T`

- <span id="chunks-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunks-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunks-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunks-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Chunks<I>`

- <span id="chunks-toowned-type-owned"></span>`type Owned = T`

- <span id="chunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chunks<I>`

- <span id="chunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chunks<I>`

- <span id="chunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunkProducer<P, F>`

```rust
struct ChunkProducer<P, F> {
    chunk_size: usize,
    len: usize,
    base: P,
    map: F,
}
```

*Defined in [`rayon-1.11.0/src/iter/chunks.rs:90-95`](../../../../.source_1765633015/rayon-1.11.0/src/iter/chunks.rs#L90-L95)*

#### Implementations

- <span id="chunkproducer-new"></span>`fn new(chunk_size: usize, len: usize, base: P, map: F) -> Self`

#### Trait Implementations

##### `impl Any for ChunkProducer<P, F>`

- <span id="chunkproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkProducer<P, F>`

- <span id="chunkproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkProducer<P, F>`

- <span id="chunkproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunkProducer<P, F>`

- <span id="chunkproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChunkProducer<P, F>`

- <span id="chunkproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ChunkProducer<P, F>`

##### `impl Pointable for ChunkProducer<P, F>`

- <span id="chunkproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkproducer-pointable-type-init"></span>`type Init = T`

- <span id="chunkproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<P, F> Producer for ChunkProducer<P, F>`

- <span id="chunkproducer-producer-type-item"></span>`type Item = T`

- <span id="chunkproducer-producer-type-intoiter"></span>`type IntoIter = Map<ChunkSeq<P>, F>`

- <span id="chunkproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="chunkproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

- <span id="chunkproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="chunkproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

##### `impl<U> TryFrom for ChunkProducer<P, F>`

- <span id="chunkproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChunkProducer<P, F>`

- <span id="chunkproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunkSeq<P>`

```rust
struct ChunkSeq<P> {
    chunk_size: usize,
    len: usize,
    inner: Option<P>,
}
```

*Defined in [`rayon-1.11.0/src/iter/chunks.rs:153-157`](../../../../.source_1765633015/rayon-1.11.0/src/iter/chunks.rs#L153-L157)*

#### Trait Implementations

##### `impl Any for ChunkSeq<P>`

- <span id="chunkseq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkSeq<P>`

- <span id="chunkseq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkSeq<P>`

- <span id="chunkseq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P> DoubleEndedIterator for ChunkSeq<P>`

- <span id="chunkseq-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<P> ExactSizeIterator for ChunkSeq<P>`

- <span id="chunkseq-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for ChunkSeq<P>`

- <span id="chunkseq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChunkSeq<P>`

- <span id="chunkseq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ChunkSeq<P>`

##### `impl IntoIterator for ChunkSeq<P>`

- <span id="chunkseq-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunkseq-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chunkseq-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<P> Iterator for ChunkSeq<P>`

- <span id="chunkseq-iterator-type-item"></span>`type Item = <P as Producer>::IntoIter`

- <span id="chunkseq-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="chunkseq-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Pointable for ChunkSeq<P>`

- <span id="chunkseq-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkseq-pointable-type-init"></span>`type Init = T`

- <span id="chunkseq-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkseq-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkseq-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkseq-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for ChunkSeq<P>`

- <span id="chunkseq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkseq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChunkSeq<P>`

- <span id="chunkseq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkseq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

