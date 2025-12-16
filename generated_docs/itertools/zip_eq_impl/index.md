*[itertools](../index.md) / [zip_eq_impl](index.md)*

---

# Module `zip_eq_impl`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ZipEq`](#zipeq) | struct | An iterator which iterates two other iterators simultaneously and panic if they have different lengths. |
| [`zip_eq`](#zip-eq) | fn | Zips two iterators but **panics** if they are not of the same length. |

## Structs

### `ZipEq<I, J>`

```rust
struct ZipEq<I, J> {
    a: I,
    b: J,
}
```

*Defined in [`itertools-0.14.0/src/zip_eq_impl.rs:9-12`](../../../.source_1765894658/itertools-0.14.0/src/zip_eq_impl.rs#L9-L12)*

An iterator which iterates two other iterators simultaneously
and panic if they have different lengths.

See [`.zip_eq()`](crate::Itertools::zip_eq) for more information.

#### Trait Implementations

##### `impl Any for ZipEq<I, J>`

- <span id="zipeq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ZipEq<I, J>`

- <span id="zipeq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ZipEq<I, J>`

- <span id="zipeq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, J: clone::Clone> Clone for ZipEq<I, J>`

- <span id="zipeq-clone"></span>`fn clone(&self) -> ZipEq<I, J>` — [`ZipEq`](#zipeq)

##### `impl CloneToUninit for ZipEq<I, J>`

- <span id="zipeq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for ZipEq<I, J>`

- <span id="zipeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, J> ExactSizeIterator for ZipEq<I, J>`

##### `impl<T> From for ZipEq<I, J>`

- <span id="zipeq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ZipEq<I, J>`

- <span id="zipeq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ZipEq<I, J>`

##### `impl<I> IntoIterator for ZipEq<I, J>`

- <span id="zipeq-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="zipeq-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="zipeq-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, J> Iterator for ZipEq<I, J>`

- <span id="zipeq-iterator-type-item"></span>`type Item = (<I as Iterator>::Item, <J as Iterator>::Item)`

- <span id="zipeq-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="zipeq-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for ZipEq<I, J>`

##### `impl<FromA, FromB> MultiUnzip for ZipEq<I, J>`

- <span id="zipeq-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB)`

##### `impl ToOwned for ZipEq<I, J>`

- <span id="zipeq-toowned-type-owned"></span>`type Owned = T`

- <span id="zipeq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="zipeq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ZipEq<I, J>`

- <span id="zipeq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zipeq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ZipEq<I, J>`

- <span id="zipeq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zipeq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `zip_eq`

```rust
fn zip_eq<I, J>(i: I, j: J) -> ZipEq<<I as >::IntoIter, <J as >::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator
```

*Defined in [`itertools-0.14.0/src/zip_eq_impl.rs:27-36`](../../../.source_1765894658/itertools-0.14.0/src/zip_eq_impl.rs#L27-L36)*

Zips two iterators but **panics** if they are not of the same length.

`IntoIterator` enabled version of [`Itertools::zip_eq`](crate::Itertools::zip_eq).

```rust
use itertools::zip_eq;

let data = [1, 2, 3, 4, 5];
for (a, b) in zip_eq(&data[..data.len() - 1], &data[1..]) {
    /* loop body */
    let _ = (a, b);
}
```

