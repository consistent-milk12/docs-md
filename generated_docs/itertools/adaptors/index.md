*[itertools](../index.md) / [adaptors](index.md)*

---

# Module `adaptors`

Licensed under the Apache License, Version 2.0
<https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<https://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.

## Contents

- [Modules](#modules)
  - [`coalesce`](#coalesce)
  - [`map`](#map)
  - [`multi_product`](#multi-product)
- [Structs](#structs)
  - [`Interleave`](#interleave)
  - [`InterleaveShortest`](#interleaveshortest)
  - [`PutBack`](#putback)
  - [`Product`](#product)
  - [`Batching`](#batching)
  - [`TakeWhileRef`](#takewhileref)
  - [`WhileSome`](#whilesome)
  - [`TupleCombinations`](#tuplecombinations)
  - [`Tuple1Combination`](#tuple1combination)
  - [`Tuple2Combination`](#tuple2combination)
  - [`Tuple3Combination`](#tuple3combination)
  - [`Tuple4Combination`](#tuple4combination)
  - [`Tuple5Combination`](#tuple5combination)
  - [`Tuple6Combination`](#tuple6combination)
  - [`Tuple7Combination`](#tuple7combination)
  - [`Tuple8Combination`](#tuple8combination)
  - [`Tuple9Combination`](#tuple9combination)
  - [`Tuple10Combination`](#tuple10combination)
  - [`Tuple11Combination`](#tuple11combination)
  - [`Tuple12Combination`](#tuple12combination)
  - [`FilterOk`](#filterok)
  - [`FilterMapOk`](#filtermapok)
  - [`Positions`](#positions)
  - [`Update`](#update)
  - [`CoalesceBy`](#coalesceby)
  - [`NoCount`](#nocount)
  - [`WithCount`](#withcount)
  - [`DedupPred2CoalescePred`](#deduppred2coalescepred)
  - [`DedupEq`](#dedupeq)
  - [`DedupPredWithCount2CoalescePred`](#deduppredwithcount2coalescepred)
  - [`MultiProduct`](#multiproduct)
  - [`MultiProductInner`](#multiproductinner)
  - [`MultiProductIter`](#multiproductiter)
- [Traits](#traits)
  - [`HasCombination`](#hascombination)
  - [`CoalescePredicate`](#coalescepredicate)
  - [`CountItem`](#countitem)
  - [`DedupPredicate`](#deduppredicate)
- [Functions](#functions)
  - [`map_into`](#map-into)
  - [`map_ok`](#map-ok)
  - [`interleave`](#interleave)
  - [`interleave_shortest`](#interleave-shortest)
  - [`put_back`](#put-back)
  - [`cartesian_product`](#cartesian-product)
  - [`batching`](#batching)
  - [`take_while_ref`](#take-while-ref)
  - [`while_some`](#while-some)
  - [`tuple_combinations`](#tuple-combinations)
  - [`checked_binomial`](#checked-binomial)
  - [`filter_ok`](#filter-ok)
  - [`transpose_result`](#transpose-result)
  - [`filter_map_ok`](#filter-map-ok)
  - [`positions`](#positions)
  - [`update`](#update)
  - [`coalesce`](#coalesce)
  - [`dedup_by`](#dedup-by)
  - [`dedup`](#dedup)
  - [`dedup_by_with_count`](#dedup-by-with-count)
  - [`dedup_with_count`](#dedup-with-count)
  - [`multi_cartesian_product`](#multi-cartesian-product)
- [Type Aliases](#type-aliases)
  - [`MapInto`](#mapinto)
  - [`MapOk`](#mapok)
  - [`Coalesce`](#coalesce)
  - [`DedupBy`](#dedupby)
  - [`Dedup`](#dedup)
  - [`DedupByWithCount`](#dedupbywithcount)
  - [`DedupWithCount`](#dedupwithcount)
- [Macros](#macros)
  - [`impl_tuple_combination!`](#impl-tuple-combination)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`coalesce`](#coalesce) | mod |  |
| [`map`](#map) | mod |  |
| [`multi_product`](#multi-product) | mod |  |
| [`Interleave`](#interleave) | struct | An iterator adaptor that alternates elements from two iterators until both run out. |
| [`InterleaveShortest`](#interleaveshortest) | struct | An iterator adaptor that alternates elements from the two iterators until one of them runs out. |
| [`PutBack`](#putback) | struct | An iterator adaptor that allows putting back a single item to the front of the iterator. |
| [`Product`](#product) | struct | An iterator adaptor that iterates over the cartesian product of the element sets of two iterators `I` and `J`. |
| [`Batching`](#batching) | struct | A “meta iterator adaptor”. |
| [`TakeWhileRef`](#takewhileref) | struct | An iterator adaptor that borrows from a `Clone`-able iterator to only pick off elements while the predicate returns `true`. |
| [`WhileSome`](#whilesome) | struct | An iterator adaptor that filters `Option<A>` iterator elements and produces `A`. |
| [`TupleCombinations`](#tuplecombinations) | struct | An iterator to iterate through all combinations in a `Clone`-able iterator that produces tuples of a specific size. |
| [`Tuple1Combination`](#tuple1combination) | struct |  |
| [`Tuple2Combination`](#tuple2combination) | struct |  |
| [`Tuple3Combination`](#tuple3combination) | struct |  |
| [`Tuple4Combination`](#tuple4combination) | struct |  |
| [`Tuple5Combination`](#tuple5combination) | struct |  |
| [`Tuple6Combination`](#tuple6combination) | struct |  |
| [`Tuple7Combination`](#tuple7combination) | struct |  |
| [`Tuple8Combination`](#tuple8combination) | struct |  |
| [`Tuple9Combination`](#tuple9combination) | struct |  |
| [`Tuple10Combination`](#tuple10combination) | struct |  |
| [`Tuple11Combination`](#tuple11combination) | struct |  |
| [`Tuple12Combination`](#tuple12combination) | struct |  |
| [`FilterOk`](#filterok) | struct | An iterator adapter to filter values within a nested `Result::Ok`. |
| [`FilterMapOk`](#filtermapok) | struct | An iterator adapter to filter and apply a transformation on values within a nested `Result::Ok`. |
| [`Positions`](#positions) | struct | An iterator adapter to get the positions of each element that matches a predicate. |
| [`Update`](#update) | struct | An iterator adapter to apply a mutating function to each element before yielding it. |
| [`CoalesceBy`](#coalesceby) | struct |  |
| [`NoCount`](#nocount) | struct |  |
| [`WithCount`](#withcount) | struct |  |
| [`DedupPred2CoalescePred`](#deduppred2coalescepred) | struct |  |
| [`DedupEq`](#dedupeq) | struct |  |
| [`DedupPredWithCount2CoalescePred`](#deduppredwithcount2coalescepred) | struct |  |
| [`MultiProduct`](#multiproduct) | struct | An iterator adaptor that iterates over the cartesian product of multiple iterators of type `I`. |
| [`MultiProductInner`](#multiproductinner) | struct | Internals for `MultiProduct`. |
| [`MultiProductIter`](#multiproductiter) | struct | Holds the state of a single iterator within a `MultiProduct`. |
| [`HasCombination`](#hascombination) | trait |  |
| [`CoalescePredicate`](#coalescepredicate) | trait |  |
| [`CountItem`](#countitem) | trait |  |
| [`DedupPredicate`](#deduppredicate) | trait |  |
| [`map_into`](#map-into) | fn |  |
| [`map_ok`](#map-ok) | fn |  |
| [`interleave`](#interleave) | fn | Create an iterator that interleaves elements in `i` and `j`. |
| [`interleave_shortest`](#interleave-shortest) | fn | Create a new `InterleaveShortest` iterator. |
| [`put_back`](#put-back) | fn | Create an iterator where you can put back a single item |
| [`cartesian_product`](#cartesian-product) | fn | Create a new cartesian product iterator |
| [`batching`](#batching) | fn | Create a new Batching iterator. |
| [`take_while_ref`](#take-while-ref) | fn | Create a new `TakeWhileRef` from a reference to clonable iterator. |
| [`while_some`](#while-some) | fn | Create a new `WhileSome<I>`. |
| [`tuple_combinations`](#tuple-combinations) | fn | Create a new `TupleCombinations` from a clonable iterator. |
| [`checked_binomial`](#checked-binomial) | fn |  |
| [`filter_ok`](#filter-ok) | fn | Create a new `FilterOk` iterator. |
| [`transpose_result`](#transpose-result) | fn |  |
| [`filter_map_ok`](#filter-map-ok) | fn | Create a new `FilterOk` iterator. |
| [`positions`](#positions) | fn | Create a new `Positions` iterator. |
| [`update`](#update) | fn | Create a new `Update` iterator. |
| [`coalesce`](#coalesce) | fn | Create a new `Coalesce`. |
| [`dedup_by`](#dedup-by) | fn | Create a new `DedupBy`. |
| [`dedup`](#dedup) | fn | Create a new `Dedup`. |
| [`dedup_by_with_count`](#dedup-by-with-count) | fn | Create a new `DedupByWithCount`. |
| [`dedup_with_count`](#dedup-with-count) | fn | Create a new `DedupWithCount`. |
| [`multi_cartesian_product`](#multi-cartesian-product) | fn | Create a new cartesian product iterator over an arbitrary number of iterators of the same type. |
| [`MapInto`](#mapinto) | type |  |
| [`MapOk`](#mapok) | type |  |
| [`Coalesce`](#coalesce) | type | An iterator adaptor that may join together adjacent elements. |
| [`DedupBy`](#dedupby) | type | An iterator adaptor that removes repeated duplicates, determining equality using a comparison function. |
| [`Dedup`](#dedup) | type | An iterator adaptor that removes repeated duplicates. |
| [`DedupByWithCount`](#dedupbywithcount) | type | An iterator adaptor that removes repeated duplicates, while keeping a count of how many repeated elements were present. |
| [`DedupWithCount`](#dedupwithcount) | type | An iterator adaptor that removes repeated duplicates, while keeping a count of how many repeated elements were present. |
| [`impl_tuple_combination!`](#impl-tuple-combination) | macro |  |

## Modules

- [`coalesce`](coalesce/index.md)
- [`map`](map/index.md)
- [`multi_product`](multi_product/index.md)

## Structs

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

- <span id="interleave-clone"></span>`fn clone(&self) -> Interleave<I, J>` — [`Interleave`](#interleave)

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

- <span id="interleaveshortest-clone"></span>`fn clone(&self) -> InterleaveShortest<I, J>` — [`InterleaveShortest`](#interleaveshortest)

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

- <span id="putback-clone"></span>`fn clone(&self) -> PutBack<I>` — [`PutBack`](#putback)

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

- <span id="product-clone"></span>`fn clone(&self) -> Product<I, J>` — [`Product`](#product)

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

- <span id="batching-clone"></span>`fn clone(&self) -> Batching<I, F>` — [`Batching`](#batching)

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

- <span id="whilesome-clone"></span>`fn clone(&self) -> WhileSome<I>` — [`WhileSome`](#whilesome)

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

- <span id="tuplecombinations-clone"></span>`fn clone(&self) -> TupleCombinations<I, T>` — [`TupleCombinations`](#tuplecombinations)

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

### `Tuple1Combination<I>`

```rust
struct Tuple1Combination<I> {
    iter: I,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:677-679`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L677-L679)*

#### Trait Implementations

##### `impl Any for Tuple1Combination<I>`

- <span id="tuple1combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple1Combination<I>`

- <span id="tuple1combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple1Combination<I>`

- <span id="tuple1combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for Tuple1Combination<I>`

- <span id="tuple1combination-clone"></span>`fn clone(&self) -> Tuple1Combination<I>` — [`Tuple1Combination`](#tuple1combination)

##### `impl CloneToUninit for Tuple1Combination<I>`

- <span id="tuple1combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for Tuple1Combination<I>`

- <span id="tuple1combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple1Combination<I>`

- <span id="tuple1combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple1Combination<I>`

- <span id="tuple1combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple1Combination<I>`

##### `impl<I> IntoIterator for Tuple1Combination<I>`

- <span id="tuple1combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple1combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple1combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I: Iterator> Iterator for Tuple1Combination<I>`

- <span id="tuple1combination-iterator-type-item"></span>`type Item = (<I as Iterator>::Item)`

- <span id="tuple1combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple1combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple1combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple1combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple1Combination<I>`

##### `impl<FromA> MultiUnzip for Tuple1Combination<I>`

- <span id="tuple1combination-multiunzip"></span>`fn multiunzip(self) -> (FromA)`

##### `impl ToOwned for Tuple1Combination<I>`

- <span id="tuple1combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple1combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple1combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple1Combination<I>`

- <span id="tuple1combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple1combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple1Combination<I>`

- <span id="tuple1combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple1combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple2Combination<I: Iterator>`

```rust
struct Tuple2Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple1Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:825`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L825)*

#### Trait Implementations

##### `impl Any for Tuple2Combination<I>`

- <span id="tuple2combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple2Combination<I>`

- <span id="tuple2combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple2Combination<I>`

- <span id="tuple2combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple2Combination<I>`

- <span id="tuple2combination-clone"></span>`fn clone(&self) -> Tuple2Combination<I>` — [`Tuple2Combination`](#tuple2combination)

##### `impl CloneToUninit for Tuple2Combination<I>`

- <span id="tuple2combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple2Combination<I>`

- <span id="tuple2combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple2Combination<I>`

- <span id="tuple2combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple2Combination<I>`

- <span id="tuple2combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple2Combination<I>`

##### `impl<I> IntoIterator for Tuple2Combination<I>`

- <span id="tuple2combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple2combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple2combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple2Combination<I>`

- <span id="tuple2combination-iterator-type-item"></span>`type Item = (A, A)`

- <span id="tuple2combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple2combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple2combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple2combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple2Combination<I>`

##### `impl<FromA, FromB> MultiUnzip for Tuple2Combination<I>`

- <span id="tuple2combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB)`

##### `impl ToOwned for Tuple2Combination<I>`

- <span id="tuple2combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple2combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple2combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple2Combination<I>`

- <span id="tuple2combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple2combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple2Combination<I>`

- <span id="tuple2combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple2combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple3Combination<I: Iterator>`

```rust
struct Tuple3Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple2Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:826`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L826)*

#### Trait Implementations

##### `impl Any for Tuple3Combination<I>`

- <span id="tuple3combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple3Combination<I>`

- <span id="tuple3combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple3Combination<I>`

- <span id="tuple3combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple3Combination<I>`

- <span id="tuple3combination-clone"></span>`fn clone(&self) -> Tuple3Combination<I>` — [`Tuple3Combination`](#tuple3combination)

##### `impl CloneToUninit for Tuple3Combination<I>`

- <span id="tuple3combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple3Combination<I>`

- <span id="tuple3combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple3Combination<I>`

- <span id="tuple3combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple3Combination<I>`

- <span id="tuple3combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple3Combination<I>`

##### `impl<I> IntoIterator for Tuple3Combination<I>`

- <span id="tuple3combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple3combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple3combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple3Combination<I>`

- <span id="tuple3combination-iterator-type-item"></span>`type Item = (A, A, A)`

- <span id="tuple3combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple3combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple3combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple3combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple3Combination<I>`

##### `impl<FromA, FromB, FromC> MultiUnzip for Tuple3Combination<I>`

- <span id="tuple3combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC)`

##### `impl ToOwned for Tuple3Combination<I>`

- <span id="tuple3combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple3combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple3combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple3Combination<I>`

- <span id="tuple3combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple3combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple3Combination<I>`

- <span id="tuple3combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple3combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple4Combination<I: Iterator>`

```rust
struct Tuple4Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple3Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:827`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L827)*

#### Trait Implementations

##### `impl Any for Tuple4Combination<I>`

- <span id="tuple4combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple4Combination<I>`

- <span id="tuple4combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple4Combination<I>`

- <span id="tuple4combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple4Combination<I>`

- <span id="tuple4combination-clone"></span>`fn clone(&self) -> Tuple4Combination<I>` — [`Tuple4Combination`](#tuple4combination)

##### `impl CloneToUninit for Tuple4Combination<I>`

- <span id="tuple4combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple4Combination<I>`

- <span id="tuple4combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple4Combination<I>`

- <span id="tuple4combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple4Combination<I>`

- <span id="tuple4combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple4Combination<I>`

##### `impl<I> IntoIterator for Tuple4Combination<I>`

- <span id="tuple4combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple4combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple4combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple4Combination<I>`

- <span id="tuple4combination-iterator-type-item"></span>`type Item = (A, A, A, A)`

- <span id="tuple4combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple4combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple4combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple4combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple4Combination<I>`

##### `impl<FromA, FromB, FromC, FromD> MultiUnzip for Tuple4Combination<I>`

- <span id="tuple4combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD)`

##### `impl ToOwned for Tuple4Combination<I>`

- <span id="tuple4combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple4combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple4combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple4Combination<I>`

- <span id="tuple4combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple4combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple4Combination<I>`

- <span id="tuple4combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple4combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple5Combination<I: Iterator>`

```rust
struct Tuple5Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple4Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:828`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L828)*

#### Trait Implementations

##### `impl Any for Tuple5Combination<I>`

- <span id="tuple5combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple5Combination<I>`

- <span id="tuple5combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple5Combination<I>`

- <span id="tuple5combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple5Combination<I>`

- <span id="tuple5combination-clone"></span>`fn clone(&self) -> Tuple5Combination<I>` — [`Tuple5Combination`](#tuple5combination)

##### `impl CloneToUninit for Tuple5Combination<I>`

- <span id="tuple5combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple5Combination<I>`

- <span id="tuple5combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple5Combination<I>`

- <span id="tuple5combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple5Combination<I>`

- <span id="tuple5combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple5Combination<I>`

##### `impl<I> IntoIterator for Tuple5Combination<I>`

- <span id="tuple5combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple5combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple5combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple5Combination<I>`

- <span id="tuple5combination-iterator-type-item"></span>`type Item = (A, A, A, A, A)`

- <span id="tuple5combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple5combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple5combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple5combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple5Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE> MultiUnzip for Tuple5Combination<I>`

- <span id="tuple5combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE)`

##### `impl ToOwned for Tuple5Combination<I>`

- <span id="tuple5combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple5combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple5combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple5Combination<I>`

- <span id="tuple5combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple5combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple5Combination<I>`

- <span id="tuple5combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple5combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple6Combination<I: Iterator>`

```rust
struct Tuple6Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple5Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:829`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L829)*

#### Trait Implementations

##### `impl Any for Tuple6Combination<I>`

- <span id="tuple6combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple6Combination<I>`

- <span id="tuple6combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple6Combination<I>`

- <span id="tuple6combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple6Combination<I>`

- <span id="tuple6combination-clone"></span>`fn clone(&self) -> Tuple6Combination<I>` — [`Tuple6Combination`](#tuple6combination)

##### `impl CloneToUninit for Tuple6Combination<I>`

- <span id="tuple6combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple6Combination<I>`

- <span id="tuple6combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple6Combination<I>`

- <span id="tuple6combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple6Combination<I>`

- <span id="tuple6combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple6Combination<I>`

##### `impl<I> IntoIterator for Tuple6Combination<I>`

- <span id="tuple6combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple6combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple6combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple6Combination<I>`

- <span id="tuple6combination-iterator-type-item"></span>`type Item = (A, A, A, A, A, A)`

- <span id="tuple6combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple6combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple6combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple6combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple6Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE, FromF> MultiUnzip for Tuple6Combination<I>`

- <span id="tuple6combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF)`

##### `impl ToOwned for Tuple6Combination<I>`

- <span id="tuple6combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple6combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple6combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple6Combination<I>`

- <span id="tuple6combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple6combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple6Combination<I>`

- <span id="tuple6combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple6combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple7Combination<I: Iterator>`

```rust
struct Tuple7Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple6Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:830`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L830)*

#### Trait Implementations

##### `impl Any for Tuple7Combination<I>`

- <span id="tuple7combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple7Combination<I>`

- <span id="tuple7combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple7Combination<I>`

- <span id="tuple7combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple7Combination<I>`

- <span id="tuple7combination-clone"></span>`fn clone(&self) -> Tuple7Combination<I>` — [`Tuple7Combination`](#tuple7combination)

##### `impl CloneToUninit for Tuple7Combination<I>`

- <span id="tuple7combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple7Combination<I>`

- <span id="tuple7combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple7Combination<I>`

- <span id="tuple7combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple7Combination<I>`

- <span id="tuple7combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple7Combination<I>`

##### `impl<I> IntoIterator for Tuple7Combination<I>`

- <span id="tuple7combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple7combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple7combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple7Combination<I>`

- <span id="tuple7combination-iterator-type-item"></span>`type Item = (A, A, A, A, A, A, A)`

- <span id="tuple7combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple7combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple7combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple7combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple7Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE, FromF, FromG> MultiUnzip for Tuple7Combination<I>`

- <span id="tuple7combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG)`

##### `impl ToOwned for Tuple7Combination<I>`

- <span id="tuple7combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple7combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple7combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple7Combination<I>`

- <span id="tuple7combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple7combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple7Combination<I>`

- <span id="tuple7combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple7combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple8Combination<I: Iterator>`

```rust
struct Tuple8Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple7Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:831`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L831)*

#### Trait Implementations

##### `impl Any for Tuple8Combination<I>`

- <span id="tuple8combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple8Combination<I>`

- <span id="tuple8combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple8Combination<I>`

- <span id="tuple8combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple8Combination<I>`

- <span id="tuple8combination-clone"></span>`fn clone(&self) -> Tuple8Combination<I>` — [`Tuple8Combination`](#tuple8combination)

##### `impl CloneToUninit for Tuple8Combination<I>`

- <span id="tuple8combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple8Combination<I>`

- <span id="tuple8combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple8Combination<I>`

- <span id="tuple8combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple8Combination<I>`

- <span id="tuple8combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple8Combination<I>`

##### `impl<I> IntoIterator for Tuple8Combination<I>`

- <span id="tuple8combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple8combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple8combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple8Combination<I>`

- <span id="tuple8combination-iterator-type-item"></span>`type Item = (A, A, A, A, A, A, A, A)`

- <span id="tuple8combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple8combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple8combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple8combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple8Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH> MultiUnzip for Tuple8Combination<I>`

- <span id="tuple8combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH)`

##### `impl ToOwned for Tuple8Combination<I>`

- <span id="tuple8combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple8combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple8combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple8Combination<I>`

- <span id="tuple8combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple8combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple8Combination<I>`

- <span id="tuple8combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple8combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple9Combination<I: Iterator>`

```rust
struct Tuple9Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple8Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:832`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L832)*

#### Trait Implementations

##### `impl Any for Tuple9Combination<I>`

- <span id="tuple9combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple9Combination<I>`

- <span id="tuple9combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple9Combination<I>`

- <span id="tuple9combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple9Combination<I>`

- <span id="tuple9combination-clone"></span>`fn clone(&self) -> Tuple9Combination<I>` — [`Tuple9Combination`](#tuple9combination)

##### `impl CloneToUninit for Tuple9Combination<I>`

- <span id="tuple9combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple9Combination<I>`

- <span id="tuple9combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple9Combination<I>`

- <span id="tuple9combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple9Combination<I>`

- <span id="tuple9combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple9Combination<I>`

##### `impl<I> IntoIterator for Tuple9Combination<I>`

- <span id="tuple9combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple9combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple9combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple9Combination<I>`

- <span id="tuple9combination-iterator-type-item"></span>`type Item = (A, A, A, A, A, A, A, A, A)`

- <span id="tuple9combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple9combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple9combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple9combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple9Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, I, FromI> MultiUnzip for Tuple9Combination<I>`

- <span id="tuple9combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI)`

##### `impl ToOwned for Tuple9Combination<I>`

- <span id="tuple9combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple9combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple9combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple9Combination<I>`

- <span id="tuple9combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple9combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple9Combination<I>`

- <span id="tuple9combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple9combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple10Combination<I: Iterator>`

```rust
struct Tuple10Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple9Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:833`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L833)*

#### Trait Implementations

##### `impl Any for Tuple10Combination<I>`

- <span id="tuple10combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple10Combination<I>`

- <span id="tuple10combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple10Combination<I>`

- <span id="tuple10combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple10Combination<I>`

- <span id="tuple10combination-clone"></span>`fn clone(&self) -> Tuple10Combination<I>` — [`Tuple10Combination`](#tuple10combination)

##### `impl CloneToUninit for Tuple10Combination<I>`

- <span id="tuple10combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple10Combination<I>`

- <span id="tuple10combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple10Combination<I>`

- <span id="tuple10combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple10Combination<I>`

- <span id="tuple10combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple10Combination<I>`

##### `impl<I> IntoIterator for Tuple10Combination<I>`

- <span id="tuple10combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple10combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple10combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple10Combination<I>`

- <span id="tuple10combination-iterator-type-item"></span>`type Item = (A, A, A, A, A, A, A, A, A, A)`

- <span id="tuple10combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple10combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple10combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple10combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple10Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, I, FromI, FromJ> MultiUnzip for Tuple10Combination<I>`

- <span id="tuple10combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ)`

##### `impl ToOwned for Tuple10Combination<I>`

- <span id="tuple10combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple10combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple10combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple10Combination<I>`

- <span id="tuple10combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple10combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple10Combination<I>`

- <span id="tuple10combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple10combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple11Combination<I: Iterator>`

```rust
struct Tuple11Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple10Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:834`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L834)*

#### Trait Implementations

##### `impl Any for Tuple11Combination<I>`

- <span id="tuple11combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple11Combination<I>`

- <span id="tuple11combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple11Combination<I>`

- <span id="tuple11combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple11Combination<I>`

- <span id="tuple11combination-clone"></span>`fn clone(&self) -> Tuple11Combination<I>` — [`Tuple11Combination`](#tuple11combination)

##### `impl CloneToUninit for Tuple11Combination<I>`

- <span id="tuple11combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple11Combination<I>`

- <span id="tuple11combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple11Combination<I>`

- <span id="tuple11combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple11Combination<I>`

- <span id="tuple11combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple11Combination<I>`

##### `impl<I> IntoIterator for Tuple11Combination<I>`

- <span id="tuple11combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple11combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple11combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple11Combination<I>`

- <span id="tuple11combination-iterator-type-item"></span>`type Item = (A, A, A, A, A, A, A, A, A, A, A)`

- <span id="tuple11combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple11combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple11combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple11combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple11Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, I, FromI, FromJ, FromK> MultiUnzip for Tuple11Combination<I>`

- <span id="tuple11combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ, FromK)`

##### `impl ToOwned for Tuple11Combination<I>`

- <span id="tuple11combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple11combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple11combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple11Combination<I>`

- <span id="tuple11combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple11combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple11Combination<I>`

- <span id="tuple11combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple11combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Tuple12Combination<I: Iterator>`

```rust
struct Tuple12Combination<I: Iterator> {
    item: Option<<I as >::Item>,
    iter: I,
    c: Tuple11Combination<I>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:835`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L835)*

#### Trait Implementations

##### `impl Any for Tuple12Combination<I>`

- <span id="tuple12combination-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Tuple12Combination<I>`

- <span id="tuple12combination-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Tuple12Combination<I>`

- <span id="tuple12combination-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone + Iterator> Clone for Tuple12Combination<I>`

- <span id="tuple12combination-clone"></span>`fn clone(&self) -> Tuple12Combination<I>` — [`Tuple12Combination`](#tuple12combination)

##### `impl CloneToUninit for Tuple12Combination<I>`

- <span id="tuple12combination-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug + Iterator> Debug for Tuple12Combination<I>`

- <span id="tuple12combination-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Tuple12Combination<I>`

- <span id="tuple12combination-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Tuple12Combination<I>`

- <span id="tuple12combination-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Tuple12Combination<I>`

##### `impl<I> IntoIterator for Tuple12Combination<I>`

- <span id="tuple12combination-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tuple12combination-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tuple12combination-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for Tuple12Combination<I>`

- <span id="tuple12combination-iterator-type-item"></span>`type Item = (A, A, A, A, A, A, A, A, A, A, A, A)`

- <span id="tuple12combination-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tuple12combination-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tuple12combination-iterator-count"></span>`fn count(self) -> usize`

- <span id="tuple12combination-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

##### `impl Itertools for Tuple12Combination<I>`

##### `impl<FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, I, FromI, FromJ, FromK, FromL> MultiUnzip for Tuple12Combination<I>`

- <span id="tuple12combination-multiunzip"></span>`fn multiunzip(self) -> (FromA, FromB, FromC, FromD, FromE, FromF, FromG, FromH, FromI, FromJ, FromK, FromL)`

##### `impl ToOwned for Tuple12Combination<I>`

- <span id="tuple12combination-toowned-type-owned"></span>`type Owned = T`

- <span id="tuple12combination-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tuple12combination-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Tuple12Combination<I>`

- <span id="tuple12combination-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tuple12combination-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Tuple12Combination<I>`

- <span id="tuple12combination-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tuple12combination-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

- <span id="filterok-clone"></span>`fn clone(&self) -> FilterOk<I, F>` — [`FilterOk`](#filterok)

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

- <span id="filtermapok-clone"></span>`fn clone(&self) -> FilterMapOk<I, F>` — [`FilterMapOk`](#filtermapok)

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

- <span id="positions-clone"></span>`fn clone(&self) -> Positions<I, F>` — [`Positions`](#positions)

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

- <span id="update-clone"></span>`fn clone(&self) -> Update<I, F>` — [`Update`](#update)

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

### `CoalesceBy<I, F, C>`

```rust
struct CoalesceBy<I, F, C>
where
    I: Iterator,
    C: CountItem<<I as >::Item> {
    iter: I,
    last: Option<Option<<C as >::CItem>>,
    f: F,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:7-18`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L7-L18)*

#### Fields

- **`last`**: `Option<Option<<C as >::CItem>>`

  `last` is `None` while no item have been taken out of `iter` (at definition).
  Then `last` will be `Some(Some(item))` until `iter` is exhausted,
  in which case `last` will be `Some(None)`.

#### Trait Implementations

##### `impl Any for CoalesceBy<I, F, C>`

- <span id="coalesceby-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CoalesceBy<I, F, C>`

- <span id="coalesceby-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CoalesceBy<I, F, C>`

- <span id="coalesceby-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F, C> Clone for CoalesceBy<I, F, C>`

- <span id="coalesceby-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for CoalesceBy<I, F, C>`

- <span id="coalesceby-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F, C> Debug for CoalesceBy<I, F, C>`

- <span id="coalesceby-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for CoalesceBy<I, F, C>`

- <span id="coalesceby-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<I, F, C> FusedIterator for CoalesceBy<I, F, C>`

##### `impl<U> Into for CoalesceBy<I, F, C>`

- <span id="coalesceby-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for CoalesceBy<I, F, C>`

##### `impl<I> IntoIterator for CoalesceBy<I, F, C>`

- <span id="coalesceby-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="coalesceby-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="coalesceby-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I, F, C> Iterator for CoalesceBy<I, F, C>`

- <span id="coalesceby-iterator-type-item"></span>`type Item = <C as CountItem>::CItem`

- <span id="coalesceby-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="coalesceby-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="coalesceby-iterator-fold"></span>`fn fold<Acc, FnAcc>(self, acc: Acc, fn_acc: FnAcc) -> Acc`

##### `impl Itertools for CoalesceBy<I, F, C>`

##### `impl MultiUnzip for CoalesceBy<I, F, C>`

- <span id="coalesceby-multiunzip"></span>`fn multiunzip(self)`

##### `impl ToOwned for CoalesceBy<I, F, C>`

- <span id="coalesceby-toowned-type-owned"></span>`type Owned = T`

- <span id="coalesceby-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="coalesceby-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CoalesceBy<I, F, C>`

- <span id="coalesceby-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="coalesceby-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CoalesceBy<I, F, C>`

- <span id="coalesceby-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="coalesceby-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NoCount`

```rust
struct NoCount;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:113`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L113)*

#### Trait Implementations

##### `impl Any for NoCount`

- <span id="nocount-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoCount`

- <span id="nocount-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoCount`

- <span id="nocount-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> CountItem for NoCount`

- <span id="nocount-countitem-type-citem"></span>`type CItem = T`

- <span id="nocount-countitem-new"></span>`fn new(t: T) -> T`

##### `impl<T> From for NoCount`

- <span id="nocount-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoCount`

- <span id="nocount-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for NoCount`

##### `impl<U> TryFrom for NoCount`

- <span id="nocount-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nocount-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoCount`

- <span id="nocount-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nocount-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WithCount`

```rust
struct WithCount;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:115`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L115)*

#### Trait Implementations

##### `impl Any for WithCount`

- <span id="withcount-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WithCount`

- <span id="withcount-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WithCount`

- <span id="withcount-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> CountItem for WithCount`

- <span id="withcount-countitem-type-citem"></span>`type CItem = (usize, T)`

- <span id="withcount-countitem-new"></span>`fn new(t: T) -> (usize, T)`

##### `impl<T> From for WithCount`

- <span id="withcount-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WithCount`

- <span id="withcount-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for WithCount`

##### `impl<U> TryFrom for WithCount`

- <span id="withcount-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="withcount-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WithCount`

- <span id="withcount-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="withcount-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DedupPred2CoalescePred<DP>`

```rust
struct DedupPred2CoalescePred<DP>(DP);
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:170`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L170)*

#### Trait Implementations

##### `impl Any for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<DP: clone::Clone> Clone for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-clone"></span>`fn clone(&self) -> DedupPred2CoalescePred<DP>` — [`DedupPred2CoalescePred`](#deduppred2coalescepred)

##### `impl CloneToUninit for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<DP, T> CoalescePredicate for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-coalescepredicate-coalesce-pair"></span>`fn coalesce_pair(&mut self, t: T, item: T) -> Result<T, (T, T)>`

##### `impl<DP> Debug for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for DedupPred2CoalescePred<DP>`

##### `impl ToOwned for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-toowned-type-owned"></span>`type Owned = T`

- <span id="deduppred2coalescepred-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="deduppred2coalescepred-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deduppred2coalescepred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deduppred2coalescepred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DedupEq`

```rust
struct DedupEq;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:195`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L195)*

#### Trait Implementations

##### `impl Any for DedupEq`

- <span id="dedupeq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DedupEq`

- <span id="dedupeq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DedupEq`

- <span id="dedupeq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DedupEq`

- <span id="dedupeq-clone"></span>`fn clone(&self) -> DedupEq` — [`DedupEq`](#dedupeq)

##### `impl CloneToUninit for DedupEq`

- <span id="dedupeq-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DedupEq`

- <span id="dedupeq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: PartialEq> DedupPredicate for DedupEq`

- <span id="dedupeq-deduppredicate-dedup-pair"></span>`fn dedup_pair(&mut self, a: &T, b: &T) -> bool`

##### `impl<T> From for DedupEq`

- <span id="dedupeq-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DedupEq`

- <span id="dedupeq-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for DedupEq`

##### `impl ToOwned for DedupEq`

- <span id="dedupeq-toowned-type-owned"></span>`type Owned = T`

- <span id="dedupeq-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dedupeq-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DedupEq`

- <span id="dedupeq-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dedupeq-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DedupEq`

- <span id="dedupeq-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dedupeq-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DedupPredWithCount2CoalescePred<DP>`

```rust
struct DedupPredWithCount2CoalescePred<DP>(DP);
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:243`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L243)*

#### Trait Implementations

##### `impl Any for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<DP: clone::Clone> Clone for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-clone"></span>`fn clone(&self) -> DedupPredWithCount2CoalescePred<DP>` — [`DedupPredWithCount2CoalescePred`](#deduppredwithcount2coalescepred)

##### `impl CloneToUninit for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<DP, T> CoalescePredicate for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-coalescepredicate-coalesce-pair"></span>`fn coalesce_pair(&mut self, (c, t): (usize, T), item: T) -> Result<(usize, T), ((usize, T), (usize, T))>`

##### `impl<DP: fmt::Debug> Debug for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for DedupPredWithCount2CoalescePred<DP>`

##### `impl ToOwned for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-toowned-type-owned"></span>`type Owned = T`

- <span id="deduppredwithcount2coalescepred-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="deduppredwithcount2coalescepred-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="deduppredwithcount2coalescepred-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="deduppredwithcount2coalescepred-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

- <span id="multiproduct-clone"></span>`fn clone(&self) -> MultiProduct<I>` — [`MultiProduct`](#multiproduct)

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

### `MultiProductInner<I>`

```rust
struct MultiProductInner<I>
where
    I: Iterator + Clone,
    <I as >::Item: Clone {
    iters: alloc::vec::Vec<MultiProductIter<I>>,
    cur: Option<alloc::vec::Vec<<I as >::Item>>,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:25-34`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/multi_product.rs#L25-L34)*

Internals for `MultiProduct`.

#### Fields

- **`iters`**: `alloc::vec::Vec<MultiProductIter<I>>`

  Holds the iterators.

- **`cur`**: `Option<alloc::vec::Vec<<I as >::Item>>`

  Not populated at the beginning then it holds the current item of each iterator.

#### Trait Implementations

##### `impl Any for MultiProductInner<I>`

- <span id="multiproductinner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProductInner<I>`

- <span id="multiproductinner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProductInner<I>`

- <span id="multiproductinner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiProductInner<I>`

- <span id="multiproductinner-clone"></span>`fn clone(&self) -> MultiProductInner<I>` — [`MultiProductInner`](multi_product/index.md#multiproductinner)

##### `impl CloneToUninit for MultiProductInner<I>`

- <span id="multiproductinner-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiProductInner<I>`

- <span id="multiproductinner-debug-fmt"></span>`fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result`

##### `impl<T> From for MultiProductInner<I>`

- <span id="multiproductinner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProductInner<I>`

- <span id="multiproductinner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiProductInner<I>`

##### `impl ToOwned for MultiProductInner<I>`

- <span id="multiproductinner-toowned-type-owned"></span>`type Owned = T`

- <span id="multiproductinner-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiproductinner-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProductInner<I>`

- <span id="multiproductinner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiproductinner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProductInner<I>`

- <span id="multiproductinner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiproductinner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MultiProductIter<I>`

```rust
struct MultiProductIter<I>
where
    I: Iterator + Clone,
    <I as >::Item: Clone {
    iter: I,
    iter_orig: I,
}
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:74-81`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/multi_product.rs#L74-L81)*

Holds the state of a single iterator within a `MultiProduct`.

#### Implementations

- <span id="multiproductiter-new"></span>`fn new(iter: I) -> Self`

#### Trait Implementations

##### `impl Any for MultiProductIter<I>`

- <span id="multiproductiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MultiProductIter<I>`

- <span id="multiproductiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MultiProductIter<I>`

- <span id="multiproductiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for MultiProductIter<I>`

- <span id="multiproductiter-clone"></span>`fn clone(&self) -> MultiProductIter<I>` — [`MultiProductIter`](multi_product/index.md#multiproductiter)

##### `impl CloneToUninit for MultiProductIter<I>`

- <span id="multiproductiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for MultiProductIter<I>`

- <span id="multiproductiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for MultiProductIter<I>`

- <span id="multiproductiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MultiProductIter<I>`

- <span id="multiproductiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MultiProductIter<I>`

##### `impl ToOwned for MultiProductIter<I>`

- <span id="multiproductiter-toowned-type-owned"></span>`type Owned = T`

- <span id="multiproductiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="multiproductiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MultiProductIter<I>`

- <span id="multiproductiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="multiproductiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MultiProductIter<I>`

- <span id="multiproductiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="multiproductiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `HasCombination<I>`

```rust
trait HasCombination<I>: Sized { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:625-627`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L625-L627)*

#### Associated Types

- `type Combination: 2`

#### Implementors

- `(<I as >::Item)`
- `(A, A)`
- `(A, A, A)`
- `(A, A, A, A)`
- `(A, A, A, A, A)`
- `(A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A, A, A)`
- `(A, A, A, A, A, A, A, A, A, A, A, A)`

### `CoalescePredicate<Item, T>`

```rust
trait CoalescePredicate<Item, T> { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:39-41`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L39-L41)*

#### Required Methods

- `fn coalesce_pair(&mut self, t: T, item: Item) -> Result<T, (T, T)>`

#### Implementors

- [`DedupPred2CoalescePred`](#deduppred2coalescepred)
- [`DedupPredWithCount2CoalescePred`](#deduppredwithcount2coalescepred)
- `F`

### `CountItem<T>`

```rust
trait CountItem<T> { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:117-120`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L117-L120)*

#### Associated Types

- `type CItem`

#### Required Methods

- `fn new(t: T) -> <Self as >::CItem`

#### Implementors

- [`NoCount`](#nocount)
- [`WithCount`](#withcount)

### `DedupPredicate<T>`

```rust
trait DedupPredicate<T> { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:176-179`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L176-L179)*

#### Required Methods

- `fn dedup_pair(&mut self, a: &T, b: &T) -> bool`

#### Implementors

- [`DedupEq`](#dedupeq)
- `F`

## Functions

### `map_into`

```rust
fn map_into<I, R>(iter: I) -> MapInto<I, R>
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:125-130`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/map.rs#L125-L130)*

Create a new [`MapInto`](map/index.md) iterator.

### `map_ok`

```rust
fn map_ok<I, F, T, U, E>(iter: I, f: F) -> MapOk<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> U
```

*Defined in [`itertools-0.14.0/src/adaptors/map.rs:88-97`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/map.rs#L88-L97)*

Create a new `MapOk` iterator.

### `interleave`

```rust
fn interleave<I, J>(i: I, j: J) -> Interleave<<I as IntoIterator>::IntoIter, <J as IntoIterator>::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator<Item = <I as >::Item>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:37-50`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L37-L50)*

Create an iterator that interleaves elements in `i` and `j`.

`IntoIterator` enabled version of [`Itertools::interleave`](crate::Itertools::interleave).

### `interleave_shortest`

```rust
fn interleave_shortest<I, J>(i: I, j: J) -> InterleaveShortest<I, J>
where
    I: Iterator,
    J: Iterator<Item = <I as >::Item>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:134-144`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L134-L144)*

Create a new `InterleaveShortest` iterator.

### `put_back`

```rust
fn put_back<I>(iterable: I) -> PutBack<<I as >::IntoIter>
where
    I: IntoIterator
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:252-260`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L252-L260)*

Create an iterator where you can put back a single item

### `cartesian_product`

```rust
fn cartesian_product<I, J>(i: I, j: J) -> Product<I, J>
where
    I: Iterator,
    J: Clone + Iterator,
    <I as >::Item: Clone
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:376-388`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L376-L388)*

Create a new cartesian product iterator

Iterator element type is `(I::Item, J::Item)`.

### `batching`

```rust
fn batching<I, F>(iter: I, f: F) -> Batching<I, F>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:492-494`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L492-L494)*

Create a new Batching iterator.

### `take_while_ref`

```rust
fn take_while_ref<I, F>(iter: &mut I, f: F) -> TakeWhileRef<'_, I, F>
where
    I: Iterator + Clone
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:526-531`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L526-L531)*

Create a new `TakeWhileRef` from a reference to clonable iterator.

### `while_some`

```rust
fn while_some<I>(iter: I) -> WhileSome<I>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:571-573`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L571-L573)*

Create a new `WhileSome<I>`.

### `tuple_combinations`

```rust
fn tuple_combinations<T, I>(iter: I) -> TupleCombinations<I, T>
where
    I: Iterator + Clone,
    <I as >::Item: Clone,
    T: HasCombination<I>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:630-640`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L630-L640)*

Create a new `TupleCombinations` from a clonable iterator.

### `checked_binomial`

```rust
fn checked_binomial(n: usize, k: usize) -> Option<usize>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:838-852`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L838-L852)*

### `filter_ok`

```rust
fn filter_ok<I, F, T, E>(iter: I, f: F) -> FilterOk<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(&T) -> bool
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:891-897`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L891-L897)*

Create a new `FilterOk` iterator.

### `transpose_result`

```rust
fn transpose_result<T, E>(result: Result<Option<T>, E>) -> Option<Result<T, E>>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:987-993`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L987-L993)*

### `filter_map_ok`

```rust
fn filter_map_ok<I, F, T, U, E>(iter: I, f: F) -> FilterMapOk<I, F>
where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(T) -> Option<U>
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:996-1002`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L996-L1002)*

Create a new `FilterOk` iterator.

### `positions`

```rust
fn positions<I, F>(iter: I, f: F) -> Positions<I, F>
where
    I: Iterator,
    F: FnMut(<I as >::Item) -> bool
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:1093-1100`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L1093-L1100)*

Create a new `Positions` iterator.

### `update`

```rust
fn update<I, F>(iter: I, f: F) -> Update<I, F>
where
    I: Iterator,
    F: FnMut(&mut <I as >::Item)
```

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:1184-1190`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L1184-L1190)*

Create a new `Update` iterator.

### `coalesce`

```rust
fn coalesce<I, F>(iter: I, f: F) -> Coalesce<I, F>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:153-162`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L153-L162)*

Create a new `Coalesce`.

### `dedup_by`

```rust
fn dedup_by<I, Pred>(iter: I, dedup_pred: Pred) -> DedupBy<I, Pred>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:210-219`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L210-L219)*

Create a new `DedupBy`.

### `dedup`

```rust
fn dedup<I>(iter: I) -> Dedup<I>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:227-232`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L227-L232)*

Create a new `Dedup`.

### `dedup_by_with_count`

```rust
fn dedup_by_with_count<I, Pred>(iter: I, dedup_pred: Pred) -> DedupByWithCount<I, Pred>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:269-278`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L269-L278)*

Create a new `DedupByWithCount`.

### `dedup_with_count`

```rust
fn dedup_with_count<I>(iter: I) -> DedupWithCount<I>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:281-286`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L281-L286)*

Create a new `DedupWithCount`.

### `multi_cartesian_product`

```rust
fn multi_cartesian_product<H>(iters: H) -> MultiProduct<<<H as >::Item as IntoIterator>::IntoIter>
where
    H: Iterator,
    <H as >::Item: IntoIterator,
    <<H as >::Item as IntoIterator>::IntoIter: Clone,
    <<H as >::Item as IntoIterator>::Item: Clone
```

*Defined in [`itertools-0.14.0/src/adaptors/multi_product.rs:56-70`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/multi_product.rs#L56-L70)*

Create a new cartesian product iterator over an arbitrary number
of iterators of the same type.

Iterator element is of type `Vec<H::Item::Item>`.

## Type Aliases

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

### `Coalesce<I, F>`

```rust
type Coalesce<I, F> = CoalesceBy<I, F, NoCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:141`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L141)*

An iterator adaptor that may join together adjacent elements.

See [`.coalesce()`](crate::Itertools::coalesce) for more information.

### `DedupBy<I, Pred>`

```rust
type DedupBy<I, Pred> = CoalesceBy<I, DedupPred2CoalescePred<Pred>, NoCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:167`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L167)*

An iterator adaptor that removes repeated duplicates, determining equality using a comparison function.

See [`.dedup_by()`](crate::Itertools::dedup_by) or [`.dedup()`](crate::Itertools::dedup) for more information.

### `Dedup<I>`

```rust
type Dedup<I> = DedupBy<I, DedupEq>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:224`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/coalesce.rs#L224)*

An iterator adaptor that removes repeated duplicates.

See [`.dedup()`](crate::Itertools::dedup) for more information.

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

## Macros

### `impl_tuple_combination!`

*Defined in [`itertools-0.14.0/src/adaptors/mod.rs:714-811`](../../../.source_1765894658/itertools-0.14.0/src/adaptors/mod.rs#L714-L811)*

