*[itertools](../index.md) / [zip_longest](index.md)*

---

# Module `zip_longest`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ZipLongest`](#ziplongest) | struct | An iterator which iterates two other iterators simultaneously and wraps the elements in [`EitherOrBoth`]. |
| [`zip_longest`](#zip-longest) | fn | Create a new `ZipLongest` iterator. |

## Structs

### `ZipLongest<T, U>`

```rust
struct ZipLongest<T, U> {
    a: std::iter::Fuse<T>,
    b: std::iter::Fuse<U>,
}
```

*Defined in [`itertools-0.14.0/src/zip_longest.rs:18-21`](../../../.source_1765900590/itertools-0.14.0/src/zip_longest.rs#L18-L21)*

An iterator which iterates two other iterators simultaneously
and wraps the elements in [`EitherOrBoth`](../either_or_both/index.md).

This iterator is *fused*.

See [`.zip_longest()`](crate::Itertools::zip_longest) for more information.

#### Trait Implementations

##### `impl<T> Any for ZipLongest<T, U>`

- <span id="ziplongest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ZipLongest<T, U>`

- <span id="ziplongest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ZipLongest<T, U>`

- <span id="ziplongest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone, U: clone::Clone> Clone for ZipLongest<T, U>`

- <span id="ziplongest-clone"></span>`fn clone(&self) -> ZipLongest<T, U>` — [`ZipLongest`](#ziplongest)

##### `impl<T> CloneToUninit for ZipLongest<T, U>`

- <span id="ziplongest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug, U: fmt::Debug> Debug for ZipLongest<T, U>`

- <span id="ziplongest-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> DoubleEndedIterator for ZipLongest<T, U>`

- <span id="ziplongest-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="ziplongest-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, U> ExactSizeIterator for ZipLongest<T, U>`

##### `impl<T> From for ZipLongest<T, U>`

- <span id="ziplongest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> FusedIterator for ZipLongest<T, U>`

##### `impl<T, U> Into for ZipLongest<T, U>`

- <span id="ziplongest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ZipLongest<T, U>`

##### `impl IntoIterator for ZipLongest<T, U>`

- <span id="ziplongest-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ziplongest-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="ziplongest-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, U> Iterator for ZipLongest<T, U>`

- <span id="ziplongest-iterator-type-item"></span>`type Item = EitherOrBoth<<T as Iterator>::Item, <U as Iterator>::Item>`

- <span id="ziplongest-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="ziplongest-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="ziplongest-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> Itertools for ZipLongest<T, U>`

##### `impl<T> ToOwned for ZipLongest<T, U>`

- <span id="ziplongest-toowned-type-owned"></span>`type Owned = T`

- <span id="ziplongest-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ziplongest-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for ZipLongest<T, U>`

- <span id="ziplongest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ziplongest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ZipLongest<T, U>`

- <span id="ziplongest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ziplongest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `zip_longest`

```rust
fn zip_longest<T, U>(a: T, b: U) -> ZipLongest<T, U>
where
    T: Iterator,
    U: Iterator
```

*Defined in [`itertools-0.14.0/src/zip_longest.rs:24-33`](../../../.source_1765900590/itertools-0.14.0/src/zip_longest.rs#L24-L33)*

Create a new `ZipLongest` iterator.

