*[itertools](../index.md) / [multipeek_impl](index.md)*

---

# Module `multipeek_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiPeek`](#multipeek) | struct | See [`multipeek()`] for more information. |
| [`multipeek`](#multipeek) | fn | An iterator adaptor that allows the user to peek at multiple `.next()` values without advancing the base iterator. |

## Structs

### `MultiPeek<I>`

```rust
struct MultiPeek<I>
where
    I: Iterator {
    iter: std::iter::Fuse<I>,
    buf: alloc::collections::VecDeque<<I as >::Item>,
    index: usize,
}
```

*Defined in [`itertools-0.14.0/src/multipeek_impl.rs:11-18`](../../../.source_1765894658/itertools-0.14.0/src/multipeek_impl.rs#L11-L18)*

See [`multipeek()`](#multipeek) for more information.

#### Implementations

- <span id="multipeek-reset-peek"></span>`fn reset_peek(&mut self)`

  Reset the peeking “cursor”

#### Trait Implementations

##### `impl Any for MultiPeek<I>`

- <span id="multipeek-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiPeek<I>`

- <span id="multipeek-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiPeek<I>`

- <span id="multipeek-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiPeek<I>`

- <span id="multipeek-clone"></span>`fn clone(&self) -> MultiPeek<I>` — [`MultiPeek`](#multipeek)

##### `impl CloneToUninit for MultiPeek<I>`

- <span id="multipeek-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiPeek<I>`

- <span id="multipeek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> ExactSizeIterator for MultiPeek<I>`

##### `impl<T> From for MultiPeek<I>`

- <span id="multipeek-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiPeek<I>`

- <span id="multipeek-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiPeek<I>`

##### `impl<I> IntoIterator for MultiPeek<I>`

- <span id="multipeek-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="multipeek-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="multipeek-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for MultiPeek<I>`

- <span id="multipeek-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="multipeek-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="multipeek-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="multipeek-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for MultiPeek<I>`

##### `impl MultiUnzip for MultiPeek<I>`

- <span id="multipeek-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I> PeekingNext for MultiPeek<I>`

- <span id="multipeek-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for MultiPeek<I>`

- <span id="multipeek-toowned-type-owned"></span>`type Owned = T`

- <span id="multipeek-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multipeek-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiPeek<I>`

- <span id="multipeek-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multipeek-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiPeek<I>`

- <span id="multipeek-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multipeek-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `multipeek`

```rust
fn multipeek<I>(iterable: I) -> MultiPeek<<I as >::IntoIter>
where
    I: IntoIterator
```

*Defined in [`itertools-0.14.0/src/multipeek_impl.rs:24-33`](../../../.source_1765894658/itertools-0.14.0/src/multipeek_impl.rs#L24-L33)*

An iterator adaptor that allows the user to peek at multiple `.next()`
values without advancing the base iterator.

`IntoIterator` enabled version of `Itertools::multipeek`.

