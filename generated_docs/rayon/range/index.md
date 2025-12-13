*[rayon](../index.md) / [range](index.md)*

---

# Module `range`

Parallel iterator types for [ranges],
the type for values created by `a..b` expressions

You will rarely need to interact with this module directly unless you have
need to name one of the iterator types.

```rust
use rayon::prelude::*;

let r = (0..100u64).into_par_iter()
                   .sum();

// compare result with sequential calculation
assert_eq!((0..100).sum::<u64>(), r);
```


## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Iter`](#iter)
  - [`IterProducer`](#iterproducer)
- [Traits](#traits)
  - [`UnindexedRangeLen`](#unindexedrangelen)
- [Macros](#macros)
  - [`indexed_range_impl!`](#indexed-range-impl)
  - [`unindexed_range_impl!`](#unindexed-range-impl)
  - [`convert_char!`](#convert-char)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | These traits help drive integer type inference. |
| [`Iter`](#iter) | struct | Parallel iterator over a range, implemented for all integer types and `char`. |
| [`IterProducer`](#iterproducer) | struct |  |
| [`UnindexedRangeLen`](#unindexedrangelen) | trait |  |
| [`indexed_range_impl!`](#indexed-range-impl) | macro |  |
| [`unindexed_range_impl!`](#unindexed-range-impl) | macro |  |
| [`convert_char!`](#convert-char) | macro |  |

## Modules

- [`private`](private/index.md) — These traits help drive integer type inference. Without them, an unknown `{integer}` type only

## Structs

### `Iter<T>`

```rust
struct Iter<T> {
    range: std::ops::Range<T>,
}
```

*Defined in [`rayon-1.11.0/src/range.rs:45-47`](../../../.source_1765633015/rayon-1.11.0/src/range.rs#L45-L47)*

Parallel iterator over a range, implemented for all integer types and `char`.

**Note:** The `zip` operation requires `IndexedParallelIterator`
which is not implemented for `u64`, `i64`, `u128`, or `i128`.

```rust
use rayon::prelude::*;

let p = (0..25usize).into_par_iter()
                  .zip(0..25usize)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum::<usize>();

let s = (0..25usize).zip(0..25)
                  .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)
                  .map(|(x, y)| x * y)
                  .sum();

assert_eq!(p, s);
```

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

### `IterProducer<T>`

```rust
struct IterProducer<T> {
    range: std::ops::Range<T>,
}
```

*Defined in [`rayon-1.11.0/src/range.rs:62-64`](../../../.source_1765633015/rayon-1.11.0/src/range.rs#L62-L64)*

#### Trait Implementations

##### `impl<T> Any for IterProducer<T>`

- <span id="iterproducer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IterProducer<T>`

- <span id="iterproducer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IterProducer<T>`

- <span id="iterproducer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for IterProducer<T>`

- <span id="iterproducer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for IterProducer<T>`

- <span id="iterproducer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for IterProducer<T>`

##### `impl<T> IntoIterator for IterProducer<T>`

- <span id="iterproducer-intoiterator-type-item"></span>`type Item = <Range<T> as Iterator>::Item`

- <span id="iterproducer-intoiterator-type-intoiter"></span>`type IntoIter = Range<T>`

- <span id="iterproducer-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T> Pointable for IterProducer<T>`

- <span id="iterproducer-pointable-const-align"></span>`const ALIGN: usize`

- <span id="iterproducer-pointable-type-init"></span>`type Init = T`

- <span id="iterproducer-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="iterproducer-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="iterproducer-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="iterproducer-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl Producer for IterProducer<u8>`

- <span id="iterproducer-producer-type-item"></span>`type Item = <Range<u8> as Iterator>::Item`

- <span id="iterproducer-producer-type-intoiter"></span>`type IntoIter = Range<u8>`

- <span id="iterproducer-producer-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter` — [`Producer`](../iter/plumbing/index.md#producer)

- <span id="iterproducer-producer-split-at"></span>`fn split_at(self, index: usize) -> (Self, Self)`

##### `impl<T, U> TryFrom for IterProducer<T>`

- <span id="iterproducer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterproducer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for IterProducer<T>`

- <span id="iterproducer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterproducer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UnindexedProducer for IterProducer<u64>`

- <span id="iterproducer-unindexedproducer-type-item"></span>`type Item = u64`

- <span id="iterproducer-unindexedproducer-split"></span>`fn split(self) -> (Self, Option<Self>)`

- <span id="iterproducer-unindexedproducer-fold-with"></span>`fn fold_with<F>(self, folder: F) -> F`

## Traits

### `UnindexedRangeLen<L>`

```rust
trait UnindexedRangeLen<L> { ... }
```

*Defined in [`rayon-1.11.0/src/range.rs:212-214`](../../../.source_1765633015/rayon-1.11.0/src/range.rs#L212-L214)*

#### Required Methods

- `fn unindexed_len(&self) -> L`

#### Implementors

- `Range<i128>`
- `Range<i64>`
- `Range<u128>`
- `Range<u64>`

## Macros

### `indexed_range_impl!`

*Defined in [`rayon-1.11.0/src/range.rs:153-210`](../../../.source_1765633015/rayon-1.11.0/src/range.rs#L153-L210)*

### `unindexed_range_impl!`

*Defined in [`rayon-1.11.0/src/range.rs:216-280`](../../../.source_1765633015/rayon-1.11.0/src/range.rs#L216-L280)*

### `convert_char!`

*Defined in [`rayon-1.11.0/src/range.rs:299-318`](../../../.source_1765633015/rayon-1.11.0/src/range.rs#L299-L318)*

