*[itertools](../index.md) / [peek_nth](index.md)*

---

# Module `peek_nth`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PeekNth`](#peeknth) | struct | See [`peek_nth()`] for more information. |
| [`peek_nth`](#peek-nth) | fn | A drop-in replacement for [`std::iter::Peekable`] which adds a `peek_nth` method allowing the user to `peek` at a value several iterations forward without advancing the base iterator. |

## Structs

### `PeekNth<I>`

```rust
struct PeekNth<I>
where
    I: Iterator {
    iter: std::iter::Fuse<I>,
    buf: alloc::collections::VecDeque<<I as >::Item>,
}
```

*Defined in [`itertools-0.14.0/src/peek_nth.rs:9-15`](../../../.source_1765894658/itertools-0.14.0/src/peek_nth.rs#L9-L15)*

See [`peek_nth()`](#peek-nth) for more information.

#### Implementations

- <span id="peeknth-peek"></span>`fn peek(&mut self) -> Option<&<I as >::Item>`

  Works exactly like the `peek` method in `std::iter::Peekable`.

- <span id="peeknth-peek-mut"></span>`fn peek_mut(&mut self) -> Option<&mut <I as >::Item>`

  Works exactly like the `peek_mut` method in `std::iter::Peekable`.

- <span id="peeknth-peek-nth"></span>`fn peek_nth(&mut self, n: usize) -> Option<&<I as >::Item>`

  Returns a reference to the `nth` value without advancing the iterator.
  
  # Examples
  
  Basic usage:
  
  ```rust
  use itertools::peek_nth;
  
  let xs = vec![1, 2, 3];
  let mut iter = peek_nth(xs.into_iter());
  
  assert_eq!(iter.peek_nth(0), Some(&1));
  assert_eq!(iter.next(), Some(1));
  
  // The iterator does not advance even if we call `peek_nth` multiple times
  assert_eq!(iter.peek_nth(0), Some(&2));
  assert_eq!(iter.peek_nth(1), Some(&3));
  assert_eq!(iter.next(), Some(2));
  
  // Calling `peek_nth` past the end of the iterator will return `None`
  assert_eq!(iter.peek_nth(1), None);
  ```

- <span id="peeknth-peek-nth-mut"></span>`fn peek_nth_mut(&mut self, n: usize) -> Option<&mut <I as >::Item>`

  Returns a mutable reference to the `nth` value without advancing the iterator.
  
  # Examples
  
  Basic usage:
  
  ```rust
  use itertools::peek_nth;
  
  let xs = vec![1, 2, 3, 4, 5];
  let mut iter = peek_nth(xs.into_iter());
  
  assert_eq!(iter.peek_nth_mut(0), Some(&mut 1));
  assert_eq!(iter.next(), Some(1));
  
  // The iterator does not advance even if we call `peek_nth_mut` multiple times
  assert_eq!(iter.peek_nth_mut(0), Some(&mut 2));
  assert_eq!(iter.peek_nth_mut(1), Some(&mut 3));
  assert_eq!(iter.next(), Some(2));
  
  // Peek into the iterator and set the value behind the mutable reference.
  if let Some(p) = iter.peek_nth_mut(1) {
      assert_eq!(*p, 4);
      *p = 9;
  }
  
  // The value we put in reappears as the iterator continues.
  assert_eq!(iter.next(), Some(3));
  assert_eq!(iter.next(), Some(9));
  
  // Calling `peek_nth_mut` past the end of the iterator will return `None`
  assert_eq!(iter.peek_nth_mut(1), None);
  ```

- <span id="peeknth-next-if"></span>`fn next_if(&mut self, func: impl FnOnce(&<I as >::Item) -> bool) -> Option<<I as >::Item>`

  Works exactly like the `next_if` method in `std::iter::Peekable`.

- <span id="peeknth-next-if-eq"></span>`fn next_if_eq<T>(&mut self, expected: &T) -> Option<<I as >::Item>`

  Works exactly like the `next_if_eq` method in `std::iter::Peekable`.

#### Trait Implementations

##### `impl Any for PeekNth<I>`

- <span id="peeknth-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeekNth<I>`

- <span id="peeknth-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeekNth<I>`

- <span id="peeknth-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for PeekNth<I>`

- <span id="peeknth-clone"></span>`fn clone(&self) -> PeekNth<I>` — [`PeekNth`](#peeknth)

##### `impl CloneToUninit for PeekNth<I>`

- <span id="peeknth-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for PeekNth<I>`

- <span id="peeknth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> ExactSizeIterator for PeekNth<I>`

##### `impl<T> From for PeekNth<I>`

- <span id="peeknth-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeekNth<I>`

- <span id="peeknth-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PeekNth<I>`

##### `impl<I> IntoIterator for PeekNth<I>`

- <span id="peeknth-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peeknth-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="peeknth-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for PeekNth<I>`

- <span id="peeknth-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peeknth-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="peeknth-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="peeknth-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for PeekNth<I>`

##### `impl MultiUnzip for PeekNth<I>`

- <span id="peeknth-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I> PeekingNext for PeekNth<I>`

- <span id="peeknth-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for PeekNth<I>`

- <span id="peeknth-toowned-type-owned"></span>`type Owned = T`

- <span id="peeknth-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="peeknth-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PeekNth<I>`

- <span id="peeknth-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="peeknth-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeekNth<I>`

- <span id="peeknth-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="peeknth-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `peek_nth`

```rust
fn peek_nth<I>(iterable: I) -> PeekNth<<I as >::IntoIter>
where
    I: IntoIterator
```

*Defined in [`itertools-0.14.0/src/peek_nth.rs:24-32`](../../../.source_1765894658/itertools-0.14.0/src/peek_nth.rs#L24-L32)*

A drop-in replacement for `std::iter::Peekable` which adds a `peek_nth`
method allowing the user to `peek` at a value several iterations forward
without advancing the base iterator.

This differs from `multipeek` in that subsequent calls to `peek` or
`peek_nth` will always return the same value until `next` is called
(making `reset_peek` unnecessary).

