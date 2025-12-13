*[rayon](../../index.md) / [iter](../index.md) / [zip](index.md)*

---

# Module `zip`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Zip`](#zip) | struct | `Zip` is an iterator that zips up `a` and `b` into a single iterator of pairs. |
| [`ZipProducer`](#zipproducer) | struct |  |

## Structs

### `Zip<A, B>`

```rust
struct Zip<A, B> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip.rs:12-15`](../../../../.source_1765633015/rayon-1.11.0/src/iter/zip.rs#L12-L15)*

`Zip` is an iterator that zips up `a` and `b` into a single iterator
of pairs. This struct is created by the `zip()` method on
[`IndexedParallelIterator`](../index.md)


#### Implementations

- <span id="zip-new"></span>`fn new(a: A, b: B) -> Self`

  Creates a new `Zip` iterator.

#### Trait Implementations

##### `impl Any for Zip<A, B>`

- <span id="zip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Zip<A, B>`

- <span id="zip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Zip<A, B>`

- <span id="zip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for Zip<A, B>`

- <span id="zip-clone"></span>`fn clone(&self) -> Zip<A, B>` — [`Zip`](#zip)

##### `impl CloneToUninit for Zip<A, B>`

- <span id="zip-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for Zip<A, B>`

- <span id="zip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Zip<A, B>`

- <span id="zip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, B> IndexedParallelIterator for Zip<A, B>`

- <span id="zip-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zip-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="zip-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for Zip<A, B>`

- <span id="zip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Zip<A, B>`

##### `impl IntoParallelIterator for Zip<A, B>`

- <span id="zip-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="zip-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zip-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for Zip<A, B>`

- <span id="zip-paralleliterator-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zip-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zip-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for Zip<A, B>`

- <span id="zip-pointable-const-align"></span>`const ALIGN: usize`

- <span id="zip-pointable-type-init"></span>`type Init = T`

- <span id="zip-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zip-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zip-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zip-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for Zip<A, B>`

- <span id="zip-toowned-type-owned"></span>`type Owned = T`

- <span id="zip-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="zip-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Zip<A, B>`

- <span id="zip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Zip<A, B>`

- <span id="zip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ZipProducer<A: Producer, B: Producer>`

```rust
struct ZipProducer<A: Producer, B: Producer> {
    a: A,
    b: B,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip.rs:118-121`](../../../../.source_1765633015/rayon-1.11.0/src/iter/zip.rs#L118-L121)*

#### Trait Implementations

##### `impl Any for ZipProducer<A, B>`

- <span id="zipproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ZipProducer<A, B>`

- <span id="zipproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ZipProducer<A, B>`

- <span id="zipproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ZipProducer<A, B>`

- <span id="zipproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ZipProducer<A, B>`

- <span id="zipproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ZipProducer<A, B>`

##### `impl Pointable for ZipProducer<A, B>`

- <span id="zipproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="zipproducer-pointable-type-init"></span>`type Init = T`

- <span id="zipproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zipproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zipproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zipproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<A: Producer, B: Producer> Producer for ZipProducer<A, B>`

- <span id="zipproducer-producer-type-item"></span>`type Item = (<A as Producer>::Item, <B as Producer>::Item)`

- <span id="zipproducer-producer-type-intoiter"></span>`type IntoIter = Zip<<A as Producer>::IntoIter, <B as Producer>::IntoIter>`

- <span id="zipproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../plumbing/index.md#producer)

- <span id="zipproducer-producer-min-len"></span>`fn min_len(&self) -> usize`

- <span id="zipproducer-producer-max-len"></span>`fn max_len(&self) -> usize`

- <span id="zipproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<U> TryFrom for ZipProducer<A, B>`

- <span id="zipproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zipproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ZipProducer<A, B>`

- <span id="zipproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zipproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

