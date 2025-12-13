*[rayon](../index.md) / [range_inclusive](index.md)*

---

# Module `range_inclusive`

Parallel iterator types for [inclusive ranges],
the type for values created by `a..=b` expressions

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

```rust
use rayon::prelude::*;

let r = (0..=100u64).into_par_iter()
                    .sum();

// compare result with sequential calculation
assert_eq!((0..=100).sum::<u64>(), r);
```


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | These traits help drive integer type inference. |
| [`Iter`](#iter) | struct | Parallel iterator over an inclusive range, implemented for all integer types and `char`. |
| [`convert!`](#convert) | macro |  |
| [`parallel_range_impl!`](#parallel-range-impl) | macro |  |
| [`indexed_range_impl!`](#indexed-range-impl) | macro |  |
| [`convert_char!`](#convert-char) | macro |  |

## Modules

- [`private`](private/index.md) — These traits help drive integer type inference. Without them, an unknown `{integer}` type only

## Structs

### `Iter<T>`

```rust
struct Iter<T> {
    range: std::ops::RangeInclusive<T>,
}
```

*Defined in [`rayon-1.11.0/src/range_inclusive.rs:45-47`](../../../.source_1765521767/rayon-1.11.0/src/range_inclusive.rs#L45-L47)*

Parallel iterator over an inclusive range, implemented for all integer types and `char`.

**Note:** The `zip` operation requires `IndexedParallelIterator`
which is only implemented for `u8`, `i8`, `u16`, `i16`, and `char`.

```rust
use rayon::prelude::*;

let p = (0..=25u16).into_par_iter()
                  .zip(0..=25u16)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum::<u16>();

let s = (0..=25u16).zip(0..=25u16)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum();

assert_eq!(p, s);
```

#### Implementations

- <span id="iter-bounds"></span>`fn bounds(&self) -> Option<(T, T)>`

  Returns `Some((start, end))` for `start..=end`, or `None` if it is exhausted.

  

  Note that `RangeInclusive` does not specify the bounds of an exhausted iterator,

  so this is a way for us to figure out what we've got.  Thankfully, all of the

  integer types we care about can be trivially cloned.

#### Trait Implementations

##### `impl<T> Any for Iter<T>`

- <span id="iter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iter<T>`

- <span id="iter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iter<T>`

- <span id="iter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Iter<T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<T>` — [`Iter`](#iter)

##### `impl<T> CloneToUninit for Iter<T>`

- <span id="iter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug> Debug for Iter<T>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Iter<T>`

- <span id="iter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: IndexedRangeInteger> IndexedParallelIterator for Iter<T>`

- <span id="iter-indexedparalleliterator-drive"></span>`fn drive<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-indexedparalleliterator-len"></span>`fn len(&self) -> usize`

- <span id="iter-indexedparalleliterator-with-producer"></span>`fn with_producer<CB>(self, callback: CB) -> <CB as >::Output` — [`ProducerCallback`](../iter/plumbing/index.md#producercallback)

##### `impl<T, U> Into for Iter<T>`

- <span id="iter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Iter<T>`

##### `impl<T> IntoParallelIterator for Iter<T>`

- <span id="iter-intoparalleliterator-type-iter"></span>`type Iter = T`

- <span id="iter-intoparalleliterator-type-item"></span>`type Item = <T as ParallelIterator>::Item`

- <span id="iter-intoparalleliterator-into-par-iter"></span>`fn into_par_iter(self) -> T`

##### `impl<T: RangeInteger> ParallelIterator for Iter<T>`

- <span id="iter-paralleliterator-type-item"></span>`type Item = T`

- <span id="iter-paralleliterator-drive-unindexed"></span>`fn drive_unindexed<C>(self, consumer: C) -> <C as >::Result` — [`Consumer`](../iter/plumbing/index.md#consumer)

- <span id="iter-paralleliterator-opt-len"></span>`fn opt_len(&self) -> Option<usize>`

##### `impl<T> Pointable for Iter<T>`

- <span id="iter-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iter-pointable-type-init"></span>`type Init = T`

- <span id="iter-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iter-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iter-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iter-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> ToOwned for Iter<T>`

- <span id="iter-toowned-type-owned"></span>`type Owned = T`

- <span id="iter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Iter<T>`

- <span id="iter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Iter<T>`

- <span id="iter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `convert!`

*Defined in [`rayon-1.11.0/src/range_inclusive.rs:161-173`](../../../.source_1765521767/rayon-1.11.0/src/range_inclusive.rs#L161-L173)*

### `parallel_range_impl!`

*Defined in [`rayon-1.11.0/src/range_inclusive.rs:175-192`](../../../.source_1765521767/rayon-1.11.0/src/range_inclusive.rs#L175-L192)*

### `indexed_range_impl!`

*Defined in [`rayon-1.11.0/src/range_inclusive.rs:194-220`](../../../.source_1765521767/rayon-1.11.0/src/range_inclusive.rs#L194-L220)*

### `convert_char!`

*Defined in [`rayon-1.11.0/src/range_inclusive.rs:239-262`](../../../.source_1765521767/rayon-1.11.0/src/range_inclusive.rs#L239-L262)*

