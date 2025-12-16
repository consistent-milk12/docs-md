*[itertools](../index.md) / [structs](index.md)*

---

# Module `structs`

The concrete iterator types.

## Contents

- [Structs](#structs)
  - [`MultiProduct`](#multiproduct)
  - [`Batching`](#batching)
  - [`FilterMapOk`](#filtermapok)
  - [`FilterOk`](#filterok)
  - [`Interleave`](#interleave)
  - [`InterleaveShortest`](#interleaveshortest)
  - [`Positions`](#positions)
  - [`Product`](#product)
  - [`PutBack`](#putback)
  - [`TakeWhileRef`](#takewhileref)
  - [`TupleCombinations`](#tuplecombinations)
  - [`Update`](#update)
  - [`WhileSome`](#whilesome)
  - [`CombinationsWithReplacement`](#combinationswithreplacement)
  - [`ExactlyOneError`](#exactlyoneerror)
  - [`FlattenOk`](#flattenok)
  - [`Format`](#format)
  - [`FormatWith`](#formatwith)
  - [`Chunk`](#chunk)
  - [`ChunkBy`](#chunkby)
  - [`Chunks`](#chunks)
  - [`Group`](#group)
  - [`Groups`](#groups)
  - [`IntoChunks`](#intochunks)
  - [`GroupingMap`](#groupingmap)
  - [`IntersperseWith`](#interspersewith)
  - [`KMergeBy`](#kmergeby)
  - [`MergeBy`](#mergeby)
  - [`MultiPeek`](#multipeek)
  - [`PadUsing`](#padusing)
  - [`PeekNth`](#peeknth)
  - [`PeekingTakeWhile`](#peekingtakewhile)
  - [`Permutations`](#permutations)
  - [`Powerset`](#powerset)
  - [`ProcessResults`](#processresults)
  - [`PutBackN`](#putbackn)
  - [`RcIter`](#rciter)
  - [`RepeatN`](#repeatn)
  - [`Iterate`](#iterate)
  - [`Unfold`](#unfold)
  - [`TakeWhileInclusive`](#takewhileinclusive)
  - [`Tee`](#tee)
  - [`CircularTupleWindows`](#circulartuplewindows)
  - [`TupleBuffer`](#tuplebuffer)
  - [`TupleWindows`](#tuplewindows)
  - [`Tuples`](#tuples)
  - [`Unique`](#unique)
  - [`UniqueBy`](#uniqueby)
  - [`WithPosition`](#withposition)
  - [`ZipEq`](#zipeq)
  - [`ZipLongest`](#ziplongest)
  - [`Zip`](#zip)
- [Type Aliases](#type-aliases)
  - [`Coalesce`](#coalesce)
  - [`Dedup`](#dedup)
  - [`DedupBy`](#dedupby)
  - [`DedupByWithCount`](#dedupbywithcount)
  - [`DedupWithCount`](#dedupwithcount)
  - [`MapInto`](#mapinto)
  - [`MapOk`](#mapok)
  - [`ArrayCombinations`](#arraycombinations)
  - [`Combinations`](#combinations)
  - [`ConsTuples`](#constuples)
  - [`Duplicates`](#duplicates)
  - [`DuplicatesBy`](#duplicatesby)
  - [`GroupBy`](#groupby)
  - [`GroupingMapBy`](#groupingmapby)
  - [`Intersperse`](#intersperse)
  - [`KMerge`](#kmerge)
  - [`Merge`](#merge)
  - [`MergeJoinBy`](#mergejoinby)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiProduct`](#multiproduct) | struct |  |
| [`Batching`](#batching) | struct |  |
| [`FilterMapOk`](#filtermapok) | struct |  |
| [`FilterOk`](#filterok) | struct |  |
| [`Interleave`](#interleave) | struct |  |
| [`InterleaveShortest`](#interleaveshortest) | struct |  |
| [`Positions`](#positions) | struct |  |
| [`Product`](#product) | struct |  |
| [`PutBack`](#putback) | struct |  |
| [`TakeWhileRef`](#takewhileref) | struct |  |
| [`TupleCombinations`](#tuplecombinations) | struct |  |
| [`Update`](#update) | struct |  |
| [`WhileSome`](#whilesome) | struct |  |
| [`CombinationsWithReplacement`](#combinationswithreplacement) | struct |  |
| [`ExactlyOneError`](#exactlyoneerror) | struct |  |
| [`FlattenOk`](#flattenok) | struct |  |
| [`Format`](#format) | struct |  |
| [`FormatWith`](#formatwith) | struct |  |
| [`Chunk`](#chunk) | struct |  |
| [`ChunkBy`](#chunkby) | struct |  |
| [`Chunks`](#chunks) | struct |  |
| [`Group`](#group) | struct |  |
| [`Groups`](#groups) | struct |  |
| [`IntoChunks`](#intochunks) | struct |  |
| [`GroupingMap`](#groupingmap) | struct |  |
| [`IntersperseWith`](#interspersewith) | struct |  |
| [`KMergeBy`](#kmergeby) | struct |  |
| [`MergeBy`](#mergeby) | struct |  |
| [`MultiPeek`](#multipeek) | struct |  |
| [`PadUsing`](#padusing) | struct |  |
| [`PeekNth`](#peeknth) | struct |  |
| [`PeekingTakeWhile`](#peekingtakewhile) | struct |  |
| [`Permutations`](#permutations) | struct |  |
| [`Powerset`](#powerset) | struct |  |
| [`ProcessResults`](#processresults) | struct |  |
| [`PutBackN`](#putbackn) | struct |  |
| [`RcIter`](#rciter) | struct |  |
| [`RepeatN`](#repeatn) | struct |  |
| [`Iterate`](#iterate) | struct |  |
| [`Unfold`](#unfold) | struct |  |
| [`TakeWhileInclusive`](#takewhileinclusive) | struct |  |
| [`Tee`](#tee) | struct |  |
| [`CircularTupleWindows`](#circulartuplewindows) | struct |  |
| [`TupleBuffer`](#tuplebuffer) | struct |  |
| [`TupleWindows`](#tuplewindows) | struct |  |
| [`Tuples`](#tuples) | struct |  |
| [`Unique`](#unique) | struct |  |
| [`UniqueBy`](#uniqueby) | struct |  |
| [`WithPosition`](#withposition) | struct |  |
| [`ZipEq`](#zipeq) | struct |  |
| [`ZipLongest`](#ziplongest) | struct |  |
| [`Zip`](#zip) | struct |  |
| [`Coalesce`](#coalesce) | type |  |
| [`Dedup`](#dedup) | type |  |
| [`DedupBy`](#dedupby) | type |  |
| [`DedupByWithCount`](#dedupbywithcount) | type |  |
| [`DedupWithCount`](#dedupwithcount) | type |  |
| [`MapInto`](#mapinto) | type |  |
| [`MapOk`](#mapok) | type |  |
| [`ArrayCombinations`](#arraycombinations) | type |  |
| [`Combinations`](#combinations) | type |  |
| [`ConsTuples`](#constuples) | type |  |
| [`Duplicates`](#duplicates) | type |  |
| [`DuplicatesBy`](#duplicatesby) | type |  |
| [`GroupBy`](#groupby) | type |  |
| [`GroupingMapBy`](#groupingmapby) | type |  |
| [`Intersperse`](#intersperse) | type |  |
| [`KMerge`](#kmerge) | type |  |
| [`Merge`](#merge) | type |  |
| [`MergeJoinBy`](#mergejoinby) | type |  |

## Structs

### `MultiProduct<I>`

```rust
struct MultiProduct<I>(Option<MultiProductInner<I>>)
where
    I: Iterator + Clone,
    <I as >::Item: Clone;
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:18-21`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/multi_product.rs#L18-L21)*

An iterator adaptor that iterates over the cartesian product of
multiple iterators of type `I`.

An iterator element type is `Vec<I::Item>`.

See [`.multi_cartesian_product()`](crate::Itertools::multi_cartesian_product)
for more information.

#### Trait Implementations

##### `impl Any for MultiProduct<I>`

- <span id="multiproduct-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProduct<I>`

- <span id="multiproduct-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProduct<I>`

- <span id="multiproduct-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiProduct<I>`

- <span id="multiproduct-clone"></span>`fn clone(&self) -> MultiProduct<I>` — [`MultiProduct`](../adaptors/index.md#multiproduct)

##### `impl CloneToUninit for MultiProduct<I>`

- <span id="multiproduct-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiProduct<I>`

- <span id="multiproduct-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for MultiProduct<I>`

- <span id="multiproduct-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for MultiProduct<I>`

##### `impl<U> Into for MultiProduct<I>`

- <span id="multiproduct-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiProduct<I>`

##### `impl<I> IntoIterator for MultiProduct<I>`

- <span id="multiproduct-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="multiproduct-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="multiproduct-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for MultiProduct<I>`

- <span id="multiproduct-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="multiproduct-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="multiproduct-iterator-count"></span>`fn count(self) -> usize`

- <span id="multiproduct-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="multiproduct-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

##### `impl Itertools for MultiProduct<I>`

##### `impl ToOwned for MultiProduct<I>`

- <span id="multiproduct-toowned-type-owned"></span>`type Owned = T`

- <span id="multiproduct-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiproduct-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProduct<I>`

- <span id="multiproduct-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiproduct-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProduct<I>`

- <span id="multiproduct-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiproduct-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Batching<I, F>`

```rust
struct Batching<I, F> {
    f: F,
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:479-482`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L479-L482)*

A “meta iterator adaptor”. Its closure receives a reference to the iterator
and may pick off as many elements as it likes, to produce the next iterator element.

Iterator element type is `X` if the return type of `F` is `Option<X>`.

See [`.batching()`](crate::Itertools::batching) for more information.

#### Trait Implementations

##### `impl Any for Batching<I, F>`

- <span id="batching-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Batching<I, F>`

- <span id="batching-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Batching<I, F>`

- <span id="batching-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Batching<I, F>`

- <span id="batching-clone"></span>`fn clone(&self) -> Batching<I, F>` — [`Batching`](../adaptors/index.md#batching)

##### `impl CloneToUninit for Batching<I, F>`

- <span id="batching-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for Batching<I, F>`

- <span id="batching-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Batching<I, F>`

- <span id="batching-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Batching<I, F>`

- <span id="batching-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Batching<I, F>`

##### `impl<I> IntoIterator for Batching<I, F>`

- <span id="batching-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="batching-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="batching-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<F, I> Iterator for Batching<I, F>`

- <span id="batching-iterator-type-item"></span>`type Item = B`

- <span id="batching-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Batching<I, F>`

##### `impl MultiUnzip for Batching<I, F>`

- <span id="batching-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Batching<I, F>`

- <span id="batching-toowned-type-owned"></span>`type Owned = T`

- <span id="batching-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="batching-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Batching<I, F>`

- <span id="batching-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="batching-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Batching<I, F>`

- <span id="batching-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="batching-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FilterMapOk<I, F>`

```rust
struct FilterMapOk<I, F> {
    iter: I,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:975-978`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L975-L978)*

An iterator adapter to filter and apply a transformation on values within a nested `Result::Ok`.

See [`.filter_map_ok()`](crate::Itertools::filter_map_ok) for more information.

#### Trait Implementations

##### `impl Any for FilterMapOk<I, F>`

- <span id="filtermapok-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterMapOk<I, F>`

- <span id="filtermapok-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterMapOk<I, F>`

- <span id="filtermapok-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FilterMapOk<I, F>`

- <span id="filtermapok-clone"></span>`fn clone(&self) -> FilterMapOk<I, F>` — [`FilterMapOk`](../adaptors/index.md#filtermapok)

##### `impl CloneToUninit for FilterMapOk<I, F>`

- <span id="filtermapok-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for FilterMapOk<I, F>`

- <span id="filtermapok-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, F> DoubleEndedIterator for FilterMapOk<I, F>`

- <span id="filtermapok-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="filtermapok-doubleendediterator-rfold"></span>`fn rfold<Acc, Fold>(self, init: Acc, fold_f: Fold) -> Acc`

##### `impl<T> From for FilterMapOk<I, F>`

- <span id="filtermapok-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for FilterMapOk<I, F>`

##### `impl<U> Into for FilterMapOk<I, F>`

- <span id="filtermapok-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterMapOk<I, F>`

##### `impl<I> IntoIterator for FilterMapOk<I, F>`

- <span id="filtermapok-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="filtermapok-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="filtermapok-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for FilterMapOk<I, F>`

- <span id="filtermapok-iterator-type-item"></span>`type Item = Result<U, E>`

- <span id="filtermapok-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="filtermapok-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="filtermapok-iterator-fold"></span>`fn fold<Acc, Fold>(self, init: Acc, fold_f: Fold) -> Acc`

- <span id="filtermapok-iterator-collect"></span>`fn collect<C>(self) -> C`

##### `impl Itertools for FilterMapOk<I, F>`

##### `impl ToOwned for FilterMapOk<I, F>`

- <span id="filtermapok-toowned-type-owned"></span>`type Owned = T`

- <span id="filtermapok-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filtermapok-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FilterMapOk<I, F>`

- <span id="filtermapok-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filtermapok-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterMapOk<I, F>`

- <span id="filtermapok-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filtermapok-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FilterOk<I, F>`

```rust
struct FilterOk<I, F> {
    iter: I,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:878-881`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L878-L881)*

An iterator adapter to filter values within a nested `Result::Ok`.

See [`.filter_ok()`](crate::Itertools::filter_ok) for more information.

#### Trait Implementations

##### `impl Any for FilterOk<I, F>`

- <span id="filterok-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FilterOk<I, F>`

- <span id="filterok-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FilterOk<I, F>`

- <span id="filterok-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for FilterOk<I, F>`

- <span id="filterok-clone"></span>`fn clone(&self) -> FilterOk<I, F>` — [`FilterOk`](../adaptors/index.md#filterok)

##### `impl CloneToUninit for FilterOk<I, F>`

- <span id="filterok-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for FilterOk<I, F>`

- <span id="filterok-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, F> DoubleEndedIterator for FilterOk<I, F>`

- <span id="filterok-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="filterok-doubleendediterator-rfold"></span>`fn rfold<Acc, Fold>(self, init: Acc, fold_f: Fold) -> Acc`

##### `impl<T> From for FilterOk<I, F>`

- <span id="filterok-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for FilterOk<I, F>`

##### `impl<U> Into for FilterOk<I, F>`

- <span id="filterok-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FilterOk<I, F>`

##### `impl<I> IntoIterator for FilterOk<I, F>`

- <span id="filterok-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="filterok-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="filterok-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for FilterOk<I, F>`

- <span id="filterok-iterator-type-item"></span>`type Item = Result<T, E>`

- <span id="filterok-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="filterok-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="filterok-iterator-fold"></span>`fn fold<Acc, Fold>(self, init: Acc, fold_f: Fold) -> Acc`

- <span id="filterok-iterator-collect"></span>`fn collect<C>(self) -> C`

##### `impl Itertools for FilterOk<I, F>`

##### `impl ToOwned for FilterOk<I, F>`

- <span id="filterok-toowned-type-owned"></span>`type Owned = T`

- <span id="filterok-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="filterok-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FilterOk<I, F>`

- <span id="filterok-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="filterok-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FilterOk<I, F>`

- <span id="filterok-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="filterok-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Interleave<I, J>`

```rust
struct Interleave<I, J> {
    i: std::iter::Fuse<I>,
    j: std::iter::Fuse<J>,
    next_coming_from_j: bool,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:28-32`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L28-L32)*

An iterator adaptor that alternates elements from two iterators until both
run out.

This iterator is *fused*.

See [`.interleave()`](crate::Itertools::interleave) for more information.

#### Trait Implementations

##### `impl Any for Interleave<I, J>`

- <span id="interleave-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Interleave<I, J>`

- <span id="interleave-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Interleave<I, J>`

- <span id="interleave-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, J: clone::Clone> Clone for Interleave<I, J>`

- <span id="interleave-clone"></span>`fn clone(&self) -> Interleave<I, J>` — [`Interleave`](../adaptors/index.md#interleave)

##### `impl CloneToUninit for Interleave<I, J>`

- <span id="interleave-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug, J: fmt::Debug> Debug for Interleave<I, J>`

- <span id="interleave-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Interleave<I, J>`

- <span id="interleave-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> FusedIterator for Interleave<I, J>`

##### `impl<U> Into for Interleave<I, J>`

- <span id="interleave-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Interleave<I, J>`

##### `impl<I> IntoIterator for Interleave<I, J>`

- <span id="interleave-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleave-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="interleave-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, J> Iterator for Interleave<I, J>`

- <span id="interleave-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleave-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="interleave-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="interleave-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Interleave<I, J>`

##### `impl MultiUnzip for Interleave<I, J>`

- <span id="interleave-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Interleave<I, J>`

- <span id="interleave-toowned-type-owned"></span>`type Owned = T`

- <span id="interleave-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interleave-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Interleave<I, J>`

- <span id="interleave-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interleave-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Interleave<I, J>`

- <span id="interleave-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interleave-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InterleaveShortest<I, J>`

```rust
struct InterleaveShortest<I, J>
where
    I: Iterator,
    J: Iterator<Item = <I as >::Item> {
    i: I,
    j: J,
    next_coming_from_j: bool,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:123-131`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L123-L131)*

An iterator adaptor that alternates elements from the two iterators until
one of them runs out.

This iterator is *fused*.

See [`.interleave_shortest()`](crate::Itertools::interleave_shortest)
for more information.

#### Trait Implementations

##### `impl Any for InterleaveShortest<I, J>`

- <span id="interleaveshortest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for InterleaveShortest<I, J>`

- <span id="interleaveshortest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InterleaveShortest<I, J>`

- <span id="interleaveshortest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, J> Clone for InterleaveShortest<I, J>`

- <span id="interleaveshortest-clone"></span>`fn clone(&self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](../adaptors/index.md#interleaveshortest)

##### `impl CloneToUninit for InterleaveShortest<I, J>`

- <span id="interleaveshortest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, J> Debug for InterleaveShortest<I, J>`

- <span id="interleaveshortest-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for InterleaveShortest<I, J>`

- <span id="interleaveshortest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> FusedIterator for InterleaveShortest<I, J>`

##### `impl<U> Into for InterleaveShortest<I, J>`

- <span id="interleaveshortest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for InterleaveShortest<I, J>`

##### `impl<I> IntoIterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleaveshortest-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="interleaveshortest-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, J> Iterator for InterleaveShortest<I, J>`

- <span id="interleaveshortest-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interleaveshortest-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="interleaveshortest-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="interleaveshortest-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for InterleaveShortest<I, J>`

##### `impl MultiUnzip for InterleaveShortest<I, J>`

- <span id="interleaveshortest-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for InterleaveShortest<I, J>`

- <span id="interleaveshortest-toowned-type-owned"></span>`type Owned = T`

- <span id="interleaveshortest-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interleaveshortest-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for InterleaveShortest<I, J>`

- <span id="interleaveshortest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interleaveshortest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InterleaveShortest<I, J>`

- <span id="interleaveshortest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interleaveshortest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Positions<I, F>`

```rust
struct Positions<I, F> {
    iter: std::iter::Enumerate<I>,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:1080-1083`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L1080-L1083)*

An iterator adapter to get the positions of each element that matches a predicate.

See [`.positions()`](crate::Itertools::positions) for more information.

#### Trait Implementations

##### `impl Any for Positions<I, F>`

- <span id="positions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Positions<I, F>`

- <span id="positions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Positions<I, F>`

- <span id="positions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Positions<I, F>`

- <span id="positions-clone"></span>`fn clone(&self) -> Positions<I, F>` — [`Positions`](../adaptors/index.md#positions)

##### `impl CloneToUninit for Positions<I, F>`

- <span id="positions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for Positions<I, F>`

- <span id="positions-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, F> DoubleEndedIterator for Positions<I, F>`

- <span id="positions-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="positions-doubleendediterator-rfold"></span>`fn rfold<B, G>(self, init: B, func: G) -> B`

##### `impl<T> From for Positions<I, F>`

- <span id="positions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for Positions<I, F>`

##### `impl<U> Into for Positions<I, F>`

- <span id="positions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Positions<I, F>`

##### `impl<I> IntoIterator for Positions<I, F>`

- <span id="positions-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="positions-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="positions-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for Positions<I, F>`

- <span id="positions-iterator-type-item"></span>`type Item = usize`

- <span id="positions-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="positions-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="positions-iterator-fold"></span>`fn fold<B, G>(self, init: B, func: G) -> B`

##### `impl Itertools for Positions<I, F>`

##### `impl ToOwned for Positions<I, F>`

- <span id="positions-toowned-type-owned"></span>`type Owned = T`

- <span id="positions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="positions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Positions<I, F>`

- <span id="positions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="positions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Positions<I, F>`

- <span id="positions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="positions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Product<I, J>`

```rust
struct Product<I, J>
where
    I: Iterator {
    a: I,
    a_cur: Option<Option<<I as >::Item>>,
    b: J,
    b_orig: J,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:360-371`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L360-L371)*

An iterator adaptor that iterates over the cartesian product of
the element sets of two iterators `I` and `J`.

Iterator element type is `(I::Item, J::Item)`.

See [`.cartesian_product()`](crate::Itertools::cartesian_product) for more information.

#### Fields

- **`a_cur`**: `Option<Option<<I as >::Item>>`

  `a_cur` is `None` while no item have been taken out of `a` (at definition).
  Then `a_cur` will be `Some(Some(item))` until `a` is exhausted,
  in which case `a_cur` will be `Some(None)`.

#### Trait Implementations

##### `impl Any for Product<I, J>`

- <span id="product-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Product<I, J>`

- <span id="product-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Product<I, J>`

- <span id="product-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, J: clone::Clone> Clone for Product<I, J>`

- <span id="product-clone"></span>`fn clone(&self) -> Product<I, J>` — [`Product`](../adaptors/index.md#product)

##### `impl CloneToUninit for Product<I, J>`

- <span id="product-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, J: fmt::Debug> Debug for Product<I, J>`

- <span id="product-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Product<I, J>`

- <span id="product-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, J> FusedIterator for Product<I, J>`

##### `impl<U> Into for Product<I, J>`

- <span id="product-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Product<I, J>`

##### `impl<I> IntoIterator for Product<I, J>`

- <span id="product-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="product-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="product-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, J> Iterator for Product<I, J>`

- <span id="product-iterator-type-item"></span>`type Item = (<I as Iterator>::Item, <J as Iterator>::Item)`

- <span id="product-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="product-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="product-iterator-fold"></span>`fn fold<Acc, G>(self, accum: Acc, f: G) -> Acc`

##### `impl Itertools for Product<I, J>`

##### `impl<FromA, FromB> MultiUnzip for Product<I, J>`

- <span id="product-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB)`

##### `impl ToOwned for Product<I, J>`

- <span id="product-toowned-type-owned"></span>`type Owned = T`

- <span id="product-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="product-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Product<I, J>`

- <span id="product-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="product-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Product<I, J>`

- <span id="product-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="product-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PutBack<I>`

```rust
struct PutBack<I>
where
    I: Iterator {
    top: Option<<I as >::Item>,
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:243-249`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L243-L249)*

An iterator adaptor that allows putting back a single
item to the front of the iterator.

Iterator element type is `I::Item`.

#### Implementations

- <span id="putback-with-value"></span>`fn with_value(self, value: <I as >::Item) -> Self`

  put back value `value` (builder method)

- <span id="putback-into-parts"></span>`fn into_parts(self) -> (Option<<I as >::Item>, I)`

  Split the `PutBack` into its parts.

- <span id="putback-put-back"></span>`fn put_back(&mut self, x: <I as >::Item) -> Option<<I as >::Item>`

  Put back a single value to the front of the iterator.
  
  If a value is already in the put back slot, it is returned.

#### Trait Implementations

##### `impl Any for PutBack<I>`

- <span id="putback-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PutBack<I>`

- <span id="putback-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PutBack<I>`

- <span id="putback-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for PutBack<I>`

- <span id="putback-clone"></span>`fn clone(&self) -> PutBack<I>` — [`PutBack`](../adaptors/index.md#putback)

##### `impl CloneToUninit for PutBack<I>`

- <span id="putback-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for PutBack<I>`

- <span id="putback-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PutBack<I>`

- <span id="putback-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PutBack<I>`

- <span id="putback-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PutBack<I>`

##### `impl<I> IntoIterator for PutBack<I>`

- <span id="putback-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="putback-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="putback-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for PutBack<I>`

- <span id="putback-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="putback-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="putback-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="putback-iterator-count"></span>`fn count(self) -> usize`

- <span id="putback-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="putback-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="putback-iterator-all"></span>`fn all<G>(&mut self, f: G) -> bool`

- <span id="putback-iterator-fold"></span>`fn fold<Acc, G>(self, init: Acc, f: G) -> Acc`

##### `impl Itertools for PutBack<I>`

##### `impl MultiUnzip for PutBack<I>`

- <span id="putback-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I> PeekingNext for crate::PutBack<I>`

- <span id="crateputback-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for PutBack<I>`

- <span id="putback-toowned-type-owned"></span>`type Owned = T`

- <span id="putback-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="putback-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PutBack<I>`

- <span id="putback-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="putback-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PutBack<I>`

- <span id="putback-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="putback-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TakeWhileRef<'a, I: 'a, F>`

```rust
struct TakeWhileRef<'a, I: 'a, F> {
    iter: &'a mut I,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:513-516`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L513-L516)*

An iterator adaptor that borrows from a `Clone`-able iterator
to only pick off elements while the predicate returns `true`.

See [`.take_while_ref()`](crate::Itertools::take_while_ref) for more information.

#### Trait Implementations

##### `impl Any for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F> Debug for TakeWhileRef<'_, I, F>`

- <span id="takewhileref-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TakeWhileRef<'a, I, F>`

##### `impl<I> IntoIterator for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="takewhileref-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="takewhileref-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for TakeWhileRef<'_, I, F>`

- <span id="takewhileref-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="takewhileref-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="takewhileref-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for TakeWhileRef<'a, I, F>`

##### `impl MultiUnzip for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="takewhileref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TakeWhileRef<'a, I, F>`

- <span id="takewhileref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="takewhileref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TupleCombinations<I, T>`

```rust
struct TupleCombinations<I, T>
where
    I: Iterator,
    T: HasCombination<I> {
    iter: <T as >::Combination,
    _mi: std::marker::PhantomData<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:616-623`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L616-L623)*

An iterator to iterate through all combinations in a `Clone`-able iterator that produces tuples
of a specific size.

See [`.tuple_combinations()`](crate::Itertools::tuple_combinations) for more
information.

#### Trait Implementations

##### `impl<T> Any for TupleCombinations<I, T>`

- <span id="tuplecombinations-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TupleCombinations<I, T>`

- <span id="tuplecombinations-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TupleCombinations<I, T>`

- <span id="tuplecombinations-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T> Clone for TupleCombinations<I, T>`

- <span id="tuplecombinations-clone"></span>`fn clone(&self) -> TupleCombinations<I, T>` — [`TupleCombinations`](../adaptors/index.md#tuplecombinations)

##### `impl<T> CloneToUninit for TupleCombinations<I, T>`

- <span id="tuplecombinations-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T> Debug for TupleCombinations<I, T>`

- <span id="tuplecombinations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TupleCombinations<I, T>`

- <span id="tuplecombinations-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T> FusedIterator for TupleCombinations<I, T>`

##### `impl<T, U> Into for TupleCombinations<I, T>`

- <span id="tuplecombinations-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for TupleCombinations<I, T>`

##### `impl<I> IntoIterator for TupleCombinations<I, T>`

- <span id="tuplecombinations-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuplecombinations-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuplecombinations-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T> Iterator for TupleCombinations<I, T>`

- <span id="tuplecombinations-iterator-type-item"></span>`type Item = T`

- <span id="tuplecombinations-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuplecombinations-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuplecombinations-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuplecombinations-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> Itertools for TupleCombinations<I, T>`

##### `impl<FromA> MultiUnzip for TupleCombinations<I, T>`

- <span id="tuplecombinations-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl<T> ToOwned for TupleCombinations<I, T>`

- <span id="tuplecombinations-toowned-type-owned"></span>`type Owned = T`

- <span id="tuplecombinations-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuplecombinations-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for TupleCombinations<I, T>`

- <span id="tuplecombinations-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuplecombinations-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TupleCombinations<I, T>`

- <span id="tuplecombinations-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuplecombinations-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Update<I, F>`

```rust
struct Update<I, F> {
    iter: I,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:1171-1174`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L1171-L1174)*

An iterator adapter to apply a mutating function to each element before yielding it.

See [`.update()`](crate::Itertools::update) for more information.

#### Trait Implementations

##### `impl Any for Update<I, F>`

- <span id="update-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Update<I, F>`

- <span id="update-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Update<I, F>`

- <span id="update-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for Update<I, F>`

- <span id="update-clone"></span>`fn clone(&self) -> Update<I, F>` — [`Update`](../adaptors/index.md#update)

##### `impl CloneToUninit for Update<I, F>`

- <span id="update-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for Update<I, F>`

- <span id="update-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, F> DoubleEndedIterator for Update<I, F>`

- <span id="update-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<I, F> ExactSizeIterator for Update<I, F>`

##### `impl<T> From for Update<I, F>`

- <span id="update-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for Update<I, F>`

##### `impl<U> Into for Update<I, F>`

- <span id="update-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Update<I, F>`

##### `impl<I> IntoIterator for Update<I, F>`

- <span id="update-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="update-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="update-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for Update<I, F>`

- <span id="update-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="update-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="update-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="update-iterator-fold"></span>`fn fold<Acc, G>(self, init: Acc, g: G) -> Acc`

- <span id="update-iterator-collect"></span>`fn collect<C>(self) -> C`

##### `impl Itertools for Update<I, F>`

##### `impl MultiUnzip for Update<I, F>`

- <span id="update-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Update<I, F>`

- <span id="update-toowned-type-owned"></span>`type Owned = T`

- <span id="update-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="update-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Update<I, F>`

- <span id="update-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="update-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Update<I, F>`

- <span id="update-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="update-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WhileSome<I>`

```rust
struct WhileSome<I> {
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:566-568`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L566-L568)*

An iterator adaptor that filters `Option<A>` iterator elements
and produces `A`. Stops on the first `None` encountered.

See [`.while_some()`](crate::Itertools::while_some) for more information.

#### Trait Implementations

##### `impl Any for WhileSome<I>`

- <span id="whilesome-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhileSome<I>`

- <span id="whilesome-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhileSome<I>`

- <span id="whilesome-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for WhileSome<I>`

- <span id="whilesome-clone"></span>`fn clone(&self) -> WhileSome<I>` — [`WhileSome`](../adaptors/index.md#whilesome)

##### `impl CloneToUninit for WhileSome<I>`

- <span id="whilesome-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for WhileSome<I>`

- <span id="whilesome-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for WhileSome<I>`

- <span id="whilesome-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhileSome<I>`

- <span id="whilesome-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WhileSome<I>`

##### `impl<I> IntoIterator for WhileSome<I>`

- <span id="whilesome-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="whilesome-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="whilesome-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for WhileSome<I>`

- <span id="whilesome-iterator-type-item"></span>`type Item = A`

- <span id="whilesome-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="whilesome-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="whilesome-iterator-fold"></span>`fn fold<B, F>(self, acc: B, f: F) -> B`

##### `impl Itertools for WhileSome<I>`

##### `impl MultiUnzip for WhileSome<I>`

- <span id="whilesome-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for WhileSome<I>`

- <span id="whilesome-toowned-type-owned"></span>`type Owned = T`

- <span id="whilesome-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whilesome-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WhileSome<I>`

- <span id="whilesome-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whilesome-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhileSome<I>`

- <span id="whilesome-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whilesome-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CombinationsWithReplacement<I>`

```rust
struct CombinationsWithReplacement<I>
where
    I: Iterator,
    <I as >::Item: Clone {
    indices: alloc::boxed::Box<[usize]>,
    pool: super::lazy_buffer::LazyBuffer<I>,
    first: bool,
}
```

*Defined in [`itertools-0.14.0/src/combinations_with_replacement.rs:15-23`](../../../.source_1765894658/itertools-0.14.0/src/combinations_with_replacement.rs#L15-L23)*

An iterator to iterate through all the `n`-length combinations in an iterator, with replacement.

See [`.combinations_with_replacement()`](crate::Itertools::combinations_with_replacement)
for more information.

#### Implementations

- <span id="combinationswithreplacement-increment-indices"></span>`fn increment_indices(&mut self) -> bool`

  Increments indices representing the combination to advance to the next
  (in lexicographic order by increasing sequence) combination.
  
  Returns true if we've run out of combinations, false otherwise.

#### Trait Implementations

##### `impl Any for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-clone"></span>`fn clone(&self) -> CombinationsWithReplacement<I>` — [`CombinationsWithReplacement`](../combinations_with_replacement/index.md#combinationswithreplacement)

##### `impl CloneToUninit for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for CombinationsWithReplacement<I>`

##### `impl<U> Into for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CombinationsWithReplacement<I>`

##### `impl<I> IntoIterator for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="combinationswithreplacement-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="combinationswithreplacement-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="combinationswithreplacement-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="combinationswithreplacement-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="combinationswithreplacement-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="combinationswithreplacement-iterator-count"></span>`fn count(self) -> usize`

##### `impl Itertools for CombinationsWithReplacement<I>`

##### `impl ToOwned for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-toowned-type-owned"></span>`type Owned = T`

- <span id="combinationswithreplacement-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="combinationswithreplacement-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="combinationswithreplacement-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CombinationsWithReplacement<I>`

- <span id="combinationswithreplacement-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="combinationswithreplacement-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ExactlyOneError<I>`

```rust
struct ExactlyOneError<I>
where
    I: Iterator {
    first_two: Option<either::Either<[<I as >::Item; 2], <I as >::Item>>,
    inner: I,
}
```

*Defined in [`itertools-0.14.0/src/exactly_one_err.rs:20-26`](../../../.source_1765894658/itertools-0.14.0/src/exactly_one_err.rs#L20-L26)*

Iterator returned for the error case of `Itertools::exactly_one()`
This iterator yields exactly the same elements as the input iterator.

During the execution of `exactly_one` the iterator must be mutated.  This wrapper
effectively "restores" the state of the input iterator when it's handed back.

This is very similar to `PutBackN` except this iterator only supports 0-2 elements and does not
use a `Vec`.

#### Implementations

- <span id="exactlyoneerror-new"></span>`fn new(first_two: Option<Either<[<I as >::Item; 2], <I as >::Item>>, inner: I) -> Self` — [`Either`](../index.md#either)

  Creates a new `ExactlyOneErr` iterator.

- <span id="exactlyoneerror-additional-len"></span>`fn additional_len(&self) -> usize`

#### Trait Implementations

##### `impl Any for ExactlyOneError<I>`

- <span id="exactlyoneerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ExactlyOneError<I>`

- <span id="exactlyoneerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ExactlyOneError<I>`

- <span id="exactlyoneerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for ExactlyOneError<I>`

- <span id="exactlyoneerror-clone"></span>`fn clone(&self) -> ExactlyOneError<I>` — [`ExactlyOneError`](../exactly_one_err/index.md#exactlyoneerror)

##### `impl CloneToUninit for ExactlyOneError<I>`

- <span id="exactlyoneerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for ExactlyOneError<I>`

- <span id="exactlyoneerror-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult`

##### `impl<I> Display for ExactlyOneError<I>`

- <span id="exactlyoneerror-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult`

##### `impl<I> Error for ExactlyOneError<I>`

##### `impl<I> ExactSizeIterator for ExactlyOneError<I>`

##### `impl<T> From for ExactlyOneError<I>`

- <span id="exactlyoneerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ExactlyOneError<I>`

- <span id="exactlyoneerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ExactlyOneError<I>`

##### `impl<I> IntoIterator for ExactlyOneError<I>`

- <span id="exactlyoneerror-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="exactlyoneerror-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="exactlyoneerror-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for ExactlyOneError<I>`

- <span id="exactlyoneerror-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="exactlyoneerror-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="exactlyoneerror-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="exactlyoneerror-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for ExactlyOneError<I>`

##### `impl MultiUnzip for ExactlyOneError<I>`

- <span id="exactlyoneerror-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for ExactlyOneError<I>`

- <span id="exactlyoneerror-toowned-type-owned"></span>`type Owned = T`

- <span id="exactlyoneerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="exactlyoneerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ExactlyOneError<I>`

- <span id="exactlyoneerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ExactlyOneError<I>`

- <span id="exactlyoneerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="exactlyoneerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ExactlyOneError<I>`

- <span id="exactlyoneerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="exactlyoneerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`itertools-0.14.0/src/flatten_ok.rs:24-32`](../../../.source_1765894658/itertools-0.14.0/src/flatten_ok.rs#L24-L32)*

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

### `Format<'a, I>`

```rust
struct Format<'a, I> {
    sep: &'a str,
    inner: std::cell::Cell<Option<I>>,
}
```

*Defined in [`itertools-0.14.0/src/format.rs:23-27`](../../../.source_1765894658/itertools-0.14.0/src/format.rs#L23-L27)*

Format all iterator elements lazily, separated by `sep`.

The format value can only be formatted once, after that the iterator is
exhausted.

See [`.format()`](crate::Itertools::format)
for more information.

#### Fields

- **`inner`**: `std::cell::Cell<Option<I>>`

  `Format` uses interior mutability because `Display::fmt` takes `&self`.

#### Implementations

- <span id="format-format"></span>`fn format(&self, f: &mut fmt::Formatter<'_>, cb: fn(&<I as >::Item, &mut fmt::Formatter<'_>) -> fmt::Result) -> fmt::Result`

#### Trait Implementations

##### `impl Any for Format<'a, I>`

- <span id="format-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<I> Binary for Format<'a, I>`

- <span id="format-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for Format<'a, I>`

- <span id="format-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Format<'a, I>`

- <span id="format-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Format<'_, I>`

- <span id="format-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Format<'a, I>`

- <span id="format-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Format<'a, I>`

- <span id="format-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Display for Format<'a, I>`

- <span id="format-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Format<'a, I>`

- <span id="format-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Format<'a, I>`

- <span id="format-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Format<'a, I>`

##### `impl<I> LowerExp for Format<'a, I>`

- <span id="format-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> LowerHex for Format<'a, I>`

- <span id="format-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Octal for Format<'a, I>`

- <span id="format-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Pointer for Format<'a, I>`

- <span id="format-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned for Format<'a, I>`

- <span id="format-toowned-type-owned"></span>`type Owned = T`

- <span id="format-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="format-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Format<'a, I>`

- <span id="format-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Format<'a, I>`

- <span id="format-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="format-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Format<'a, I>`

- <span id="format-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="format-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UpperExp for Format<'a, I>`

- <span id="format-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> UpperHex for Format<'a, I>`

- <span id="format-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FormatWith<'a, I, F>`

```rust
struct FormatWith<'a, I, F> {
    sep: &'a str,
    inner: std::cell::Cell<Option<(I, F)>>,
}
```

*Defined in [`itertools-0.14.0/src/format.rs:10-14`](../../../.source_1765894658/itertools-0.14.0/src/format.rs#L10-L14)*

Format all iterator elements lazily, separated by `sep`.

The format value can only be formatted once, after that the iterator is
exhausted.

See [`.format_with()`](crate::Itertools::format_with) for more information.

#### Fields

- **`inner`**: `std::cell::Cell<Option<(I, F)>>`

  `FormatWith` uses interior mutability because `Display::fmt` takes `&self`.

#### Trait Implementations

##### `impl Any for FormatWith<'a, I, F>`

- <span id="formatwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FormatWith<'a, I, F>`

- <span id="formatwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FormatWith<'a, I, F>`

- <span id="formatwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F> Clone for FormatWith<'_, I, F>`

- <span id="formatwith-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FormatWith<'a, I, F>`

- <span id="formatwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for FormatWith<'_, I, F>`

- <span id="formatwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> Display for FormatWith<'_, I, F>`

- <span id="formatwith-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FormatWith<'a, I, F>`

- <span id="formatwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FormatWith<'a, I, F>`

- <span id="formatwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FormatWith<'a, I, F>`

##### `impl ToOwned for FormatWith<'a, I, F>`

- <span id="formatwith-toowned-type-owned"></span>`type Owned = T`

- <span id="formatwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="formatwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for FormatWith<'a, I, F>`

- <span id="formatwith-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FormatWith<'a, I, F>`

- <span id="formatwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="formatwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FormatWith<'a, I, F>`

- <span id="formatwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="formatwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Chunk<'a, I>`

```rust
struct Chunk<'a, I>
where
    I: Iterator + 'a,
    <I as >::Item: 'a {
    parent: &'a IntoChunks<I>,
    index: usize,
    first: Option<<I as >::Item>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:580-588`](../../../.source_1765894658/itertools-0.14.0/src/groupbylazy.rs#L580-L588)*

An iterator for the elements in a single chunk.

Iterator element type is `I::Item`.

#### Trait Implementations

##### `impl Any for Chunk<'a, I>`

- <span id="chunk-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chunk<'a, I>`

- <span id="chunk-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chunk<'a, I>`

- <span id="chunk-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Drop for Chunk<'a, I>`

- <span id="chunk-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Chunk<'a, I>`

- <span id="chunk-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Chunk<'a, I>`

- <span id="chunk-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chunk<'a, I>`

##### `impl<I> IntoIterator for Chunk<'a, I>`

- <span id="chunk-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunk-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chunk-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Chunk<'a, I>`

- <span id="chunk-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunk-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Chunk<'a, I>`

##### `impl MultiUnzip for Chunk<'a, I>`

- <span id="chunk-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for Chunk<'a, I>`

- <span id="chunk-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunk-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chunk<'a, I>`

- <span id="chunk-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunk-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ChunkBy<K, I, F>`

```rust
struct ChunkBy<K, I, F>
where
    I: Iterator {
    inner: std::cell::RefCell<GroupInner<K, I, F>>,
    index: std::cell::Cell<usize>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:304-312`](../../../.source_1765894658/itertools-0.14.0/src/groupbylazy.rs#L304-L312)*

`ChunkBy` is the storage for the lazy grouping operation.

If the groups are consumed in their original order, or if each
group is dropped without keeping it around, then `ChunkBy` uses
no allocations. It needs allocations only if several group iterators
are alive at the same time.

This type implements `IntoIterator` (it is **not** an iterator
itself), because the group iterators need to borrow from this
value. It should be stored in a local variable or temporary and
iterated.

See [`.chunk_by()`](crate::Itertools::chunk_by) for more information.

#### Implementations

- <span id="chunkby-step"></span>`fn step(&self, client: usize) -> Option<<I as >::Item>`

  `client`: Index of group that requests next element

- <span id="chunkby-drop-group"></span>`fn drop_group(&self, client: usize)`

  `client`: Index of group

#### Trait Implementations

##### `impl Any for ChunkBy<K, I, F>`

- <span id="chunkby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ChunkBy<K, I, F>`

- <span id="chunkby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ChunkBy<K, I, F>`

- <span id="chunkby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ChunkBy<K, I, F>`

- <span id="chunkby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ChunkBy<K, I, F>`

- <span id="chunkby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ChunkBy<K, I, F>`

##### `impl<K, I, F> IntoIterator for &'a ChunkBy<K, I, F>`

- <span id="a-chunkby-intoiterator-type-item"></span>`type Item = (K, Group<'a, K, I, F>)`

- <span id="a-chunkby-intoiterator-type-intoiter"></span>`type IntoIter = Groups<'a, K, I, F>`

- <span id="a-chunkby-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<U> TryFrom for ChunkBy<K, I, F>`

- <span id="chunkby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunkby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ChunkBy<K, I, F>`

- <span id="chunkby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunkby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Chunks<'a, I>`

```rust
struct Chunks<'a, I>
where
    I: Iterator + 'a,
    <I as >::Item: 'a {
    parent: &'a IntoChunks<I>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:549-555`](../../../.source_1765894658/itertools-0.14.0/src/groupbylazy.rs#L549-L555)*

An iterator that yields the Chunk iterators.

Iterator element type is `Chunk`.

See [`.chunks()`](crate::Itertools::chunks) for more information.

#### Trait Implementations

##### `impl Any for Chunks<'a, I>`

- <span id="chunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Chunks<'a, I>`

- <span id="chunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Chunks<'a, I>`

- <span id="chunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Chunks<'a, I>`

- <span id="chunks-clone"></span>`fn clone(&self) -> Chunks<'a, I>` — [`Chunks`](../groupbylazy/index.md#chunks)

##### `impl CloneToUninit for Chunks<'a, I>`

- <span id="chunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for Chunks<'a, I>`

- <span id="chunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Chunks<'a, I>`

- <span id="chunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Chunks<'a, I>`

##### `impl<I> IntoIterator for Chunks<'a, I>`

- <span id="chunks-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="chunks-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="chunks-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Chunks<'a, I>`

- <span id="chunks-iterator-type-item"></span>`type Item = Chunk<'a, I>`

- <span id="chunks-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Chunks<'a, I>`

##### `impl ToOwned for Chunks<'a, I>`

- <span id="chunks-toowned-type-owned"></span>`type Owned = T`

- <span id="chunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="chunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Chunks<'a, I>`

- <span id="chunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="chunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Chunks<'a, I>`

- <span id="chunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="chunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Group<'a, K, I, F>`

```rust
struct Group<'a, K, I, F>
where
    I: Iterator + 'a,
    <I as >::Item: 'a,
    K: 'a,
    F: 'a {
    parent: &'a ChunkBy<K, I, F>,
    index: usize,
    first: Option<<I as >::Item>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:419-429`](../../../.source_1765894658/itertools-0.14.0/src/groupbylazy.rs#L419-L429)*

An iterator for the elements in a single group.

Iterator element type is `I::Item`.

#### Trait Implementations

##### `impl Any for Group<'a, K, I, F>`

- <span id="group-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Group<'a, K, I, F>`

- <span id="group-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Group<'a, K, I, F>`

- <span id="group-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<K, I, F> Drop for Group<'a, K, I, F>`

- <span id="group-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for Group<'a, K, I, F>`

- <span id="group-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Group<'a, K, I, F>`

- <span id="group-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Group<'a, K, I, F>`

##### `impl<I> IntoIterator for Group<'a, K, I, F>`

- <span id="group-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="group-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="group-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, I, F> Iterator for Group<'a, K, I, F>`

- <span id="group-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="group-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Group<'a, K, I, F>`

##### `impl MultiUnzip for Group<'a, K, I, F>`

- <span id="group-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for Group<'a, K, I, F>`

- <span id="group-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="group-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Group<'a, K, I, F>`

- <span id="group-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="group-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Groups<'a, K, I, F>`

```rust
struct Groups<'a, K, I, F>
where
    I: Iterator + 'a,
    <I as >::Item: 'a,
    K: 'a,
    F: 'a {
    parent: &'a ChunkBy<K, I, F>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:378-386`](../../../.source_1765894658/itertools-0.14.0/src/groupbylazy.rs#L378-L386)*

An iterator that yields the Group iterators.

Iterator element type is `(K, Group)`:
the group's key `K` and the group's iterator.

See [`.chunk_by()`](crate::Itertools::chunk_by) for more information.

#### Trait Implementations

##### `impl Any for Groups<'a, K, I, F>`

- <span id="groups-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Groups<'a, K, I, F>`

- <span id="groups-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Groups<'a, K, I, F>`

- <span id="groups-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Groups<'a, K, I, F>`

- <span id="groups-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Groups<'a, K, I, F>`

- <span id="groups-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Groups<'a, K, I, F>`

##### `impl<I> IntoIterator for Groups<'a, K, I, F>`

- <span id="groups-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="groups-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="groups-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K, I, F> Iterator for Groups<'a, K, I, F>`

- <span id="groups-iterator-type-item"></span>`type Item = (K, Group<'a, K, I, F>)`

- <span id="groups-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Groups<'a, K, I, F>`

##### `impl<FromA, FromB> MultiUnzip for Groups<'a, K, I, F>`

- <span id="groups-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB)`

##### `impl<U> TryFrom for Groups<'a, K, I, F>`

- <span id="groups-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groups-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Groups<'a, K, I, F>`

- <span id="groups-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groups-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntoChunks<I>`

```rust
struct IntoChunks<I>
where
    I: Iterator {
    inner: std::cell::RefCell<GroupInner<usize, I, ChunkIndex>>,
    index: std::cell::Cell<usize>,
}
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:496-504`](../../../.source_1765894658/itertools-0.14.0/src/groupbylazy.rs#L496-L504)*

`ChunkLazy` is the storage for a lazy chunking operation.

`IntoChunks` behaves just like `ChunkBy`: it is iterable, and
it only buffers if several chunk iterators are alive at the same time.

This type implements `IntoIterator` (it is **not** an iterator
itself), because the chunk iterators need to borrow from this
value. It should be stored in a local variable or temporary and
iterated.

Iterator element type is `Chunk`, each chunk's iterator.

See [`.chunks()`](crate::Itertools::chunks) for more information.

#### Implementations

- <span id="intochunks-step"></span>`fn step(&self, client: usize) -> Option<<I as >::Item>`

  `client`: Index of chunk that requests next element

- <span id="intochunks-drop-group"></span>`fn drop_group(&self, client: usize)`

  `client`: Index of chunk

#### Trait Implementations

##### `impl Any for IntoChunks<I>`

- <span id="intochunks-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntoChunks<I>`

- <span id="intochunks-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntoChunks<I>`

- <span id="intochunks-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for IntoChunks<I>`

- <span id="intochunks-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for IntoChunks<I>`

- <span id="intochunks-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for IntoChunks<I>`

- <span id="intochunks-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntoChunks<I>`

- <span id="intochunks-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntoChunks<I>`

##### `impl<I> IntoIterator for &'a IntoChunks<I>`

- <span id="a-intochunks-intoiterator-type-item"></span>`type Item = Chunk<'a, I>`

- <span id="a-intochunks-intoiterator-type-intoiter"></span>`type IntoIter = Chunks<'a, I>`

- <span id="a-intochunks-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl ToOwned for IntoChunks<I>`

- <span id="intochunks-toowned-type-owned"></span>`type Owned = T`

- <span id="intochunks-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intochunks-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IntoChunks<I>`

- <span id="intochunks-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intochunks-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntoChunks<I>`

- <span id="intochunks-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intochunks-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GroupingMap<I>`

```rust
struct GroupingMap<I> {
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:59-61`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L59-L61)*

`GroupingMap` is an intermediate struct for efficient group-and-fold operations.
It groups elements by their key and at the same time fold each group
using some aggregating operation.

No method on this struct performs temporary allocations.

#### Implementations

- <span id="groupingmap-aggregate"></span>`fn aggregate<FO, R>(self, operation: FO) -> HashMap<K, R>`

  This is the generic way to perform any operation on a `GroupingMap`.
  It's suggested to use this method only to implement custom operations
  when the already provided ones are not enough.
  
  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in an `HashMap`.
  
  The `operation` function is invoked on each element with the following parameters:
   - the current value of the accumulator of the group if there is currently one;
   - a reference to the key of the group this element belongs to;
   - the element from the source being aggregated;
  
  If `operation` returns `Some(element)` then the accumulator is updated with `element`,
  otherwise the previous accumulation is discarded.
  
  Return a `HashMap` associating the key of each group with the result of aggregation of
  that group's elements. If the aggregation of the last element of a group discards the
  accumulator then there won't be an entry associated to that group's key.
  
  ```rust
  use itertools::Itertools;
  
  let data = vec![2, 8, 5, 7, 9, 0, 4, 10];
  let lookup = data.into_iter()
      .into_grouping_map_by(|&n| n % 4)
      .aggregate(|acc, _key, val| {
          if val == 0 || val == 10 {
              None
          } else {
              Some(acc.unwrap_or(0) + val)
          }
      });
  
  assert_eq!(lookup[&0], 4);        // 0 resets the accumulator so only 4 is summed
  assert_eq!(lookup[&1], 5 + 9);
  assert_eq!(lookup.get(&2), None); // 10 resets the accumulator and nothing is summed afterward
  assert_eq!(lookup[&3], 7);
  assert_eq!(lookup.len(), 3);      // The final keys are only 0, 1 and 2
  ```

- <span id="groupingmap-fold-with"></span>`fn fold_with<FI, FO, R>(self, init: FI, operation: FO) -> HashMap<K, R>`

  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in a new map.
  
  `init` is called to obtain the initial value of each accumulator.
  
  `operation` is a function that is invoked on each element with the following parameters:
   - the current value of the accumulator of the group;
   - a reference to the key of the group this element belongs to;
   - the element from the source being accumulated.
  
  Return a `HashMap` associating the key of each group with the result of folding that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  #[derive(Debug, Default)]
  struct Accumulator {
    acc: usize,
  }
  
  let lookup = (1..=7)
      .into_grouping_map_by(|&n| n % 3)
      .fold_with(|_key, _val| Default::default(), |Accumulator { acc }, _key, val| {
          let acc = acc + val;
          Accumulator { acc }
       });
  
  assert_eq!(lookup[&0].acc, 3 + 6);
  assert_eq!(lookup[&1].acc, 1 + 4 + 7);
  assert_eq!(lookup[&2].acc, 2 + 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-fold"></span>`fn fold<FO, R>(self, init: R, operation: FO) -> HashMap<K, R>`

  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in a new map.
  
  `init` is the value from which will be cloned the initial value of each accumulator.
  
  `operation` is a function that is invoked on each element with the following parameters:
   - the current value of the accumulator of the group;
   - a reference to the key of the group this element belongs to;
   - the element from the source being accumulated.
  
  Return a `HashMap` associating the key of each group with the result of folding that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = (1..=7)
      .into_grouping_map_by(|&n| n % 3)
      .fold(0, |acc, _key, val| acc + val);
  
  assert_eq!(lookup[&0], 3 + 6);
  assert_eq!(lookup[&1], 1 + 4 + 7);
  assert_eq!(lookup[&2], 2 + 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-reduce"></span>`fn reduce<FO>(self, operation: FO) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and applies `operation` to the elements
  of each group sequentially, passing the previously accumulated value, a reference to the key
  and the current element as arguments, and stores the results in a new map.
  
  This is similar to `fold` but the initial value of the accumulator is the first element of the group.
  
  `operation` is a function that is invoked on each element with the following parameters:
   - the current value of the accumulator of the group;
   - a reference to the key of the group this element belongs to;
   - the element from the source being accumulated.
  
  Return a `HashMap` associating the key of each group with the result of folding that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = (1..=7)
      .into_grouping_map_by(|&n| n % 3)
      .reduce(|acc, _key, val| acc + val);
  
  assert_eq!(lookup[&0], 3 + 6);
  assert_eq!(lookup[&1], 1 + 4 + 7);
  assert_eq!(lookup[&2], 2 + 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-fold-first"></span>`fn fold_first<FO>(self, operation: FO) -> HashMap<K, V>`

  See [`.reduce()`](GroupingMap::reduce).

- <span id="groupingmap-collect"></span>`fn collect<C>(self) -> HashMap<K, C>`

  Groups elements from the `GroupingMap` source by key and collects the elements of each group in
  an instance of `C`. The iteration order is preserved when inserting elements.
  
  Return a `HashMap` associating the key of each group with the collection containing that group's elements.
  
  ```rust
  use itertools::Itertools;
  use std::collections::HashSet;
  
  let lookup = vec![0, 1, 2, 3, 4, 5, 6, 2, 3, 6].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .collect::<HashSet<_>>();
  
  assert_eq!(lookup[&0], vec![0, 3, 6].into_iter().collect::<HashSet<_>>());
  assert_eq!(lookup[&1], vec![1, 4].into_iter().collect::<HashSet<_>>());
  assert_eq!(lookup[&2], vec![2, 5].into_iter().collect::<HashSet<_>>());
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-max"></span>`fn max(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the maximum of each group.
  
  If several elements are equally maximum, the last element is picked.
  
  Returns a `HashMap` associating the key of each group with the maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .max();
  
  assert_eq!(lookup[&0], 12);
  assert_eq!(lookup[&1], 7);
  assert_eq!(lookup[&2], 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-max-by"></span>`fn max_by<F>(self, compare: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the maximum of each group
  with respect to the specified comparison function.
  
  If several elements are equally maximum, the last element is picked.
  
  Returns a `HashMap` associating the key of each group with the maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .max_by(|_key, x, y| y.cmp(x));
  
  assert_eq!(lookup[&0], 3);
  assert_eq!(lookup[&1], 1);
  assert_eq!(lookup[&2], 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-max-by-key"></span>`fn max_by_key<F, CK>(self, f: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the element of each group
  that gives the maximum from the specified function.
  
  If several elements are equally maximum, the last element is picked.
  
  Returns a `HashMap` associating the key of each group with the maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .max_by_key(|_key, &val| val % 4);
  
  assert_eq!(lookup[&0], 3);
  assert_eq!(lookup[&1], 7);
  assert_eq!(lookup[&2], 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-min"></span>`fn min(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the minimum of each group.
  
  If several elements are equally minimum, the first element is picked.
  
  Returns a `HashMap` associating the key of each group with the minimum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .min();
  
  assert_eq!(lookup[&0], 3);
  assert_eq!(lookup[&1], 1);
  assert_eq!(lookup[&2], 5);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-min-by"></span>`fn min_by<F>(self, compare: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the minimum of each group
  with respect to the specified comparison function.
  
  If several elements are equally minimum, the first element is picked.
  
  Returns a `HashMap` associating the key of each group with the minimum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .min_by(|_key, x, y| y.cmp(x));
  
  assert_eq!(lookup[&0], 12);
  assert_eq!(lookup[&1], 7);
  assert_eq!(lookup[&2], 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-min-by-key"></span>`fn min_by_key<F, CK>(self, f: F) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and finds the element of each group
  that gives the minimum from the specified function.
  
  If several elements are equally minimum, the first element is picked.
  
  Returns a `HashMap` associating the key of each group with the minimum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .min_by_key(|_key, &val| val % 4);
  
  assert_eq!(lookup[&0], 12);
  assert_eq!(lookup[&1], 4);
  assert_eq!(lookup[&2], 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-minmax"></span>`fn minmax(self) -> HashMap<K, MinMaxResult<V>>` — [`MinMaxResult`](../minmax/index.md#minmaxresult)

  Groups elements from the `GroupingMap` source by key and find the maximum and minimum of
  each group.
  
  If several elements are equally maximum, the last element is picked.
  If several elements are equally minimum, the first element is picked.
  
  See [`Itertools::minmax`](crate::Itertools::minmax) for the non-grouping version.
  
  Differences from the non grouping version:
  - It never produces a `MinMaxResult::NoElements`
  - It doesn't have any speedup
  
  Returns a `HashMap` associating the key of each group with the minimum and maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{OneElement, MinMax};
  
  let lookup = vec![1, 3, 4, 5, 7, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .minmax();
  
  assert_eq!(lookup[&0], MinMax(3, 12));
  assert_eq!(lookup[&1], MinMax(1, 7));
  assert_eq!(lookup[&2], OneElement(5));
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-minmax-by"></span>`fn minmax_by<F>(self, compare: F) -> HashMap<K, MinMaxResult<V>>` — [`MinMaxResult`](../minmax/index.md#minmaxresult)

  Groups elements from the `GroupingMap` source by key and find the maximum and minimum of
  each group with respect to the specified comparison function.
  
  If several elements are equally maximum, the last element is picked.
  If several elements are equally minimum, the first element is picked.
  
  It has the same differences from the non-grouping version as `minmax`.
  
  Returns a `HashMap` associating the key of each group with the minimum and maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{OneElement, MinMax};
  
  let lookup = vec![1, 3, 4, 5, 7, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .minmax_by(|_key, x, y| y.cmp(x));
  
  assert_eq!(lookup[&0], MinMax(12, 3));
  assert_eq!(lookup[&1], MinMax(7, 1));
  assert_eq!(lookup[&2], OneElement(5));
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-minmax-by-key"></span>`fn minmax_by_key<F, CK>(self, f: F) -> HashMap<K, MinMaxResult<V>>` — [`MinMaxResult`](../minmax/index.md#minmaxresult)

  Groups elements from the `GroupingMap` source by key and find the elements of each group
  that gives the minimum and maximum from the specified function.
  
  If several elements are equally maximum, the last element is picked.
  If several elements are equally minimum, the first element is picked.
  
  It has the same differences from the non-grouping version as `minmax`.
  
  Returns a `HashMap` associating the key of each group with the minimum and maximum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  use itertools::MinMaxResult::{OneElement, MinMax};
  
  let lookup = vec![1, 3, 4, 5, 7, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .minmax_by_key(|_key, &val| val % 4);
  
  assert_eq!(lookup[&0], MinMax(12, 3));
  assert_eq!(lookup[&1], MinMax(4, 7));
  assert_eq!(lookup[&2], OneElement(5));
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-sum"></span>`fn sum(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and sums them.
  
  This is just a shorthand for `self.reduce(|acc, _, val| acc + val)`.
  It is more limited than `Iterator::sum` since it doesn't use the `Sum` trait.
  
  Returns a `HashMap` associating the key of each group with the sum of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .sum();
  
  assert_eq!(lookup[&0], 3 + 9 + 12);
  assert_eq!(lookup[&1], 1 + 4 + 7);
  assert_eq!(lookup[&2], 5 + 8);
  assert_eq!(lookup.len(), 3);
  ```

- <span id="groupingmap-product"></span>`fn product(self) -> HashMap<K, V>`

  Groups elements from the `GroupingMap` source by key and multiply them.
  
  This is just a shorthand for `self.reduce(|acc, _, val| acc * val)`.
  It is more limited than `Iterator::product` since it doesn't use the `Product` trait.
  
  Returns a `HashMap` associating the key of each group with the product of that group's elements.
  
  ```rust
  use itertools::Itertools;
  
  let lookup = vec![1, 3, 4, 5, 7, 8, 9, 12].into_iter()
      .into_grouping_map_by(|&n| n % 3)
      .product();
  
  assert_eq!(lookup[&0], 3 * 9 * 12);
  assert_eq!(lookup[&1], 1 * 4 * 7);
  assert_eq!(lookup[&2], 5 * 8);
  assert_eq!(lookup.len(), 3);
  ```

#### Trait Implementations

##### `impl Any for GroupingMap<I>`

- <span id="groupingmap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GroupingMap<I>`

- <span id="groupingmap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GroupingMap<I>`

- <span id="groupingmap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for GroupingMap<I>`

- <span id="groupingmap-clone"></span>`fn clone(&self) -> GroupingMap<I>` — [`GroupingMap`](../grouping_map/index.md#groupingmap)

##### `impl CloneToUninit for GroupingMap<I>`

- <span id="groupingmap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for GroupingMap<I>`

- <span id="groupingmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GroupingMap<I>`

- <span id="groupingmap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GroupingMap<I>`

- <span id="groupingmap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for GroupingMap<I>`

##### `impl ToOwned for GroupingMap<I>`

- <span id="groupingmap-toowned-type-owned"></span>`type Owned = T`

- <span id="groupingmap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="groupingmap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GroupingMap<I>`

- <span id="groupingmap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groupingmap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GroupingMap<I>`

- <span id="groupingmap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groupingmap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntersperseWith<I, ElemF>`

```rust
struct IntersperseWith<I, ElemF>
where
    I: Iterator {
    element: ElemF,
    iter: std::iter::Fuse<I>,
    peek: Option<Option<<I as >::Item>>,
}
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:51-61`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L51-L61)*

An iterator adaptor to insert a particular value created by a function
between each element of the adapted iterator.

Iterator element type is `I::Item`

This iterator is *fused*.

See [`.intersperse_with()`](crate::Itertools::intersperse_with) for more information.

#### Fields

- **`peek`**: `Option<Option<<I as >::Item>>`

  `peek` is None while no item have been taken out of `iter` (at definition).
  Then `peek` will alternatively be `Some(None)` and `Some(Some(item))`,
  where `None` indicates it's time to generate from `element` (unless `iter` is empty).

#### Trait Implementations

##### `impl Any for IntersperseWith<I, ElemF>`

- <span id="interspersewith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntersperseWith<I, ElemF>`

- <span id="interspersewith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntersperseWith<I, ElemF>`

- <span id="interspersewith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, ElemF: clone::Clone> Clone for IntersperseWith<I, ElemF>`

- <span id="interspersewith-clone"></span>`fn clone(&self) -> IntersperseWith<I, ElemF>` — [`IntersperseWith`](../intersperse/index.md#interspersewith)

##### `impl CloneToUninit for IntersperseWith<I, ElemF>`

- <span id="interspersewith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, ElemF: fmt::Debug> Debug for IntersperseWith<I, ElemF>`

- <span id="interspersewith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntersperseWith<I, ElemF>`

- <span id="interspersewith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, ElemF> FusedIterator for IntersperseWith<I, ElemF>`

##### `impl<U> Into for IntersperseWith<I, ElemF>`

- <span id="interspersewith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for IntersperseWith<I, ElemF>`

##### `impl<I> IntoIterator for IntersperseWith<I, ElemF>`

- <span id="interspersewith-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interspersewith-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="interspersewith-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, ElemF> Iterator for IntersperseWith<I, ElemF>`

- <span id="interspersewith-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="interspersewith-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="interspersewith-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="interspersewith-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for IntersperseWith<I, ElemF>`

##### `impl MultiUnzip for IntersperseWith<I, ElemF>`

- <span id="interspersewith-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for IntersperseWith<I, ElemF>`

- <span id="interspersewith-toowned-type-owned"></span>`type Owned = T`

- <span id="interspersewith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="interspersewith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IntersperseWith<I, ElemF>`

- <span id="interspersewith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="interspersewith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntersperseWith<I, ElemF>`

- <span id="interspersewith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="interspersewith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `KMergeBy<I, F>`

```rust
struct KMergeBy<I, F>
where
    I: Iterator {
    heap: alloc::vec::Vec<HeadTail<I>>,
    less_than: F,
}
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:157-163`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L157-L163)*

An iterator adaptor that merges an abitrary number of base iterators
according to an ordering function.

Iterator element type is `I::Item`.

See [`.kmerge_by()`](crate::Itertools::kmerge_by) for more
information.

#### Trait Implementations

##### `impl Any for KMergeBy<I, F>`

- <span id="kmergeby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for KMergeBy<I, F>`

- <span id="kmergeby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for KMergeBy<I, F>`

- <span id="kmergeby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F> Clone for KMergeBy<I, F>`

- <span id="kmergeby-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for KMergeBy<I, F>`

- <span id="kmergeby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for KMergeBy<I, F>`

- <span id="kmergeby-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for KMergeBy<I, F>`

- <span id="kmergeby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F> FusedIterator for KMergeBy<I, F>`

##### `impl<U> Into for KMergeBy<I, F>`

- <span id="kmergeby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for KMergeBy<I, F>`

##### `impl<I> IntoIterator for KMergeBy<I, F>`

- <span id="kmergeby-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="kmergeby-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="kmergeby-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for KMergeBy<I, F>`

- <span id="kmergeby-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="kmergeby-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="kmergeby-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for KMergeBy<I, F>`

##### `impl MultiUnzip for KMergeBy<I, F>`

- <span id="kmergeby-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for KMergeBy<I, F>`

- <span id="kmergeby-toowned-type-owned"></span>`type Owned = T`

- <span id="kmergeby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="kmergeby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for KMergeBy<I, F>`

- <span id="kmergeby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="kmergeby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for KMergeBy<I, F>`

- <span id="kmergeby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="kmergeby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

### `MultiPeek<I>`

```rust
struct MultiPeek<I>
where
    I: Iterator {
    iter: std::iter::Fuse<I>,
    buf: alloc::collections::VecDeque<<I as >::Item>,
    index: usize,
}
```

*Defined in [`itertools-0.14.0/src/multipeek_impl.rs:11-18`](../../../.source_1765894658/itertools-0.14.0/src/multipeek_impl.rs#L11-L18)*

See [`multipeek()`](../multipeek_impl/index.md) for more information.

#### Implementations

- <span id="multipeek-reset-peek"></span>`fn reset_peek(&mut self)`

  Reset the peeking “cursor”

#### Trait Implementations

##### `impl Any for MultiPeek<I>`

- <span id="multipeek-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiPeek<I>`

- <span id="multipeek-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiPeek<I>`

- <span id="multipeek-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiPeek<I>`

- <span id="multipeek-clone"></span>`fn clone(&self) -> MultiPeek<I>` — [`MultiPeek`](../multipeek_impl/index.md#multipeek)

##### `impl CloneToUninit for MultiPeek<I>`

- <span id="multipeek-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiPeek<I>`

- <span id="multipeek-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> ExactSizeIterator for MultiPeek<I>`

##### `impl<T> From for MultiPeek<I>`

- <span id="multipeek-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiPeek<I>`

- <span id="multipeek-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiPeek<I>`

##### `impl<I> IntoIterator for MultiPeek<I>`

- <span id="multipeek-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="multipeek-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="multipeek-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for MultiPeek<I>`

- <span id="multipeek-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="multipeek-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="multipeek-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="multipeek-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for MultiPeek<I>`

##### `impl MultiUnzip for MultiPeek<I>`

- <span id="multipeek-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I> PeekingNext for MultiPeek<I>`

- <span id="multipeek-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for MultiPeek<I>`

- <span id="multipeek-toowned-type-owned"></span>`type Owned = T`

- <span id="multipeek-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multipeek-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiPeek<I>`

- <span id="multipeek-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multipeek-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiPeek<I>`

- <span id="multipeek-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multipeek-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

- <span id="padusing-clone"></span>`fn clone(&self) -> PadUsing<I, F>` — [`PadUsing`](../pad_tail/index.md#padusing)

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

### `PeekNth<I>`

```rust
struct PeekNth<I>
where
    I: Iterator {
    iter: std::iter::Fuse<I>,
    buf: alloc::collections::VecDeque<<I as >::Item>,
}
```

*Defined in [`itertools-0.14.0/src/peek_nth.rs:9-15`](../../../.source_1765894658/itertools-0.14.0/src/peek_nth.rs#L9-L15)*

See [`peek_nth()`](../peek_nth/index.md) for more information.

#### Implementations

- <span id="peeknth-peek"></span>`fn peek(&mut self) -> Option<&<I as >::Item>`

  Works exactly like the `peek` method in `std::iter::Peekable`.

- <span id="peeknth-peek-mut"></span>`fn peek_mut(&mut self) -> Option<&mut <I as >::Item>`

  Works exactly like the `peek_mut` method in `std::iter::Peekable`.

- <span id="peeknth-peek-nth"></span>`fn peek_nth(&mut self, n: usize) -> Option<&<I as >::Item>`

  Returns a reference to the `nth` value without advancing the iterator.
  
  # Examples
  
  Basic usage:
  
  ```rust
  use itertools::peek_nth;
  
  let xs = vec![1, 2, 3];
  let mut iter = peek_nth(xs.into_iter());
  
  assert_eq!(iter.peek_nth(0), Some(&1));
  assert_eq!(iter.next(), Some(1));
  
  // The iterator does not advance even if we call `peek_nth` multiple times
  assert_eq!(iter.peek_nth(0), Some(&2));
  assert_eq!(iter.peek_nth(1), Some(&3));
  assert_eq!(iter.next(), Some(2));
  
  // Calling `peek_nth` past the end of the iterator will return `None`
  assert_eq!(iter.peek_nth(1), None);
  ```

- <span id="peeknth-peek-nth-mut"></span>`fn peek_nth_mut(&mut self, n: usize) -> Option<&mut <I as >::Item>`

  Returns a mutable reference to the `nth` value without advancing the iterator.
  
  # Examples
  
  Basic usage:
  
  ```rust
  use itertools::peek_nth;
  
  let xs = vec![1, 2, 3, 4, 5];
  let mut iter = peek_nth(xs.into_iter());
  
  assert_eq!(iter.peek_nth_mut(0), Some(&mut 1));
  assert_eq!(iter.next(), Some(1));
  
  // The iterator does not advance even if we call `peek_nth_mut` multiple times
  assert_eq!(iter.peek_nth_mut(0), Some(&mut 2));
  assert_eq!(iter.peek_nth_mut(1), Some(&mut 3));
  assert_eq!(iter.next(), Some(2));
  
  // Peek into the iterator and set the value behind the mutable reference.
  if let Some(p) = iter.peek_nth_mut(1) {
      assert_eq!(*p, 4);
      *p = 9;
  }
  
  // The value we put in reappears as the iterator continues.
  assert_eq!(iter.next(), Some(3));
  assert_eq!(iter.next(), Some(9));
  
  // Calling `peek_nth_mut` past the end of the iterator will return `None`
  assert_eq!(iter.peek_nth_mut(1), None);
  ```

- <span id="peeknth-next-if"></span>`fn next_if(&mut self, func: impl FnOnce(&<I as >::Item) -> bool) -> Option<<I as >::Item>`

  Works exactly like the `next_if` method in `std::iter::Peekable`.

- <span id="peeknth-next-if-eq"></span>`fn next_if_eq<T>(&mut self, expected: &T) -> Option<<I as >::Item>`

  Works exactly like the `next_if_eq` method in `std::iter::Peekable`.

#### Trait Implementations

##### `impl Any for PeekNth<I>`

- <span id="peeknth-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeekNth<I>`

- <span id="peeknth-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeekNth<I>`

- <span id="peeknth-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for PeekNth<I>`

- <span id="peeknth-clone"></span>`fn clone(&self) -> PeekNth<I>` — [`PeekNth`](../peek_nth/index.md#peeknth)

##### `impl CloneToUninit for PeekNth<I>`

- <span id="peeknth-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for PeekNth<I>`

- <span id="peeknth-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> ExactSizeIterator for PeekNth<I>`

##### `impl<T> From for PeekNth<I>`

- <span id="peeknth-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeekNth<I>`

- <span id="peeknth-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PeekNth<I>`

##### `impl<I> IntoIterator for PeekNth<I>`

- <span id="peeknth-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peeknth-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="peeknth-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for PeekNth<I>`

- <span id="peeknth-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peeknth-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="peeknth-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="peeknth-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for PeekNth<I>`

##### `impl MultiUnzip for PeekNth<I>`

- <span id="peeknth-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I> PeekingNext for PeekNth<I>`

- <span id="peeknth-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for PeekNth<I>`

- <span id="peeknth-toowned-type-owned"></span>`type Owned = T`

- <span id="peeknth-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="peeknth-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PeekNth<I>`

- <span id="peeknth-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="peeknth-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeekNth<I>`

- <span id="peeknth-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="peeknth-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PeekingTakeWhile<'a, I, F>`

```rust
struct PeekingTakeWhile<'a, I, F>
where
    I: Iterator + 'a {
    iter: &'a mut I,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/peeking_take_while.rs:113-119`](../../../.source_1765894658/itertools-0.14.0/src/peeking_take_while.rs#L113-L119)*

An iterator adaptor that takes items while a closure returns `true`.

See [`.peeking_take_while()`](crate::Itertools::peeking_take_while)
for more information.

#### Trait Implementations

##### `impl Any for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F> Debug for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PeekingTakeWhile<'a, I, F>`

##### `impl<I> IntoIterator for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peekingtakewhile-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="peekingtakewhile-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F> Iterator for PeekingTakeWhile<'_, I, F>`

- <span id="peekingtakewhile-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="peekingtakewhile-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="peekingtakewhile-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for PeekingTakeWhile<'a, I, F>`

##### `impl MultiUnzip for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I, F> PeekingNext for PeekingTakeWhile<'_, I, F>`

- <span id="peekingtakewhile-peekingnext-peeking-next"></span>`fn peeking_next<G>(&mut self, g: G) -> Option<<Self as >::Item>`

##### `impl<U> TryFrom for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="peekingtakewhile-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PeekingTakeWhile<'a, I, F>`

- <span id="peekingtakewhile-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="peekingtakewhile-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Permutations<I: Iterator>`

```rust
struct Permutations<I: Iterator> {
    vals: super::lazy_buffer::LazyBuffer<I>,
    state: PermutationState,
}
```

*Defined in [`itertools-0.14.0/src/permutations.rs:16-19`](../../../.source_1765894658/itertools-0.14.0/src/permutations.rs#L16-L19)*

An iterator adaptor that iterates through all the `k`-permutations of the
elements from an iterator.

See [`.permutations()`](crate::Itertools::permutations) for
more information.

#### Trait Implementations

##### `impl Any for Permutations<I>`

- <span id="permutations-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Permutations<I>`

- <span id="permutations-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Permutations<I>`

- <span id="permutations-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Permutations<I>`

- <span id="permutations-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Permutations<I>`

- <span id="permutations-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Permutations<I>`

- <span id="permutations-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Permutations<I>`

- <span id="permutations-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for Permutations<I>`

##### `impl<U> Into for Permutations<I>`

- <span id="permutations-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Permutations<I>`

##### `impl<I> IntoIterator for Permutations<I>`

- <span id="permutations-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="permutations-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="permutations-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Permutations<I>`

- <span id="permutations-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="permutations-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="permutations-iterator-count"></span>`fn count(self) -> usize`

- <span id="permutations-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for Permutations<I>`

##### `impl ToOwned for Permutations<I>`

- <span id="permutations-toowned-type-owned"></span>`type Owned = T`

- <span id="permutations-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="permutations-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Permutations<I>`

- <span id="permutations-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="permutations-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Permutations<I>`

- <span id="permutations-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="permutations-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Powerset<I: Iterator>`

```rust
struct Powerset<I: Iterator> {
    combs: super::combinations::Combinations<I>,
}
```

*Defined in [`itertools-0.14.0/src/powerset.rs:14-16`](../../../.source_1765894658/itertools-0.14.0/src/powerset.rs#L14-L16)*

An iterator to iterate through the powerset of the elements from an iterator.

See [`.powerset()`](crate::Itertools::powerset) for more
information.

#### Implementations

- <span id="powerset-increment-k"></span>`fn increment_k(&mut self) -> bool`

  Returns true if `k` has been incremented, false otherwise.

#### Trait Implementations

##### `impl Any for Powerset<I>`

- <span id="powerset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Powerset<I>`

- <span id="powerset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Powerset<I>`

- <span id="powerset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Powerset<I>`

- <span id="powerset-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Powerset<I>`

- <span id="powerset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Powerset<I>`

- <span id="powerset-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Powerset<I>`

- <span id="powerset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for Powerset<I>`

##### `impl<U> Into for Powerset<I>`

- <span id="powerset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Powerset<I>`

##### `impl<I> IntoIterator for Powerset<I>`

- <span id="powerset-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="powerset-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="powerset-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Powerset<I>`

- <span id="powerset-iterator-type-item"></span>`type Item = Vec<<I as Iterator>::Item>`

- <span id="powerset-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="powerset-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="powerset-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="powerset-iterator-count"></span>`fn count(self) -> usize`

- <span id="powerset-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Powerset<I>`

##### `impl ToOwned for Powerset<I>`

- <span id="powerset-toowned-type-owned"></span>`type Owned = T`

- <span id="powerset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="powerset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Powerset<I>`

- <span id="powerset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="powerset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Powerset<I>`

- <span id="powerset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="powerset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ProcessResults<'a, I, E: 'a>`

```rust
struct ProcessResults<'a, I, E: 'a> {
    error: &'a mut Result<(), E>,
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/process_results_impl.rs:11-14`](../../../.source_1765894658/itertools-0.14.0/src/process_results_impl.rs#L11-L14)*

An iterator that produces only the `T` values as long as the
inner iterator produces `Ok(T)`.

Used by [`process_results`](crate::process_results), see its docs
for more information.

#### Implementations

- <span id="processresults-next-body"></span>`fn next_body<T>(&mut self, item: Option<Result<T, E>>) -> Option<T>`

#### Trait Implementations

##### `impl Any for ProcessResults<'a, I, E>`

- <span id="processresults-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProcessResults<'a, I, E>`

- <span id="processresults-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProcessResults<'a, I, E>`

- <span id="processresults-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: fmt::Debug, E: fmt::Debug + 'a> Debug for ProcessResults<'a, I, E>`

- <span id="processresults-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, E> DoubleEndedIterator for ProcessResults<'_, I, E>`

- <span id="processresults-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="processresults-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> From for ProcessResults<'a, I, E>`

- <span id="processresults-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ProcessResults<'a, I, E>`

- <span id="processresults-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for ProcessResults<'a, I, E>`

##### `impl<I> IntoIterator for ProcessResults<'a, I, E>`

- <span id="processresults-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="processresults-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="processresults-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, E> Iterator for ProcessResults<'_, I, E>`

- <span id="processresults-iterator-type-item"></span>`type Item = T`

- <span id="processresults-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="processresults-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="processresults-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for ProcessResults<'a, I, E>`

##### `impl MultiUnzip for ProcessResults<'a, I, E>`

- <span id="processresults-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for ProcessResults<'a, I, E>`

- <span id="processresults-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="processresults-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ProcessResults<'a, I, E>`

- <span id="processresults-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="processresults-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PutBackN<I: Iterator>`

```rust
struct PutBackN<I: Iterator> {
    top: alloc::vec::Vec<<I as >::Item>,
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/put_back_n_impl.rs:11-14`](../../../.source_1765894658/itertools-0.14.0/src/put_back_n_impl.rs#L11-L14)*

An iterator adaptor that allows putting multiple
items in front of the iterator.

Iterator element type is `I::Item`.

#### Implementations

- <span id="putbackn-put-back"></span>`fn put_back(&mut self, x: <I as >::Item)`

  Puts `x` in front of the iterator.
  
  The values are yielded in order of the most recently put back
  values first.
  
  ```rust
  use itertools::put_back_n;
  
  let mut it = put_back_n(1..5);
  it.next();
  it.put_back(1);
  it.put_back(0);
  
  assert!(itertools::equal(it, 0..5));
  ```

#### Trait Implementations

##### `impl Any for PutBackN<I>`

- <span id="putbackn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PutBackN<I>`

- <span id="putbackn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PutBackN<I>`

- <span id="putbackn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for PutBackN<I>`

- <span id="putbackn-clone"></span>`fn clone(&self) -> PutBackN<I>` — [`PutBackN`](../put_back_n_impl/index.md#putbackn)

##### `impl CloneToUninit for PutBackN<I>`

- <span id="putbackn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for PutBackN<I>`

- <span id="putbackn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PutBackN<I>`

- <span id="putbackn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PutBackN<I>`

- <span id="putbackn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for PutBackN<I>`

##### `impl<I> IntoIterator for PutBackN<I>`

- <span id="putbackn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="putbackn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="putbackn-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator> Iterator for PutBackN<I>`

- <span id="putbackn-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="putbackn-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="putbackn-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="putbackn-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for PutBackN<I>`

##### `impl MultiUnzip for PutBackN<I>`

- <span id="putbackn-multiunzip"></span>`fn multiunzip(self)`

##### `impl<I> PeekingNext for crate::PutBackN<I>`

- <span id="crateputbackn-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for PutBackN<I>`

- <span id="putbackn-toowned-type-owned"></span>`type Owned = T`

- <span id="putbackn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="putbackn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PutBackN<I>`

- <span id="putbackn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="putbackn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PutBackN<I>`

- <span id="putbackn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="putbackn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RcIter<I>`

```rust
struct RcIter<I> {
    pub rciter: alloc::rc::Rc<std::cell::RefCell<I>>,
}
```

*Defined in [`itertools-0.14.0/src/rciter_impl.rs:8-11`](../../../.source_1765894658/itertools-0.14.0/src/rciter_impl.rs#L8-L11)*

A wrapper for `Rc<RefCell<I>>`, that implements the `Iterator` trait.

#### Fields

- **`rciter`**: `alloc::rc::Rc<std::cell::RefCell<I>>`

  The boxed iterator.

#### Trait Implementations

##### `impl Any for RcIter<I>`

- <span id="rciter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RcIter<I>`

- <span id="rciter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RcIter<I>`

- <span id="rciter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for RcIter<I>`

- <span id="rciter-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for RcIter<I>`

- <span id="rciter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for RcIter<I>`

- <span id="rciter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> DoubleEndedIterator for RcIter<I>`

- <span id="rciter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> From for RcIter<I>`

- <span id="rciter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for RcIter<I>`

##### `impl<U> Into for RcIter<I>`

- <span id="rciter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RcIter<I>`

##### `impl<I> IntoIterator for RcIter<I>`

- <span id="rciter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rciter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rciter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for RcIter<I>`

- <span id="rciter-iterator-type-item"></span>`type Item = A`

- <span id="rciter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="rciter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for RcIter<I>`

##### `impl MultiUnzip for RcIter<I>`

- <span id="rciter-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for RcIter<I>`

- <span id="rciter-toowned-type-owned"></span>`type Owned = T`

- <span id="rciter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rciter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RcIter<I>`

- <span id="rciter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rciter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RcIter<I>`

- <span id="rciter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rciter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RepeatN<A>`

```rust
struct RepeatN<A> {
    elt: Option<A>,
    n: usize,
}
```

*Defined in [`itertools-0.14.0/src/repeatn.rs:8-11`](../../../.source_1765894658/itertools-0.14.0/src/repeatn.rs#L8-L11)*

An iterator that produces *n* repetitions of an element.

See [`repeat_n()`](crate::repeat_n) for more information.

#### Trait Implementations

##### `impl Any for RepeatN<A>`

- <span id="repeatn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RepeatN<A>`

- <span id="repeatn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RepeatN<A>`

- <span id="repeatn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: clone::Clone> Clone for RepeatN<A>`

- <span id="repeatn-clone"></span>`fn clone(&self) -> RepeatN<A>` — [`RepeatN`](../repeatn/index.md#repeatn)

##### `impl CloneToUninit for RepeatN<A>`

- <span id="repeatn-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: fmt::Debug> Debug for RepeatN<A>`

- <span id="repeatn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A> DoubleEndedIterator for RepeatN<A>`

- <span id="repeatn-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="repeatn-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<A> ExactSizeIterator for RepeatN<A>`

##### `impl<T> From for RepeatN<A>`

- <span id="repeatn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A> FusedIterator for RepeatN<A>`

##### `impl<U> Into for RepeatN<A>`

- <span id="repeatn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for RepeatN<A>`

##### `impl IntoIterator for RepeatN<A>`

- <span id="repeatn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="repeatn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="repeatn-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A> Iterator for RepeatN<A>`

- <span id="repeatn-iterator-type-item"></span>`type Item = A`

- <span id="repeatn-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="repeatn-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="repeatn-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for RepeatN<A>`

##### `impl MultiUnzip for RepeatN<A>`

- <span id="repeatn-multiunzip"></span>`fn multiunzip(self)`

##### `impl<T: Clone> PeekingNext for crate::RepeatN<T>`

- <span id="craterepeatn-peekingnext-peeking-next"></span>`fn peeking_next<F>(&mut self, accept: F) -> Option<<Self as >::Item>`

##### `impl ToOwned for RepeatN<A>`

- <span id="repeatn-toowned-type-owned"></span>`type Owned = T`

- <span id="repeatn-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="repeatn-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RepeatN<A>`

- <span id="repeatn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repeatn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RepeatN<A>`

- <span id="repeatn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repeatn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Iterate<St, F>`

```rust
struct Iterate<St, F> {
    state: St,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/sources.rs:96-99`](../../../.source_1765894658/itertools-0.14.0/src/sources.rs#L96-L99)*

An iterator that infinitely applies function to value and yields results.

This `struct` is created by the [`iterate()`](crate::iterate) function.
See its documentation for more.

#### Trait Implementations

##### `impl Any for Iterate<St, F>`

- <span id="iterate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Iterate<St, F>`

- <span id="iterate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Iterate<St, F>`

- <span id="iterate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<St: clone::Clone, F: clone::Clone> Clone for Iterate<St, F>`

- <span id="iterate-clone"></span>`fn clone(&self) -> Iterate<St, F>` — [`Iterate`](../sources/index.md#iterate)

##### `impl CloneToUninit for Iterate<St, F>`

- <span id="iterate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<St, F> Debug for Iterate<St, F>`

- <span id="iterate-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Iterate<St, F>`

- <span id="iterate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Iterate<St, F>`

- <span id="iterate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Iterate<St, F>`

##### `impl IntoIterator for Iterate<St, F>`

- <span id="iterate-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iterate-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iterate-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<St, F> Iterator for Iterate<St, F>`

- <span id="iterate-iterator-type-item"></span>`type Item = St`

- <span id="iterate-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iterate-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for Iterate<St, F>`

##### `impl MultiUnzip for Iterate<St, F>`

- <span id="iterate-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Iterate<St, F>`

- <span id="iterate-toowned-type-owned"></span>`type Owned = T`

- <span id="iterate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="iterate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Iterate<St, F>`

- <span id="iterate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="iterate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Iterate<St, F>`

- <span id="iterate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="iterate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Unfold<St, F>`

```rust
struct Unfold<St, F> {
    f: F,
    pub state: St,
}
```

*Defined in [`itertools-0.14.0/src/sources.rs:72-76`](../../../.source_1765894658/itertools-0.14.0/src/sources.rs#L72-L76)*

See [`unfold`](crate::unfold) for more information.

#### Fields

- **`state`**: `St`

  Internal state that will be passed to the closure on the next iteration

#### Trait Implementations

##### `impl Any for Unfold<St, F>`

- <span id="unfold-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unfold<St, F>`

- <span id="unfold-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unfold<St, F>`

- <span id="unfold-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<St: clone::Clone, F: clone::Clone> Clone for Unfold<St, F>`

- <span id="unfold-clone"></span>`fn clone(&self) -> Unfold<St, F>` — [`Unfold`](../sources/index.md#unfold)

##### `impl CloneToUninit for Unfold<St, F>`

- <span id="unfold-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<St, F> Debug for Unfold<St, F>`

- <span id="unfold-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for Unfold<St, F>`

- <span id="unfold-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Unfold<St, F>`

- <span id="unfold-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Unfold<St, F>`

##### `impl IntoIterator for Unfold<St, F>`

- <span id="unfold-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unfold-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unfold-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<St, F> Iterator for Unfold<St, F>`

- <span id="unfold-iterator-type-item"></span>`type Item = A`

- <span id="unfold-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl Itertools for Unfold<St, F>`

##### `impl MultiUnzip for Unfold<St, F>`

- <span id="unfold-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Unfold<St, F>`

- <span id="unfold-toowned-type-owned"></span>`type Owned = T`

- <span id="unfold-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unfold-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Unfold<St, F>`

- <span id="unfold-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unfold-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unfold<St, F>`

- <span id="unfold-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unfold-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new [`TakeWhileInclusive`](../take_while_inclusive/index.md) from an iterator and a predicate.

#### Trait Implementations

##### `impl Any for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone, F: clone::Clone> Clone for TakeWhileInclusive<I, F>`

- <span id="takewhileinclusive-clone"></span>`fn clone(&self) -> TakeWhileInclusive<I, F>` — [`TakeWhileInclusive`](../take_while_inclusive/index.md#takewhileinclusive)

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

### `Tee<I>`

```rust
struct Tee<I>
where
    I: Iterator {
    rcbuffer: alloc::rc::Rc<std::cell::RefCell<TeeBuffer<<I as >::Item, I>>>,
    id: bool,
}
```

*Defined in [`itertools-0.14.0/src/tee.rs:21-27`](../../../.source_1765894658/itertools-0.14.0/src/tee.rs#L21-L27)*

One half of an iterator pair where both return the same elements.

See [`.tee()`](crate::Itertools::tee) for more information.

#### Trait Implementations

##### `impl Any for Tee<I>`

- <span id="tee-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tee<I>`

- <span id="tee-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tee<I>`

- <span id="tee-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Debug for Tee<I>`

- <span id="tee-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> ExactSizeIterator for Tee<I>`

##### `impl<T> From for Tee<I>`

- <span id="tee-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tee<I>`

- <span id="tee-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tee<I>`

##### `impl<I> IntoIterator for Tee<I>`

- <span id="tee-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tee-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tee-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tee<I>`

- <span id="tee-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tee-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tee-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl Itertools for Tee<I>`

##### `impl MultiUnzip for Tee<I>`

- <span id="tee-multiunzip"></span>`fn multiunzip(self)`

##### `impl<U> TryFrom for Tee<I>`

- <span id="tee-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tee-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tee<I>`

- <span id="tee-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tee-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CircularTupleWindows<I, T>`

```rust
struct CircularTupleWindows<I, T>
where
    I: Iterator<Item = <T as >::Item> + Clone,
    T: TupleCollect + Clone {
    iter: TupleWindows<std::iter::Cycle<I>, T>,
    len: usize,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:247-254`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L247-L254)*

An iterator over all windows, wrapping back to the first elements when the
window would otherwise exceed the length of the iterator, producing tuples
of a specific size.

See [`.circular_tuple_windows()`](crate::Itertools::circular_tuple_windows) for more
information.

#### Trait Implementations

##### `impl<T> Any for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T> Clone for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-clone"></span>`fn clone(&self) -> CircularTupleWindows<I, T>` — [`CircularTupleWindows`](../tuple_impl/index.md#circulartuplewindows)

##### `impl<T> CloneToUninit for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T> Debug for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T> ExactSizeIterator for CircularTupleWindows<I, T>`

##### `impl<T> From for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T> FusedIterator for CircularTupleWindows<I, T>`

##### `impl<T, U> Into for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for CircularTupleWindows<I, T>`

##### `impl<I> IntoIterator for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="circulartuplewindows-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="circulartuplewindows-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T> Iterator for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-iterator-type-item"></span>`type Item = T`

- <span id="circulartuplewindows-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="circulartuplewindows-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for CircularTupleWindows<I, T>`

##### `impl<FromA> MultiUnzip for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl<T> ToOwned for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-toowned-type-owned"></span>`type Owned = T`

- <span id="circulartuplewindows-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="circulartuplewindows-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="circulartuplewindows-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CircularTupleWindows<I, T>`

- <span id="circulartuplewindows-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="circulartuplewindows-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TupleBuffer<T>`

```rust
struct TupleBuffer<T>
where
    T: HomogeneousTuple {
    cur: usize,
    buf: <T as >::Buffer,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:24-30`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L24-L30)*

An iterator over a incomplete tuple.

See [`.tuples()`](crate::Itertools::tuples) and
`Tuples::into_buffer()`.

#### Implementations

- <span id="tuplebuffer-new"></span>`fn new(buf: <T as >::Buffer) -> Self` — [`TupleCollect`](../tuple_impl/index.md#tuplecollect)

#### Trait Implementations

##### `impl<T> Any for TupleBuffer<T>`

- <span id="tuplebuffer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TupleBuffer<T>`

- <span id="tuplebuffer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TupleBuffer<T>`

- <span id="tuplebuffer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Clone for TupleBuffer<T>`

- <span id="tuplebuffer-clone"></span>`fn clone(&self) -> TupleBuffer<T>` — [`TupleBuffer`](../tuple_impl/index.md#tuplebuffer)

##### `impl<T> CloneToUninit for TupleBuffer<T>`

- <span id="tuplebuffer-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> Debug for TupleBuffer<T>`

- <span id="tuplebuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ExactSizeIterator for TupleBuffer<T>`

##### `impl<T> From for TupleBuffer<T>`

- <span id="tuplebuffer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for TupleBuffer<T>`

- <span id="tuplebuffer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for TupleBuffer<T>`

##### `impl IntoIterator for TupleBuffer<T>`

- <span id="tuplebuffer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuplebuffer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuplebuffer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for TupleBuffer<T>`

- <span id="tuplebuffer-iterator-type-item"></span>`type Item = <T as TupleCollect>::Item`

- <span id="tuplebuffer-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuplebuffer-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for TupleBuffer<T>`

##### `impl MultiUnzip for TupleBuffer<T>`

- <span id="tuplebuffer-multiunzip"></span>`fn multiunzip(self)`

##### `impl<T> ToOwned for TupleBuffer<T>`

- <span id="tuplebuffer-toowned-type-owned"></span>`type Owned = T`

- <span id="tuplebuffer-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuplebuffer-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for TupleBuffer<T>`

- <span id="tuplebuffer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuplebuffer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TupleBuffer<T>`

- <span id="tuplebuffer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuplebuffer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TupleWindows<I, T>`

```rust
struct TupleWindows<I, T>
where
    I: Iterator<Item = <T as >::Item>,
    T: HomogeneousTuple {
    iter: I,
    last: Option<T>,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:166-173`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L166-L173)*

An iterator over all contiguous windows that produces tuples of a specific size.

See [`.tuple_windows()`](crate::Itertools::tuple_windows) for more
information.

#### Trait Implementations

##### `impl<T> Any for TupleWindows<I, T>`

- <span id="tuplewindows-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TupleWindows<I, T>`

- <span id="tuplewindows-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TupleWindows<I, T>`

- <span id="tuplewindows-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T> Clone for TupleWindows<I, T>`

- <span id="tuplewindows-clone"></span>`fn clone(&self) -> TupleWindows<I, T>` — [`TupleWindows`](../tuple_impl/index.md#tuplewindows)

##### `impl<T> CloneToUninit for TupleWindows<I, T>`

- <span id="tuplewindows-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T> Debug for TupleWindows<I, T>`

- <span id="tuplewindows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T> ExactSizeIterator for TupleWindows<I, T>`

##### `impl<T> From for TupleWindows<I, T>`

- <span id="tuplewindows-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, T> FusedIterator for TupleWindows<I, T>`

##### `impl<T, U> Into for TupleWindows<I, T>`

- <span id="tuplewindows-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for TupleWindows<I, T>`

##### `impl<I> IntoIterator for TupleWindows<I, T>`

- <span id="tuplewindows-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuplewindows-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuplewindows-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T> Iterator for TupleWindows<I, T>`

- <span id="tuplewindows-iterator-type-item"></span>`type Item = T`

- <span id="tuplewindows-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuplewindows-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for TupleWindows<I, T>`

##### `impl<FromA> MultiUnzip for TupleWindows<I, T>`

- <span id="tuplewindows-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl<T> ToOwned for TupleWindows<I, T>`

- <span id="tuplewindows-toowned-type-owned"></span>`type Owned = T`

- <span id="tuplewindows-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuplewindows-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for TupleWindows<I, T>`

- <span id="tuplewindows-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuplewindows-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for TupleWindows<I, T>`

- <span id="tuplewindows-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuplewindows-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuples<I, T>`

```rust
struct Tuples<I, T>
where
    I: Iterator<Item = <T as >::Item>,
    T: HomogeneousTuple {
    iter: std::iter::Fuse<I>,
    buf: <T as >::Buffer,
}
```

*Defined in [`itertools-0.14.0/src/tuple_impl.rs:78-85`](../../../.source_1765894658/itertools-0.14.0/src/tuple_impl.rs#L78-L85)*

An iterator that groups the items in tuples of a specific size.

See [`.tuples()`](crate::Itertools::tuples) for more information.

#### Implementations

- <span id="tuples-into-buffer"></span>`fn into_buffer(self) -> TupleBuffer<T>` — [`TupleBuffer`](../tuple_impl/index.md#tuplebuffer)

  Return a buffer with the produced items that was not enough to be grouped in a tuple.
  
  ```rust
  use itertools::Itertools;
  
  let mut iter = (0..5).tuples();
  assert_eq!(Some((0, 1, 2)), iter.next());
  assert_eq!(None, iter.next());
  itertools::assert_equal(vec![3, 4], iter.into_buffer());
  ```

#### Trait Implementations

##### `impl<T> Any for Tuples<I, T>`

- <span id="tuples-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuples<I, T>`

- <span id="tuples-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuples<I, T>`

- <span id="tuples-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, T> Clone for Tuples<I, T>`

- <span id="tuples-clone"></span>`fn clone(&self) -> Tuples<I, T>` — [`Tuples`](../tuple_impl/index.md#tuples)

##### `impl<T> CloneToUninit for Tuples<I, T>`

- <span id="tuples-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, T> Debug for Tuples<I, T>`

- <span id="tuples-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, T> ExactSizeIterator for Tuples<I, T>`

##### `impl<T> From for Tuples<I, T>`

- <span id="tuples-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> Into for Tuples<I, T>`

- <span id="tuples-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for Tuples<I, T>`

##### `impl<I> IntoIterator for Tuples<I, T>`

- <span id="tuples-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuples-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuples-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, T> Iterator for Tuples<I, T>`

- <span id="tuples-iterator-type-item"></span>`type Item = T`

- <span id="tuples-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuples-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Itertools for Tuples<I, T>`

##### `impl<FromA> MultiUnzip for Tuples<I, T>`

- <span id="tuples-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl<T> ToOwned for Tuples<I, T>`

- <span id="tuples-toowned-type-owned"></span>`type Owned = T`

- <span id="tuples-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuples-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Tuples<I, T>`

- <span id="tuples-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuples-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Tuples<I, T>`

- <span id="tuples-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuples-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Unique<I>`

```rust
struct Unique<I>
where
    I: Iterator,
    <I as >::Item: Eq + Hash + Clone {
    iter: UniqueBy<I, <I as >::Item, ()>,
}
```

*Defined in [`itertools-0.14.0/src/unique_impl.rs:160-166`](../../../.source_1765894658/itertools-0.14.0/src/unique_impl.rs#L160-L166)*

An iterator adapter to filter out duplicate elements.

See [`.unique()`](crate::Itertools::unique) for more information.

#### Trait Implementations

##### `impl Any for Unique<I>`

- <span id="unique-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Unique<I>`

- <span id="unique-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Unique<I>`

- <span id="unique-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Unique<I>`

- <span id="unique-clone"></span>`fn clone(&self) -> Unique<I>` — [`Unique`](../unique_impl/index.md#unique)

##### `impl CloneToUninit for Unique<I>`

- <span id="unique-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Unique<I>`

- <span id="unique-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I> DoubleEndedIterator for Unique<I>`

- <span id="unique-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> From for Unique<I>`

- <span id="unique-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I> FusedIterator for Unique<I>`

##### `impl<U> Into for Unique<I>`

- <span id="unique-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Unique<I>`

##### `impl<I> IntoIterator for Unique<I>`

- <span id="unique-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unique-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unique-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Unique<I>`

- <span id="unique-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unique-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="unique-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="unique-iterator-count"></span>`fn count(self) -> usize`

##### `impl Itertools for Unique<I>`

##### `impl MultiUnzip for Unique<I>`

- <span id="unique-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for Unique<I>`

- <span id="unique-toowned-type-owned"></span>`type Owned = T`

- <span id="unique-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="unique-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Unique<I>`

- <span id="unique-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="unique-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Unique<I>`

- <span id="unique-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="unique-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `UniqueBy<I: Iterator, V, F>`

```rust
struct UniqueBy<I: Iterator, V, F> {
    iter: I,
    used: std::collections::HashMap<V, ()>,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/unique_impl.rs:12-19`](../../../.source_1765894658/itertools-0.14.0/src/unique_impl.rs#L12-L19)*

An iterator adapter to filter out duplicate elements.

See [`.unique_by()`](crate::Itertools::unique) for more information.

#### Trait Implementations

##### `impl Any for UniqueBy<I, V, F>`

- <span id="uniqueby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for UniqueBy<I, V, F>`

- <span id="uniqueby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for UniqueBy<I, V, F>`

- <span id="uniqueby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator, V: clone::Clone, F: clone::Clone> Clone for UniqueBy<I, V, F>`

- <span id="uniqueby-clone"></span>`fn clone(&self) -> UniqueBy<I, V, F>` — [`UniqueBy`](../unique_impl/index.md#uniqueby)

##### `impl CloneToUninit for UniqueBy<I, V, F>`

- <span id="uniqueby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, V, F> Debug for UniqueBy<I, V, F>`

- <span id="uniqueby-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I, V, F> DoubleEndedIterator for UniqueBy<I, V, F>`

- <span id="uniqueby-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> From for UniqueBy<I, V, F>`

- <span id="uniqueby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, V, F> FusedIterator for UniqueBy<I, V, F>`

##### `impl<U> Into for UniqueBy<I, V, F>`

- <span id="uniqueby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for UniqueBy<I, V, F>`

##### `impl<I> IntoIterator for UniqueBy<I, V, F>`

- <span id="uniqueby-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="uniqueby-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="uniqueby-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, V, F> Iterator for UniqueBy<I, V, F>`

- <span id="uniqueby-iterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="uniqueby-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="uniqueby-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="uniqueby-iterator-count"></span>`fn count(self) -> usize`

##### `impl Itertools for UniqueBy<I, V, F>`

##### `impl MultiUnzip for UniqueBy<I, V, F>`

- <span id="uniqueby-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for UniqueBy<I, V, F>`

- <span id="uniqueby-toowned-type-owned"></span>`type Owned = T`

- <span id="uniqueby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="uniqueby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for UniqueBy<I, V, F>`

- <span id="uniqueby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="uniqueby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for UniqueBy<I, V, F>`

- <span id="uniqueby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="uniqueby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WithPosition<I>`

```rust
struct WithPosition<I>
where
    I: Iterator {
    handled_first: bool,
    peekable: std::iter::Peekable<std::iter::Fuse<I>>,
}
```

*Defined in [`itertools-0.14.0/src/with_position.rs:10-16`](../../../.source_1765894658/itertools-0.14.0/src/with_position.rs#L10-L16)*

An iterator adaptor that wraps each element in an [`Position`](../with_position/index.md).

Iterator element type is `(Position, I::Item)`.

See [`.with_position()`](crate::Itertools::with_position) for more information.

#### Trait Implementations

##### `impl Any for WithPosition<I>`

- <span id="withposition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithPosition<I>`

- <span id="withposition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithPosition<I>`

- <span id="withposition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for WithPosition<I>`

- <span id="withposition-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for WithPosition<I>`

- <span id="withposition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for WithPosition<I>`

- <span id="withposition-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<I> ExactSizeIterator for WithPosition<I>`

##### `impl<T> From for WithPosition<I>`

- <span id="withposition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I: Iterator> FusedIterator for WithPosition<I>`

##### `impl<U> Into for WithPosition<I>`

- <span id="withposition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WithPosition<I>`

##### `impl<I> IntoIterator for WithPosition<I>`

- <span id="withposition-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="withposition-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="withposition-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator> Iterator for WithPosition<I>`

- <span id="withposition-iterator-type-item"></span>`type Item = (Position, <I as Iterator>::Item)`

- <span id="withposition-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="withposition-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="withposition-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for WithPosition<I>`

##### `impl<FromA, FromB> MultiUnzip for WithPosition<I>`

- <span id="withposition-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB)`

##### `impl ToOwned for WithPosition<I>`

- <span id="withposition-toowned-type-owned"></span>`type Owned = T`

- <span id="withposition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="withposition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WithPosition<I>`

- <span id="withposition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withposition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithPosition<I>`

- <span id="withposition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withposition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

- <span id="zipeq-clone"></span>`fn clone(&self) -> ZipEq<I, J>` — [`ZipEq`](../zip_eq_impl/index.md#zipeq)

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

### `ZipLongest<T, U>`

```rust
struct ZipLongest<T, U> {
    a: std::iter::Fuse<T>,
    b: std::iter::Fuse<U>,
}
```

*Defined in [`itertools-0.14.0/src/zip_longest.rs:18-21`](../../../.source_1765894658/itertools-0.14.0/src/zip_longest.rs#L18-L21)*

An iterator which iterates two other iterators simultaneously
and wraps the elements in [`EitherOrBoth`](../either_or_both/index.md).

This iterator is *fused*.

See [`.zip_longest()`](crate::Itertools::zip_longest) for more information.

#### Trait Implementations

##### `impl<T> Any for ZipLongest<T, U>`

- <span id="ziplongest-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ZipLongest<T, U>`

- <span id="ziplongest-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ZipLongest<T, U>`

- <span id="ziplongest-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone, U: clone::Clone> Clone for ZipLongest<T, U>`

- <span id="ziplongest-clone"></span>`fn clone(&self) -> ZipLongest<T, U>` — [`ZipLongest`](../zip_longest/index.md#ziplongest)

##### `impl<T> CloneToUninit for ZipLongest<T, U>`

- <span id="ziplongest-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: fmt::Debug, U: fmt::Debug> Debug for ZipLongest<T, U>`

- <span id="ziplongest-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U> DoubleEndedIterator for ZipLongest<T, U>`

- <span id="ziplongest-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="ziplongest-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, U> ExactSizeIterator for ZipLongest<T, U>`

##### `impl<T> From for ZipLongest<T, U>`

- <span id="ziplongest-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T, U> FusedIterator for ZipLongest<T, U>`

##### `impl<T, U> Into for ZipLongest<T, U>`

- <span id="ziplongest-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoEither for ZipLongest<T, U>`

##### `impl IntoIterator for ZipLongest<T, U>`

- <span id="ziplongest-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ziplongest-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="ziplongest-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, U> Iterator for ZipLongest<T, U>`

- <span id="ziplongest-iterator-type-item"></span>`type Item = EitherOrBoth<<T as Iterator>::Item, <U as Iterator>::Item>`

- <span id="ziplongest-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="ziplongest-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="ziplongest-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl<T> Itertools for ZipLongest<T, U>`

##### `impl<T> ToOwned for ZipLongest<T, U>`

- <span id="ziplongest-toowned-type-owned"></span>`type Owned = T`

- <span id="ziplongest-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ziplongest-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for ZipLongest<T, U>`

- <span id="ziplongest-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ziplongest-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ZipLongest<T, U>`

- <span id="ziplongest-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ziplongest-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Zip<T>`

```rust
struct Zip<T> {
    t: T,
}
```

*Defined in [`itertools-0.14.0/src/ziptuple.rs:6-8`](../../../.source_1765894658/itertools-0.14.0/src/ziptuple.rs#L6-L8)*

See [`multizip`](../ziptuple/index.md) for more information.

#### Trait Implementations

##### `impl<T> Any for Zip<T>`

- <span id="zip-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Zip<T>`

- <span id="zip-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Zip<T>`

- <span id="zip-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Zip<T>`

- <span id="zip-clone"></span>`fn clone(&self) -> Zip<T>` — [`Zip`](../ziptuple/index.md#zip)

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

## Type Aliases

### `Coalesce<I, F>`

```rust
type Coalesce<I, F> = CoalesceBy<I, F, NoCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:141`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L141)*

An iterator adaptor that may join together adjacent elements.

See [`.coalesce()`](crate::Itertools::coalesce) for more information.

### `Dedup<I>`

```rust
type Dedup<I> = DedupBy<I, DedupEq>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:224`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L224)*

An iterator adaptor that removes repeated duplicates.

See [`.dedup()`](crate::Itertools::dedup) for more information.

### `DedupBy<I, Pred>`

```rust
type DedupBy<I, Pred> = CoalesceBy<I, DedupPred2CoalescePred<Pred>, NoCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:167`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L167)*

An iterator adaptor that removes repeated duplicates, determining equality using a comparison function.

See [`.dedup_by()`](crate::Itertools::dedup_by) or [`.dedup()`](crate::Itertools::dedup) for more information.

### `DedupByWithCount<I, Pred>`

```rust
type DedupByWithCount<I, Pred> = CoalesceBy<I, DedupPredWithCount2CoalescePred<Pred>, WithCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:239-240`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L239-L240)*

An iterator adaptor that removes repeated duplicates, while keeping a count of how many
repeated elements were present. This will determine equality using a comparison function.

See [`.dedup_by_with_count()`](crate::Itertools::dedup_by_with_count) or
[`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information.

### `DedupWithCount<I>`

```rust
type DedupWithCount<I> = DedupByWithCount<I, DedupEq>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:266`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L266)*

An iterator adaptor that removes repeated duplicates, while keeping a count of how many
repeated elements were present.

See [`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information.

### `MapInto<I, R>`

```rust
type MapInto<I, R> = MapSpecialCase<I, MapSpecialCaseFnInto<R>>;
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:102`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/map.rs#L102)*

An iterator adapter to apply `Into` conversion to each element.

See [`.map_into()`](crate::Itertools::map_into) for more information.

### `MapOk<I, F>`

```rust
type MapOk<I, F> = MapSpecialCase<I, MapSpecialCaseFnOk<F>>;
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:68`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/map.rs#L68)*

An iterator adapter to apply a transformation within a nested `Result::Ok`.

See [`.map_ok()`](crate::Itertools::map_ok) for more information.

### `ArrayCombinations<I, const K: usize>`

```rust
type ArrayCombinations<I, const K: usize> = CombinationsGeneric<I, [usize; K]>;
```

*Defined in [`itertools-0.14.0/src/combinations.rs:14`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L14)*

Iterator for const generic combinations returned by [`.array_combinations()`](crate::Itertools::array_combinations)

### `Combinations<I>`

```rust
type Combinations<I> = CombinationsGeneric<I, alloc::vec::Vec<usize>>;
```

*Defined in [`itertools-0.14.0/src/combinations.rs:12`](../../../.source_1765894658/itertools-0.14.0/src/combinations.rs#L12)*

Iterator for `Vec` valued combinations returned by [`.combinations()`](crate::Itertools::combinations)

### `ConsTuples<I>`

```rust
type ConsTuples<I> = crate::adaptors::map::MapSpecialCase<I, ConsTuplesFn>;
```

*Defined in [`itertools-0.14.0/src/cons_tuples_impl.rs:27`](../../../.source_1765894658/itertools-0.14.0/src/cons_tuples_impl.rs#L27)*

An iterator that maps an iterator of tuples like
`((A, B), C)` to an iterator of `(A, B, C)`.

Used by the `iproduct!()` macro.

### `Duplicates<I>`

```rust
type Duplicates<I> = private::DuplicatesBy<I, <I as Iterator>::Item, private::ById>;
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:207`](../../../.source_1765894658/itertools-0.14.0/src/duplicates_impl.rs#L207)*

An iterator adapter to filter out duplicate elements.

See [`.duplicates()`](crate::Itertools::duplicates) for more information.

### `DuplicatesBy<I, V, F>`

```rust
type DuplicatesBy<I, V, F> = private::DuplicatesBy<I, V, private::ByFn<F>>;
```

*Defined in [`itertools-0.14.0/src/duplicates_impl.rs:192`](../../../.source_1765894658/itertools-0.14.0/src/duplicates_impl.rs#L192)*

An iterator adapter to filter for duplicate elements.

See [`.duplicates_by()`](crate::Itertools::duplicates_by) for more information.

### `GroupBy<K, I, F>`

```rust
type GroupBy<K, I, F> = ChunkBy<K, I, F>;
```

*Defined in [`itertools-0.14.0/src/groupbylazy.rs:288`](../../../.source_1765894658/itertools-0.14.0/src/groupbylazy.rs#L288)*

See [`ChunkBy`](crate::structs::ChunkBy).

### `GroupingMapBy<I, F>`

```rust
type GroupingMapBy<I, F> = GroupingMap<crate::adaptors::map::MapSpecialCase<I, GroupingMapFn<F>>>;
```

*Defined in [`itertools-0.14.0/src/grouping_map.rs:50`](../../../.source_1765894658/itertools-0.14.0/src/grouping_map.rs#L50)*

`GroupingMapBy` is an intermediate struct for efficient group-and-fold operations.

See [`GroupingMap`](../grouping_map/index.md) for more informations.

### `Intersperse<I>`

```rust
type Intersperse<I> = IntersperseWith<I, IntersperseElementSimple<<I as Iterator>::Item>>;
```

*Defined in [`itertools-0.14.0/src/intersperse.rs:25`](../../../.source_1765894658/itertools-0.14.0/src/intersperse.rs#L25)*

An iterator adaptor to insert a particular value
between each element of the adapted iterator.

Iterator element type is `I::Item`

This iterator is *fused*.

See [`.intersperse()`](crate::Itertools::intersperse) for more information.

### `KMerge<I>`

```rust
type KMerge<I> = KMergeBy<I, KMergeByLt>;
```

*Defined in [`itertools-0.14.0/src/kmerge_impl.rs:106`](../../../.source_1765894658/itertools-0.14.0/src/kmerge_impl.rs#L106)*

An iterator adaptor that merges an abitrary number of base iterators in ascending order.
If all base iterators are sorted (ascending), the result is sorted.

Iterator element type is `I::Item`.

See [`.kmerge()`](crate::Itertools::kmerge) for more information.

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

