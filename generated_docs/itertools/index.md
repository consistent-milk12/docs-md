# Crate `itertools`

Extra iterator adaptors, functions and macros.

To extend `Iterator` with methods in this crate, import
the [`Itertools`](#itertools) trait:

```rust
#[allow(unused_imports)]
use itertools::Itertools;
```

Now, new methods like [`interleave`](Itertools::interleave)
are available on all iterators:

```rust
use itertools::Itertools;

let it = (1..3).interleave(vec![-1, -2]);
itertools::assert_equal(it, vec![1, -1, 2, -2]);
```

Most iterator methods are also provided as functions (with the benefit
that they convert parameters using `IntoIterator`):

```rust
use itertools::interleave;

for elt in interleave(&[1, 2, 3], &[2, 3, 4]) {
    /* loop body */
    let _ = elt;
}
```

## Crate Features

- `use_std`
  - Enabled by default.
  - Disable to compile itertools using `#![no_std]`. This disables
    any item that depend on allocations (see the `use_alloc` feature)
    and hash maps (like `unique`, `counts`, `into_grouping_map` and more).
- `use_alloc`
  - Enabled by default.
  - Enables any item that depend on allocations (like `chunk_by`,
    `kmerge`, `join` and many more).

## Rust Version

This version of itertools requires Rust 1.63.0 or later.

## Contents

- [Modules](#modules)
  - [`impl_macros`](#impl-macros)
  - [`structs`](#structs)
  - [`traits`](#traits)
  - [`adaptors`](#adaptors)
  - [`either_or_both`](#either-or-both)
  - [`combinations`](#combinations)
  - [`combinations_with_replacement`](#combinations-with-replacement)
  - [`concat_impl`](#concat-impl)
  - [`cons_tuples_impl`](#cons-tuples-impl)
  - [`diff`](#diff)
  - [`duplicates_impl`](#duplicates-impl)
  - [`exactly_one_err`](#exactly-one-err)
  - [`extrema_set`](#extrema-set)
  - [`flatten_ok`](#flatten-ok)
  - [`format`](#format)
  - [`group_map`](#group-map)
  - [`groupbylazy`](#groupbylazy)
  - [`grouping_map`](#grouping-map)
  - [`intersperse`](#intersperse)
  - [`iter_index`](#iter-index)
  - [`k_smallest`](#k-smallest)
  - [`kmerge_impl`](#kmerge-impl)
  - [`lazy_buffer`](#lazy-buffer)
  - [`merge_join`](#merge-join)
  - [`minmax`](#minmax)
  - [`multipeek_impl`](#multipeek-impl)
  - [`next_array`](#next-array)
  - [`pad_tail`](#pad-tail)
  - [`peek_nth`](#peek-nth)
  - [`peeking_take_while`](#peeking-take-while)
  - [`permutations`](#permutations)
  - [`powerset`](#powerset)
  - [`process_results_impl`](#process-results-impl)
  - [`put_back_n_impl`](#put-back-n-impl)
  - [`rciter_impl`](#rciter-impl)
  - [`repeatn`](#repeatn)
  - [`size_hint`](#size-hint)
  - [`sources`](#sources)
  - [`take_while_inclusive`](#take-while-inclusive)
  - [`tee`](#tee)
  - [`tuple_impl`](#tuple-impl)
  - [`unique_impl`](#unique-impl)
  - [`unziptuple`](#unziptuple)
  - [`with_position`](#with-position)
  - [`zip_eq_impl`](#zip-eq-impl)
  - [`zip_longest`](#zip-longest)
  - [`ziptuple`](#ziptuple)
- [Enums](#enums)
  - [`Diff`](#diff)
  - [`MinMaxResult`](#minmaxresult)
  - [`Position`](#position)
  - [`EitherOrBoth`](#eitherorboth)
  - [`FoldWhile`](#foldwhile)
- [Traits](#traits)
  - [`PeekingNext`](#peekingnext)
  - [`MultiUnzip`](#multiunzip)
  - [`Itertools`](#itertools)
- [Functions](#functions)
  - [`Either`](#either)
  - [`concat`](#concat)
  - [`cons_tuples`](#cons-tuples)
  - [`diff_with`](#diff-with)
  - [`kmerge_by`](#kmerge-by)
  - [`process_results`](#process-results)
  - [`repeat_n`](#repeat-n)
  - [`iterate`](#iterate)
  - [`unfold`](#unfold)
  - [`multiunzip`](#multiunzip)
  - [`multizip`](#multizip)
  - [`equal`](#equal)
  - [`assert_equal`](#assert-equal)
  - [`partition`](#partition)
- [Type Aliases](#type-aliases)
  - [`VecDequeIntoIter`](#vecdequeintoiter)
  - [`VecIntoIter`](#vecintoiter)
- [Macros](#macros)
  - [`iproduct!`](#iproduct)
  - [`izip!`](#izip)
  - [`chain!`](#chain)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impl_macros`](#impl-macros) | mod | Implementation's internal macros |
| [`structs`](#structs) | mod | The concrete iterator types. |
| [`traits`](#traits) | mod | Traits helpful for using certain `Itertools` methods in generic contexts. |
| [`adaptors`](#adaptors) | mod | Licensed under the Apache License, Version 2.0 <https://www.apache.org/licenses/LICENSE-2.0> or the MIT license <https://opensource.org/licenses/MIT>, at your option. |
| [`either_or_both`](#either-or-both) | mod |  |
| [`combinations`](#combinations) | mod |  |
| [`combinations_with_replacement`](#combinations-with-replacement) | mod |  |
| [`concat_impl`](#concat-impl) | mod |  |
| [`cons_tuples_impl`](#cons-tuples-impl) | mod |  |
| [`diff`](#diff) | mod | "Diff"ing iterators for caching elements to sequential collections without requiring the new elements' iterator to be `Clone`. |
| [`duplicates_impl`](#duplicates-impl) | mod |  |
| [`exactly_one_err`](#exactly-one-err) | mod |  |
| [`extrema_set`](#extrema-set) | mod |  |
| [`flatten_ok`](#flatten-ok) | mod |  |
| [`format`](#format) | mod |  |
| [`group_map`](#group-map) | mod |  |
| [`groupbylazy`](#groupbylazy) | mod |  |
| [`grouping_map`](#grouping-map) | mod |  |
| [`intersperse`](#intersperse) | mod |  |
| [`iter_index`](#iter-index) | mod |  |
| [`k_smallest`](#k-smallest) | mod |  |
| [`kmerge_impl`](#kmerge-impl) | mod |  |
| [`lazy_buffer`](#lazy-buffer) | mod |  |
| [`merge_join`](#merge-join) | mod |  |
| [`minmax`](#minmax) | mod |  |
| [`multipeek_impl`](#multipeek-impl) | mod |  |
| [`next_array`](#next-array) | mod |  |
| [`pad_tail`](#pad-tail) | mod |  |
| [`peek_nth`](#peek-nth) | mod |  |
| [`peeking_take_while`](#peeking-take-while) | mod |  |
| [`permutations`](#permutations) | mod |  |
| [`powerset`](#powerset) | mod |  |
| [`process_results_impl`](#process-results-impl) | mod |  |
| [`put_back_n_impl`](#put-back-n-impl) | mod |  |
| [`rciter_impl`](#rciter-impl) | mod |  |
| [`repeatn`](#repeatn) | mod |  |
| [`size_hint`](#size-hint) | mod | Arithmetic on `Iterator.size_hint()` values. |
| [`sources`](#sources) | mod | Iterators that are sources (produce elements from parameters, not from another iterator). |
| [`take_while_inclusive`](#take-while-inclusive) | mod |  |
| [`tee`](#tee) | mod |  |
| [`tuple_impl`](#tuple-impl) | mod | Some iterator that produces tuples |
| [`unique_impl`](#unique-impl) | mod |  |
| [`unziptuple`](#unziptuple) | mod |  |
| [`with_position`](#with-position) | mod |  |
| [`zip_eq_impl`](#zip-eq-impl) | mod |  |
| [`zip_longest`](#zip-longest) | mod |  |
| [`ziptuple`](#ziptuple) | mod |  |
| [`Diff`](#diff) | enum |  |
| [`MinMaxResult`](#minmaxresult) | enum |  |
| [`Position`](#position) | enum |  |
| [`EitherOrBoth`](#eitherorboth) | enum |  |
| [`FoldWhile`](#foldwhile) | enum | An enum used for controlling the execution of `fold_while`. |
| [`PeekingNext`](#peekingnext) | trait |  |
| [`MultiUnzip`](#multiunzip) | trait |  |
| [`Itertools`](#itertools) | trait | An [`Iterator`] blanket implementation that provides extra adaptors and methods. |
| [`Either`](#either) | fn |  |
| [`concat`](#concat) | fn |  |
| [`cons_tuples`](#cons-tuples) | fn |  |
| [`diff_with`](#diff-with) | fn |  |
| [`kmerge_by`](#kmerge-by) | fn |  |
| [`process_results`](#process-results) | fn |  |
| [`repeat_n`](#repeat-n) | fn |  |
| [`iterate`](#iterate) | fn |  |
| [`unfold`](#unfold) | fn |  |
| [`multiunzip`](#multiunzip) | fn |  |
| [`multizip`](#multizip) | fn |  |
| [`equal`](#equal) | fn | Return `true` if both iterables produce equal sequences (elements pairwise equal and sequences of the same length), `false` otherwise. |
| [`assert_equal`](#assert-equal) | fn | Assert that two iterables produce equal sequences, with the same semantics as [`equal(a, b)`](equal). |
| [`partition`](#partition) | fn | Partition a sequence using predicate `pred` so that elements that map to `true` are placed before elements which map to `false`. |
| [`VecDequeIntoIter`](#vecdequeintoiter) | type |  |
| [`VecIntoIter`](#vecintoiter) | type |  |
| [`iproduct!`](#iproduct) | macro | Create an iterator over the “cartesian product” of iterators. |
| [`izip!`](#izip) | macro | Create an iterator running multiple iterators in lockstep. |
| [`chain!`](#chain) | macro | [Chain][`chain`] zero or more iterators together into one sequence. |

## Modules

- [`impl_macros`](impl_macros/index.md)
- [`structs`](structs/index.md) — The concrete iterator types.
- [`traits`](traits/index.md) — Traits helpful for using certain `Itertools` methods in generic contexts.
- [`adaptors`](adaptors/index.md) — Licensed under the Apache License, Version 2.0
- [`either_or_both`](either_or_both/index.md)
- [`combinations`](combinations/index.md)
- [`combinations_with_replacement`](combinations_with_replacement/index.md)
- [`concat_impl`](concat_impl/index.md)
- [`cons_tuples_impl`](cons_tuples_impl/index.md)
- [`diff`](diff/index.md) — "Diff"ing iterators for caching elements to sequential collections without requiring the new
- [`duplicates_impl`](duplicates_impl/index.md)
- [`exactly_one_err`](exactly_one_err/index.md)
- [`extrema_set`](extrema_set/index.md)
- [`flatten_ok`](flatten_ok/index.md)
- [`format`](format/index.md)
- [`group_map`](group_map/index.md)
- [`groupbylazy`](groupbylazy/index.md)
- [`grouping_map`](grouping_map/index.md)
- [`intersperse`](intersperse/index.md)
- [`iter_index`](iter_index/index.md)
- [`k_smallest`](k_smallest/index.md)
- [`kmerge_impl`](kmerge_impl/index.md)
- [`lazy_buffer`](lazy_buffer/index.md)
- [`merge_join`](merge_join/index.md)
- [`minmax`](minmax/index.md)
- [`multipeek_impl`](multipeek_impl/index.md)
- [`next_array`](next_array/index.md)
- [`pad_tail`](pad_tail/index.md)
- [`peek_nth`](peek_nth/index.md)
- [`peeking_take_while`](peeking_take_while/index.md)
- [`permutations`](permutations/index.md)
- [`powerset`](powerset/index.md)
- [`process_results_impl`](process_results_impl/index.md)
- [`put_back_n_impl`](put_back_n_impl/index.md)
- [`rciter_impl`](rciter_impl/index.md)
- [`repeatn`](repeatn/index.md)
- [`size_hint`](size_hint/index.md) — Arithmetic on `Iterator.size_hint()` values.
- [`sources`](sources/index.md) — Iterators that are sources (produce elements from parameters,
- [`take_while_inclusive`](take_while_inclusive/index.md)
- [`tee`](tee/index.md)
- [`tuple_impl`](tuple_impl/index.md) — Some iterator that produces tuples
- [`unique_impl`](unique_impl/index.md)
- [`unziptuple`](unziptuple/index.md)
- [`with_position`](with_position/index.md)
- [`zip_eq_impl`](zip_eq_impl/index.md)
- [`zip_longest`](zip_longest/index.md)
- [`ziptuple`](ziptuple/index.md)

## Enums

### `Diff<I, J>`

```rust
enum Diff<I, J>
where
    I: Iterator,
    J: Iterator {
    FirstMismatch(usize, crate::structs::PutBack<I>, crate::structs::PutBack<J>),
    Shorter(usize, crate::structs::PutBack<I>),
    Longer(usize, crate::structs::PutBack<J>),
}
```

*Defined in [`itertools-0.14.0/src/diff.rs:17-29`](../../.source_1765894658/itertools-0.14.0/src/diff.rs#L17-L29)*

A type returned by the [`diff_with`](diff/index.md) function.

`Diff` represents the way in which the elements yielded by the iterator `I` differ to some
iterator `J`.

#### Variants

- **`FirstMismatch`**

  The index of the first non-matching element along with both iterator's remaining elements
  starting with the first mis-match.

- **`Shorter`**

  The total number of elements that were in `J` along with the remaining elements of `I`.

- **`Longer`**

  The total number of elements that were in `I` along with the remaining elements of `J`.

#### Trait Implementations

##### `impl Any for Diff<I, J>`

- <span id="diff-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Diff<I, J>`

- <span id="diff-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Diff<I, J>`

- <span id="diff-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, J> Clone for Diff<I, J>`

- <span id="diff-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Diff<I, J>`

- <span id="diff-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, J> Debug for Diff<I, J>`

- <span id="diff-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Diff<I, J>`

- <span id="diff-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Diff<I, J>`

- <span id="diff-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Diff<I, J>`

##### `impl ToOwned for Diff<I, J>`

- <span id="diff-toowned-type-owned"></span>`type Owned = T`

- <span id="diff-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="diff-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Diff<I, J>`

- <span id="diff-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="diff-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Diff<I, J>`

- <span id="diff-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="diff-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MinMaxResult<T>`

```rust
enum MinMaxResult<T> {
    NoElements,
    OneElement(T),
    MinMax(T, T),
}
```

*Defined in [`itertools-0.14.0/src/minmax.rs:5-15`](../../.source_1765894658/itertools-0.14.0/src/minmax.rs#L5-L15)*

`MinMaxResult` is an enum returned by `minmax`.

See [`.minmax()`](crate::Itertools::minmax) for more detail.

#### Variants

- **`NoElements`**

  Empty iterator

- **`OneElement`**

  Iterator with one element, so the minimum and maximum are the same

- **`MinMax`**

  More than one element in the iterator, the first element is not larger
  than the second

#### Implementations

- <span id="minmaxresult-into-option"></span>`fn into_option(self) -> Option<(T, T)>`

  `into_option` creates an `Option` of type `(T, T)`. The returned `Option`
  has variant `None` if and only if the `MinMaxResult` has variant
  `NoElements`. Otherwise `Some((x, y))` is returned where `x <= y`.
  If the `MinMaxResult` has variant `OneElement(x)`, performing this
  operation will make one clone of `x`.
  
  # Examples
  
  ```rust
  use itertools::MinMaxResult::{self, NoElements, OneElement, MinMax};
  
  let r: MinMaxResult<i32> = NoElements;
  assert_eq!(r.into_option(), None);
  
  let r = OneElement(1);
  assert_eq!(r.into_option(), Some((1, 1)));
  
  let r = MinMax(1, 2);
  assert_eq!(r.into_option(), Some((1, 2)));
  ```

#### Trait Implementations

##### `impl<T> Any for MinMaxResult<T>`

- <span id="minmaxresult-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MinMaxResult<T>`

- <span id="minmaxresult-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MinMaxResult<T>`

- <span id="minmaxresult-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for MinMaxResult<T>`

- <span id="minmaxresult-clone"></span>`fn clone(&self) -> MinMaxResult<T>` — [`MinMaxResult`](minmax/index.md#minmaxresult)

##### `impl<T> CloneToUninit for MinMaxResult<T>`

- <span id="minmaxresult-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for MinMaxResult<T>`

##### `impl<T: fmt::Debug> Debug for MinMaxResult<T>`

- <span id="minmaxresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for MinMaxResult<T>`

##### `impl<T> From for MinMaxResult<T>`

- <span id="minmaxresult-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for MinMaxResult<T>`

- <span id="minmaxresult-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for MinMaxResult<T>`

##### `impl<T: cmp::PartialEq> PartialEq for MinMaxResult<T>`

- <span id="minmaxresult-partialeq-eq"></span>`fn eq(&self, other: &MinMaxResult<T>) -> bool` — [`MinMaxResult`](minmax/index.md#minmaxresult)

##### `impl<T> StructuralPartialEq for MinMaxResult<T>`

##### `impl<T> ToOwned for MinMaxResult<T>`

- <span id="minmaxresult-toowned-type-owned"></span>`type Owned = T`

- <span id="minmaxresult-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="minmaxresult-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for MinMaxResult<T>`

- <span id="minmaxresult-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="minmaxresult-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for MinMaxResult<T>`

- <span id="minmaxresult-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="minmaxresult-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Position`

```rust
enum Position {
    First,
    Middle,
    Last,
    Only,
}
```

*Defined in [`itertools-0.14.0/src/with_position.rs:50-59`](../../.source_1765894658/itertools-0.14.0/src/with_position.rs#L50-L59)*

The first component of the value yielded by `WithPosition`.
Indicates the position of this element in the iterator results.

See [`.with_position()`](crate::Itertools::with_position) for more information.

#### Variants

- **`First`**

  This is the first element.

- **`Middle`**

  This is neither the first nor the last element.

- **`Last`**

  This is the last element.

- **`Only`**

  This is the only element.

#### Trait Implementations

##### `impl Any for Position`

- <span id="position-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Position`

- <span id="position-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Position`

- <span id="position-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Position`

- <span id="position-clone"></span>`fn clone(&self) -> Position` — [`Position`](with_position/index.md#position)

##### `impl CloneToUninit for Position`

- <span id="position-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Position`

##### `impl Debug for Position`

- <span id="position-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Position`

##### `impl<T> From for Position`

- <span id="position-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Position`

- <span id="position-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Position`

##### `impl PartialEq for Position`

- <span id="position-partialeq-eq"></span>`fn eq(&self, other: &Position) -> bool` — [`Position`](with_position/index.md#position)

##### `impl StructuralPartialEq for Position`

##### `impl ToOwned for Position`

- <span id="position-toowned-type-owned"></span>`type Owned = T`

- <span id="position-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="position-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Position`

- <span id="position-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="position-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Position`

- <span id="position-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="position-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `EitherOrBoth<A, B>`

```rust
enum EitherOrBoth<A, B> {
    Both(A, B),
    Left(A),
    Right(B),
}
```

*Defined in [`itertools-0.14.0/src/either_or_both.rs:9-16`](../../.source_1765894658/itertools-0.14.0/src/either_or_both.rs#L9-L16)*

Value that either holds a single A or B, or both.

#### Variants

- **`Both`**

  Both values are present.

- **`Left`**

  Only the left value of type `A` is present.

- **`Right`**

  Only the right value of type `B` is present.

#### Implementations

- <span id="eitherorboth-has-left"></span>`fn has_left(&self) -> bool`

  If `Left`, or `Both`, return true. Otherwise, return false.

- <span id="eitherorboth-has-right"></span>`fn has_right(&self) -> bool`

  If `Right`, or `Both`, return true, otherwise, return false.

- <span id="eitherorboth-is-left"></span>`fn is_left(&self) -> bool`

  If `Left`, return true. Otherwise, return false.
  Exclusive version of [`has_left`](EitherOrBoth::has_left).

- <span id="eitherorboth-is-right"></span>`fn is_right(&self) -> bool`

  If `Right`, return true. Otherwise, return false.
  Exclusive version of [`has_right`](EitherOrBoth::has_right).

- <span id="eitherorboth-is-both"></span>`fn is_both(&self) -> bool`

  If `Both`, return true. Otherwise, return false.

- <span id="eitherorboth-left"></span>`fn left(self) -> Option<A>`

  If `Left`, or `Both`, return `Some` with the left value. Otherwise, return `None`.

- <span id="eitherorboth-right"></span>`fn right(self) -> Option<B>`

  If `Right`, or `Both`, return `Some` with the right value. Otherwise, return `None`.

- <span id="eitherorboth-left-and-right"></span>`fn left_and_right(self) -> (Option<A>, Option<B>)`

  Return tuple of options corresponding to the left and right value respectively
  
  If `Left` return `(Some(..), None)`, if `Right` return `(None,Some(..))`, else return
  `(Some(..),Some(..))`

- <span id="eitherorboth-just-left"></span>`fn just_left(self) -> Option<A>`

  If `Left`, return `Some` with the left value. If `Right` or `Both`, return `None`.
  
  # Examples
  
  ```rust
  // On the `Left` variant.
  use itertools::{EitherOrBoth, EitherOrBoth::{Left, Right, Both}};
  let x: EitherOrBoth<_, ()> = Left("bonjour");
  assert_eq!(x.just_left(), Some("bonjour"));
  
  // On the `Right` variant.
  let x: EitherOrBoth<(), _> = Right("hola");
  assert_eq!(x.just_left(), None);
  
  // On the `Both` variant.
  let x = Both("bonjour", "hola");
  assert_eq!(x.just_left(), None);
  ```

- <span id="eitherorboth-just-right"></span>`fn just_right(self) -> Option<B>`

  If `Right`, return `Some` with the right value. If `Left` or `Both`, return `None`.
  
  # Examples
  
  ```rust
  // On the `Left` variant.
  use itertools::{EitherOrBoth::{Left, Right, Both}, EitherOrBoth};
  let x: EitherOrBoth<_, ()> = Left("auf wiedersehen");
  assert_eq!(x.just_left(), Some("auf wiedersehen"));
  
  // On the `Right` variant.
  let x: EitherOrBoth<(), _> = Right("adios");
  assert_eq!(x.just_left(), None);
  
  // On the `Both` variant.
  let x = Both("auf wiedersehen", "adios");
  assert_eq!(x.just_left(), None);
  ```

- <span id="eitherorboth-both"></span>`fn both(self) -> Option<(A, B)>`

  If `Both`, return `Some` containing the left and right values. Otherwise, return `None`.

- <span id="eitherorboth-into-left"></span>`fn into_left(self) -> A`

  If `Left` or `Both`, return the left value. Otherwise, convert the right value and return it.

- <span id="eitherorboth-into-right"></span>`fn into_right(self) -> B`

  If `Right` or `Both`, return the right value. Otherwise, convert the left value and return it.

- <span id="eitherorboth-as-ref"></span>`fn as_ref(&self) -> EitherOrBoth<&A, &B>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Converts from `&EitherOrBoth<A, B>` to `EitherOrBoth<&A, &B>`.

- <span id="eitherorboth-as-mut"></span>`fn as_mut(&mut self) -> EitherOrBoth<&mut A, &mut B>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Converts from `&mut EitherOrBoth<A, B>` to `EitherOrBoth<&mut A, &mut B>`.

- <span id="eitherorboth-as-deref"></span>`fn as_deref(&self) -> EitherOrBoth<&<A as >::Target, &<B as >::Target>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Converts from `&EitherOrBoth<A, B>` to `EitherOrBoth<&_, &_>` using the `Deref` trait.

- <span id="eitherorboth-as-deref-mut"></span>`fn as_deref_mut(&mut self) -> EitherOrBoth<&mut <A as >::Target, &mut <B as >::Target>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Converts from `&mut EitherOrBoth<A, B>` to `EitherOrBoth<&mut _, &mut _>` using the `DerefMut` trait.

- <span id="eitherorboth-flip"></span>`fn flip(self) -> EitherOrBoth<B, A>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Convert `EitherOrBoth<A, B>` to `EitherOrBoth<B, A>`.

- <span id="eitherorboth-map-left"></span>`fn map_left<F, M>(self, f: F) -> EitherOrBoth<M, B>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Apply the function `f` on the value `a` in `Left(a)` or `Both(a, b)` variants. If it is
  present rewrapping the result in `self`'s original variant.

- <span id="eitherorboth-map-right"></span>`fn map_right<F, M>(self, f: F) -> EitherOrBoth<A, M>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Apply the function `f` on the value `b` in `Right(b)` or `Both(a, b)` variants.
  If it is present rewrapping the result in `self`'s original variant.

- <span id="eitherorboth-map-any"></span>`fn map_any<F, L, G, R>(self, f: F, g: G) -> EitherOrBoth<L, R>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Apply the functions `f` and `g` on the value `a` and `b` respectively;
  found in `Left(a)`, `Right(b)`, or `Both(a, b)` variants.
  The Result is rewrapped `self`'s original variant.

- <span id="eitherorboth-left-and-then"></span>`fn left_and_then<F, L>(self, f: F) -> EitherOrBoth<L, B>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Apply the function `f` on the value `a` in `Left(a)` or `Both(a, _)` variants if it is
  present.

- <span id="eitherorboth-right-and-then"></span>`fn right_and_then<F, R>(self, f: F) -> EitherOrBoth<A, R>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

  Apply the function `f` on the value `b`
  in `Right(b)` or `Both(_, b)` variants if it is present.

- <span id="eitherorboth-or"></span>`fn or(self, l: A, r: B) -> (A, B)`

  Returns a tuple consisting of the `l` and `r` in `Both(l, r)`, if present.
  Otherwise, returns the wrapped value for the present element, and the supplied
  value for the other. The first (`l`) argument is used for a missing `Left`
  value. The second (`r`) argument is used for a missing `Right` value.
  
  Arguments passed to `or` are eagerly evaluated; if you are passing
  the result of a function call, it is recommended to use `or_else`,
  which is lazily evaluated.
  
  # Examples
  
  ```rust
  use itertools::EitherOrBoth;
  assert_eq!(EitherOrBoth::Both("tree", 1).or("stone", 5), ("tree", 1));
  assert_eq!(EitherOrBoth::Left("tree").or("stone", 5), ("tree", 5));
  assert_eq!(EitherOrBoth::Right(1).or("stone", 5), ("stone", 1));
  ```

- <span id="eitherorboth-or-default"></span>`fn or_default(self) -> (A, B)`

  Returns a tuple consisting of the `l` and `r` in `Both(l, r)`, if present.
  Otherwise, returns the wrapped value for the present element, and the [`default`](Default::default)
  for the other.

- <span id="eitherorboth-or-else"></span>`fn or_else<L: FnOnce() -> A, R: FnOnce() -> B>(self, l: L, r: R) -> (A, B)`

  Returns a tuple consisting of the `l` and `r` in `Both(l, r)`, if present.
  Otherwise, returns the wrapped value for the present element, and computes the
  missing value with the supplied closure. The first argument (`l`) is used for a
  missing `Left` value. The second argument (`r`) is used for a missing `Right` value.
  
  # Examples
  
  ```rust
  use itertools::EitherOrBoth;
  let k = 10;
  assert_eq!(EitherOrBoth::Both("tree", 1).or_else(|| "stone", || 2 * k), ("tree", 1));
  assert_eq!(EitherOrBoth::Left("tree").or_else(|| "stone", || 2 * k), ("tree", 20));
  assert_eq!(EitherOrBoth::Right(1).or_else(|| "stone", || 2 * k), ("stone", 1));
  ```

- <span id="eitherorboth-left-or-insert"></span>`fn left_or_insert(&mut self, val: A) -> &mut A`

  Returns a mutable reference to the left value. If the left value is not present,
  it is replaced with `val`.

- <span id="eitherorboth-right-or-insert"></span>`fn right_or_insert(&mut self, val: B) -> &mut B`

  Returns a mutable reference to the right value. If the right value is not present,
  it is replaced with `val`.

- <span id="eitherorboth-left-or-insert-with"></span>`fn left_or_insert_with<F>(&mut self, f: F) -> &mut A`

  If the left value is not present, replace it the value computed by the closure `f`.
  Returns a mutable reference to the now-present left value.

- <span id="eitherorboth-right-or-insert-with"></span>`fn right_or_insert_with<F>(&mut self, f: F) -> &mut B`

  If the right value is not present, replace it the value computed by the closure `f`.
  Returns a mutable reference to the now-present right value.

- <span id="eitherorboth-insert-left"></span>`fn insert_left(&mut self, val: A) -> &mut A`

  Sets the `left` value of this instance, and returns a mutable reference to it.
  Does not affect the `right` value.
  
  # Examples
  ```rust
  use itertools::{EitherOrBoth, EitherOrBoth::{Left, Right, Both}};
  
  // Overwriting a pre-existing value.
  let mut either: EitherOrBoth<_, ()> = Left(0_u32);
  assert_eq!(*either.insert_left(69), 69);
  
  // Inserting a second value.
  let mut either = Right("no");
  assert_eq!(*either.insert_left("yes"), "yes");
  assert_eq!(either, Both("yes", "no"));
  ```

- <span id="eitherorboth-insert-right"></span>`fn insert_right(&mut self, val: B) -> &mut B`

  Sets the `right` value of this instance, and returns a mutable reference to it.
  Does not affect the `left` value.
  
  # Examples
  ```rust
  use itertools::{EitherOrBoth, EitherOrBoth::{Left, Both}};
  // Overwriting a pre-existing value.
  let mut either: EitherOrBoth<_, ()> = Left(0_u32);
  assert_eq!(*either.insert_left(69), 69);
  
  // Inserting a second value.
  let mut either = Left("what's");
  assert_eq!(*either.insert_right(9 + 10), 21 - 2);
  assert_eq!(either, Both("what's", 9+10));
  ```

- <span id="eitherorboth-insert-both"></span>`fn insert_both(&mut self, left: A, right: B) -> (&mut A, &mut B)`

  Set `self` to `Both(..)`, containing the specified left and right values,
  and returns a mutable reference to those values.

#### Trait Implementations

##### `impl Any for EitherOrBoth<A, B>`

- <span id="eitherorboth-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for EitherOrBoth<A, B>`

- <span id="eitherorboth-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for EitherOrBoth<A, B>`

- <span id="eitherorboth-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone, B: clone::Clone> Clone for EitherOrBoth<A, B>`

- <span id="eitherorboth-clone"></span>`fn clone(&self) -> EitherOrBoth<A, B>` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

##### `impl CloneToUninit for EitherOrBoth<A, B>`

- <span id="eitherorboth-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug, B: fmt::Debug> Debug for EitherOrBoth<A, B>`

- <span id="eitherorboth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: cmp::Eq, B: cmp::Eq> Eq for EitherOrBoth<A, B>`

##### `impl<T> From for EitherOrBoth<A, B>`

- <span id="eitherorboth-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: hash::Hash, B: hash::Hash> Hash for EitherOrBoth<A, B>`

- <span id="eitherorboth-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for EitherOrBoth<A, B>`

- <span id="eitherorboth-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for EitherOrBoth<A, B>`

##### `impl<A: cmp::PartialEq, B: cmp::PartialEq> PartialEq for EitherOrBoth<A, B>`

- <span id="eitherorboth-partialeq-eq"></span>`fn eq(&self, other: &EitherOrBoth<A, B>) -> bool` — [`EitherOrBoth`](either_or_both/index.md#eitherorboth)

##### `impl<A, B> StructuralPartialEq for EitherOrBoth<A, B>`

##### `impl ToOwned for EitherOrBoth<A, B>`

- <span id="eitherorboth-toowned-type-owned"></span>`type Owned = T`

- <span id="eitherorboth-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="eitherorboth-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for EitherOrBoth<A, B>`

- <span id="eitherorboth-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="eitherorboth-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for EitherOrBoth<A, B>`

- <span id="eitherorboth-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="eitherorboth-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FoldWhile<T>`

```rust
enum FoldWhile<T> {
    Continue(T),
    Done(T),
}
```

*Defined in [`itertools-0.14.0/src/lib.rs:4691-4696`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L4691-L4696)*

An enum used for controlling the execution of `fold_while`.

See [`.fold_while()`](Itertools::fold_while) for more information.

#### Variants

- **`Continue`**

  Continue folding with this value

- **`Done`**

  Fold is complete and will return this value

#### Implementations

- <span id="foldwhile-into-inner"></span>`fn into_inner(self) -> T`

  Return the value in the continue or done.

- <span id="foldwhile-is-done"></span>`fn is_done(&self) -> bool`

  Return true if `self` is `Done`, false if it is `Continue`.

#### Trait Implementations

##### `impl<T> Any for FoldWhile<T>`

- <span id="foldwhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldWhile<T>`

- <span id="foldwhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldWhile<T>`

- <span id="foldwhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for FoldWhile<T>`

- <span id="foldwhile-clone"></span>`fn clone(&self) -> FoldWhile<T>` — [`FoldWhile`](#foldwhile)

##### `impl<T> CloneToUninit for FoldWhile<T>`

- <span id="foldwhile-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for FoldWhile<T>`

##### `impl<T: fmt::Debug> Debug for FoldWhile<T>`

- <span id="foldwhile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for FoldWhile<T>`

##### `impl<T> From for FoldWhile<T>`

- <span id="foldwhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for FoldWhile<T>`

- <span id="foldwhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for FoldWhile<T>`

##### `impl<T: cmp::PartialEq> PartialEq for FoldWhile<T>`

- <span id="foldwhile-partialeq-eq"></span>`fn eq(&self, other: &FoldWhile<T>) -> bool` — [`FoldWhile`](#foldwhile)

##### `impl<T> StructuralPartialEq for FoldWhile<T>`

##### `impl<T> ToOwned for FoldWhile<T>`

- <span id="foldwhile-toowned-type-owned"></span>`type Owned = T`

- <span id="foldwhile-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="foldwhile-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for FoldWhile<T>`

- <span id="foldwhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldwhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for FoldWhile<T>`

- <span id="foldwhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldwhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `PeekingNext`

```rust
trait PeekingNext: Iterator { ... }
```

*Defined in [`itertools-0.14.0/src/peeking_take_while.rs:15-23`](../../.source_1765894658/itertools-0.14.0/src/peeking_take_while.rs#L15-L23)*

An iterator that allows peeking at an element before deciding to accept it.

See [`.peeking_take_while()`](crate::Itertools::peeking_take_while)
for more information.

This is implemented by peeking adaptors like peekable and put back,
but also by a few iterators that can be peeked natively, like the slice’s
by reference iterator (`std::slice::Iter`).

#### Required Methods

- `fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

  Pass a reference to the next iterator element to the closure `accept`;
  if `accept` returns `true`, return it as the next element,
  else `None`.

#### Implementors

- [`MultiPeek`](multipeek_impl/index.md#multipeek)
- [`PeekNth`](peek_nth/index.md#peeknth)
- [`PeekingTakeWhile`](peeking_take_while/index.md#peekingtakewhile)
- [`PutBackN`](put_back_n_impl/index.md#putbackn)
- [`PutBack`](adaptors/index.md#putback)
- [`RepeatN`](repeatn/index.md#repeatn)
- `&mut I`
- `::std::iter::Empty<T>`
- `::std::iter::Rev<I>`
- `::std::option::Iter<'a, T>`
- `::std::result::Iter<'a, T>`
- `::std::slice::Iter<'a, T>`
- `::std::str::Bytes<'a>`
- `::std::str::CharIndices<'a>`
- `::std::str::Chars<'a>`
- `alloc::collections::linked_list::Iter<'a, T>`
- `alloc::collections::vec_deque::Iter<'a, T>`
- `std::iter::Peekable<I>`

### `MultiUnzip<FromI>`

```rust
trait MultiUnzip<FromI>: Iterator { ... }
```

*Defined in [`itertools-0.14.0/src/unziptuple.rs:32-35`](../../.source_1765894658/itertools-0.14.0/src/unziptuple.rs#L32-L35)*

An iterator that can be unzipped into multiple collections.

See [`.multiunzip()`](crate::Itertools::multiunzip) for more information.

#### Required Methods

- `fn multiunzip(self) -> FromI`

  Unzip this iterator into multiple collections.

#### Implementors

- `IT`

### `Itertools`

```rust
trait Itertools: Iterator { ... }
```

*Defined in [`itertools-0.14.0/src/lib.rs:438-4584`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L438-L4584)*

An `Iterator` blanket implementation that provides extra adaptors and
methods.

This trait defines a number of methods. They are divided into two groups:

* *Adaptors* take an iterator and parameter as input, and return
  a new iterator value. These are listed first in the trait. An example
  of an adaptor is [`.interleave()`](Itertools::interleave)

* *Regular methods* are those that don't return iterators and instead
  return a regular value of some other kind.
  [`.next_tuple()`](Itertools::next_tuple) is an example and the first regular
  method in the list.

#### Provided Methods

- `fn interleave<J>(self, other: J) -> Interleave<Self, <J as >::IntoIter>`

  Alternate elements from two iterators until both have run out.
  
  Iterator element type is `Self::Item`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  let it = (1..7).interleave(vec![-1, -2]);
  itertools::assert_equal(it, vec![1, -1, 2, -2, 3, 4, 5, 6]);
  ```

- `fn interleave_shortest<J>(self, other: J) -> InterleaveShortest<Self, <J as >::IntoIter>`

  Alternate elements from two iterators until at least one of them has run
  out.
  
  Iterator element type is `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  
  let it = (1..7).interleave_shortest(vec![-1, -2]);
  itertools::assert_equal(it, vec![1, -1, 2, -2, 3]);
  ```

- `fn intersperse(self, element: <Self as >::Item) -> Intersperse<Self>`

  An iterator adaptor to insert a particular value
  between each element of the adapted iterator.
  
  Iterator element type is `Self::Item`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  itertools::assert_equal((0..3).intersperse(8), vec![0, 8, 1, 8, 2]);
  ```

- `fn intersperse_with<F>(self, element: F) -> IntersperseWith<Self, F>`

  An iterator adaptor to insert a particular value created by a function
  between each element of the adapted iterator.
  
  Iterator element type is `Self::Item`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  let mut i = 10;
  itertools::assert_equal((0..3).intersperse_with(|| { i -= 1; i }), vec![0, 9, 1, 8, 2]);
  assert_eq!(i, 8);
  ```

- `fn get<R>(self, index: R) -> <R as >::Output`

  Returns an iterator over a subsection of the iterator.
  
  Works similarly to [`slice::get`](https://doc.rust-lang.org/std/primitive.slice.html#method.get).
  
  **Panics** for ranges `..=usize::MAX` and `0..=usize::MAX`.
  
  It's a generalisation of `Iterator::take` and `Iterator::skip`,
  and uses these under the hood.
  Therefore, the resulting iterator is:
  - `ExactSizeIterator` if the adapted iterator is `ExactSizeIterator`.
  - `DoubleEndedIterator` if the adapted iterator is `DoubleEndedIterator` and `ExactSizeIterator`.
  
  # Unspecified Behavior
  The result of indexing with an exhausted `core::ops::RangeInclusive` is unspecified.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let vec = vec![3, 1, 4, 1, 5];
  
  let mut range: Vec<_> =
          vec.iter().get(1..=3).copied().collect();
  assert_eq!(&range, &[1, 4, 1]);
  
  // It works with other types of ranges, too
  range = vec.iter().get(..2).copied().collect();
  assert_eq!(&range, &[3, 1]);
  
  range = vec.iter().get(0..1).copied().collect();
  assert_eq!(&range, &[3]);
  
  range = vec.iter().get(2..).copied().collect();
  assert_eq!(&range, &[4, 1, 5]);
  
  range = vec.iter().get(..=2).copied().collect();
  assert_eq!(&range, &[3, 1, 4]);
  
  range = vec.iter().get(..).copied().collect();
  assert_eq!(range, vec);
  ```

- `fn zip_longest<J>(self, other: J) -> ZipLongest<Self, <J as >::IntoIter>`

  Create an iterator which iterates over both this and the specified
  iterator simultaneously, yielding pairs of two optional elements.
  
  This iterator is *fused*.
  
  As long as neither input iterator is exhausted yet, it yields two values
  via `EitherOrBoth::Both`.
  
  When the parameter iterator is exhausted, it only yields a value from the
  `self` iterator via `EitherOrBoth::Left`.
  
  When the `self` iterator is exhausted, it only yields a value from the
  parameter iterator via `EitherOrBoth::Right`.
  
  When both iterators return `None`, all further invocations of `.next()`
  will return `None`.
  
  Iterator element type is
  [`EitherOrBoth<Self::Item, J::Item>`](EitherOrBoth).
  
  ```rust
  use itertools::EitherOrBoth::{Both, Right};
  use itertools::Itertools;
  let it = (0..1).zip_longest(1..3);
  itertools::assert_equal(it, vec![Both(0, 1), Right(2)]);
  ```

- `fn zip_eq<J>(self, other: J) -> ZipEq<Self, <J as >::IntoIter>`

  Create an iterator which iterates over both this and the specified
  iterator simultaneously, yielding pairs of elements.
  
  **Panics** if the iterators reach an end and they are not of equal
  lengths.

- `fn batching<B, F>(self, f: F) -> Batching<Self, F>`

  A “meta iterator adaptor”. Its closure receives a reference to the
  iterator and may pick off as many elements as it likes, to produce the
  next iterator element.
  
  Iterator element type is `B`.
  
  ```rust
  use itertools::Itertools;
  
  // An adaptor that gathers elements in pairs
  let pit = (0..4).batching(|it| {
             match it.next() {
                 None => None,
                 Some(x) => match it.next() {
                     None => None,
                     Some(y) => Some((x, y)),
                 }
             }
         });
  
  itertools::assert_equal(pit, vec![(0, 1), (2, 3)]);
  ```

- `fn chunk_by<K, F>(self, key: F) -> ChunkBy<K, Self, F>`

  Return an *iterable* that can group iterator elements.
  Consecutive elements that map to the same key (“runs”), are assigned
  to the same group.
  
  `ChunkBy` is the storage for the lazy grouping operation.
  
  If the groups are consumed in order, or if each group's iterator is
  dropped without keeping it around, then `ChunkBy` uses no
  allocations.  It needs allocations only if several group iterators
  are alive at the same time.
  
  This type implements `IntoIterator` (it is **not** an iterator
  itself), because the group iterators need to borrow from this
  value. It should be stored in a local variable or temporary and
  iterated.
  
  Iterator element type is `(K, Group)`: the group's key and the
  group iterator.
  
  ```rust
  use itertools::Itertools;
  
  // chunk data into runs of larger than zero or not.
  let data = vec![1, 3, -2, -2, 1, 0, 1, 2];
  // chunks:     |---->|------>|--------->|
  
  // Note: The `&` is significant here, `ChunkBy` is iterable
  // only by reference. You can also call `.into_iter()` explicitly.
  let mut data_grouped = Vec::new();
  for (key, chunk) in &data.into_iter().chunk_by(|elt| *elt >= 0) {
      data_grouped.push((key, chunk.collect()));
  }
  assert_eq!(data_grouped, vec![(true, vec![1, 3]), (false, vec![-2, -2]), (true, vec![1, 0, 1, 2])]);
  ```

- `fn group_by<K, F>(self, key: F) -> ChunkBy<K, Self, F>`

  See [`.chunk_by()`](Itertools::chunk_by).

- `fn chunks(self, size: usize) -> IntoChunks<Self>`

  Return an *iterable* that can chunk the iterator.
  
  Yield subiterators (chunks) that each yield a fixed number elements,
  determined by `size`. The last chunk will be shorter if there aren't
  enough elements.
  
  `IntoChunks` is based on `ChunkBy`: it is iterable (implements
  `IntoIterator`, **not** `Iterator`), and it only buffers if several
  chunk iterators are alive at the same time.
  
  Iterator element type is `Chunk`, each chunk's iterator.
  
  **Panics** if `size` is 0.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![1, 1, 2, -2, 6, 0, 3, 1];
  //chunk size=3 |------->|-------->|--->|
  
  // Note: The `&` is significant here, `IntoChunks` is iterable
  // only by reference. You can also call `.into_iter()` explicitly.
  for chunk in &data.into_iter().chunks(3) {
      // Check that the sum of each chunk is 4.
      assert_eq!(4, chunk.sum());
  }
  ```

- `fn tuple_windows<T>(self) -> TupleWindows<Self, T>`

  Return an iterator over all contiguous windows producing tuples of
  a specific size (up to 12).
  
  `tuple_windows` clones the iterator elements so that they can be
  part of successive windows, this makes it most suited for iterators
  of references and other values that are cheap to copy.
  
  ```rust
  use itertools::Itertools;
  let mut v = Vec::new();
  
  // pairwise iteration
  for (a, b) in (1..5).tuple_windows() {
      v.push((a, b));
  }
  assert_eq!(v, vec![(1, 2), (2, 3), (3, 4)]);
  
  let mut it = (1..5).tuple_windows();
  assert_eq!(Some((1, 2, 3)), it.next());
  assert_eq!(Some((2, 3, 4)), it.next());
  assert_eq!(None, it.next());
  
  // this requires a type hint
  let it = (1..5).tuple_windows::<(_, _, _)>();
  itertools::assert_equal(it, vec![(1, 2, 3), (2, 3, 4)]);
  
  // you can also specify the complete type
  use itertools::TupleWindows;
  use std::ops::Range;
  
  let it: TupleWindows<Range<u32>, (u32, u32, u32)> = (1..5).tuple_windows();
  itertools::assert_equal(it, vec![(1, 2, 3), (2, 3, 4)]);
  ```

- `fn circular_tuple_windows<T>(self) -> CircularTupleWindows<Self, T>`

  Return an iterator over all windows, wrapping back to the first
  elements when the window would otherwise exceed the length of the
  iterator, producing tuples of a specific size (up to 12).
  
  `circular_tuple_windows` clones the iterator elements so that they can be
  part of successive windows, this makes it most suited for iterators
  of references and other values that are cheap to copy.
  
  ```rust
  use itertools::Itertools;
  let mut v = Vec::new();
  for (a, b) in (1..5).circular_tuple_windows() {
      v.push((a, b));
  }
  assert_eq!(v, vec![(1, 2), (2, 3), (3, 4), (4, 1)]);
  
  let mut it = (1..5).circular_tuple_windows();
  assert_eq!(Some((1, 2, 3)), it.next());
  assert_eq!(Some((2, 3, 4)), it.next());
  assert_eq!(Some((3, 4, 1)), it.next());
  assert_eq!(Some((4, 1, 2)), it.next());
  assert_eq!(None, it.next());
  
  // this requires a type hint
  let it = (1..5).circular_tuple_windows::<(_, _, _)>();
  itertools::assert_equal(it, vec![(1, 2, 3), (2, 3, 4), (3, 4, 1), (4, 1, 2)]);
  ```

- `fn tuples<T>(self) -> Tuples<Self, T>`

  Return an iterator that groups the items in tuples of a specific size
  (up to 12).
  
  See also the method [`.next_tuple()`](Itertools::next_tuple).
  
  ```rust
  use itertools::Itertools;
  let mut v = Vec::new();
  for (a, b) in (1..5).tuples() {
      v.push((a, b));
  }
  assert_eq!(v, vec![(1, 2), (3, 4)]);
  
  let mut it = (1..7).tuples();
  assert_eq!(Some((1, 2, 3)), it.next());
  assert_eq!(Some((4, 5, 6)), it.next());
  assert_eq!(None, it.next());
  
  // this requires a type hint
  let it = (1..7).tuples::<(_, _, _)>();
  itertools::assert_equal(it, vec![(1, 2, 3), (4, 5, 6)]);
  
  // you can also specify the complete type
  use itertools::Tuples;
  use std::ops::Range;
  
  let it: Tuples<Range<u32>, (u32, u32, u32)> = (1..7).tuples();
  itertools::assert_equal(it, vec![(1, 2, 3), (4, 5, 6)]);
  ```
  
  See also `Tuples::into_buffer`.

- `fn tee(self) -> (Tee<Self>, Tee<Self>)`

  Split into an iterator pair that both yield all elements from
  the original iterator.
  
  **Note:** If the iterator is clonable, prefer using that instead
  of using this method. Cloning is likely to be more efficient.
  
  Iterator element type is `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  let xs = vec![0, 1, 2, 3];
  
  let (mut t1, t2) = xs.into_iter().tee();
  itertools::assert_equal(t1.next(), Some(0));
  itertools::assert_equal(t2, 0..4);
  itertools::assert_equal(t1, 1..4);
  ```

- `fn map_into<R>(self) -> MapInto<Self, R>`

  Convert each item of the iterator using the `Into` trait.
  
  ```rust
  use itertools::Itertools;
  
  (1i32..42i32).map_into::<f64>().collect_vec();
  ```

- `fn map_ok<F, T, U, E>(self, f: F) -> MapOk<Self, F>`

  Return an iterator adaptor that applies the provided closure
  to every `Result::Ok` value. `Result::Err` values are
  unchanged.
  
  ```rust
  use itertools::Itertools;
  
  let input = vec![Ok(41), Err(false), Ok(11)];
  let it = input.into_iter().map_ok(|i| i + 1);
  itertools::assert_equal(it, vec![Ok(42), Err(false), Ok(12)]);
  ```

- `fn filter_ok<F, T, E>(self, f: F) -> FilterOk<Self, F>`

  Return an iterator adaptor that filters every `Result::Ok`
  value with the provided closure. `Result::Err` values are
  unchanged.
  
  ```rust
  use itertools::Itertools;
  
  let input = vec![Ok(22), Err(false), Ok(11)];
  let it = input.into_iter().filter_ok(|&i| i > 20);
  itertools::assert_equal(it, vec![Ok(22), Err(false)]);
  ```

- `fn filter_map_ok<F, T, U, E>(self, f: F) -> FilterMapOk<Self, F>`

  Return an iterator adaptor that filters and transforms every
  `Result::Ok` value with the provided closure. `Result::Err`
  values are unchanged.
  
  ```rust
  use itertools::Itertools;
  
  let input = vec![Ok(22), Err(false), Ok(11)];
  let it = input.into_iter().filter_map_ok(|i| if i > 20 { Some(i * 2) } else { None });
  itertools::assert_equal(it, vec![Ok(44), Err(false)]);
  ```

- `fn flatten_ok<T, E>(self) -> FlattenOk<Self, T, E>`

  Return an iterator adaptor that flattens every `Result::Ok` value into
  a series of `Result::Ok` values. `Result::Err` values are unchanged.
  
  This is useful when you have some common error type for your crate and
  need to propagate it upwards, but the `Result::Ok` case needs to be flattened.
  
  ```rust
  use itertools::Itertools;
  
  let input = vec![Ok(0..2), Err(false), Ok(2..4)];
  let it = input.iter().cloned().flatten_ok();
  itertools::assert_equal(it.clone(), vec![Ok(0), Ok(1), Err(false), Ok(2), Ok(3)]);
  
  // This can also be used to propagate errors when collecting.
  let output_result: Result<Vec<i32>, bool> = it.collect();
  assert_eq!(output_result, Err(false));
  ```

- `fn process_results<F, T, E, R>(self, processor: F) -> Result<R, E>`

  “Lift” a function of the values of the current iterator so as to process
  an iterator of `Result` values instead.
  
  `processor` is a closure that receives an adapted version of the iterator
  as the only argument — the adapted iterator produces elements of type `T`,
  as long as the original iterator produces `Ok` values.
  
  If the original iterable produces an error at any point, the adapted
  iterator ends and it will return the error iself.
  
  Otherwise, the return value from the closure is returned wrapped
  inside `Ok`.
  
  # Example
  
  ```rust
  use itertools::Itertools;
  
  type Item = Result<i32, &'static str>;
  
  let first_values: Vec<Item> = vec![Ok(1), Ok(0), Ok(3)];
  let second_values: Vec<Item> = vec![Ok(2), Ok(1), Err("overflow")];
  
  // “Lift” the iterator .max() method to work on the Ok-values.
  let first_max = first_values.into_iter().process_results(|iter| iter.max().unwrap_or(0));
  let second_max = second_values.into_iter().process_results(|iter| iter.max().unwrap_or(0));
  
  assert_eq!(first_max, Ok(3));
  assert!(second_max.is_err());
  ```

- `fn merge<J>(self, other: J) -> Merge<Self, <J as >::IntoIter>`

  Return an iterator adaptor that merges the two base iterators in
  ascending order.  If both base iterators are sorted (ascending), the
  result is sorted.
  
  Iterator element type is `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  
  let a = (0..11).step_by(3);
  let b = (0..11).step_by(5);
  let it = a.merge(b);
  itertools::assert_equal(it, vec![0, 0, 3, 5, 6, 9, 10]);
  ```

- `fn merge_by<J, F>(self, other: J, is_first: F) -> MergeBy<Self, <J as >::IntoIter, F>`

  Return an iterator adaptor that merges the two base iterators in order.
  This is much like [`.merge()`](Itertools::merge) but allows for a custom ordering.
  
  This can be especially useful for sequences of tuples.
  
  Iterator element type is `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  
  let a = (0..).zip("bc".chars());
  let b = (0..).zip("ad".chars());
  let it = a.merge_by(b, |x, y| x.1 <= y.1);
  itertools::assert_equal(it, vec![(0, 'a'), (0, 'b'), (1, 'c'), (1, 'd')]);
  ```

- `fn merge_join_by<J, F, T>(self, other: J, cmp_fn: F) -> MergeJoinBy<Self, <J as >::IntoIter, F>`

  Create an iterator that merges items from both this and the specified
  iterator in ascending order.
  
  The function can either return an `Ordering` variant or a boolean.
  
  If `cmp_fn` returns `Ordering`,
  it chooses whether to pair elements based on the `Ordering` returned by the
  specified compare function. At any point, inspecting the tip of the
  iterators `I` and `J` as items `i` of type `I::Item` and `j` of type
  `J::Item` respectively, the resulting iterator will:
  
  - Emit `EitherOrBoth::Left(i)` when `i < j`,
    and remove `i` from its source iterator
  - Emit `EitherOrBoth::Right(j)` when `i > j`,
    and remove `j` from its source iterator
  - Emit `EitherOrBoth::Both(i, j)` when  `i == j`,
    and remove both `i` and `j` from their respective source iterators
  
  ```rust
  use itertools::Itertools;
  use itertools::EitherOrBoth::{Left, Right, Both};
  
  let a = vec![0, 2, 4, 6, 1].into_iter();
  let b = (0..10).step_by(3);
  
  itertools::assert_equal(
      // This performs a diff in the style of the Unix command comm(1),
      // generalized to arbitrary types rather than text.
      a.merge_join_by(b, Ord::cmp),
      vec![Both(0, 0), Left(2), Right(3), Left(4), Both(6, 6), Left(1), Right(9)]
  );
  ```
  
  If `cmp_fn` returns `bool`,
  it chooses whether to pair elements based on the boolean returned by the
  specified function. At any point, inspecting the tip of the
  iterators `I` and `J` as items `i` of type `I::Item` and `j` of type
  `J::Item` respectively, the resulting iterator will:
  
  - Emit `Either::Left(i)` when `true`,
    and remove `i` from its source iterator
  - Emit `Either::Right(j)` when `false`,
    and remove `j` from its source iterator
  
  It is similar to the `Ordering` case if the first argument is considered
  "less" than the second argument.
  
  ```rust
  use itertools::Itertools;
  use itertools::Either::{Left, Right};
  
  let a = vec![0, 2, 4, 6, 1].into_iter();
  let b = (0..10).step_by(3);
  
  itertools::assert_equal(
      a.merge_join_by(b, |i, j| i <= j),
      vec![Left(0), Right(0), Left(2), Right(3), Left(4), Left(6), Left(1), Right(6), Right(9)]
  );
  ```

- `fn kmerge(self) -> KMerge<<<Self as >::Item as IntoIterator>::IntoIter>`

  Return an iterator adaptor that flattens an iterator of iterators by
  merging them in ascending order.
  
  If all base iterators are sorted (ascending), the result is sorted.
  
  Iterator element type is `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  
  let a = (0..6).step_by(3);
  let b = (1..6).step_by(3);
  let c = (2..6).step_by(3);
  let it = vec![a, b, c].into_iter().kmerge();
  itertools::assert_equal(it, vec![0, 1, 2, 3, 4, 5]);
  ```

- `fn kmerge_by<F>(self, first: F) -> KMergeBy<<<Self as >::Item as IntoIterator>::IntoIter, F>`

  Return an iterator adaptor that flattens an iterator of iterators by
  merging them according to the given closure.
  
  The closure `first` is called with two elements *a*, *b* and should
  return `true` if *a* is ordered before *b*.
  
  If all base iterators are sorted according to `first`, the result is
  sorted.
  
  Iterator element type is `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  
  let a = vec![-1f64, 2., 3., -5., 6., -7.];
  let b = vec![0., 2., -4.];
  let mut it = vec![a, b].into_iter().kmerge_by(|a, b| a.abs() < b.abs());
  assert_eq!(it.next(), Some(0.));
  assert_eq!(it.last(), Some(-7.));
  ```

- `fn cartesian_product<J>(self, other: J) -> Product<Self, <J as >::IntoIter>`

  Return an iterator adaptor that iterates over the cartesian product of
  the element sets of two iterators `self` and `J`.
  
  Iterator element type is `(Self::Item, J::Item)`.
  
  ```rust
  use itertools::Itertools;
  
  let it = (0..2).cartesian_product("αβ".chars());
  itertools::assert_equal(it, vec![(0, 'α'), (0, 'β'), (1, 'α'), (1, 'β')]);
  ```

- `fn multi_cartesian_product(self) -> MultiProduct<<<Self as >::Item as IntoIterator>::IntoIter>`

  Return an iterator adaptor that iterates over the cartesian product of
  all subiterators returned by meta-iterator `self`.
  
  All provided iterators must yield the same `Item` type. To generate
  the product of iterators yielding multiple types, use the
  [`iproduct`](#iproduct) macro instead.
  
  The iterator element type is `Vec<T>`, where `T` is the iterator element
  of the subiterators.
  
  Note that the iterator is fused.
  
  ```rust
  use itertools::Itertools;
  let mut multi_prod = (0..3).map(|i| (i * 2)..(i * 2 + 2))
      .multi_cartesian_product();
  assert_eq!(multi_prod.next(), Some(vec![0, 2, 4]));
  assert_eq!(multi_prod.next(), Some(vec![0, 2, 5]));
  assert_eq!(multi_prod.next(), Some(vec![0, 3, 4]));
  assert_eq!(multi_prod.next(), Some(vec![0, 3, 5]));
  assert_eq!(multi_prod.next(), Some(vec![1, 2, 4]));
  assert_eq!(multi_prod.next(), Some(vec![1, 2, 5]));
  assert_eq!(multi_prod.next(), Some(vec![1, 3, 4]));
  assert_eq!(multi_prod.next(), Some(vec![1, 3, 5]));
  assert_eq!(multi_prod.next(), None);
  ```
  
  If the adapted iterator is empty, the result is an iterator yielding a single empty vector.
  This is known as the [nullary cartesian product](https://en.wikipedia.org/wiki/Empty_product#Nullary_Cartesian_product).
  
  ```rust
  use itertools::Itertools;
  let mut nullary_cartesian_product = (0..0).map(|i| (i * 2)..(i * 2 + 2)).multi_cartesian_product();
  assert_eq!(nullary_cartesian_product.next(), Some(vec![]));
  assert_eq!(nullary_cartesian_product.next(), None);
  ```

- `fn coalesce<F>(self, f: F) -> Coalesce<Self, F>`

  Return an iterator adaptor that uses the passed-in closure to
  optionally merge together consecutive elements.
  
  The closure `f` is passed two elements, `previous` and `current` and may
  return either (1) `Ok(combined)` to merge the two values or
  (2) `Err((previous', current'))` to indicate they can't be merged.
  In (2), the value `previous'` is emitted by the iterator.
  Either (1) `combined` or (2) `current'` becomes the previous value
  when coalesce continues with the next pair of elements to merge. The
  value that remains at the end is also emitted by the iterator.
  
  Iterator element type is `Self::Item`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  // sum same-sign runs together
  let data = vec![-1., -2., -3., 3., 1., 0., -1.];
  itertools::assert_equal(data.into_iter().coalesce(|x, y|
          if (x >= 0.) == (y >= 0.) {
              Ok(x + y)
          } else {
              Err((x, y))
          }),
          vec![-6., 4., -1.]);
  ```

- `fn dedup(self) -> Dedup<Self>`

  Remove duplicates from sections of consecutive identical elements.
  If the iterator is sorted, all elements will be unique.
  
  Iterator element type is `Self::Item`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![1., 1., 2., 3., 3., 2., 2.];
  itertools::assert_equal(data.into_iter().dedup(),
                          vec![1., 2., 3., 2.]);
  ```

- `fn dedup_by<Cmp>(self, cmp: Cmp) -> DedupBy<Self, Cmp>`

  Remove duplicates from sections of consecutive identical elements,
  determining equality using a comparison function.
  If the iterator is sorted, all elements will be unique.
  
  Iterator element type is `Self::Item`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![(0, 1.), (1, 1.), (0, 2.), (0, 3.), (1, 3.), (1, 2.), (2, 2.)];
  itertools::assert_equal(data.into_iter().dedup_by(|x, y| x.1 == y.1),
                          vec![(0, 1.), (0, 2.), (0, 3.), (1, 2.)]);
  ```

- `fn dedup_with_count(self) -> DedupWithCount<Self>`

  Remove duplicates from sections of consecutive identical elements, while keeping a count of
  how many repeated elements were present.
  If the iterator is sorted, all elements will be unique.
  
  Iterator element type is `(usize, Self::Item)`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec!['a', 'a', 'b', 'c', 'c', 'b', 'b'];
  itertools::assert_equal(data.into_iter().dedup_with_count(),
                          vec![(2, 'a'), (1, 'b'), (2, 'c'), (2, 'b')]);
  ```

- `fn dedup_by_with_count<Cmp>(self, cmp: Cmp) -> DedupByWithCount<Self, Cmp>`

  Remove duplicates from sections of consecutive identical elements, while keeping a count of
  how many repeated elements were present.
  This will determine equality using a comparison function.
  If the iterator is sorted, all elements will be unique.
  
  Iterator element type is `(usize, Self::Item)`.
  
  This iterator is *fused*.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![(0, 'a'), (1, 'a'), (0, 'b'), (0, 'c'), (1, 'c'), (1, 'b'), (2, 'b')];
  itertools::assert_equal(data.into_iter().dedup_by_with_count(|x, y| x.1 == y.1),
                          vec![(2, (0, 'a')), (1, (0, 'b')), (2, (0, 'c')), (2, (1, 'b'))]);
  ```

- `fn duplicates(self) -> Duplicates<Self>`

  Return an iterator adaptor that produces elements that appear more than once during the
  iteration. Duplicates are detected using hash and equality.
  
  The iterator is stable, returning the duplicate items in the order in which they occur in
  the adapted iterator. Each duplicate item is returned exactly once. If an item appears more
  than twice, the second item is the item retained and the rest are discarded.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![10, 20, 30, 20, 40, 10, 50];
  itertools::assert_equal(data.into_iter().duplicates(),
                          vec![20, 10]);
  ```

- `fn duplicates_by<V, F>(self, f: F) -> DuplicatesBy<Self, V, F>`

  Return an iterator adaptor that produces elements that appear more than once during the
  iteration. Duplicates are detected using hash and equality.
  
  Duplicates are detected by comparing the key they map to with the keying function `f` by
  hash and equality. The keys are stored in a hash map in the iterator.
  
  The iterator is stable, returning the duplicate items in the order in which they occur in
  the adapted iterator. Each duplicate item is returned exactly once. If an item appears more
  than twice, the second item is the item retained and the rest are discarded.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec!["a", "bb", "aa", "c", "ccc"];
  itertools::assert_equal(data.into_iter().duplicates_by(|s| s.len()),
                          vec!["aa", "c"]);
  ```

- `fn unique(self) -> Unique<Self>`

  Return an iterator adaptor that filters out elements that have
  already been produced once during the iteration. Duplicates
  are detected using hash and equality.
  
  Clones of visited elements are stored in a hash set in the
  iterator.
  
  The iterator is stable, returning the non-duplicate items in the order
  in which they occur in the adapted iterator. In a set of duplicate
  items, the first item encountered is the item retained.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![10, 20, 30, 20, 40, 10, 50];
  itertools::assert_equal(data.into_iter().unique(),
                          vec![10, 20, 30, 40, 50]);
  ```

- `fn unique_by<V, F>(self, f: F) -> UniqueBy<Self, V, F>`

  Return an iterator adaptor that filters out elements that have
  already been produced once during the iteration.
  
  Duplicates are detected by comparing the key they map to
  with the keying function `f` by hash and equality.
  The keys are stored in a hash set in the iterator.
  
  The iterator is stable, returning the non-duplicate items in the order
  in which they occur in the adapted iterator. In a set of duplicate
  items, the first item encountered is the item retained.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec!["a", "bb", "aa", "c", "ccc"];
  itertools::assert_equal(data.into_iter().unique_by(|s| s.len()),
                          vec!["a", "bb", "ccc"]);
  ```

- `fn peeking_take_while<F>(&mut self, accept: F) -> PeekingTakeWhile<'_, Self, F>`

  Return an iterator adaptor that borrows from this iterator and
  takes items while the closure `accept` returns `true`.
  
  This adaptor can only be used on iterators that implement `PeekingNext`
  like `.peekable()`, `put_back` and a few other collection iterators.
  
  The last and rejected element (first `false`) is still available when
  `peeking_take_while` is done.
  
  
  See also [`.take_while_ref()`](Itertools::take_while_ref)
  which is a similar adaptor.

- `fn take_while_ref<F>(&mut self, accept: F) -> TakeWhileRef<'_, Self, F>`

  Return an iterator adaptor that borrows from a `Clone`-able iterator
  to only pick off elements while the predicate `accept` returns `true`.
  
  It uses the `Clone` trait to restore the original iterator so that the
  last and rejected element (first `false`) is still available when
  `take_while_ref` is done.
  
  ```rust
  use itertools::Itertools;
  
  let mut hexadecimals = "0123456789abcdef".chars();
  
  let decimals = hexadecimals.take_while_ref(|c| c.is_numeric())
                             .collect::<String>();
  assert_eq!(decimals, "0123456789");
  assert_eq!(hexadecimals.next(), Some('a'));
  
  ```

- `fn take_while_inclusive<F>(self, accept: F) -> TakeWhileInclusive<Self, F>`

  Returns an iterator adaptor that consumes elements while the given
  predicate is `true`, *including* the element for which the predicate
  first returned `false`.
  
  The `.take_while()` adaptor is useful
  when you want items satisfying a predicate, but to know when to stop
  taking elements, we have to consume that first element that doesn't
  satisfy the predicate. This adaptor includes that element where
  `.take_while()` would drop it.
  
  The `.take_while_ref()` adaptor
  serves a similar purpose, but this adaptor doesn't require `Clone`ing
  the underlying elements.
  
  ```rust
  use itertools::Itertools;
  let items = vec![1, 2, 3, 4, 5];
  let filtered: Vec<_> = items
      .into_iter()
      .take_while_inclusive(|&n| n % 3 != 0)
      .collect();
  
  assert_eq!(filtered, vec![1, 2, 3]);
  ```
  
  ```rust
  use itertools::Itertools;
  let items = vec![1, 2, 3, 4, 5];
  
  let take_while_inclusive_result: Vec<_> = items
      .iter()
      .copied()
      .take_while_inclusive(|&n| n % 3 != 0)
      .collect();
  let take_while_result: Vec<_> = items
      .into_iter()
      .take_while(|&n| n % 3 != 0)
      .collect();
  
  assert_eq!(take_while_inclusive_result, vec![1, 2, 3]);
  assert_eq!(take_while_result, vec![1, 2]);
  // both iterators have the same items remaining at this point---the 3
  // is lost from the `take_while` vec
  ```
  
  ```rust
  use itertools::Itertools;
  #[derive(Debug, PartialEq)]
  struct NoCloneImpl(i32);
  
  let non_clonable_items: Vec<_> = vec![1, 2, 3, 4, 5]
      .into_iter()
      .map(NoCloneImpl)
      .collect();
  let filtered: Vec<_> = non_clonable_items
      .into_iter()
      .take_while_inclusive(|n| n.0 % 3 != 0)
      .collect();
  let expected: Vec<_> = vec![1, 2, 3].into_iter().map(NoCloneImpl).collect();
  assert_eq!(filtered, expected);

- `fn while_some<A>(self) -> WhileSome<Self>`

  Return an iterator adaptor that filters `Option<A>` iterator elements
  and produces `A`. Stops on the first `None` encountered.
  
  Iterator element type is `A`, the unwrapped element.
  
  ```rust
  use itertools::Itertools;
  
  // List all hexadecimal digits
  itertools::assert_equal(
      (0..).map(|i| std::char::from_digit(i, 16)).while_some(),
      "0123456789abcdef".chars());
  
  ```

- `fn tuple_combinations<T>(self) -> TupleCombinations<Self, T>`

  Return an iterator adaptor that iterates over the combinations of the
  elements from an iterator.
  
  Iterator element can be any homogeneous tuple of type `Self::Item` with
  size up to 12.
  
  # Guarantees
  
  If the adapted iterator is deterministic,
  this iterator adapter yields items in a reliable order.
  
  ```rust
  use itertools::Itertools;
  
  let mut v = Vec::new();
  for (a, b) in (1..5).tuple_combinations() {
      v.push((a, b));
  }
  assert_eq!(v, vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]);
  
  let mut it = (1..5).tuple_combinations();
  assert_eq!(Some((1, 2, 3)), it.next());
  assert_eq!(Some((1, 2, 4)), it.next());
  assert_eq!(Some((1, 3, 4)), it.next());
  assert_eq!(Some((2, 3, 4)), it.next());
  assert_eq!(None, it.next());
  
  // this requires a type hint
  let it = (1..5).tuple_combinations::<(_, _, _)>();
  itertools::assert_equal(it, vec![(1, 2, 3), (1, 2, 4), (1, 3, 4), (2, 3, 4)]);
  
  // you can also specify the complete type
  use itertools::TupleCombinations;
  use std::ops::Range;
  
  let it: TupleCombinations<Range<u32>, (u32, u32, u32)> = (1..5).tuple_combinations();
  itertools::assert_equal(it, vec![(1, 2, 3), (1, 2, 4), (1, 3, 4), (2, 3, 4)]);
  ```

- `fn array_combinations<const K: usize>(self) -> ArrayCombinations<Self, K>`

  Return an iterator adaptor that iterates over the combinations of the
  elements from an iterator.
  
  Iterator element type is [Self::Item; K]. The iterator produces a new
  array per iteration, and clones the iterator elements.
  
  # Guarantees
  
  If the adapted iterator is deterministic,
  this iterator adapter yields items in a reliable order.
  
  ```rust
  use itertools::Itertools;
  
  let mut v = Vec::new();
  for [a, b] in (1..5).array_combinations() {
      v.push([a, b]);
  }
  assert_eq!(v, vec![[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]);
  
  let mut it = (1..5).array_combinations();
  assert_eq!(Some([1, 2, 3]), it.next());
  assert_eq!(Some([1, 2, 4]), it.next());
  assert_eq!(Some([1, 3, 4]), it.next());
  assert_eq!(Some([2, 3, 4]), it.next());
  assert_eq!(None, it.next());
  
  // this requires a type hint
  let it = (1..5).array_combinations::<3>();
  itertools::assert_equal(it, vec![[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]);
  
  // you can also specify the complete type
  use itertools::ArrayCombinations;
  use std::ops::Range;
  
  let it: ArrayCombinations<Range<u32>, 3> = (1..5).array_combinations();
  itertools::assert_equal(it, vec![[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]);
  ```

- `fn combinations(self, k: usize) -> Combinations<Self>`

  Return an iterator adaptor that iterates over the `k`-length combinations of
  the elements from an iterator.
  
  Iterator element type is `Vec<Self::Item>`. The iterator produces a new `Vec` per iteration,
  and clones the iterator elements.
  
  # Guarantees
  
  If the adapted iterator is deterministic,
  this iterator adapter yields items in a reliable order.
  
  ```rust
  use itertools::Itertools;
  
  let it = (1..5).combinations(3);
  itertools::assert_equal(it, vec![
      vec![1, 2, 3],
      vec![1, 2, 4],
      vec![1, 3, 4],
      vec![2, 3, 4],
  ]);
  ```
  
  Note: Combinations does not take into account the equality of the iterated values.
  ```rust
  use itertools::Itertools;
  
  let it = vec![1, 2, 2].into_iter().combinations(2);
  itertools::assert_equal(it, vec![
      vec![1, 2], // Note: these are the same
      vec![1, 2], // Note: these are the same
      vec![2, 2],
  ]);
  ```

- `fn combinations_with_replacement(self, k: usize) -> CombinationsWithReplacement<Self>`

  Return an iterator that iterates over the `k`-length combinations of
  the elements from an iterator, with replacement.
  
  Iterator element type is `Vec<Self::Item>`. The iterator produces a new `Vec` per iteration,
  and clones the iterator elements.
  
  ```rust
  use itertools::Itertools;
  
  let it = (1..4).combinations_with_replacement(2);
  itertools::assert_equal(it, vec![
      vec![1, 1],
      vec![1, 2],
      vec![1, 3],
      vec![2, 2],
      vec![2, 3],
      vec![3, 3],
  ]);
  ```

- `fn permutations(self, k: usize) -> Permutations<Self>`

  Return an iterator adaptor that iterates over all k-permutations of the
  elements from an iterator.
  
  Iterator element type is `Vec<Self::Item>` with length `k`. The iterator
  produces a new `Vec` per iteration, and clones the iterator elements.
  
  If `k` is greater than the length of the input iterator, the resultant
  iterator adaptor will be empty.
  
  If you are looking for permutations with replacements,
  use `repeat_n(iter, k).multi_cartesian_product()` instead.
  
  ```rust
  use itertools::Itertools;
  
  let perms = (5..8).permutations(2);
  itertools::assert_equal(perms, vec![
      vec![5, 6],
      vec![5, 7],
      vec![6, 5],
      vec![6, 7],
      vec![7, 5],
      vec![7, 6],
  ]);
  ```
  
  Note: Permutations does not take into account the equality of the iterated values.
  
  ```rust
  use itertools::Itertools;
  
  let it = vec![2, 2].into_iter().permutations(2);
  itertools::assert_equal(it, vec![
      vec![2, 2], // Note: these are the same
      vec![2, 2], // Note: these are the same
  ]);
  ```
  
  Note: The source iterator is collected lazily, and will not be
  re-iterated if the permutations adaptor is completed and re-iterated.

- `fn powerset(self) -> Powerset<Self>`

  Return an iterator that iterates through the powerset of the elements from an
  iterator.
  
  Iterator element type is `Vec<Self::Item>`. The iterator produces a new `Vec`
  per iteration, and clones the iterator elements.
  
  The powerset of a set contains all subsets including the empty set and the full
  input set. A powerset has length _2^n_ where _n_ is the length of the input
  set.
  
  Each `Vec` produced by this iterator represents a subset of the elements
  produced by the source iterator.
  
  ```rust
  use itertools::Itertools;
  
  let sets = (1..4).powerset().collect::<Vec<_>>();
  itertools::assert_equal(sets, vec![
      vec![],
      vec![1],
      vec![2],
      vec![3],
      vec![1, 2],
      vec![1, 3],
      vec![2, 3],
      vec![1, 2, 3],
  ]);
  ```

- `fn pad_using<F>(self, min: usize, f: F) -> PadUsing<Self, F>`

  Return an iterator adaptor that pads the sequence to a minimum length of
  `min` by filling missing elements using a closure `f`.
  
  Iterator element type is `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  
  let it = (0..5).pad_using(10, |i| 2*i);
  itertools::assert_equal(it, vec![0, 1, 2, 3, 4, 10, 12, 14, 16, 18]);
  
  let it = (0..10).pad_using(5, |i| 2*i);
  itertools::assert_equal(it, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
  
  let it = (0..5).pad_using(10, |i| 2*i).rev();
  itertools::assert_equal(it, vec![18, 16, 14, 12, 10, 4, 3, 2, 1, 0]);
  ```

- `fn with_position(self) -> WithPosition<Self>`

  Return an iterator adaptor that combines each element with a `Position` to
  ease special-case handling of the first or last elements.
  
  Iterator element type is
  [`(Position, Self::Item)`](Position)
  
  ```rust
  use itertools::{Itertools, Position};
  
  let it = (0..4).with_position();
  itertools::assert_equal(it,
                          vec![(Position::First, 0),
                               (Position::Middle, 1),
                               (Position::Middle, 2),
                               (Position::Last, 3)]);
  
  let it = (0..1).with_position();
  itertools::assert_equal(it, vec![(Position::Only, 0)]);
  ```

- `fn positions<P>(self, predicate: P) -> Positions<Self, P>`

  Return an iterator adaptor that yields the indices of all elements
  satisfying a predicate, counted from the start of the iterator.
  
  Equivalent to `iter.enumerate().filter(|(_, v)| predicate(*v)).map(|(i, _)| i)`.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![1, 2, 3, 3, 4, 6, 7, 9];
  itertools::assert_equal(data.iter().positions(|v| v % 2 == 0), vec![1, 4, 5]);
  
  itertools::assert_equal(data.iter().positions(|v| v % 2 == 1).rev(), vec![7, 6, 3, 2, 0]);
  ```

- `fn update<F>(self, updater: F) -> Update<Self, F>`

  Return an iterator adaptor that applies a mutating function
  to each element before yielding it.
  
  ```rust
  use itertools::Itertools;
  
  let input = vec![vec![1], vec![3, 2, 1]];
  let it = input.into_iter().update(|v| v.push(0));
  itertools::assert_equal(it, vec![vec![1, 0], vec![3, 2, 1, 0]]);
  ```

- `fn next_array<const N: usize>(&mut self) -> Option<[<Self as >::Item; N]>`

  Advances the iterator and returns the next items grouped in an array of
  a specific size.
  
  If there are enough elements to be grouped in an array, then the array
  is returned inside `Some`, otherwise `None` is returned.
  
  ```rust
  use itertools::Itertools;
  
  let mut iter = 1..5;
  
  assert_eq!(Some([1, 2]), iter.next_array());
  ```

- `fn collect_array<const N: usize>(self) -> Option<[<Self as >::Item; N]>`

  Collects all items from the iterator into an array of a specific size.
  
  If the number of elements inside the iterator is **exactly** equal to
  the array size, then the array is returned inside `Some`, otherwise
  `None` is returned.
  
  ```rust
  use itertools::Itertools;
  
  let iter = 1..3;
  
  if let Some([x, y]) = iter.collect_array() {
      assert_eq!([x, y], [1, 2])
  } else {
      panic!("Expected two elements")
  }
  ```

- `fn next_tuple<T>(&mut self) -> Option<T>`

  Advances the iterator and returns the next items grouped in a tuple of
  a specific size (up to 12).
  
  If there are enough elements to be grouped in a tuple, then the tuple is
  returned inside `Some`, otherwise `None` is returned.
  
  ```rust
  use itertools::Itertools;
  
  let mut iter = 1..5;
  
  assert_eq!(Some((1, 2)), iter.next_tuple());
  ```

- `fn collect_tuple<T>(self) -> Option<T>`

  Collects all items from the iterator into a tuple of a specific size
  (up to 12).
  
  If the number of elements inside the iterator is **exactly** equal to
  the tuple size, then the tuple is returned inside `Some`, otherwise
  `None` is returned.
  
  ```rust
  use itertools::Itertools;
  
  let iter = 1..3;
  
  if let Some((x, y)) = iter.collect_tuple() {
      assert_eq!((x, y), (1, 2))
  } else {
      panic!("Expected two elements")
  }
  ```

- `fn find_position<P>(&mut self, pred: P) -> Option<(usize, <Self as >::Item)>`

  Find the position and value of the first element satisfying a predicate.
  
  The iterator is not advanced past the first element found.
  
  ```rust
  use itertools::Itertools;
  
  let text = "Hα";
  assert_eq!(text.chars().find_position(|ch| ch.is_lowercase()), Some((1, 'α')));
  ```

- `fn find_or_last<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Find the value of the first element satisfying a predicate or return the last element, if any.
  
  The iterator is not advanced past the first element found.
  
  ```rust
  use itertools::Itertools;
  
  let numbers = [1, 2, 3, 4];
  assert_eq!(numbers.iter().find_or_last(|&&x| x > 5), Some(&4));
  assert_eq!(numbers.iter().find_or_last(|&&x| x > 2), Some(&3));
  assert_eq!(std::iter::empty::<i32>().find_or_last(|&x| x > 5), None);
  
  // An iterator of Results can return the first Ok or the last Err:
  let input = vec![Err(()), Ok(11), Err(()), Ok(22)];
  assert_eq!(input.into_iter().find_or_last(Result::is_ok), Some(Ok(11)));
  
  let input: Vec<Result<(), i32>> = vec![Err(11), Err(22)];
  assert_eq!(input.into_iter().find_or_last(Result::is_ok), Some(Err(22)));
  
  assert_eq!(std::iter::empty::<Result<(), i32>>().find_or_last(Result::is_ok), None);
  ```

- `fn find_or_first<P>(self, predicate: P) -> Option<<Self as >::Item>`

  Find the value of the first element satisfying a predicate or return the first element, if any.
  
  The iterator is not advanced past the first element found.
  
  ```rust
  use itertools::Itertools;
  
  let numbers = [1, 2, 3, 4];
  assert_eq!(numbers.iter().find_or_first(|&&x| x > 5), Some(&1));
  assert_eq!(numbers.iter().find_or_first(|&&x| x > 2), Some(&3));
  assert_eq!(std::iter::empty::<i32>().find_or_first(|&x| x > 5), None);
  
  // An iterator of Results can return the first Ok or the first Err:
  let input = vec![Err(()), Ok(11), Err(()), Ok(22)];
  assert_eq!(input.into_iter().find_or_first(Result::is_ok), Some(Ok(11)));
  
  let input: Vec<Result<(), i32>> = vec![Err(11), Err(22)];
  assert_eq!(input.into_iter().find_or_first(Result::is_ok), Some(Err(11)));
  
  assert_eq!(std::iter::empty::<Result<(), i32>>().find_or_first(Result::is_ok), None);
  ```

- `fn contains<Q>(&mut self, query: &Q) -> bool`

  Returns `true` if the given item is present in this iterator.
  
  This method is short-circuiting. If the given item is present in this
  iterator, this method will consume the iterator up-to-and-including
  the item. If the given item is not present in this iterator, the
  iterator will be exhausted.
  
  ```rust
  use itertools::Itertools;
  
  #[derive(PartialEq, Debug)]
  enum Enum { A, B, C, D, E, }
  
  let mut iter = vec![Enum::A, Enum::B, Enum::C, Enum::D].into_iter();
  
  // search `iter` for `B`
  assert_eq!(iter.contains(&Enum::B), true);
  // `B` was found, so the iterator now rests at the item after `B` (i.e, `C`).
  assert_eq!(iter.next(), Some(Enum::C));
  
  // search `iter` for `E`
  assert_eq!(iter.contains(&Enum::E), false);
  // `E` wasn't found, so `iter` is now exhausted
  assert_eq!(iter.next(), None);
  ```

- `fn all_equal(&mut self) -> bool`

  Check whether all elements compare equal.
  
  Empty iterators are considered to have equal elements:
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 5, 5];
  assert!(!data.iter().all_equal());
  assert!(data[0..3].iter().all_equal());
  assert!(data[3..5].iter().all_equal());
  assert!(data[5..8].iter().all_equal());
  
  let data : Option<usize> = None;
  assert!(data.into_iter().all_equal());
  ```

- `fn all_equal_value(&mut self) -> Result<<Self as >::Item, Option<(<Self as >::Item, <Self as >::Item)>>`

  If there are elements and they are all equal, return a single copy of that element.
  If there are no elements, return an Error containing None.
  If there are elements and they are not all equal, return a tuple containing the first
  two non-equal elements found.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 5, 5];
  assert_eq!(data.iter().all_equal_value(), Err(Some((&1, &2))));
  assert_eq!(data[0..3].iter().all_equal_value(), Ok(&1));
  assert_eq!(data[3..5].iter().all_equal_value(), Ok(&2));
  assert_eq!(data[5..8].iter().all_equal_value(), Ok(&3));
  
  let data : Option<usize> = None;
  assert_eq!(data.into_iter().all_equal_value(), Err(None));
  ```

- `fn all_unique(&mut self) -> bool`

  Check whether all elements are unique (non equal).
  
  Empty iterators are considered to have unique elements:
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![1, 2, 3, 4, 1, 5];
  assert!(!data.iter().all_unique());
  assert!(data[0..4].iter().all_unique());
  assert!(data[1..6].iter().all_unique());
  
  let data : Option<usize> = None;
  assert!(data.into_iter().all_unique());
  ```

- `fn dropping(self, n: usize) -> Self`

  Consume the first `n` elements from the iterator eagerly,
  and return the same iterator again.
  
  It works similarly to `.skip(n)` except it is eager and
  preserves the iterator type.
  
  ```rust
  use itertools::Itertools;
  
  let iter = "αβγ".chars().dropping(2);
  itertools::assert_equal(iter, "γ".chars());
  ```
  
  *Fusing notes: if the iterator is exhausted by dropping,
  the result of calling `.next()` again depends on the iterator implementation.*

- `fn dropping_back(self, n: usize) -> Self`

  Consume the last `n` elements from the iterator eagerly,
  and return the same iterator again.
  
  This is only possible on double ended iterators. `n` may be
  larger than the number of elements.
  
  Note: This method is eager, dropping the back elements immediately and
  preserves the iterator type.
  
  ```rust
  use itertools::Itertools;
  
  let init = vec![0, 3, 6, 9].into_iter().dropping_back(1);
  itertools::assert_equal(init, vec![0, 3, 6]);
  ```

- `fn concat(self) -> <Self as >::Item`

  Combine all an iterator's elements into one element by using `Extend`.
  
  This combinator will extend the first item with each of the rest of the
  items of the iterator. If the iterator is empty, the default value of
  `I::Item` is returned.
  
  ```rust
  use itertools::Itertools;
  
  let input = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
  assert_eq!(input.into_iter().concat(),
             vec![1, 2, 3, 4, 5, 6]);
  ```

- `fn collect_vec(self) -> Vec<<Self as >::Item>`

  `.collect_vec()` is simply a type specialization of `Iterator::collect`,
  for convenience.

- `fn try_collect<T, U, E>(self) -> Result<U, E>`

  `.try_collect()` is more convenient way of writing
  `.collect::<Result<_, _>>()`
  
  # Example
  
  ```rust
  use std::{fs, io};
  use itertools::Itertools;
  
  fn process_dir_entries(entries: &[fs::DirEntry]) {
      // ...
      let _ = entries;
  }
  
  fn do_stuff() -> io::Result<()> {
      let entries: Vec<_> = fs::read_dir(".")?.try_collect()?;
      process_dir_entries(&entries);
  
      Ok(())
  }
  
  let _ = do_stuff;
  ```

- `fn set_from<'a, A: 'a, J>(&mut self, from: J) -> usize`

  Assign to each reference in `self` from the `from` iterator,
  stopping at the shortest of the two iterators.
  
  The `from` iterator is queried for its next element before the `self`
  iterator, and if either is exhausted the method is done.
  
  Return the number of elements written.
  
  ```rust
  use itertools::Itertools;
  
  let mut xs = [0; 4];
  xs.iter_mut().set_from(1..);
  assert_eq!(xs, [1, 2, 3, 4]);
  ```

- `fn join(&mut self, sep: &str) -> String`

  Combine all iterator elements into one `String`, separated by `sep`.
  
  Use the `Display` implementation of each element.
  
  ```rust
  use itertools::Itertools;
  
  assert_eq!(["a", "b", "c"].iter().join(", "), "a, b, c");
  assert_eq!([1, 2, 3].iter().join(", "), "1, 2, 3");
  ```

- `fn format(self, sep: &str) -> Format<'_, Self>`

  Format all iterator elements, separated by `sep`.
  
  All elements are formatted (any formatting trait)
  with `sep` inserted between each element.
  
  **Panics** if the formatter helper is formatted more than once.
  
  ```rust
  use itertools::Itertools;
  
  let data = [1.1, 2.71828, -3.];
  assert_eq!(
      format!("{:.2}", data.iter().format(", ")),
             "1.10, 2.72, -3.00");
  ```

- `fn format_with<F>(self, sep: &str, format: F) -> FormatWith<'_, Self, F>`

  Format all iterator elements, separated by `sep`.
  
  This is a customizable version of [`.format()`](Itertools::format).
  
  The supplied closure `format` is called once per iterator element,
  with two arguments: the element and a callback that takes a
  `&Display` value, i.e. any reference to type that implements `Display`.
  
  Using `&format_args!(...)` is the most versatile way to apply custom
  element formatting. The callback can be called multiple times if needed.
  
  **Panics** if the formatter helper is formatted more than once.
  
  ```rust
  use itertools::Itertools;
  
  let data = [1.1, 2.71828, -3.];
  let data_formatter = data.iter().format_with(", ", |elt, f| f(&format_args!("{:.2}", elt)));
  assert_eq!(format!("{}", data_formatter),
             "1.10, 2.72, -3.00");
  
  // .format_with() is recursively composable
  let matrix = [[1., 2., 3.],
                [4., 5., 6.]];
  let matrix_formatter = matrix.iter().format_with("\n", |row, f| {
                                  f(&row.iter().format_with(", ", |elt, g| g(&elt)))
                               });
  assert_eq!(format!("{}", matrix_formatter),
             "1, 2, 3\n4, 5, 6");
  
  
  ```

- `fn fold_ok<A, E, B, F>(&mut self, start: B, f: F) -> Result<B, E>`

  Fold `Result` values from an iterator.
  
  Only `Ok` values are folded. If no error is encountered, the folded
  value is returned inside `Ok`. Otherwise, the operation terminates
  and returns the first `Err` value it encounters. No iterator elements are
  consumed after the first error.
  
  The first accumulator value is the `start` parameter.
  Each iteration passes the accumulator value and the next value inside `Ok`
  to the fold function `f` and its return value becomes the new accumulator value.
  
  For example the sequence *Ok(1), Ok(2), Ok(3)* will result in a
  computation like this:
  
  ```no_run
  let start = 0;
  let f = |x, y| x + y;
  let mut accum = start;
  accum = f(accum, 1);
  accum = f(accum, 2);
  accum = f(accum, 3);
  let _ = accum;
  ```
  
  With a `start` value of 0 and an addition as folding function,
  this effectively results in *((0 + 1) + 2) + 3*
  
  ```rust
  use std::ops::Add;
  use itertools::Itertools;
  
  let values = [1, 2, -2, -1, 2, 1];
  assert_eq!(
      values.iter()
            .map(Ok::<_, ()>)
            .fold_ok(0, Add::add),
      Ok(3)
  );
  assert!(
      values.iter()
            .map(|&x| if x >= 0 { Ok(x) } else { Err("Negative number") })
            .fold_ok(0, Add::add)
            .is_err()
  );
  ```

- `fn fold_options<A, B, F>(&mut self, start: B, f: F) -> Option<B>`

  Fold `Option` values from an iterator.
  
  Only `Some` values are folded. If no `None` is encountered, the folded
  value is returned inside `Some`. Otherwise, the operation terminates
  and returns `None`. No iterator elements are consumed after the `None`.
  
  This is the `Option` equivalent to [`fold_ok`](Itertools::fold_ok).
  
  ```rust
  use std::ops::Add;
  use itertools::Itertools;
  
  let mut values = vec![Some(1), Some(2), Some(-2)].into_iter();
  assert_eq!(values.fold_options(5, Add::add), Some(5 + 1 + 2 - 2));
  
  let mut more_values = vec![Some(2), None, Some(0)].into_iter();
  assert!(more_values.fold_options(0, Add::add).is_none());
  assert_eq!(more_values.next().unwrap(), Some(0));
  ```

- `fn fold1<F>(self, f: F) -> Option<<Self as >::Item>`

  Accumulator of the elements in the iterator.
  
  Like `.fold()`, without a base case. If the iterator is
  empty, return `None`. With just one element, return it.
  Otherwise elements are accumulated in sequence using the closure `f`.
  
  ```rust
  use itertools::Itertools;
  
  assert_eq!((0..10).fold1(|x, y| x + y).unwrap_or(0), 45);
  assert_eq!((0..0).fold1(|x, y| x * y), None);
  ```

- `fn tree_reduce<F>(self, f: F) -> Option<<Self as >::Item>`

  Accumulate the elements in the iterator in a tree-like manner.
  
  You can think of it as, while there's more than one item, repeatedly
  combining adjacent items.  It does so in bottom-up-merge-sort order,
  however, so that it needs only logarithmic stack space.
  
  This produces a call tree like the following (where the calls under
  an item are done after reading that item):
  
  ```text
  1 2 3 4 5 6 7
  │ │ │ │ │ │ │
  └─f └─f └─f │
    │   │   │ │
    └───f   └─f
        │     │
        └─────f
  ```
  
  Which, for non-associative functions, will typically produce a different
  result than the linear call tree used by `Iterator::reduce`:
  
  ```text
  1 2 3 4 5 6 7
  │ │ │ │ │ │ │
  └─f─f─f─f─f─f
  ```
  
  If `f` is associative you should also decide carefully:
  
  For an iterator producing `n` elements, both `Iterator::reduce` and `tree_reduce` will
  call `f` `n - 1` times. However, `tree_reduce` will call `f` on earlier intermediate
  results, which is beneficial for `f` that allocate and produce longer results for longer
  arguments. For example if `f` combines arguments using `format!`, then `tree_reduce` will
  operate on average on shorter arguments resulting in less bytes being allocated overall.
  
  Moreover, the output of `tree_reduce` is preferable to that of `Iterator::reduce` in
  certain cases. For example, building a binary search tree using `tree_reduce` will result in
  a balanced tree with height `O(ln(n))`, while `Iterator::reduce` will output a tree with
  height `O(n)`, essentially a linked list.
  
  If `f` does not benefit from such a reordering, like `u32::wrapping_add`, prefer the
  normal `Iterator::reduce` instead since it will most likely result in the generation of
  simpler code because the compiler is able to optimize it.
  
  ```rust
  use itertools::Itertools;
  
  let f = |a: String, b: String| {
      format!("f({a}, {b})")
  };
  
  // The same tree as above
  assert_eq!((1..8).map(|x| x.to_string()).tree_reduce(f),
             Some(String::from("f(f(f(1, 2), f(3, 4)), f(f(5, 6), 7))")));
  
  // Like reduce, an empty iterator produces None
  assert_eq!((0..0).tree_reduce(|x, y| x * y), None);
  
  // tree_reduce matches reduce for associative operations...
  assert_eq!((0..10).tree_reduce(|x, y| x + y),
      (0..10).reduce(|x, y| x + y));
  
  // ...but not for non-associative ones
  assert_ne!((0..10).tree_reduce(|x, y| x - y),
      (0..10).reduce(|x, y| x - y));
  
  let mut total_len_reduce = 0;
  let reduce_res = (1..100).map(|x| x.to_string())
      .reduce(|a, b| {
          let r = f(a, b);
          total_len_reduce += r.len();
          r
      })
      .unwrap();
  
  let mut total_len_tree_reduce = 0;
  let tree_reduce_res = (1..100).map(|x| x.to_string())
      .tree_reduce(|a, b| {
          let r = f(a, b);
          total_len_tree_reduce += r.len();
          r
      })
      .unwrap();
  
  assert_eq!(total_len_reduce, 33299);
  assert_eq!(total_len_tree_reduce, 4228);
  assert_eq!(reduce_res.len(), tree_reduce_res.len());
  ```

- `fn tree_fold1<F>(self, f: F) -> Option<<Self as >::Item>`

  See [`.tree_reduce()`](Itertools::tree_reduce).

- `fn fold_while<B, F>(&mut self, init: B, f: F) -> FoldWhile<B>`

  An iterator method that applies a function, producing a single, final value.
  
  `fold_while()` is basically equivalent to `Iterator::fold` but with additional support for
  early exit via short-circuiting.
  
  ```rust
  use itertools::Itertools;
  use itertools::FoldWhile::{Continue, Done};
  
  let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  
  let mut result = 0;
  
  // for loop:
  for i in &numbers {
      if *i > 5 {
          break;
      }
      result = result + i;
  }
  
  // fold:
  let result2 = numbers.iter().fold(0, |acc, x| {
      if *x > 5 { acc } else { acc + x }
  });
  
  // fold_while:
  let result3 = numbers.iter().fold_while(0, |acc, x| {
      if *x > 5 { Done(acc) } else { Continue(acc + x) }
  }).into_inner();
  
  // they're the same
  assert_eq!(result, result2);
  assert_eq!(result2, result3);
  ```
  
  The big difference between the computations of `result2` and `result3` is that while
  `fold()` called the provided closure for every item of the callee iterator,
  `fold_while()` actually stopped iterating as soon as it encountered `Fold::Done(_)`.

- `fn sum1<S>(self) -> Option<S>`

  Iterate over the entire iterator and add all the elements.
  
  An empty iterator returns `None`, otherwise `Some(sum)`.
  
  # Panics
  
  When calling `sum1()` and a primitive integer type is being returned, this
  method will panic if the computation overflows and debug assertions are
  enabled.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let empty_sum = (1..1).sum1::<i32>();
  assert_eq!(empty_sum, None);
  
  let nonempty_sum = (1..11).sum1::<i32>();
  assert_eq!(nonempty_sum, Some(55));
  ```

- `fn product1<P>(self) -> Option<P>`

  Iterate over the entire iterator and multiply all the elements.
  
  An empty iterator returns `None`, otherwise `Some(product)`.
  
  # Panics
  
  When calling `product1()` and a primitive integer type is being returned,
  method will panic if the computation overflows and debug assertions are
  enabled.
  
  # Examples
  ```rust
  use itertools::Itertools;
  
  let empty_product = (1..1).product1::<i32>();
  assert_eq!(empty_product, None);
  
  let nonempty_product = (1..11).product1::<i32>();
  assert_eq!(nonempty_product, Some(3628800));
  ```

- `fn sorted_unstable(self) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort all iterator elements into a new iterator in ascending order.
  
  **Note:** This consumes the entire iterator, uses the
  `slice::sort_unstable` method and returns the result as a new
  iterator that owns its elements.
  
  This sort is unstable (i.e., may reorder equal elements).
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  ```rust
  use itertools::Itertools;
  
  // sort the letters of the text in ascending order
  let text = "bdacfe";
  itertools::assert_equal(text.chars().sorted_unstable(),
                          "abcdef".chars());
  ```

- `fn sorted_unstable_by<F>(self, cmp: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort all iterator elements into a new iterator in ascending order.
  
  **Note:** This consumes the entire iterator, uses the
  `slice::sort_unstable_by` method and returns the result as a new
  iterator that owns its elements.
  
  This sort is unstable (i.e., may reorder equal elements).
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  ```rust
  use itertools::Itertools;
  
  // sort people in descending order by age
  let people = vec![("Jane", 20), ("John", 18), ("Jill", 30), ("Jack", 27)];
  
  let oldest_people_first = people
      .into_iter()
      .sorted_unstable_by(|a, b| Ord::cmp(&b.1, &a.1))
      .map(|(person, _age)| person);
  
  itertools::assert_equal(oldest_people_first,
                          vec!["Jill", "Jack", "Jane", "John"]);
  ```

- `fn sorted_unstable_by_key<K, F>(self, f: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort all iterator elements into a new iterator in ascending order.
  
  **Note:** This consumes the entire iterator, uses the
  `slice::sort_unstable_by_key` method and returns the result as a new
  iterator that owns its elements.
  
  This sort is unstable (i.e., may reorder equal elements).
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  ```rust
  use itertools::Itertools;
  
  // sort people in descending order by age
  let people = vec![("Jane", 20), ("John", 18), ("Jill", 30), ("Jack", 27)];
  
  let oldest_people_first = people
      .into_iter()
      .sorted_unstable_by_key(|x| -x.1)
      .map(|(person, _age)| person);
  
  itertools::assert_equal(oldest_people_first,
                          vec!["Jill", "Jack", "Jane", "John"]);
  ```

- `fn sorted(self) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort all iterator elements into a new iterator in ascending order.
  
  **Note:** This consumes the entire iterator, uses the
  `slice::sort` method and returns the result as a new
  iterator that owns its elements.
  
  This sort is stable (i.e., does not reorder equal elements).
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  ```rust
  use itertools::Itertools;
  
  // sort the letters of the text in ascending order
  let text = "bdacfe";
  itertools::assert_equal(text.chars().sorted(),
                          "abcdef".chars());
  ```

- `fn sorted_by<F>(self, cmp: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort all iterator elements into a new iterator in ascending order.
  
  **Note:** This consumes the entire iterator, uses the
  `slice::sort_by` method and returns the result as a new
  iterator that owns its elements.
  
  This sort is stable (i.e., does not reorder equal elements).
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  ```rust
  use itertools::Itertools;
  
  // sort people in descending order by age
  let people = vec![("Jane", 20), ("John", 18), ("Jill", 30), ("Jack", 30)];
  
  let oldest_people_first = people
      .into_iter()
      .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
      .map(|(person, _age)| person);
  
  itertools::assert_equal(oldest_people_first,
                          vec!["Jill", "Jack", "Jane", "John"]);
  ```

- `fn sorted_by_key<K, F>(self, f: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort all iterator elements into a new iterator in ascending order.
  
  **Note:** This consumes the entire iterator, uses the
  `slice::sort_by_key` method and returns the result as a new
  iterator that owns its elements.
  
  This sort is stable (i.e., does not reorder equal elements).
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  ```rust
  use itertools::Itertools;
  
  // sort people in descending order by age
  let people = vec![("Jane", 20), ("John", 18), ("Jill", 30), ("Jack", 30)];
  
  let oldest_people_first = people
      .into_iter()
      .sorted_by_key(|x| -x.1)
      .map(|(person, _age)| person);
  
  itertools::assert_equal(oldest_people_first,
                          vec!["Jill", "Jack", "Jane", "John"]);
  ```

- `fn sorted_by_cached_key<K, F>(self, f: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort all iterator elements into a new iterator in ascending order. The key function is
  called exactly once per key.
  
  **Note:** This consumes the entire iterator, uses the
  `slice::sort_by_cached_key` method and returns the result as a new
  iterator that owns its elements.
  
  This sort is stable (i.e., does not reorder equal elements).
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  ```rust
  use itertools::Itertools;
  
  // sort people in descending order by age
  let people = vec![("Jane", 20), ("John", 18), ("Jill", 30), ("Jack", 30)];
  
  let oldest_people_first = people
      .into_iter()
      .sorted_by_cached_key(|x| -x.1)
      .map(|(person, _age)| person);
  
  itertools::assert_equal(oldest_people_first,
                          vec!["Jill", "Jack", "Jane", "John"]);
  ```

- `fn k_smallest(self, k: usize) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k smallest elements into a new iterator, in ascending order.
  
  **Note:** This consumes the entire iterator, and returns the result
  as a new iterator that owns its elements.  If the input contains
  less than k elements, the result is equivalent to `self.sorted()`.
  
  This is guaranteed to use `k * sizeof(Self::Item) + O(1)` memory
  and `O(n log k)` time, with `n` the number of elements in the input.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  **Note:** This is functionally-equivalent to `self.sorted().take(k)`
  but much more efficient.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_smallest = numbers
      .into_iter()
      .k_smallest(5);
  
  itertools::assert_equal(five_smallest, 0..5);
  ```

- `fn k_smallest_by<F>(self, k: usize, cmp: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k smallest elements into a new iterator using the provided comparison.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  This corresponds to `self.sorted_by(cmp).take(k)` in the same way that
  [`k_smallest`](Itertools::k_smallest) corresponds to `self.sorted().take(k)`,
  in both semantics and complexity.
  
  Particularly, a custom heap implementation ensures the comparison is not cloned.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_smallest = numbers
      .into_iter()
      .k_smallest_by(5, |a, b| (a % 7).cmp(&(b % 7)).then(a.cmp(b)));
  
  itertools::assert_equal(five_smallest, vec![0, 7, 14, 1, 8]);
  ```

- `fn k_smallest_by_key<F, K>(self, k: usize, key: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Return the elements producing the k smallest outputs of the provided function.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  This corresponds to `self.sorted_by_key(key).take(k)` in the same way that
  [`k_smallest`](Itertools::k_smallest) corresponds to `self.sorted().take(k)`,
  in both semantics and complexity.
  
  Particularly, a custom heap implementation ensures the comparison is not cloned.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_smallest = numbers
      .into_iter()
      .k_smallest_by_key(5, |n| (n % 7, *n));
  
  itertools::assert_equal(five_smallest, vec![0, 7, 14, 1, 8]);
  ```

- `fn k_smallest_relaxed(self, k: usize) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k smallest elements into a new iterator, in ascending order, relaxing the amount of memory required.
  
  **Note:** This consumes the entire iterator, and returns the result
  as a new iterator that owns its elements.  If the input contains
  less than k elements, the result is equivalent to `self.sorted()`.
  
  This is guaranteed to use `2 * k * sizeof(Self::Item) + O(1)` memory
  and `O(n + k log k)` time, with `n` the number of elements in the input,
  meaning it uses more memory than the minimum obtained by [`k_smallest`](Itertools::k_smallest)
  but achieves linear time in the number of elements.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  **Note:** This is functionally-equivalent to `self.sorted().take(k)`
  but much more efficient.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_smallest = numbers
      .into_iter()
      .k_smallest_relaxed(5);
  
  itertools::assert_equal(five_smallest, 0..5);
  ```

- `fn k_smallest_relaxed_by<F>(self, k: usize, cmp: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k smallest elements into a new iterator using the provided comparison, relaxing the amount of memory required.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  This corresponds to `self.sorted_by(cmp).take(k)` in the same way that
  [`k_smallest_relaxed`](Itertools::k_smallest_relaxed) corresponds to `self.sorted().take(k)`,
  in both semantics and complexity.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_smallest = numbers
      .into_iter()
      .k_smallest_relaxed_by(5, |a, b| (a % 7).cmp(&(b % 7)).then(a.cmp(b)));
  
  itertools::assert_equal(five_smallest, vec![0, 7, 14, 1, 8]);
  ```

- `fn k_smallest_relaxed_by_key<F, K>(self, k: usize, key: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Return the elements producing the k smallest outputs of the provided function, relaxing the amount of memory required.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  This corresponds to `self.sorted_by_key(key).take(k)` in the same way that
  [`k_smallest_relaxed`](Itertools::k_smallest_relaxed) corresponds to `self.sorted().take(k)`,
  in both semantics and complexity.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_smallest = numbers
      .into_iter()
      .k_smallest_relaxed_by_key(5, |n| (n % 7, *n));
  
  itertools::assert_equal(five_smallest, vec![0, 7, 14, 1, 8]);
  ```

- `fn k_largest(self, k: usize) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k largest elements into a new iterator, in descending order.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  It is semantically equivalent to [`k_smallest`](Itertools::k_smallest)
  with a reversed `Ord`.
  However, this is implemented with a custom binary heap which does not
  have the same performance characteristics for very large `Self::Item`.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_largest = numbers
      .into_iter()
      .k_largest(5);
  
  itertools::assert_equal(five_largest, vec![14, 13, 12, 11, 10]);
  ```

- `fn k_largest_by<F>(self, k: usize, cmp: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k largest elements into a new iterator using the provided comparison.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  Functionally equivalent to [`k_smallest_by`](Itertools::k_smallest_by)
  with a reversed `Ord`.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_largest = numbers
      .into_iter()
      .k_largest_by(5, |a, b| (a % 7).cmp(&(b % 7)).then(a.cmp(b)));
  
  itertools::assert_equal(five_largest, vec![13, 6, 12, 5, 11]);
  ```

- `fn k_largest_by_key<F, K>(self, k: usize, key: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Return the elements producing the k largest outputs of the provided function.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  Functionally equivalent to [`k_smallest_by_key`](Itertools::k_smallest_by_key)
  with a reversed `Ord`.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_largest = numbers
      .into_iter()
      .k_largest_by_key(5, |n| (n % 7, *n));
  
  itertools::assert_equal(five_largest, vec![13, 6, 12, 5, 11]);
  ```

- `fn k_largest_relaxed(self, k: usize) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k largest elements into a new iterator, in descending order, relaxing the amount of memory required.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  It is semantically equivalent to [`k_smallest_relaxed`](Itertools::k_smallest_relaxed)
  with a reversed `Ord`.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_largest = numbers
      .into_iter()
      .k_largest_relaxed(5);
  
  itertools::assert_equal(five_largest, vec![14, 13, 12, 11, 10]);
  ```

- `fn k_largest_relaxed_by<F>(self, k: usize, cmp: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Sort the k largest elements into a new iterator using the provided comparison, relaxing the amount of memory required.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  Functionally equivalent to [`k_smallest_relaxed_by`](Itertools::k_smallest_relaxed_by)
  with a reversed `Ord`.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_largest = numbers
      .into_iter()
      .k_largest_relaxed_by(5, |a, b| (a % 7).cmp(&(b % 7)).then(a.cmp(b)));
  
  itertools::assert_equal(five_largest, vec![13, 6, 12, 5, 11]);
  ```

- `fn k_largest_relaxed_by_key<F, K>(self, k: usize, key: F) -> alloc::vec::IntoIter<<Self as >::Item>`

  Return the elements producing the k largest outputs of the provided function, relaxing the amount of memory required.
  
  The sorted iterator, if directly collected to a `Vec`, is converted
  without any extra copying or allocation cost.
  
  Functionally equivalent to [`k_smallest_relaxed_by_key`](Itertools::k_smallest_relaxed_by_key)
  with a reversed `Ord`.
  
  ```rust
  use itertools::Itertools;
  
  // A random permutation of 0..15
  let numbers = vec![6, 9, 1, 14, 0, 4, 8, 7, 11, 2, 10, 3, 13, 12, 5];
  
  let five_largest = numbers
      .into_iter()
      .k_largest_relaxed_by_key(5, |n| (n % 7, *n));
  
  itertools::assert_equal(five_largest, vec![13, 6, 12, 5, 11]);
  ```

- `fn tail(self, n: usize) -> alloc::collections::vec_deque::IntoIter<<Self as >::Item>`

  Consumes the iterator and return an iterator of the last `n` elements.
  
  The iterator, if directly collected to a `VecDeque`, is converted
  without any extra copying or allocation cost.
  If directly collected to a `Vec`, it may need some data movement
  but no re-allocation.
  
  ```rust
  use itertools::{assert_equal, Itertools};
  
  let v = vec![5, 9, 8, 4, 2, 12, 0];
  assert_equal(v.iter().tail(3), &[2, 12, 0]);
  assert_equal(v.iter().tail(10), &v);
  
  assert_equal(v.iter().tail(1), v.iter().last());
  
  assert_equal((0..100).tail(10), 90..100);
  
  assert_equal((0..100).filter(|x| x % 3 == 0).tail(10), (72..100).step_by(3));
  ```
  
  For double ended iterators without side-effects, you might prefer
  `.rev().take(n).rev()` to have a similar result (lazy and non-allocating)
  without consuming the entire iterator.

- `fn partition_map<A, B, F, L, R>(self, predicate: F) -> (A, B)`

  Collect all iterator elements into one of two
  partitions. Unlike `Iterator::partition`, each partition may
  have a distinct type.
  
  ```rust
  use itertools::{Itertools, Either};
  
  let successes_and_failures = vec![Ok(1), Err(false), Err(true), Ok(2)];
  
  let (successes, failures): (Vec<_>, Vec<_>) = successes_and_failures
      .into_iter()
      .partition_map(|r| {
          match r {
              Ok(v) => Either::Left(v),
              Err(v) => Either::Right(v),
          }
      });
  
  assert_eq!(successes, [1, 2]);
  assert_eq!(failures, [false, true]);
  ```

- `fn partition_result<A, B, T, E>(self) -> (A, B)`

  Partition a sequence of `Result`s into one list of all the `Ok` elements
  and another list of all the `Err` elements.
  
  ```rust
  use itertools::Itertools;
  
  let successes_and_failures = vec![Ok(1), Err(false), Err(true), Ok(2)];
  
  let (successes, failures): (Vec<_>, Vec<_>) = successes_and_failures
      .into_iter()
      .partition_result();
  
  assert_eq!(successes, [1, 2]);
  assert_eq!(failures, [false, true]);
  ```

- `fn into_group_map<K, V>(self) -> HashMap<K, Vec<V>>`

  Return a `HashMap` of keys mapped to `Vec`s of values. Keys and values
  are taken from `(Key, Value)` tuple pairs yielded by the input iterator.
  
  Essentially a shorthand for `.into_grouping_map().collect::<Vec<_>>()`.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![(0, 10), (2, 12), (3, 13), (0, 20), (3, 33), (2, 42)];
  let lookup = data.into_iter().into_group_map();
  
  assert_eq!(lookup[&0], vec![10, 20]);
  assert_eq!(lookup.get(&1), None);
  assert_eq!(lookup[&2], vec![12, 42]);
  assert_eq!(lookup[&3], vec![13, 33]);
  ```

- `fn into_group_map_by<K, V, F>(self, f: F) -> HashMap<K, Vec<V>>`

  Return a `HashMap` of keys mapped to `Vec`s of values. The key is specified
  in the closure. The values are taken from the input iterator.
  
  Essentially a shorthand for `.into_grouping_map_by(f).collect::<Vec<_>>()`.
  
  ```rust
  use itertools::Itertools;
  use std::collections::HashMap;
  
  let data = vec![(0, 10), (2, 12), (3, 13), (0, 20), (3, 33), (2, 42)];
  let lookup: HashMap<u32,Vec<(u32, u32)>> =
      data.clone().into_iter().into_group_map_by(|a| a.0);
  
  assert_eq!(lookup[&0], vec![(0,10), (0,20)]);
  assert_eq!(lookup.get(&1), None);
  assert_eq!(lookup[&2], vec![(2,12), (2,42)]);
  assert_eq!(lookup[&3], vec![(3,13), (3,33)]);
  
  assert_eq!(
      data.into_iter()
          .into_group_map_by(|x| x.0)
          .into_iter()
          .map(|(key, values)| (key, values.into_iter().fold(0,|acc, (_,v)| acc + v )))
          .collect::<HashMap<u32,u32>>()[&0],
      30,
  );
  ```

- `fn into_grouping_map<K, V>(self) -> GroupingMap<Self>`

  Constructs a `GroupingMap` to be used later with one of the efficient
  group-and-fold operations it allows to perform.
  
  The input iterator must yield item in the form of `(K, V)` where the
  value of type `K` will be used as key to identify the groups and the
  value of type `V` as value for the folding operation.
  
  See [`GroupingMap`](grouping_map/index.md) for more informations
  on what operations are available.

- `fn into_grouping_map_by<K, V, F>(self, key_mapper: F) -> GroupingMapBy<Self, F>`

  Constructs a `GroupingMap` to be used later with one of the efficient
  group-and-fold operations it allows to perform.
  
  The values from this iterator will be used as values for the folding operation
  while the keys will be obtained from the values by calling `key_mapper`.
  
  See [`GroupingMap`](grouping_map/index.md) for more informations
  on what operations are available.

- `fn min_set(self) -> Vec<<Self as >::Item>`

  Return all minimum elements of an iterator.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().min_set(), Vec::<&i32>::new());
  
  let a = [1];
  assert_eq!(a.iter().min_set(), vec![&1]);
  
  let a = [1, 2, 3, 4, 5];
  assert_eq!(a.iter().min_set(), vec![&1]);
  
  let a = [1, 1, 1, 1];
  assert_eq!(a.iter().min_set(), vec![&1, &1, &1, &1]);
  ```
  
  The elements can be floats but no particular result is guaranteed
  if an element is NaN.

- `fn min_set_by<F>(self, compare: F) -> Vec<<Self as >::Item>`

  Return all minimum elements of an iterator, as determined by
  the specified function.
  
  # Examples
  
  ```rust
  use std::cmp::Ordering;
  use itertools::Itertools;
  
  let a: [(i32, i32); 0] = [];
  assert_eq!(a.iter().min_set_by(|_, _| Ordering::Equal), Vec::<&(i32, i32)>::new());
  
  let a = [(1, 2)];
  assert_eq!(a.iter().min_set_by(|&&(k1,_), &&(k2, _)| k1.cmp(&k2)), vec![&(1, 2)]);
  
  let a = [(1, 2), (2, 2), (3, 9), (4, 8), (5, 9)];
  assert_eq!(a.iter().min_set_by(|&&(_,k1), &&(_,k2)| k1.cmp(&k2)), vec![&(1, 2), &(2, 2)]);
  
  let a = [(1, 2), (1, 3), (1, 4), (1, 5)];
  assert_eq!(a.iter().min_set_by(|&&(k1,_), &&(k2, _)| k1.cmp(&k2)), vec![&(1, 2), &(1, 3), &(1, 4), &(1, 5)]);
  ```
  
  The elements can be floats but no particular result is guaranteed
  if an element is NaN.

- `fn min_set_by_key<K, F>(self, key: F) -> Vec<<Self as >::Item>`

  Return all minimum elements of an iterator, as determined by
  the specified function.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [(i32, i32); 0] = [];
  assert_eq!(a.iter().min_set_by_key(|_| ()), Vec::<&(i32, i32)>::new());
  
  let a = [(1, 2)];
  assert_eq!(a.iter().min_set_by_key(|&&(k,_)| k), vec![&(1, 2)]);
  
  let a = [(1, 2), (2, 2), (3, 9), (4, 8), (5, 9)];
  assert_eq!(a.iter().min_set_by_key(|&&(_, k)| k), vec![&(1, 2), &(2, 2)]);
  
  let a = [(1, 2), (1, 3), (1, 4), (1, 5)];
  assert_eq!(a.iter().min_set_by_key(|&&(k, _)| k), vec![&(1, 2), &(1, 3), &(1, 4), &(1, 5)]);
  ```
  
  The elements can be floats but no particular result is guaranteed
  if an element is NaN.

- `fn max_set(self) -> Vec<<Self as >::Item>`

  Return all maximum elements of an iterator.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().max_set(), Vec::<&i32>::new());
  
  let a = [1];
  assert_eq!(a.iter().max_set(), vec![&1]);
  
  let a = [1, 2, 3, 4, 5];
  assert_eq!(a.iter().max_set(), vec![&5]);
  
  let a = [1, 1, 1, 1];
  assert_eq!(a.iter().max_set(), vec![&1, &1, &1, &1]);
  ```
  
  The elements can be floats but no particular result is guaranteed
  if an element is NaN.

- `fn max_set_by<F>(self, compare: F) -> Vec<<Self as >::Item>`

  Return all maximum elements of an iterator, as determined by
  the specified function.
  
  # Examples
  
  ```rust
  use std::cmp::Ordering;
  use itertools::Itertools;
  
  let a: [(i32, i32); 0] = [];
  assert_eq!(a.iter().max_set_by(|_, _| Ordering::Equal), Vec::<&(i32, i32)>::new());
  
  let a = [(1, 2)];
  assert_eq!(a.iter().max_set_by(|&&(k1,_), &&(k2, _)| k1.cmp(&k2)), vec![&(1, 2)]);
  
  let a = [(1, 2), (2, 2), (3, 9), (4, 8), (5, 9)];
  assert_eq!(a.iter().max_set_by(|&&(_,k1), &&(_,k2)| k1.cmp(&k2)), vec![&(3, 9), &(5, 9)]);
  
  let a = [(1, 2), (1, 3), (1, 4), (1, 5)];
  assert_eq!(a.iter().max_set_by(|&&(k1,_), &&(k2, _)| k1.cmp(&k2)), vec![&(1, 2), &(1, 3), &(1, 4), &(1, 5)]);
  ```
  
  The elements can be floats but no particular result is guaranteed
  if an element is NaN.

- `fn max_set_by_key<K, F>(self, key: F) -> Vec<<Self as >::Item>`

  Return all maximum elements of an iterator, as determined by
  the specified function.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [(i32, i32); 0] = [];
  assert_eq!(a.iter().max_set_by_key(|_| ()), Vec::<&(i32, i32)>::new());
  
  let a = [(1, 2)];
  assert_eq!(a.iter().max_set_by_key(|&&(k,_)| k), vec![&(1, 2)]);
  
  let a = [(1, 2), (2, 2), (3, 9), (4, 8), (5, 9)];
  assert_eq!(a.iter().max_set_by_key(|&&(_, k)| k), vec![&(3, 9), &(5, 9)]);
  
  let a = [(1, 2), (1, 3), (1, 4), (1, 5)];
  assert_eq!(a.iter().max_set_by_key(|&&(k, _)| k), vec![&(1, 2), &(1, 3), &(1, 4), &(1, 5)]);
  ```
  
  The elements can be floats but no particular result is guaranteed
  if an element is NaN.

- `fn minmax(self) -> MinMaxResult<<Self as >::Item>`

  Return the minimum and maximum elements in the iterator.
  
  The return type `MinMaxResult` is an enum of three variants:
  
  - `NoElements` if the iterator is empty.
  - `OneElement(x)` if the iterator has exactly one element.
  - `MinMax(x, y)` is returned otherwise, where `x <= y`. Two
     values are equal if and only if there is more than one
     element in the iterator and all elements are equal.
  
  On an iterator of length `n`, `minmax` does `1.5 * n` comparisons,
  and so is faster than calling `min` and `max` separately which does
  `2 * n` comparisons.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{NoElements, OneElement, MinMax};
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().minmax(), NoElements);
  
  let a = [1];
  assert_eq!(a.iter().minmax(), OneElement(&1));
  
  let a = [1, 2, 3, 4, 5];
  assert_eq!(a.iter().minmax(), MinMax(&1, &5));
  
  let a = [1, 1, 1, 1];
  assert_eq!(a.iter().minmax(), MinMax(&1, &1));
  ```
  
  The elements can be floats but no particular result is guaranteed
  if an element is NaN.

- `fn minmax_by_key<K, F>(self, key: F) -> MinMaxResult<<Self as >::Item>`

  Return the minimum and maximum element of an iterator, as determined by
  the specified function.
  
  The return value is a variant of [`MinMaxResult`](minmax/index.md) like for [`.minmax()`](Itertools::minmax).
  
  For the minimum, the first minimal element is returned.  For the maximum,
  the last maximal element wins.  This matches the behavior of the standard
  `Iterator::min` and `Iterator::max` methods.
  
  The keys can be floats but no particular result is guaranteed
  if a key is NaN.

- `fn minmax_by<F>(self, compare: F) -> MinMaxResult<<Self as >::Item>`

  Return the minimum and maximum element of an iterator, as determined by
  the specified comparison function.
  
  The return value is a variant of [`MinMaxResult`](minmax/index.md) like for [`.minmax()`](Itertools::minmax).
  
  For the minimum, the first minimal element is returned.  For the maximum,
  the last maximal element wins.  This matches the behavior of the standard
  `Iterator::min` and `Iterator::max` methods.

- `fn position_max(self) -> Option<usize>`

  Return the position of the maximum element in the iterator.
  
  If several elements are equally maximum, the position of the
  last of them is returned.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_max(), None);
  
  let a = [-3, 0, 1, 5, -10];
  assert_eq!(a.iter().position_max(), Some(3));
  
  let a = [1, 1, -1, -1];
  assert_eq!(a.iter().position_max(), Some(1));
  ```

- `fn position_max_by_key<K, F>(self, key: F) -> Option<usize>`

  Return the position of the maximum element in the iterator, as
  determined by the specified function.
  
  If several elements are equally maximum, the position of the
  last of them is returned.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_max_by_key(|x| x.abs()), None);
  
  let a = [-3_i32, 0, 1, 5, -10];
  assert_eq!(a.iter().position_max_by_key(|x| x.abs()), Some(4));
  
  let a = [1_i32, 1, -1, -1];
  assert_eq!(a.iter().position_max_by_key(|x| x.abs()), Some(3));
  ```

- `fn position_max_by<F>(self, compare: F) -> Option<usize>`

  Return the position of the maximum element in the iterator, as
  determined by the specified comparison function.
  
  If several elements are equally maximum, the position of the
  last of them is returned.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_max_by(|x, y| x.cmp(y)), None);
  
  let a = [-3_i32, 0, 1, 5, -10];
  assert_eq!(a.iter().position_max_by(|x, y| x.cmp(y)), Some(3));
  
  let a = [1_i32, 1, -1, -1];
  assert_eq!(a.iter().position_max_by(|x, y| x.cmp(y)), Some(1));
  ```

- `fn position_min(self) -> Option<usize>`

  Return the position of the minimum element in the iterator.
  
  If several elements are equally minimum, the position of the
  first of them is returned.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_min(), None);
  
  let a = [-3, 0, 1, 5, -10];
  assert_eq!(a.iter().position_min(), Some(4));
  
  let a = [1, 1, -1, -1];
  assert_eq!(a.iter().position_min(), Some(2));
  ```

- `fn position_min_by_key<K, F>(self, key: F) -> Option<usize>`

  Return the position of the minimum element in the iterator, as
  determined by the specified function.
  
  If several elements are equally minimum, the position of the
  first of them is returned.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_min_by_key(|x| x.abs()), None);
  
  let a = [-3_i32, 0, 1, 5, -10];
  assert_eq!(a.iter().position_min_by_key(|x| x.abs()), Some(1));
  
  let a = [1_i32, 1, -1, -1];
  assert_eq!(a.iter().position_min_by_key(|x| x.abs()), Some(0));
  ```

- `fn position_min_by<F>(self, compare: F) -> Option<usize>`

  Return the position of the minimum element in the iterator, as
  determined by the specified comparison function.
  
  If several elements are equally minimum, the position of the
  first of them is returned.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_min_by(|x, y| x.cmp(y)), None);
  
  let a = [-3_i32, 0, 1, 5, -10];
  assert_eq!(a.iter().position_min_by(|x, y| x.cmp(y)), Some(4));
  
  let a = [1_i32, 1, -1, -1];
  assert_eq!(a.iter().position_min_by(|x, y| x.cmp(y)), Some(2));
  ```

- `fn position_minmax(self) -> MinMaxResult<usize>`

  Return the positions of the minimum and maximum elements in
  the iterator.
  
  The return type [`MinMaxResult`](minmax/index.md) is an enum of three variants:
  
  - `NoElements` if the iterator is empty.
  - `OneElement(xpos)` if the iterator has exactly one element.
  - `MinMax(xpos, ypos)` is returned otherwise, where the
     element at `xpos` ≤ the element at `ypos`. While the
     referenced elements themselves may be equal, `xpos` cannot
     be equal to `ypos`.
  
  On an iterator of length `n`, `position_minmax` does `1.5 * n`
  comparisons, and so is faster than calling `position_min` and
  `position_max` separately which does `2 * n` comparisons.
  
  For the minimum, if several elements are equally minimum, the
  position of the first of them is returned. For the maximum, if
  several elements are equally maximum, the position of the last
  of them is returned.
  
  The elements can be floats but no particular result is
  guaranteed if an element is NaN.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{NoElements, OneElement, MinMax};
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_minmax(), NoElements);
  
  let a = [10];
  assert_eq!(a.iter().position_minmax(), OneElement(0));
  
  let a = [-3, 0, 1, 5, -10];
  assert_eq!(a.iter().position_minmax(), MinMax(4, 3));
  
  let a = [1, 1, -1, -1];
  assert_eq!(a.iter().position_minmax(), MinMax(2, 1));
  ```

- `fn position_minmax_by_key<K, F>(self, key: F) -> MinMaxResult<usize>`

  Return the postions of the minimum and maximum elements of an
  iterator, as determined by the specified function.
  
  The return value is a variant of [`MinMaxResult`](minmax/index.md) like for
  `position_minmax`.
  
  For the minimum, if several elements are equally minimum, the
  position of the first of them is returned. For the maximum, if
  several elements are equally maximum, the position of the last
  of them is returned.
  
  The keys can be floats but no particular result is guaranteed
  if a key is NaN.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{NoElements, OneElement, MinMax};
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_minmax_by_key(|x| x.abs()), NoElements);
  
  let a = [10_i32];
  assert_eq!(a.iter().position_minmax_by_key(|x| x.abs()), OneElement(0));
  
  let a = [-3_i32, 0, 1, 5, -10];
  assert_eq!(a.iter().position_minmax_by_key(|x| x.abs()), MinMax(1, 4));
  
  let a = [1_i32, 1, -1, -1];
  assert_eq!(a.iter().position_minmax_by_key(|x| x.abs()), MinMax(0, 3));
  ```

- `fn position_minmax_by<F>(self, compare: F) -> MinMaxResult<usize>`

  Return the postions of the minimum and maximum elements of an
  iterator, as determined by the specified comparison function.
  
  The return value is a variant of [`MinMaxResult`](minmax/index.md) like for
  `position_minmax`.
  
  For the minimum, if several elements are equally minimum, the
  position of the first of them is returned. For the maximum, if
  several elements are equally maximum, the position of the last
  of them is returned.
  
  # Examples
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{NoElements, OneElement, MinMax};
  
  let a: [i32; 0] = [];
  assert_eq!(a.iter().position_minmax_by(|x, y| x.cmp(y)), NoElements);
  
  let a = [10_i32];
  assert_eq!(a.iter().position_minmax_by(|x, y| x.cmp(y)), OneElement(0));
  
  let a = [-3_i32, 0, 1, 5, -10];
  assert_eq!(a.iter().position_minmax_by(|x, y| x.cmp(y)), MinMax(4, 3));
  
  let a = [1_i32, 1, -1, -1];
  assert_eq!(a.iter().position_minmax_by(|x, y| x.cmp(y)), MinMax(2, 1));
  ```

- `fn exactly_one(self) -> Result<<Self as >::Item, ExactlyOneError<Self>>`

  If the iterator yields exactly one element, that element will be returned, otherwise
  an error will be returned containing an iterator that has the same output as the input
  iterator.
  
  This provides an additional layer of validation over just calling `Iterator::next()`.
  If your assumption that there should only be one element yielded is false this provides
  the opportunity to detect and handle that, preventing errors at a distance.
  
  # Examples
  ```rust
  use itertools::Itertools;
  
  assert_eq!((0..10).filter(|&x| x == 2).exactly_one().unwrap(), 2);
  assert!((0..10).filter(|&x| x > 1 && x < 4).exactly_one().unwrap_err().eq(2..4));
  assert!((0..10).filter(|&x| x > 1 && x < 5).exactly_one().unwrap_err().eq(2..5));
  assert!((0..10).filter(|&_| false).exactly_one().unwrap_err().eq(0..0));
  ```

- `fn at_most_one(self) -> Result<Option<<Self as >::Item>, ExactlyOneError<Self>>`

  If the iterator yields no elements, `Ok(None)` will be returned. If the iterator yields
  exactly one element, that element will be returned, otherwise an error will be returned
  containing an iterator that has the same output as the input iterator.
  
  This provides an additional layer of validation over just calling `Iterator::next()`.
  If your assumption that there should be at most one element yielded is false this provides
  the opportunity to detect and handle that, preventing errors at a distance.
  
  # Examples
  ```rust
  use itertools::Itertools;
  
  assert_eq!((0..10).filter(|&x| x == 2).at_most_one().unwrap(), Some(2));
  assert!((0..10).filter(|&x| x > 1 && x < 4).at_most_one().unwrap_err().eq(2..4));
  assert!((0..10).filter(|&x| x > 1 && x < 5).at_most_one().unwrap_err().eq(2..5));
  assert_eq!((0..10).filter(|&_| false).at_most_one().unwrap(), None);
  ```

- `fn multipeek(self) -> MultiPeek<Self>`

  An iterator adaptor that allows the user to peek at multiple `.next()`
  values without advancing the base iterator.
  
  # Examples
  ```rust
  use itertools::Itertools;
  
  let mut iter = (0..10).multipeek();
  assert_eq!(iter.peek(), Some(&0));
  assert_eq!(iter.peek(), Some(&1));
  assert_eq!(iter.peek(), Some(&2));
  assert_eq!(iter.next(), Some(0));
  assert_eq!(iter.peek(), Some(&1));
  ```

- `fn counts(self) -> HashMap<<Self as >::Item, usize>`

  Collect the items in this iterator and return a `HashMap` which
  contains each item that appears in the iterator and the number
  of times it appears.
  
  # Examples
  ```rust
  use itertools::Itertools;
  let counts = [1, 1, 1, 3, 3, 5].iter().counts();
  assert_eq!(counts[&1], 3);
  assert_eq!(counts[&3], 2);
  assert_eq!(counts[&5], 1);
  assert_eq!(counts.get(&0), None);
  ```

- `fn counts_by<K, F>(self, f: F) -> HashMap<K, usize>`

  Collect the items in this iterator and return a `HashMap` which
  contains each item that appears in the iterator and the number
  of times it appears,
  determining identity using a keying function.
  
  ```rust
  use itertools::Itertools;
  struct Character {
    first_name: &'static str,
    #[allow(dead_code)]
    last_name:  &'static str,
  }
  
  let characters =
      vec![
          Character { first_name: "Amy",   last_name: "Pond"      },
          Character { first_name: "Amy",   last_name: "Wong"      },
          Character { first_name: "Amy",   last_name: "Santiago"  },
          Character { first_name: "James", last_name: "Bond"      },
          Character { first_name: "James", last_name: "Sullivan"  },
          Character { first_name: "James", last_name: "Norington" },
          Character { first_name: "James", last_name: "Kirk"      },
      ];
  
  let first_name_frequency =
      characters
          .into_iter()
          .counts_by(|c| c.first_name);
  
  assert_eq!(first_name_frequency["Amy"], 3);
  assert_eq!(first_name_frequency["James"], 4);
  assert_eq!(first_name_frequency.contains_key("Asha"), false);
  ```

- `fn multiunzip<FromI>(self) -> FromI`

  Converts an iterator of tuples into a tuple of containers.
  
  It consumes an entire iterator of n-ary tuples, producing `n` collections, one for each
  column.
  
  This function is, in some sense, the opposite of [`multizip`](ziptuple/index.md).
  
  ```rust
  use itertools::Itertools;
  
  let inputs = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)];
  
  let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = inputs
      .into_iter()
      .multiunzip();
  
  assert_eq!(a, vec![1, 4, 7]);
  assert_eq!(b, vec![2, 5, 8]);
  assert_eq!(c, vec![3, 6, 9]);
  ```

- `fn try_len(&self) -> Result<usize, (usize, Option<usize>)>`

  Returns the length of the iterator if one exists.
  Otherwise return `self.size_hint()`.
  
  Fallible `ExactSizeIterator::len`.
  
  Inherits guarantees and restrictions from `Iterator::size_hint`.
  
  ```rust
  use itertools::Itertools;
  
  assert_eq!([0; 10].iter().try_len(), Ok(10));
  assert_eq!((10..15).try_len(), Ok(5));
  assert_eq!((15..10).try_len(), Ok(0));
  assert_eq!((10..).try_len(), Err((usize::MAX, None)));
  assert_eq!((10..15).filter(|x| x % 2 == 0).try_len(), Err((0, Some(5))));
  ```

#### Implementors

- `T`

## Functions

### `Either`

```rust
unsafe fn Either(chunk: V, masks: [Mask<V>; 4]) -> (V, V, V, V)
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:1140-1162`](../../.source_1765894658/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L1140-L1162)*

Return a candidate for Teddy (fat or slim) that is searching for 4-byte
candidates.

If candidates are returned, each will be a collection of 8-bit bitsets
(one bitset per lane), where the ith bit is set in the jth lane if and
only if the byte occurring at the jth lane in `chunk` is in the bucket
`i`. Each candidate returned corresponds to the first, second, third
and fourth bytes of the patterns being searched. If no candidate is
found, then all of the lanes will be set to zero in at least one of the
vectors returned.

`chunk` should correspond to a `V::BYTES` window of the haystack (where
the least significant byte corresponds to the start of the window). For
fat Teddy, the haystack window length should be `V::BYTES / 2`, with
the window repeated in each half of the vector.

The masks should correspond to the masks computed for the first,
second, third and fourth bytes of all patterns that are being searched.

### `concat`

```rust
fn concat<I>(iterable: I) -> <I as >::Item
where
    I: IntoIterator,
    <I as >::Item: Extend<<<I as IntoIterator>::Item as IntoIterator>::Item> + IntoIterator + Default
```

*Defined in [`itertools-0.14.0/src/concat_impl.rs:15-27`](../../.source_1765894658/itertools-0.14.0/src/concat_impl.rs#L15-L27)*

Combine all an iterator's elements into one element by using `Extend`.

`IntoIterator`-enabled version of [`Itertools::concat`](crate::Itertools::concat).

This combinator will extend the first item with each of the rest of the
items of the iterator. If the iterator is empty, the default value of
`I::Item` is returned.

```rust
use itertools::concat;

let input = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
assert_eq!(concat(input), vec![1, 2, 3, 4, 5, 6]);
```

### `cons_tuples`

```rust
fn cons_tuples<I>(iterable: I) -> ConsTuples<<I as >::IntoIter>
where
    I: IntoIterator
```

*Defined in [`itertools-0.14.0/src/cons_tuples_impl.rs:31-39`](../../.source_1765894658/itertools-0.14.0/src/cons_tuples_impl.rs#L31-L39)*

Create an iterator that maps for example iterators of
`((A, B), C)` to `(A, B, C)`.

### `diff_with`

```rust
fn diff_with<I, J, F>(i: I, j: J, is_equal: F) -> Option<Diff<<I as >::IntoIter, <J as >::IntoIter>>
where
    I: IntoIterator,
    J: IntoIterator,
    F: FnMut(&<I as >::Item, &<J as >::Item) -> bool
```

*Defined in [`itertools-0.14.0/src/diff.rs:80-104`](../../.source_1765894658/itertools-0.14.0/src/diff.rs#L80-L104)*

Compares every element yielded by both `i` and `j` with the given function in lock-step and
returns a [`Diff`](diff/index.md) which describes how `j` differs from `i`.

If the number of elements yielded by `j` is less than the number of elements yielded by `i`,
the number of `j` elements yielded will be returned along with `i`'s remaining elements as
`Diff::Shorter`.

If the two elements of a step differ, the index of those elements along with the remaining
elements of both `i` and `j` are returned as `Diff::FirstMismatch`.

If `i` becomes exhausted before `j` becomes exhausted, the number of elements in `i` along with
the remaining `j` elements will be returned as `Diff::Longer`.

### `kmerge_by`

```rust
fn kmerge_by<I, F>(iterable: I, less_than: F) -> KMergeBy<<<I as >::Item as IntoIterator>::IntoIter, F>
where
    I: IntoIterator,
    <I as >::Item: IntoIterator,
    F: KMergePredicate<<<I as IntoIterator>::Item as IntoIterator>::Item>
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:176-191`](../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L176-L191)*

Create an iterator that merges elements of the contained iterators.

`IntoIterator` enabled version of [`Itertools::kmerge_by`](crate::Itertools::kmerge_by).

### `process_results`

```rust
fn process_results<I, F, T, E, R>(iterable: I, processor: F) -> Result<R, E>
where
    I: IntoIterator<Item = Result<T, E>>,
    F: FnOnce(ProcessResults<'_, <I as >::IntoIter, E>) -> R
```

*Defined in [`itertools-0.14.0/src/process_results_impl.rs:94-108`](../../.source_1765894658/itertools-0.14.0/src/process_results_impl.rs#L94-L108)*

“Lift” a function of the values of an iterator so that it can process
an iterator of `Result` values instead.

`IntoIterator` enabled version of `Itertools::process_results`.

### `repeat_n`

```rust
fn repeat_n<A>(element: A, n: usize) -> RepeatN<A>
where
    A: Clone
```

*Defined in [`itertools-0.14.0/src/repeatn.rs:14-26`](../../.source_1765894658/itertools-0.14.0/src/repeatn.rs#L14-L26)*

Create an iterator that produces `n` repetitions of `element`.

### `iterate`

```rust
fn iterate<St, F>(initial_value: St, f: F) -> Iterate<St, F>
where
    F: FnMut(&St) -> St
```

*Defined in [`itertools-0.14.0/src/sources.rs:145-153`](../../.source_1765894658/itertools-0.14.0/src/sources.rs#L145-L153)*

Creates a new iterator that infinitely applies function to value and yields results.

```rust
use itertools::iterate;

itertools::assert_equal(iterate(1, |i| i % 3 + 1).take(5), vec![1, 2, 3, 1, 2]);
```

**Panics** if compute the next value does.

```should_panic
use itertools::iterate;
let mut it = iterate(25u32, |x| x - 10).take_while(|&x| x > 10);
assert_eq!(it.next(), Some(25)); // `Iterate` holds 15.
assert_eq!(it.next(), Some(15)); // `Iterate` holds 5.
it.next(); // `5 - 10` overflows.
```

You can alternatively use `core::iter::successors` as it better describes a finite iterator.

### `unfold`

```rust
fn unfold<A, St, F>(initial_state: St, f: F) -> Unfold<St, F>
where
    F: FnMut(&mut St) -> Option<A>
```

*Defined in [`itertools-0.14.0/src/sources.rs:48-56`](../../.source_1765894658/itertools-0.14.0/src/sources.rs#L48-L56)*

Creates a new unfold source with the specified closure as the "iterator
function" and an initial state to eventually pass to the closure

`unfold` is a general iterator builder: it has a mutable state value,
and a closure with access to the state that produces the next value.

This more or less equivalent to a regular struct with an `Iterator`
implementation, and is useful for one-off iterators.

```rust
// an iterator that yields sequential Fibonacci numbers,
// and stops at the maximum representable value.

use itertools::unfold;

let mut fibonacci = unfold((1u32, 1u32), |(x1, x2)| {
    // Attempt to get the next Fibonacci number
    let next = x1.saturating_add(*x2);

    // Shift left: ret <- x1 <- x2 <- next
    let ret = *x1;
    *x1 = *x2;
    *x2 = next;

    // If addition has saturated at the maximum, we are finished
    if ret == *x1 && ret > 1 {
        None
    } else {
        Some(ret)
    }
});

itertools::assert_equal(fibonacci.by_ref().take(8),
                        vec![1, 1, 2, 3, 5, 8, 13, 21]);
assert_eq!(fibonacci.last(), Some(2_971_215_073))
```

### `multiunzip`

```rust
fn multiunzip<FromI, I>(i: I) -> FromI
where
    I: IntoIterator,
    <I as >::IntoIter: MultiUnzip<FromI>
```

*Defined in [`itertools-0.14.0/src/unziptuple.rs:21-27`](../../.source_1765894658/itertools-0.14.0/src/unziptuple.rs#L21-L27)*

Converts an iterator of tuples into a tuple of containers.

`multiunzip()` consumes an entire iterator of n-ary tuples, producing `n` collections, one for each
column.

This function is, in some sense, the opposite of [`multizip`](ziptuple/index.md).

```rust
use itertools::multiunzip;

let inputs = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)];

let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(inputs);

assert_eq!(a, vec![1, 4, 7]);
assert_eq!(b, vec![2, 5, 8]);
assert_eq!(c, vec![3, 6, 9]);
```


### `multizip`

```rust
fn multizip<T, U>(t: U) -> Zip<T>
where
    Zip<T>: From<U> + Iterator
```

*Defined in [`itertools-0.14.0/src/ziptuple.rs:39-44`](../../.source_1765894658/itertools-0.14.0/src/ziptuple.rs#L39-L44)*

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

### `equal`

```rust
fn equal<I, J>(a: I, b: J) -> bool
where
    I: IntoIterator,
    J: IntoIterator,
    <I as >::Item: PartialEq<<J as >::Item>
```

*Defined in [`itertools-0.14.0/src/lib.rs:4598-4605`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L4598-L4605)*

Return `true` if both iterables produce equal sequences
(elements pairwise equal and sequences of the same length),
`false` otherwise.

`IntoIterator` enabled version of `Iterator::eq`.

```rust
assert!(itertools::equal(vec![1, 2, 3], 1..4));
assert!(!itertools::equal(&[0, 0], &[0, 0, 0]));
```

### `assert_equal`

```rust
fn assert_equal<I, J>(a: I, b: J)
where
    I: IntoIterator,
    J: IntoIterator,
    <I as >::Item: fmt::Debug + PartialEq<<J as >::Item>,
    <J as >::Item: fmt::Debug
```

*Defined in [`itertools-0.14.0/src/lib.rs:4619-4648`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L4619-L4648)*

Assert that two iterables produce equal sequences, with the same
semantics as [`equal(a, b)`](equal).

**Panics** on assertion failure with a message that shows the
two different elements and the iteration index.

```should_panic
use itertools::assert_equal;
assert_equal("exceed".split('c'), "excess".split('c'));
// ^PANIC: panicked at 'Failed assertion Some("eed") == Some("ess") for iteration 1'.
```

### `partition`

```rust
fn partition<'a, A: 'a, I, F>(iter: I, pred: F) -> usize
where
    I: IntoIterator<Item = &'a mut A>,
    <I as >::IntoIter: DoubleEndedIterator,
    F: FnMut(&A) -> bool
```

*Defined in [`itertools-0.14.0/src/lib.rs:4667-4685`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L4667-L4685)*

Partition a sequence using predicate `pred` so that elements
that map to `true` are placed before elements which map to `false`.

The order within the partitions is arbitrary.

Return the index of the split point.

```rust
use itertools::partition;

// use repeated numbers to not promise any ordering
let mut data = [7, 1, 1, 7, 1, 1, 7];
let split_index = partition(&mut data, |elt| *elt >= 3);

assert_eq!(data, [7, 7, 7, 1, 1, 1, 1]);
assert_eq!(split_index, 3);
```

## Type Aliases

### `VecDequeIntoIter<T>`

```rust
type VecDequeIntoIter<T> = alloc::collections::vec_deque::IntoIter<T>;
```

*Defined in [`itertools-0.14.0/src/lib.rs:78`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L78)*

### `VecIntoIter<T>`

```rust
type VecIntoIter<T> = alloc::vec::IntoIter<T>;
```

*Defined in [`itertools-0.14.0/src/lib.rs:80`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L80)*

## Macros

### `iproduct!`

*Defined in [`itertools-0.14.0/src/lib.rs:259-284`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L259-L284)*

Create an iterator over the “cartesian product” of iterators.

Iterator element type is like `(A, B, ..., E)` if formed
from iterators `(I, J, ..., M)` with element types `I::Item = A`, `J::Item = B`, etc.

```rust
use itertools::iproduct;

fn main() {
// Iterate over the coordinates of a 4 x 4 x 4 grid
// from (0, 0, 0), (0, 0, 1), .., (0, 1, 0), (0, 1, 1), .. etc until (3, 3, 3)
for (i, j, k) in iproduct!(0..4, 0..4, 0..4) {
   // ..
   let _ = (i, j, k);
}
}
```

### `izip!`

*Defined in [`itertools-0.14.0/src/lib.rs:321-360`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L321-L360)*

Create an iterator running multiple iterators in lockstep.

The `izip!` iterator yields elements until any subiterator
returns `None`.

This is a version of the standard ``.zip()`` that's supporting more than
two iterators. The iterator element type is a tuple with one element
from each of the input iterators. Just like ``.zip()``, the iteration stops
when the shortest of the inputs reaches its end.

**Note:** The result of this macro is in the general case an iterator
composed of repeated `.zip()` and a `.map()`; it has an anonymous type.
The special cases of one and two arguments produce the equivalent of
`$a.into_iter()` and `$a.into_iter().zip($b)` respectively.

Prefer this macro `izip!()` over [`multizip`](ziptuple/index.md) for the performance benefits
of using the standard library `.zip()`.

```rust
use itertools::izip;

fn main() {

// iterate over three sequences side-by-side
let mut results = [0, 0, 0, 0];
let inputs = [3, 7, 9, 6];

for (r, index, input) in izip!(&mut results, 0..10, &inputs) {
    *r = index * 10 + input;
}

assert_eq!(results, [0 + 3, 10 + 7, 29, 36]);
}
```

### `chain!`

*Defined in [`itertools-0.14.0/src/lib.rs:407-423`](../../.source_1765894658/itertools-0.14.0/src/lib.rs#L407-L423)*

[Chain][`chain`](#chain) zero or more iterators together into one sequence.

The comma-separated arguments must implement `IntoIterator`.
The final argument may be followed by a trailing comma.

# Examples

Empty invocations of `chain!` expand to an invocation of `std::iter::empty`:
```rust
use std::iter;
use itertools::chain;

let _: iter::Empty<()> = chain!();
let _: iter::Empty<i8> = chain!();
```

Invocations of `chain!` with one argument expand to [`arg.into_iter()`](IntoIterator):
```rust
use std::ops::Range;
use itertools::chain;
let _: <Range<_> as IntoIterator>::IntoIter = chain!(2..6,); // trailing comma optional!
let _:     <&[_] as IntoIterator>::IntoIter = chain!(&[2, 3, 4]);
```

Invocations of `chain!` with multiple arguments [`.into_iter()`](IntoIterator) each
argument, and then [`chain`](#chain) them together:
```rust
use std::{iter::*, slice};
use itertools::{assert_equal, chain};

// e.g., this:
let with_macro:  Chain<Chain<Once<_>, Take<Repeat<_>>>, slice::Iter<_>> =
    chain![once(&0), repeat(&1).take(2), &[2, 3, 5],];

// ...is equivalent to this:
let with_method: Chain<Chain<Once<_>, Take<Repeat<_>>>, slice::Iter<_>> =
    once(&0)
        .chain(repeat(&1).take(2))
        .chain(&[2, 3, 5]);

assert_equal(with_macro, with_method);
```

