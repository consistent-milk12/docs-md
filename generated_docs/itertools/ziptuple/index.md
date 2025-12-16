*[itertools](../index.md) / [ziptuple](index.md)*

---

# Module `ziptuple`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Zip`](#zip) | struct | See [`multizip`] for more information. |
| [`multizip`](#multizip) | fn | An iterator that generalizes `.zip()` and allows running multiple iterators in lockstep. |
| [`impl_zip_iter!`](#impl-zip-iter) | macro |  |

## Structs

### `Zip<T>`

```rust
struct Zip<T> {
    t: T,
}
```

*Defined in [`itertools-0.14.0/src/ziptuple.rs:6-8`](../../../.source_1765894658/itertools-0.14.0/src/ziptuple.rs#L6-L8)*

See [`multizip`](#multizip) for more information.

#### Trait Implementations

##### `impl<T> Any for Zip<T>`

- <span id="zip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Zip<T>`

- <span id="zip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Zip<T>`

- <span id="zip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Zip<T>`

- <span id="zip-clone"></span>`fn clone(&self) -> Zip<T>` — [`Zip`](#zip)

##### `impl<T> CloneToUninit for Zip<T>`

- <span id="zip-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Zip<T>`

- <span id="zip-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> DoubleEndedIterator for Zip<(A)>`

- <span id="zip-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<A> ExactSizeIterator for Zip<(A)>`

##### `impl<T> From for Zip<T>`

- <span id="zip-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Zip<T>`

- <span id="zip-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Zip<T>`

##### `impl IntoIterator for Zip<T>`

- <span id="zip-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="zip-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="zip-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for Zip<(A)>`

- <span id="zip-iterator-type-item"></span>`type Item = (<A as Iterator>::Item)`

- <span id="zip-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="zip-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for Zip<T>`

##### `impl MultiUnzip for Zip<T>`

- <span id="zip-multiunzip"></span>`fn multiunzip(self)`

##### `impl<T> ToOwned for Zip<T>`

- <span id="zip-toowned-type-owned"></span>`type Owned = T`

- <span id="zip-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="zip-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Zip<T>`

- <span id="zip-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="zip-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Zip<T>`

- <span id="zip-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="zip-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `multizip`

```rust
fn multizip<T, U>(t: U) -> Zip<T>
where
    Zip<T>: From<U> + Iterator
```

*Defined in [`itertools-0.14.0/src/ziptuple.rs:39-44`](../../../.source_1765894658/itertools-0.14.0/src/ziptuple.rs#L39-L44)*

An iterator that generalizes `.zip()` and allows running multiple iterators in lockstep.

The iterator `Zip<(I, J, ..., M)>` is formed from a tuple of iterators (or values that
implement `IntoIterator`) and yields elements
until any of the subiterators yields `None`.

The iterator element type is a tuple like like `(A, B, ..., E)` where `A` to `E` are the
element types of the subiterator.

**Note:** The result of this function is a value of a named type (`Zip<(I, J,
..)>` of each component iterator `I, J, ...`) if each component iterator is
nameable.

Prefer [`izip!()`](crate::izip) over `multizip` for the performance benefits of using the
standard library `.zip()`. Prefer `multizip` if a nameable type is needed.

```rust
use itertools::multizip;

// iterate over three sequences side-by-side
let mut results = [0, 0, 0, 0];
let inputs = [3, 7, 9, 6];

for (r, index, input) in multizip((&mut results, 0..10, &inputs)) {
    *r = index * 10 + input;
}

assert_eq!(results, [0 + 3, 10 + 7, 29, 36]);
```

## Macros

### `impl_zip_iter!`

*Defined in [`itertools-0.14.0/src/ziptuple.rs:46-124`](../../../.source_1765894658/itertools-0.14.0/src/ziptuple.rs#L46-L124)*

