*[itertools](../index.md) / [take_while_inclusive](index.md)*

---

# Module `take_while_inclusive`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TakeWhileInclusive`](#takewhileinclusive) | struct | An iterator adaptor that consumes elements while the given predicate is `true`, including the element for which the predicate first returned `false`. |

## Structs

### `TakeWhileInclusive<I, F>`

```rust
struct TakeWhileInclusive<I, F> {
    iter: I,
    predicate: F,
    done: bool,
}
```

*Defined in [`itertools-0.14.0/src/take_while_inclusive.rs:12-16`](../../../.source_1765894658/itertools-0.14.0/src/take_while_inclusive.rs#L12-L16)*

An iterator adaptor that consumes elements while the given predicate is
`true`, including the element for which the predicate first returned
`false`.

See [`.take_while_inclusive()`](crate::Itertools::take_while_inclusive)
for more information.

#### Implementations

- <span id="takewhileinclusive-new"></span>`fn new(iter: I, predicate: F) -> Self`

  Create a new [`TakeWhileInclusive`](#takewhileinclusive) from an iterator and a predicate.

#### Trait Implementations

##### `impl Any for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-clone"></span>`fn clone(&self) -> TakeWhileInclusive<I, F>` — [`TakeWhileInclusive`](#takewhileinclusive)

##### `impl CloneToUninit for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for TakeWhileInclusive<I, F>`

##### `impl<U> Into for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeWhileInclusive<I, F>`

##### `impl<I> IntoIterator for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="takewhileinclusive-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="takewhileinclusive-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="takewhileinclusive-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="takewhileinclusive-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="takewhileinclusive-iterator-fold"></span>`fn fold<B, Fold>(self, init: B, f: Fold) -> B`

##### `impl Itertools for TakeWhileInclusive<I, F>`

##### `impl MultiUnzip for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-toowned-type-owned"></span>`type Owned = T`

- <span id="takewhileinclusive-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="takewhileinclusive-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takewhileinclusive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takewhileinclusive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

