*[itertools](../index.md) / [merge_join](index.md)*

---

# Module `merge_join`

## Contents

- [Structs](#structs)
  - [`MergeLte`](#mergelte)
  - [`MergeBy`](#mergeby)
  - [`MergeFuncLR`](#mergefunclr)
- [Traits](#traits)
  - [`FuncLR`](#funclr)
  - [`OrderingOrBool`](#orderingorbool)
- [Functions](#functions)
  - [`merge`](#merge)
  - [`merge_by_new`](#merge-by-new)
  - [`merge_join_by`](#merge-join-by)
- [Type Aliases](#type-aliases)
  - [`Merge`](#merge)
  - [`MergeJoinBy`](#mergejoinby)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MergeLte`](#mergelte) | struct |  |
| [`MergeBy`](#mergeby) | struct | An iterator adaptor that merges the two base iterators in ascending order. |
| [`MergeFuncLR`](#mergefunclr) | struct |  |
| [`FuncLR`](#funclr) | trait |  |
| [`OrderingOrBool`](#orderingorbool) | trait |  |
| [`merge`](#merge) | fn | Create an iterator that merges elements in `i` and `j`. |
| [`merge_by_new`](#merge-by-new) | fn | Create a `MergeBy` iterator. |
| [`merge_join_by`](#merge-join-by) | fn | Return an iterator adaptor that merge-joins items from the two base iterators in ascending order. |
| [`Merge`](#merge) | type | An iterator adaptor that merges the two base iterators in ascending order. |
| [`MergeJoinBy`](#mergejoinby) | type | An iterator adaptor that merge-joins items from the two base iterators in ascending order. |

## Structs

### `MergeLte`

```rust
struct MergeLte;
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:15`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L15)*

#### Trait Implementations

##### `impl Any for MergeLte`

- <span id="mergelte-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MergeLte`

- <span id="mergelte-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MergeLte`

- <span id="mergelte-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MergeLte`

- <span id="mergelte-clone"></span>`fn clone(&self) -> MergeLte` — [`MergeLte`](#mergelte)

##### `impl CloneToUninit for MergeLte`

- <span id="mergelte-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MergeLte`

- <span id="mergelte-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MergeLte`

- <span id="mergelte-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MergeLte`

- <span id="mergelte-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MergeLte`

##### `impl<T: PartialOrd> OrderingOrBool for MergeLte`

- <span id="mergelte-orderingorbool-type-mergeresult"></span>`type MergeResult = T`

- <span id="mergelte-orderingorbool-left"></span>`fn left(left: T) -> <Self as >::MergeResult` — [`OrderingOrBool`](#orderingorbool)

- <span id="mergelte-orderingorbool-right"></span>`fn right(right: T) -> <Self as >::MergeResult` — [`OrderingOrBool`](#orderingorbool)

- <span id="mergelte-orderingorbool-merge"></span>`fn merge(&mut self, left: T, right: T) -> (Option<Either<T, T>>, <Self as >::MergeResult)` — [`Either`](../index.md#either), [`OrderingOrBool`](#orderingorbool)

- <span id="mergelte-orderingorbool-size-hint"></span>`fn size_hint(left: (usize, Option<usize>), right: (usize, Option<usize>)) -> (usize, Option<usize>)`

##### `impl ToOwned for MergeLte`

- <span id="mergelte-toowned-type-owned"></span>`type Owned = T`

- <span id="mergelte-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mergelte-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MergeLte`

- <span id="mergelte-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mergelte-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MergeLte`

- <span id="mergelte-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mergelte-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MergeBy<I: Iterator, J: Iterator, F>`

```rust
struct MergeBy<I: Iterator, J: Iterator, F> {
    left: super::adaptors::PutBack<std::iter::Fuse<I>>,
    right: super::adaptors::PutBack<std::iter::Fuse<J>>,
    cmp_fn: F,
}
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:56-60`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L56-L60)*

An iterator adaptor that merges the two base iterators in ascending order.
If both base iterators are sorted (ascending), the result is sorted.

Iterator element type is `I::Item`.

See [`.merge_by()`](crate::Itertools::merge_by) for more information.

#### Trait Implementations

##### `impl Any for MergeBy<I, J, F>`

- <span id="mergeby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MergeBy<I, J, F>`

- <span id="mergeby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MergeBy<I, J, F>`

- <span id="mergeby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, J, F> Clone for MergeBy<I, J, F>`

- <span id="mergeby-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for MergeBy<I, J, F>`

- <span id="mergeby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, J, F> Debug for MergeBy<I, J, F>`

- <span id="mergeby-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for MergeBy<I, J, F>`

- <span id="mergeby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J, F> FusedIterator for MergeBy<I, J, F>`

##### `impl<U> Into for MergeBy<I, J, F>`

- <span id="mergeby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MergeBy<I, J, F>`

##### `impl<I> IntoIterator for MergeBy<I, J, F>`

- <span id="mergeby-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="mergeby-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="mergeby-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, J, F> Iterator for MergeBy<I, J, F>`

- <span id="mergeby-iterator-type-item"></span>`type Item = <F as OrderingOrBool>::MergeResult`

- <span id="mergeby-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="mergeby-iterator-fold"></span>`fn fold<B, G>(self, init: B, f: G) -> B`

- <span id="mergeby-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="mergeby-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl Itertools for MergeBy<I, J, F>`

##### `impl MultiUnzip for MergeBy<I, J, F>`

- <span id="mergeby-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for MergeBy<I, J, F>`

- <span id="mergeby-toowned-type-owned"></span>`type Owned = T`

- <span id="mergeby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mergeby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MergeBy<I, J, F>`

- <span id="mergeby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mergeby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MergeBy<I, J, F>`

- <span id="mergeby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mergeby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MergeFuncLR<F, T>`

```rust
struct MergeFuncLR<F, T>(F, std::marker::PhantomData<T>);
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:102`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L102)*

#### Trait Implementations

##### `impl<T> Any for MergeFuncLR<F, T>`

- <span id="mergefunclr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MergeFuncLR<F, T>`

- <span id="mergefunclr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MergeFuncLR<F, T>`

- <span id="mergefunclr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: clone::Clone, T: clone::Clone> Clone for MergeFuncLR<F, T>`

- <span id="mergefunclr-clone"></span>`fn clone(&self) -> MergeFuncLR<F, T>` — [`MergeFuncLR`](#mergefunclr)

##### `impl<T> CloneToUninit for MergeFuncLR<F, T>`

- <span id="mergefunclr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<F: fmt::Debug, T: fmt::Debug> Debug for MergeFuncLR<F, T>`

- <span id="mergefunclr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MergeFuncLR<F, T>`

- <span id="mergefunclr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for MergeFuncLR<F, T>`

- <span id="mergefunclr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for MergeFuncLR<F, T>`

##### `impl<L, R, F: FnMut(&L, &R) -> std::cmp::Ordering> OrderingOrBool for MergeFuncLR<F, std::cmp::Ordering>`

- <span id="mergefunclr-orderingorbool-type-mergeresult"></span>`type MergeResult = EitherOrBoth<L, R>`

- <span id="mergefunclr-orderingorbool-left"></span>`fn left(left: L) -> <Self as >::MergeResult` — [`OrderingOrBool`](#orderingorbool)

- <span id="mergefunclr-orderingorbool-right"></span>`fn right(right: R) -> <Self as >::MergeResult` — [`OrderingOrBool`](#orderingorbool)

- <span id="mergefunclr-orderingorbool-merge"></span>`fn merge(&mut self, left: L, right: R) -> (Option<Either<L, R>>, <Self as >::MergeResult)` — [`Either`](../index.md#either), [`OrderingOrBool`](#orderingorbool)

- <span id="mergefunclr-orderingorbool-size-hint"></span>`fn size_hint(left: (usize, Option<usize>), right: (usize, Option<usize>)) -> (usize, Option<usize>)`

##### `impl<T> ToOwned for MergeFuncLR<F, T>`

- <span id="mergefunclr-toowned-type-owned"></span>`type Owned = T`

- <span id="mergefunclr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="mergefunclr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for MergeFuncLR<F, T>`

- <span id="mergefunclr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mergefunclr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for MergeFuncLR<F, T>`

- <span id="mergefunclr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mergefunclr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `FuncLR<L, R>`

```rust
trait FuncLR<L, R> { ... }
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:104-106`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L104-L106)*

#### Associated Types

- `type T`

#### Implementors

- `F`

### `OrderingOrBool<L, R>`

```rust
trait OrderingOrBool<L, R> { ... }
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:112-121`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L112-L121)*

#### Associated Types

- `type MergeResult`

#### Required Methods

- `fn left(left: L) -> <Self as >::MergeResult`

- `fn right(right: R) -> <Self as >::MergeResult`

- `fn merge(&mut self, left: L, right: R) -> (Option<Either<L, R>>, <Self as >::MergeResult)`

- `fn size_hint(left: (usize, Option<usize>), right: (usize, Option<usize>)) -> (usize, Option<usize>)`

#### Implementors

- [`MergeFuncLR`](#mergefunclr)
- [`MergeLte`](#mergelte)
- `F`

## Functions

### `merge`

```rust
fn merge<I, J>(i: I, j: J) -> Merge<<I as IntoIterator>::IntoIter, <J as IntoIterator>::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator<Item = <I as >::Item>,
    <I as >::Item: PartialOrd
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:37-47`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L37-L47)*

Create an iterator that merges elements in `i` and `j`.

`IntoIterator` enabled version of [`Itertools::merge`](crate::Itertools::merge).

```rust
use itertools::merge;

for elt in merge(&[1, 2, 3], &[2, 3, 4]) {
    /* loop body */
    let _ = elt;
}
```

### `merge_by_new`

```rust
fn merge_by_new<I, J, F>(a: I, b: J, cmp: F) -> MergeBy<<I as >::IntoIter, <J as >::IntoIter, F>
where
    I: IntoIterator,
    J: IntoIterator<Item = <I as >::Item>
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:63-73`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L63-L73)*

Create a `MergeBy` iterator.

### `merge_join_by`

```rust
fn merge_join_by<I, J, F, T>(left: I, right: J, cmp_fn: F) -> MergeJoinBy<<I as >::IntoIter, <J as >::IntoIter, F>
where
    I: IntoIterator,
    J: IntoIterator,
    F: FnMut(&<I as >::Item, &<J as >::Item) -> T
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:78-93`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L78-L93)*

Return an iterator adaptor that merge-joins items from the two base iterators in ascending order.

`IntoIterator` enabled version of `Itertools::merge_join_by`.

## Type Aliases

### `Merge<I, J>`

```rust
type Merge<I, J> = MergeBy<I, J, MergeLte>;
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:23`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L23)*

An iterator adaptor that merges the two base iterators in ascending order.
If both base iterators are sorted (ascending), the result is sorted.

Iterator element type is `I::Item`.

See [`.merge()`](crate::Itertools::merge_by) for more information.

### `MergeJoinBy<I, J, F>`

```rust
type MergeJoinBy<I, J, F> = MergeBy<I, J, MergeFuncLR<F, <F as FuncLR>::T>>;
```

*Defined in [`itertools-0.14.0/src/merge_join.rs:98-99`](../../../.source_1765894658/itertools-0.14.0/src/merge_join.rs#L98-L99)*

An iterator adaptor that merge-joins items from the two base iterators in ascending order.

See [`.merge_join_by()`](crate::Itertools::merge_join_by) for more information.

