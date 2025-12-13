*[rayon](../index.md) / [string](index.md)*

---

# Module `string`

This module contains the parallel iterator types for owned strings
(`String`). You will rarely need to interact with it directly
unless you have need to name one of the iterator types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | Draining parallel iterator that moves a range of characters out of a string, but keeps the total capacity. |

## Structs

### `Drain<'a>`

```rust
struct Drain<'a> {
    string: &'a mut String,
    range: std::ops::Range<usize>,
}
```

*Defined in [`rayon-1.11.0/src/string.rs:25-28`](../../../.source_1765633015/rayon-1.11.0/src/string.rs#L25-L28)*

Draining parallel iterator that moves a range of characters out of a string,
but keeps the total capacity.

#### Trait Implementations

##### `impl Any for Drain<'a>`

- <span id="drain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Drain<'a>`

- <span id="drain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Drain<'a>`

- <span id="drain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Drain<'a>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for Drain<'a>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Drain<'a>`

- <span id="drain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Drain<'a>`

- <span id="drain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Drain<'a>`

##### `impl IntoParallelIterator for Drain<'a>`

- <span id="drain-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="drain-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="drain-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl ParallelIterator for Drain<'a>`

- <span id="drain-paralleliterator-type-item"></span>`type Item = char`

- <span id="drain-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

##### `impl Pointable for Drain<'a>`

- <span id="drain-pointable-const-align"></span>`const ALIGN: usize`

- <span id="drain-pointable-type-init"></span>`type Init = T`

- <span id="drain-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="drain-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="drain-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="drain-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for Drain<'a>`

- <span id="drain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="drain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Drain<'a>`

- <span id="drain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="drain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

