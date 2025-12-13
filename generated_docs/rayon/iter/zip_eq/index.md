*[rayon](../../index.md) / [iter](../index.md) / [zip_eq](index.md)*

---

# Module `zip_eq`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ZipEq`](#zipeq) | struct | An [`IndexedParallelIterator`] that iterates over two parallel iterators of equal length simultaneously. |

## Structs

### `ZipEq<A, B>`

```rust
struct ZipEq<A, B> {
    zip: Zip<A, B>,
}
```

*Defined in [`rayon-1.11.0/src/iter/zip_eq.rs:13-15`](../../../../.source_1765521767/rayon-1.11.0/src/iter/zip_eq.rs#L13-L15)*

An [`IndexedParallelIterator`](../index.md) that iterates over two parallel iterators of equal
length simultaneously.

This struct is created by the [`zip_eq`](#zip-eq) method on [`IndexedParallelIterator`](../index.md),
see its documentation for more information.


#### Implementations

- <span id="zipeq-new"></span>`fn new(a: A, b: B) -> Self`

  Creates a new `ZipEq` iterator.

#### Trait Implementations

##### `impl Any for ZipEq<A, B>`

- <span id="zipeq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ZipEq<A, B>`

- <span id="zipeq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ZipEq<A, B>`

- <span id="zipeq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for ZipEq<A, B>`

- <span id="zipeq-clone"></span>`fn clone(&self) -> ZipEq<A, B>` — [`ZipEq`](#zipeq)

##### `impl CloneToUninit for ZipEq<A, B>`

- <span id="zipeq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for ZipEq<A, B>`

- <span id="zipeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ZipEq<A, B>`

- <span id="zipeq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, B> IndexedParallelIterator for ZipEq<A, B>`

- <span id="zipeq-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zipeq-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="zipeq-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../plumbing/index.md#producercallback)

##### `impl<U> Into for ZipEq<A, B>`

- <span id="zipeq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ZipEq<A, B>`

##### `impl IntoParallelIterator for ZipEq<A, B>`

- <span id="zipeq-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="zipeq-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="zipeq-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<A, B> ParallelIterator for ZipEq<A, B>`

- <span id="zipeq-paralleliterator-type-item"></span>`type Item = (<A as ParallelIterator>::Item, <B as ParallelIterator>::Item)`

- <span id="zipeq-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../plumbing/index.md#consumer)

- <span id="zipeq-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl Pointable for ZipEq<A, B>`

- <span id="zipeq-pointable-const-align"></span>`const ALIGN: usize`

- <span id="zipeq-pointable-type-init"></span>`type Init = T`

- <span id="zipeq-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="zipeq-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="zipeq-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="zipeq-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for ZipEq<A, B>`

- <span id="zipeq-toowned-type-owned"></span>`type Owned = T`

- <span id="zipeq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="zipeq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ZipEq<A, B>`

- <span id="zipeq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zipeq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ZipEq<A, B>`

- <span id="zipeq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zipeq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

