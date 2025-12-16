*[itertools](../index.md) / [exactly_one_err](index.md)*

---

# Module `exactly_one_err`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ExactlyOneError`](#exactlyoneerror) | struct | Iterator returned for the error case of `Itertools::exactly_one()` This iterator yields exactly the same elements as the input iterator. |

## Structs

### `ExactlyOneError<I>`

```rust
struct ExactlyOneError<I>
where
    I: Iterator {
    first_two: Option<either::Either<[<I as >::Item; 2], <I as >::Item>>,
    inner: I,
}
```

*Defined in [`itertools-0.14.0/src/exactly_one_err.rs:20-26`](../../../.source_1765900590/itertools-0.14.0/src/exactly_one_err.rs#L20-L26)*

Iterator returned for the error case of `Itertools::exactly_one()`
This iterator yields exactly the same elements as the input iterator.

During the execution of `exactly_one` the iterator must be mutated.  This wrapper
effectively "restores" the state of the input iterator when it's handed back.

This is very similar to `PutBackN` except this iterator only supports 0-2 elements and does not
use a `Vec`.

#### Implementations

- <span id="exactlyoneerror-new"></span>`fn new(first_two: Option<Either<[<I as >::Item; 2], <I as >::Item>>, inner: I) -> Self` — [`Either`](../index.md#either)

  Creates a new `ExactlyOneErr` iterator.

- <span id="exactlyoneerror-additional-len"></span>`fn additional_len(&self) -> usize`

#### Trait Implementations

##### `impl Any for ExactlyOneError<I>`

- <span id="exactlyoneerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExactlyOneError<I>`

- <span id="exactlyoneerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExactlyOneError<I>`

- <span id="exactlyoneerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for ExactlyOneError<I>`

- <span id="exactlyoneerror-clone"></span>`fn clone(&self) -> ExactlyOneError<I>` — [`ExactlyOneError`](#exactlyoneerror)

##### `impl CloneToUninit for ExactlyOneError<I>`

- <span id="exactlyoneerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for ExactlyOneError<I>`

- <span id="exactlyoneerror-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult`

##### `impl<I> Display for ExactlyOneError<I>`

- <span id="exactlyoneerror-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult`

##### `impl<I> Error for ExactlyOneError<I>`

##### `impl<I> ExactSizeIterator for ExactlyOneError<I>`

##### `impl<T> From for ExactlyOneError<I>`

- <span id="exactlyoneerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExactlyOneError<I>`

- <span id="exactlyoneerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ExactlyOneError<I>`

##### `impl<I> IntoIterator for ExactlyOneError<I>`

- <span id="exactlyoneerror-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="exactlyoneerror-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="exactlyoneerror-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for ExactlyOneError<I>`

- <span id="exactlyoneerror-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="exactlyoneerror-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="exactlyoneerror-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="exactlyoneerror-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for ExactlyOneError<I>`

##### `impl MultiUnzip for ExactlyOneError<I>`

- <span id="exactlyoneerror-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for ExactlyOneError<I>`

- <span id="exactlyoneerror-toowned-type-owned"></span>`type Owned = T`

- <span id="exactlyoneerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exactlyoneerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ExactlyOneError<I>`

- <span id="exactlyoneerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ExactlyOneError<I>`

- <span id="exactlyoneerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exactlyoneerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExactlyOneError<I>`

- <span id="exactlyoneerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exactlyoneerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

