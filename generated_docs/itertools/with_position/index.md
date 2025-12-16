*[itertools](../index.md) / [with_position](index.md)*

---

# Module `with_position`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WithPosition`](#withposition) | struct | An iterator adaptor that wraps each element in an [`Position`]. |
| [`Position`](#position) | enum | The first component of the value yielded by `WithPosition`. |
| [`with_position`](#with-position) | fn | Create a new `WithPosition` iterator. |

## Structs

### `WithPosition<I>`

```rust
struct WithPosition<I>
where
    I: Iterator {
    handled_first: bool,
    peekable: std::iter::Peekable<std::iter::Fuse<I>>,
}
```

*Defined in [`itertools-0.14.0/src/with_position.rs:10-16`](../../../.source_1765894658/itertools-0.14.0/src/with_position.rs#L10-L16)*

An iterator adaptor that wraps each element in an [`Position`](#position).

Iterator element type is `(Position, I::Item)`.

See [`.with_position()`](crate::Itertools::with_position) for more information.

#### Trait Implementations

##### `impl Any for WithPosition<I>`

- <span id="withposition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithPosition<I>`

- <span id="withposition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithPosition<I>`

- <span id="withposition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for WithPosition<I>`

- <span id="withposition-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for WithPosition<I>`

- <span id="withposition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for WithPosition<I>`

- <span id="withposition-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I> ExactSizeIterator for WithPosition<I>`

##### `impl<T> From for WithPosition<I>`

- <span id="withposition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator> FusedIterator for WithPosition<I>`

##### `impl<U> Into for WithPosition<I>`

- <span id="withposition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WithPosition<I>`

##### `impl<I> IntoIterator for WithPosition<I>`

- <span id="withposition-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="withposition-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="withposition-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator> Iterator for WithPosition<I>`

- <span id="withposition-iterator-type-item"></span>`type Item = (Position, <I as Iterator>::Item)`

- <span id="withposition-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="withposition-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="withposition-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for WithPosition<I>`

##### `impl<FromA, FromB> MultiUnzip for WithPosition<I>`

- <span id="withposition-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB)`

##### `impl ToOwned for WithPosition<I>`

- <span id="withposition-toowned-type-owned"></span>`type Owned = T`

- <span id="withposition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="withposition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WithPosition<I>`

- <span id="withposition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withposition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithPosition<I>`

- <span id="withposition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withposition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Position`

```rust
enum Position {
    First,
    Middle,
    Last,
    Only,
}
```

*Defined in [`itertools-0.14.0/src/with_position.rs:50-59`](../../../.source_1765894658/itertools-0.14.0/src/with_position.rs#L50-L59)*

The first component of the value yielded by `WithPosition`.
Indicates the position of this element in the iterator results.

See [`.with_position()`](crate::Itertools::with_position) for more information.

#### Variants

- **`First`**

  This is the first element.

- **`Middle`**

  This is neither the first nor the last element.

- **`Last`**

  This is the last element.

- **`Only`**

  This is the only element.

#### Trait Implementations

##### `impl Any for Position`

- <span id="position-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Position`

- <span id="position-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Position`

- <span id="position-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Position`

- <span id="position-clone"></span>`fn clone(&self) -> Position` — [`Position`](#position)

##### `impl CloneToUninit for Position`

- <span id="position-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Position`

##### `impl Debug for Position`

- <span id="position-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Position`

##### `impl<T> From for Position`

- <span id="position-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Position`

- <span id="position-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Position`

##### `impl PartialEq for Position`

- <span id="position-partialeq-eq"></span>`fn eq(&self, other: &Position) -> bool` — [`Position`](#position)

##### `impl StructuralPartialEq for Position`

##### `impl ToOwned for Position`

- <span id="position-toowned-type-owned"></span>`type Owned = T`

- <span id="position-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="position-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Position`

- <span id="position-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="position-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Position`

- <span id="position-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="position-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `with_position`

```rust
fn with_position<I>(iter: I) -> WithPosition<I>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/with_position.rs:35-43`](../../../.source_1765894658/itertools-0.14.0/src/with_position.rs#L35-L43)*

Create a new `WithPosition` iterator.

