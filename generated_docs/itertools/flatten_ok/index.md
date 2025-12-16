*[itertools](../index.md) / [flatten_ok](index.md)*

---

# Module `flatten_ok`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlattenOk`](#flattenok) | struct | An iterator adaptor that flattens `Result::Ok` values and allows `Result::Err` values through unchanged. |
| [`flatten_ok`](#flatten-ok) | fn |  |

## Structs

### `FlattenOk<I, T, E>`

```rust
struct FlattenOk<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
    T: IntoIterator {
    iter: I,
    inner_front: Option<<T as >::IntoIter>,
    inner_back: Option<<T as >::IntoIter>,
}
```

*Defined in [`itertools-0.14.0/src/flatten_ok.rs:24-32`](../../../.source_1765900590/itertools-0.14.0/src/flatten_ok.rs#L24-L32)*

An iterator adaptor that flattens `Result::Ok` values and
allows `Result::Err` values through unchanged.

See [`.flatten_ok()`](crate::Itertools::flatten_ok) for more information.

#### Trait Implementations

##### `impl<T> Any for FlattenOk<I, T, E>`

- <span id="flattenok-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlattenOk<I, T, E>`

- <span id="flattenok-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlattenOk<I, T, E>`

- <span id="flattenok-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T, E> Clone for FlattenOk<I, T, E>`

- <span id="flattenok-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> CloneToUninit for FlattenOk<I, T, E>`

- <span id="flattenok-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T, E> Debug for FlattenOk<I, T, E>`

- <span id="flattenok-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, T, E> DoubleEndedIterator for FlattenOk<I, T, E>`

- <span id="flattenok-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="flattenok-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> From for FlattenOk<I, T, E>`

- <span id="flattenok-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T, E> FusedIterator for FlattenOk<I, T, E>`

##### `impl<T, U> Into for FlattenOk<I, T, E>`

- <span id="flattenok-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for FlattenOk<I, T, E>`

##### `impl<I> IntoIterator for FlattenOk<I, T, E>`

- <span id="flattenok-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="flattenok-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="flattenok-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T, E> Iterator for FlattenOk<I, T, E>`

- <span id="flattenok-iterator-type-item"></span>`type Item = Result<<T as IntoIterator>::Item, E>`

- <span id="flattenok-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="flattenok-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

- <span id="flattenok-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for FlattenOk<I, T, E>`

##### `impl<T> ToOwned for FlattenOk<I, T, E>`

- <span id="flattenok-toowned-type-owned"></span>`type Owned = T`

- <span id="flattenok-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="flattenok-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for FlattenOk<I, T, E>`

- <span id="flattenok-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flattenok-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FlattenOk<I, T, E>`

- <span id="flattenok-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flattenok-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `flatten_ok`

```rust
fn flatten_ok<I, T, E>(iter: I) -> FlattenOk<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
    T: IntoIterator
```

*Defined in [`itertools-0.14.0/src/flatten_ok.rs:7-17`](../../../.source_1765900590/itertools-0.14.0/src/flatten_ok.rs#L7-L17)*

