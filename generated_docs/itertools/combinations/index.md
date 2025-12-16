*[itertools](../index.md) / [combinations](index.md)*

---

# Module `combinations`

## Contents

- [Structs](#structs)
  - [`CombinationsGeneric`](#combinationsgeneric)
- [Traits](#traits)
  - [`PoolIndex`](#poolindex)
- [Functions](#functions)
  - [`combinations`](#combinations)
  - [`array_combinations`](#array-combinations)
  - [`remaining_for`](#remaining-for)
- [Type Aliases](#type-aliases)
  - [`Combinations`](#combinations)
  - [`ArrayCombinations`](#arraycombinations)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CombinationsGeneric`](#combinationsgeneric) | struct | An iterator to iterate through all the `k`-length combinations in an iterator. |
| [`PoolIndex`](#poolindex) | trait | A type holding indices of elements in a pool or buffer of items from an inner iterator and used to pick out different combinations in a generic way. |
| [`combinations`](#combinations) | fn | Create a new `Combinations` from a clonable iterator. |
| [`array_combinations`](#array-combinations) | fn | Create a new `ArrayCombinations` from a clonable iterator. |
| [`remaining_for`](#remaining-for) | fn | For a given size `n`, return the count of remaining combinations or None if it would overflow. |
| [`Combinations`](#combinations) | type | Iterator for `Vec` valued combinations returned by [`.combinations()`](crate::Itertools::combinations) |
| [`ArrayCombinations`](#arraycombinations) | type | Iterator for const generic combinations returned by [`.array_combinations()`](crate::Itertools::array_combinations) |

## Structs

### `CombinationsGeneric<I: Iterator, Idx>`

```rust
struct CombinationsGeneric<I: Iterator, Idx> {
    indices: Idx,
    pool: super::lazy_buffer::LazyBuffer<I>,
    first: bool,
}
```

*Defined in [`itertools-0.14.0/src/combinations.rs:36-40`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L36-L40)*

An iterator to iterate through all the `k`-length combinations in an iterator.

See [`.combinations()`](crate::Itertools::combinations) and [`.array_combinations()`](crate::Itertools::array_combinations) for more information.

#### Implementations

- <span id="combinationsgeneric-new"></span>`fn new(iter: I, indices: Idx) -> Self`

  Constructor with arguments the inner iterator and the initial state for the indices.

- <span id="combinationsgeneric-k"></span>`fn k(&self) -> usize`

  Returns the length of a combination produced by this iterator.

- <span id="combinationsgeneric-n"></span>`fn n(&self) -> usize`

  Returns the (current) length of the pool from which combination elements are
  selected. This value can change between invocations of [`next`](Combinations::next).

- <span id="combinationsgeneric-src"></span>`fn src(&self) -> &LazyBuffer<I>` — [`LazyBuffer`](../lazy_buffer/index.md#lazybuffer)

  Returns a reference to the source pool.

- <span id="combinationsgeneric-n-and-count"></span>`fn n_and_count(self) -> (usize, usize)`

  Return the length of the inner iterator and the count of remaining combinations.

- <span id="combinationsgeneric-init"></span>`fn init(&mut self) -> bool`

  Initialises the iterator by filling a buffer with elements from the
  iterator. Returns true if there are no combinations, false otherwise.

- <span id="combinationsgeneric-increment-indices"></span>`fn increment_indices(&mut self) -> bool`

  Increments indices representing the combination to advance to the next
  (in lexicographic order by increasing sequence) combination. For example
  if we have n=4 & k=2 then `[0, 1] -> [0, 2] -> [0, 3] -> [1, 2] -> ...`
  
  Returns true if we've run out of combinations, false otherwise.

- <span id="combinationsgeneric-try-nth"></span>`fn try_nth(&mut self, n: usize) -> Result<<Self as Iterator>::Item, usize>`

  Returns the n-th item or the number of successful steps.

#### Trait Implementations

##### `impl Any for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, Idx> Clone for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, Idx> Debug for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, Idx> FusedIterator for CombinationsGeneric<I, Idx>`

##### `impl<U> Into for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CombinationsGeneric<I, Idx>`

##### `impl<I> IntoIterator for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="combinationsgeneric-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="combinationsgeneric-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, Idx> Iterator for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-iterator-type-item"></span>`type Item = <Idx as PoolIndex>::Item`

- <span id="combinationsgeneric-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="combinationsgeneric-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="combinationsgeneric-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="combinationsgeneric-iterator-count"></span>`fn count(self) -> usize`

##### `impl Itertools for CombinationsGeneric<I, Idx>`

##### `impl MultiUnzip for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-toowned-type-owned"></span>`type Owned = T`

- <span id="combinationsgeneric-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="combinationsgeneric-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="combinationsgeneric-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CombinationsGeneric<I, Idx>`

- <span id="combinationsgeneric-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="combinationsgeneric-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `PoolIndex<T>`

```rust
trait PoolIndex<T>: BorrowMut<[usize]> { ... }
```

*Defined in [`itertools-0.14.0/src/combinations.rs:44-54`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L44-L54)*

A type holding indices of elements in a pool or buffer of items from an inner iterator
and used to pick out different combinations in a generic way.

#### Associated Types

- `type Item`

#### Required Methods

- `fn extract_item<I: Iterator<Item = T>>(&self, pool: &LazyBuffer<I>) -> <Self as >::Item`

#### Provided Methods

- `fn len(&self) -> usize`

#### Implementors

- `[usize; K]`
- `alloc::vec::Vec<usize>`

## Functions

### `combinations`

```rust
fn combinations<I: Iterator>(iter: I, k: usize) -> Combinations<I>
where
    <I as >::Item: Clone
```

*Defined in [`itertools-0.14.0/src/combinations.rs:17-22`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L17-L22)*

Create a new `Combinations` from a clonable iterator.

### `array_combinations`

```rust
fn array_combinations<I: Iterator, const K: usize>(iter: I) -> ArrayCombinations<I, K>
where
    <I as >::Item: Clone
```

*Defined in [`itertools-0.14.0/src/combinations.rs:25-30`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L25-L30)*

Create a new `ArrayCombinations` from a clonable iterator.

### `remaining_for`

```rust
fn remaining_for(n: usize, first: bool, indices: &[usize]) -> Option<usize>
```

*Defined in [`itertools-0.14.0/src/combinations.rs:279-308`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L279-L308)*

For a given size `n`, return the count of remaining combinations or None if it would overflow.

## Type Aliases

### `Combinations<I>`

```rust
type Combinations<I> = CombinationsGeneric<I, alloc::vec::Vec<usize>>;
```

*Defined in [`itertools-0.14.0/src/combinations.rs:12`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L12)*

Iterator for `Vec` valued combinations returned by [`.combinations()`](crate::Itertools::combinations)

### `ArrayCombinations<I, const K: usize>`

```rust
type ArrayCombinations<I, const K: usize> = CombinationsGeneric<I, [usize; K]>;
```

*Defined in [`itertools-0.14.0/src/combinations.rs:14`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L14)*

Iterator for const generic combinations returned by [`.array_combinations()`](crate::Itertools::array_combinations)

