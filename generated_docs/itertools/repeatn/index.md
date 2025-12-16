*[itertools](../index.md) / [repeatn](index.md)*

---

# Module `repeatn`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RepeatN`](#repeatn) | struct | An iterator that produces *n* repetitions of an element. |
| [`repeat_n`](#repeat-n) | fn | Create an iterator that produces `n` repetitions of `element`. |

## Structs

### `RepeatN<A>`

```rust
struct RepeatN<A> {
    elt: Option<A>,
    n: usize,
}
```

*Defined in [`itertools-0.14.0/src/repeatn.rs:8-11`](../../../.source_1765900590/itertools-0.14.0/src/repeatn.rs#L8-L11)*

An iterator that produces *n* repetitions of an element.

See [`repeat_n()`](crate::repeat_n) for more information.

#### Trait Implementations

##### `impl Any for RepeatN<A>`

- <span id="repeatn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepeatN<A>`

- <span id="repeatn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepeatN<A>`

- <span id="repeatn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone> Clone for RepeatN<A>`

- <span id="repeatn-clone"></span>`fn clone(&self) -> RepeatN<A>` — [`RepeatN`](#repeatn)

##### `impl CloneToUninit for RepeatN<A>`

- <span id="repeatn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug> Debug for RepeatN<A>`

- <span id="repeatn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> DoubleEndedIterator for RepeatN<A>`

- <span id="repeatn-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="repeatn-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<A> ExactSizeIterator for RepeatN<A>`

##### `impl<T> From for RepeatN<A>`

- <span id="repeatn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A> FusedIterator for RepeatN<A>`

##### `impl<U> Into for RepeatN<A>`

- <span id="repeatn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RepeatN<A>`

##### `impl IntoIterator for RepeatN<A>`

- <span id="repeatn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="repeatn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="repeatn-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for RepeatN<A>`

- <span id="repeatn-iterator-type-item"></span>`type Item = A`

- <span id="repeatn-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="repeatn-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="repeatn-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for RepeatN<A>`

##### `impl MultiUnzip for RepeatN<A>`

- <span id="repeatn-multiunzip"></span>`fn multiunzip(self)`

##### `impl<T: Clone> PeekingNext for crate::RepeatN<T>`

- <span id="craterepeatn-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for RepeatN<A>`

- <span id="repeatn-toowned-type-owned"></span>`type Owned = T`

- <span id="repeatn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repeatn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RepeatN<A>`

- <span id="repeatn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeatn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RepeatN<A>`

- <span id="repeatn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeatn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `repeat_n`

```rust
fn repeat_n<A>(element: A, n: usize) -> RepeatN<A>
where
    A: Clone
```

*Defined in [`itertools-0.14.0/src/repeatn.rs:14-26`](../../../.source_1765900590/itertools-0.14.0/src/repeatn.rs#L14-L26)*

Create an iterator that produces `n` repetitions of `element`.

