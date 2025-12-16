*[itertools](../index.md) / [pad_tail](index.md)*

---

# Module `pad_tail`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PadUsing`](#padusing) | struct | An iterator adaptor that pads a sequence to a minimum length by filling missing elements using a closure. |
| [`pad_using`](#pad-using) | fn | Create a new `PadUsing` iterator. |

## Structs

### `PadUsing<I, F>`

```rust
struct PadUsing<I, F> {
    iter: std::iter::Fuse<I>,
    min: usize,
    pos: usize,
    filler: F,
}
```

*Defined in [`itertools-0.14.0/src/pad_tail.rs:12-17`](../../../.source_1765894658/itertools-0.14.0/src/pad_tail.rs#L12-L17)*

An iterator adaptor that pads a sequence to a minimum length by filling
missing elements using a closure.

Iterator element type is `I::Item`.

See [`.pad_using()`](crate::Itertools::pad_using) for more information.

#### Trait Implementations

##### `impl Any for PadUsing<I, F>`

- <span id="padusing-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PadUsing<I, F>`

- <span id="padusing-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PadUsing<I, F>`

- <span id="padusing-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for PadUsing<I, F>`

- <span id="padusing-clone"></span>`fn clone(&self) -> PadUsing<I, F>` — [`PadUsing`](#padusing)

##### `impl CloneToUninit for PadUsing<I, F>`

- <span id="padusing-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for PadUsing<I, F>`

- <span id="padusing-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, F> DoubleEndedIterator for PadUsing<I, F>`

- <span id="padusing-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="padusing-doubleendediterator-rfold"></span>`fn rfold<B, G>(self, init: B, f: G) -> B`

##### `impl<I, F> ExactSizeIterator for PadUsing<I, F>`

##### `impl<T> From for PadUsing<I, F>`

- <span id="padusing-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for PadUsing<I, F>`

##### `impl<U> Into for PadUsing<I, F>`

- <span id="padusing-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PadUsing<I, F>`

##### `impl<I> IntoIterator for PadUsing<I, F>`

- <span id="padusing-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="padusing-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="padusing-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for PadUsing<I, F>`

- <span id="padusing-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="padusing-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="padusing-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="padusing-iterator-fold"></span>`fn fold<B, G>(self, init: B, f: G) -> B`

##### `impl Itertools for PadUsing<I, F>`

##### `impl MultiUnzip for PadUsing<I, F>`

- <span id="padusing-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for PadUsing<I, F>`

- <span id="padusing-toowned-type-owned"></span>`type Owned = T`

- <span id="padusing-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="padusing-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PadUsing<I, F>`

- <span id="padusing-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="padusing-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PadUsing<I, F>`

- <span id="padusing-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="padusing-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `pad_using`

```rust
fn pad_using<I, F>(iter: I, min: usize, filler: F) -> PadUsing<I, F>
where
    I: Iterator,
    F: FnMut(usize) -> <I as >::Item
```

*Defined in [`itertools-0.14.0/src/pad_tail.rs:27-38`](../../../.source_1765894658/itertools-0.14.0/src/pad_tail.rs#L27-L38)*

Create a new `PadUsing` iterator.

