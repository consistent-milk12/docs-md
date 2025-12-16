*[itertools](../index.md) / [combinations_with_replacement](index.md)*

---

# Module `combinations_with_replacement`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CombinationsWithReplacement`](#combinationswithreplacement) | struct | An iterator to iterate through all the `n`-length combinations in an iterator, with replacement. |
| [`combinations_with_replacement`](#combinations-with-replacement) | fn | Create a new `CombinationsWithReplacement` from a clonable iterator. |
| [`remaining_for`](#remaining-for) | fn | For a given size `n`, return the count of remaining combinations with replacement or None if it would overflow. |

## Structs

### `CombinationsWithReplacement<I>`

```rust
struct CombinationsWithReplacement<I>
where
    I: Iterator,
    <I as >::Item: Clone {
    indices: alloc::boxed::Box<[usize]>,
    pool: super::lazy_buffer::LazyBuffer<I>,
    first: bool,
}
```

*Defined in [`itertools-0.14.0/src/combinations_with_replacement.rs:15-23`](../../../.source_1765894658/itertools-0.14.0/src/combinations_with_replacement.rs#L15-L23)*

An iterator to iterate through all the `n`-length combinations in an iterator, with replacement.

See [`.combinations_with_replacement()`](crate::Itertools::combinations_with_replacement)
for more information.

#### Implementations

- <span id="combinationswithreplacement-increment-indices"></span>`fn increment_indices(&mut self) -> bool`

  Increments indices representing the combination to advance to the next
  (in lexicographic order by increasing sequence) combination.
  
  Returns true if we've run out of combinations, false otherwise.

#### Trait Implementations

##### `impl Any for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-clone"></span>`fn clone(&self) -> CombinationsWithReplacement<I>` — [`CombinationsWithReplacement`](#combinationswithreplacement)

##### `impl CloneToUninit for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for CombinationsWithReplacement<I>`

##### `impl<U> Into for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CombinationsWithReplacement<I>`

##### `impl<I> IntoIterator for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="combinationswithreplacement-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="combinationswithreplacement-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="combinationswithreplacement-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="combinationswithreplacement-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="combinationswithreplacement-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="combinationswithreplacement-iterator-count"></span>`fn count(self) -> usize`

##### `impl Itertools for CombinationsWithReplacement<I>`

##### `impl ToOwned for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-toowned-type-owned"></span>`type Owned = T`

- <span id="combinationswithreplacement-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="combinationswithreplacement-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="combinationswithreplacement-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="combinationswithreplacement-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `combinations_with_replacement`

```rust
fn combinations_with_replacement<I>(iter: I, k: usize) -> CombinationsWithReplacement<I>
where
    I: Iterator,
    <I as >::Item: Clone
```

*Defined in [`itertools-0.14.0/src/combinations_with_replacement.rs:34-47`](../../../.source_1765894658/itertools-0.14.0/src/combinations_with_replacement.rs#L34-L47)*

Create a new `CombinationsWithReplacement` from a clonable iterator.

### `remaining_for`

```rust
fn remaining_for(n: usize, first: bool, indices: &[usize]) -> Option<usize>
```

*Defined in [`itertools-0.14.0/src/combinations_with_replacement.rs:149-188`](../../../.source_1765894658/itertools-0.14.0/src/combinations_with_replacement.rs#L149-L188)*

For a given size `n`, return the count of remaining combinations with replacement or None if it would overflow.

