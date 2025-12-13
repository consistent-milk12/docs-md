*[rayon](../index.md) / [array](index.md)*

---

# Module `array`

Parallel iterator types for [arrays] (`[T; N]`)

You will rarely need to interact with this module directly unless you need
to name one of the iterator types.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | Parallel iterator that moves out of an array. |

## Structs

### `IntoIter<T, const N: usize>`

```rust
struct IntoIter<T, const N: usize> {
    array: [T; N],
}
```

*Defined in [`rayon-1.11.0/src/array.rs:43-45`](../../../.source_1765633015/rayon-1.11.0/src/array.rs#L43-L45)*

Parallel iterator that moves out of an array.

#### Trait Implementations

##### `impl<T> Any for IntoIter<T, N>`

- <span id="intoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoIter<T, N>`

- <span id="intoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoIter<T, N>`

- <span id="intoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for IntoIter<T, N>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<T, N>` — [`IntoIter`](#intoiter)

##### `impl<T> CloneToUninit for IntoIter<T, N>`

- <span id="intoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for IntoIter<T, N>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntoIter<T, N>`

- <span id="intoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Send> IndexedParallelIterator for IntoIter<T, N>`

- <span id="intoiter-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="intoiter-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for IntoIter<T, N>`

- <span id="intoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IntoIter<T, N>`

##### `impl<T> IntoParallelIterator for IntoIter<T, N>`

- <span id="intoiter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="intoiter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="intoiter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: Send> ParallelIterator for IntoIter<T, N>`

- <span id="intoiter-paralleliterator-type-item"></span>`type Item = T`

- <span id="intoiter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="intoiter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for IntoIter<T, N>`

- <span id="intoiter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="intoiter-pointable-type-init"></span>`type Init = T`

- <span id="intoiter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="intoiter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="intoiter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="intoiter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for IntoIter<T, N>`

- <span id="intoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="intoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for IntoIter<T, N>`

- <span id="intoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IntoIter<T, N>`

- <span id="intoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

