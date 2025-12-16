*[itertools](../index.md) / [powerset](index.md)*

---

# Module `powerset`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Powerset`](#powerset) | struct | An iterator to iterate through the powerset of the elements from an iterator. |
| [`powerset`](#powerset) | fn | Create a new `Powerset` from a clonable iterator. |
| [`remaining_for`](#remaining-for) | fn |  |

## Structs

### `Powerset<I: Iterator>`

```rust
struct Powerset<I: Iterator> {
    combs: super::combinations::Combinations<I>,
}
```

*Defined in [`itertools-0.14.0/src/powerset.rs:14-16`](../../../.source_1765894658/itertools-0.14.0/src/powerset.rs#L14-L16)*

An iterator to iterate through the powerset of the elements from an iterator.

See [`.powerset()`](crate::Itertools::powerset) for more
information.

#### Implementations

- <span id="powerset-increment-k"></span>`fn increment_k(&mut self) -> bool`

  Returns true if `k` has been incremented, false otherwise.

#### Trait Implementations

##### `impl Any for Powerset<I>`

- <span id="powerset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Powerset<I>`

- <span id="powerset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Powerset<I>`

- <span id="powerset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Powerset<I>`

- <span id="powerset-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Powerset<I>`

- <span id="powerset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Powerset<I>`

- <span id="powerset-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Powerset<I>`

- <span id="powerset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for Powerset<I>`

##### `impl<U> Into for Powerset<I>`

- <span id="powerset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Powerset<I>`

##### `impl<I> IntoIterator for Powerset<I>`

- <span id="powerset-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="powerset-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="powerset-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Powerset<I>`

- <span id="powerset-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="powerset-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="powerset-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="powerset-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="powerset-iterator-count"></span>`fn count(self) -> usize`

- <span id="powerset-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Powerset<I>`

##### `impl ToOwned for Powerset<I>`

- <span id="powerset-toowned-type-owned"></span>`type Owned = T`

- <span id="powerset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="powerset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Powerset<I>`

- <span id="powerset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="powerset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Powerset<I>`

- <span id="powerset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="powerset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `powerset`

```rust
fn powerset<I>(src: I) -> Powerset<I>
where
    I: Iterator,
    <I as >::Item: Clone
```

*Defined in [`itertools-0.14.0/src/powerset.rs:35-43`](../../../.source_1765894658/itertools-0.14.0/src/powerset.rs#L35-L43)*

Create a new `Powerset` from a clonable iterator.

### `remaining_for`

```rust
fn remaining_for(n: usize, k: usize) -> Option<usize>
```

*Defined in [`itertools-0.14.0/src/powerset.rs:129-131`](../../../.source_1765894658/itertools-0.14.0/src/powerset.rs#L129-L131)*

