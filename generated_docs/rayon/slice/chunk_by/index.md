*[rayon](../../index.md) / [slice](../index.md) / [chunk_by](index.md)*

---

# Module `chunk_by`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ChunkByProducer`](#chunkbyproducer) | struct |  |
| [`ChunkBy`](#chunkby) | struct | Parallel iterator over slice in (non-overlapping) chunks separated by a predicate. |
| [`ChunkByMut`](#chunkbymut) | struct | Parallel iterator over slice in (non-overlapping) mutable chunks separated by a predicate. |
| [`ChunkBySlice`](#chunkbyslice) | trait |  |

## Structs

### `ChunkByProducer<'p, T, Slice, Pred>`

```rust
struct ChunkByProducer<'p, T, Slice, Pred> {
    slice: Slice,
    pred: &'p Pred,
    tail: usize,
    marker: std::marker::PhantomData<fn(&T)>,
}
```

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:45-50`](../../../../.source_1765521767/rayon-1.11.0/src/slice/chunk_by.rs#L45-L50)*

#### Trait Implementations

##### `impl<T> Any for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunkByProducer<'p, T, Slice, Pred>`

##### `impl<T> Pointable for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkbyproducer-pointable-type-init"></span>`type Init = T`

- <span id="chunkbyproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkbyproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkbyproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkbyproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkbyproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunkByProducer<'p, T, Slice, Pred>`

- <span id="chunkbyproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkbyproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T, Slice, Pred> UnindexedProducer for ChunkByProducer<'_, T, Slice, Pred>`

- <span id="chunkbyproducer-unindexedproducer-type-item"></span>`type Item = Slice`

- <span id="chunkbyproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="chunkbyproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

### `ChunkBy<'data, T, P>`

```rust
struct ChunkBy<'data, T, P> {
    pred: P,
    slice: &'data [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:142-145`](../../../../.source_1765521767/rayon-1.11.0/src/slice/chunk_by.rs#L142-L145)*

Parallel iterator over slice in (non-overlapping) chunks separated by a predicate.

This struct is created by the `par_chunk_by` method on `&[T]`.


#### Implementations

- <span id="chunkby-new"></span>`fn new(slice: &'data [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T> Any for ChunkBy<'data, T, P>`

- <span id="chunkby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkBy<'data, T, P>`

- <span id="chunkby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkBy<'data, T, P>`

- <span id="chunkby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T, P: Clone> Clone for ChunkBy<'_, T, P>`

- <span id="chunkby-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for ChunkBy<'data, T, P>`

- <span id="chunkby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug, P> Debug for ChunkBy<'_, T, P>`

- <span id="chunkby-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunkBy<'data, T, P>`

- <span id="chunkby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunkBy<'data, T, P>`

- <span id="chunkby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunkBy<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunkby-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkby-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for ChunkBy<'data, T, P>`

- <span id="chunkby-paralleliterator-type-item"></span>`type Item = &'data [T]`

- <span id="chunkby-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for ChunkBy<'data, T, P>`

- <span id="chunkby-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkby-pointable-type-init"></span>`type Init = T`

- <span id="chunkby-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkby-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkby-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkby-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for ChunkBy<'data, T, P>`

- <span id="chunkby-toowned-type-owned"></span>`type Owned = T`

- <span id="chunkby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunkby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for ChunkBy<'data, T, P>`

- <span id="chunkby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunkBy<'data, T, P>`

- <span id="chunkby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunkByMut<'data, T, P>`

```rust
struct ChunkByMut<'data, T, P> {
    pred: P,
    slice: &'data mut [T],
}
```

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:199-202`](../../../../.source_1765521767/rayon-1.11.0/src/slice/chunk_by.rs#L199-L202)*

Parallel iterator over slice in (non-overlapping) mutable chunks
separated by a predicate.

This struct is created by the `par_chunk_by_mut` method on `&mut [T]`.


#### Implementations

- <span id="chunkbymut-new"></span>`fn new(slice: &'data mut [T], pred: P) -> Self`

#### Trait Implementations

##### `impl<T> Any for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Debug, P> Debug for ChunkByMut<'_, T, P>`

- <span id="chunkbymut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ChunkByMut<'data, T, P>`

##### `impl<T> IntoParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="chunkbymut-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="chunkbymut-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T, P> ParallelIterator for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-paralleliterator-type-item"></span>`type Item = &'data mut [T]`

- <span id="chunkbymut-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../../iter/plumbing/index.md#consumer)

##### `impl<T> Pointable for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-pointable-const-align"></span>`const ALIGN: usize`

- <span id="chunkbymut-pointable-type-init"></span>`type Init = T`

- <span id="chunkbymut-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="chunkbymut-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="chunkbymut-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="chunkbymut-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T, U> TryFrom for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkbymut-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ChunkByMut<'data, T, P>`

- <span id="chunkbymut-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkbymut-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `ChunkBySlice<T>`

```rust
trait ChunkBySlice<T>: AsRef<[T]> + Default + Send { ... }
```

*Defined in [`rayon-1.11.0/src/slice/chunk_by.rs:6-23`](../../../../.source_1765521767/rayon-1.11.0/src/slice/chunk_by.rs#L6-L23)*

#### Required Methods

- `fn split(self, index: usize) -> (Self, Self)`

- `fn chunk_by(self, pred: &impl Fn(&T, &T) -> bool) -> impl Iterator<Item = Self>`

#### Provided Methods

- `fn find(&self, pred: &impl Fn(&T, &T) -> bool, start: usize, end: usize) -> Option<usize>`

- `fn rfind(&self, pred: &impl Fn(&T, &T) -> bool, end: usize) -> Option<usize>`

#### Implementors

- `&[T]`
- `&mut [T]`

