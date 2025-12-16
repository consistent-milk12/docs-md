*[itertools](../../index.md) / [adaptors](../index.md) / [coalesce](index.md)*

---

# Module `coalesce`

## Contents

- [Structs](#structs)
  - [`CoalesceBy`](#coalesceby)
  - [`NoCount`](#nocount)
  - [`WithCount`](#withcount)
  - [`DedupPred2CoalescePred`](#deduppred2coalescepred)
  - [`DedupEq`](#dedupeq)
  - [`DedupPredWithCount2CoalescePred`](#deduppredwithcount2coalescepred)
- [Traits](#traits)
  - [`CoalescePredicate`](#coalescepredicate)
  - [`CountItem`](#countitem)
  - [`DedupPredicate`](#deduppredicate)
- [Functions](#functions)
  - [`coalesce`](#coalesce)
  - [`dedup_by`](#dedup-by)
  - [`dedup`](#dedup)
  - [`dedup_by_with_count`](#dedup-by-with-count)
  - [`dedup_with_count`](#dedup-with-count)
- [Type Aliases](#type-aliases)
  - [`Coalesce`](#coalesce)
  - [`DedupBy`](#dedupby)
  - [`Dedup`](#dedup)
  - [`DedupByWithCount`](#dedupbywithcount)
  - [`DedupWithCount`](#dedupwithcount)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CoalesceBy`](#coalesceby) | struct |  |
| [`NoCount`](#nocount) | struct |  |
| [`WithCount`](#withcount) | struct |  |
| [`DedupPred2CoalescePred`](#deduppred2coalescepred) | struct |  |
| [`DedupEq`](#dedupeq) | struct |  |
| [`DedupPredWithCount2CoalescePred`](#deduppredwithcount2coalescepred) | struct |  |
| [`CoalescePredicate`](#coalescepredicate) | trait |  |
| [`CountItem`](#countitem) | trait |  |
| [`DedupPredicate`](#deduppredicate) | trait |  |
| [`coalesce`](#coalesce) | fn | Create a new `Coalesce`. |
| [`dedup_by`](#dedup-by) | fn | Create a new `DedupBy`. |
| [`dedup`](#dedup) | fn | Create a new `Dedup`. |
| [`dedup_by_with_count`](#dedup-by-with-count) | fn | Create a new `DedupByWithCount`. |
| [`dedup_with_count`](#dedup-with-count) | fn | Create a new `DedupWithCount`. |
| [`Coalesce`](#coalesce) | type | An iterator adaptor that may join together adjacent elements. |
| [`DedupBy`](#dedupby) | type | An iterator adaptor that removes repeated duplicates, determining equality using a comparison function. |
| [`Dedup`](#dedup) | type | An iterator adaptor that removes repeated duplicates. |
| [`DedupByWithCount`](#dedupbywithcount) | type | An iterator adaptor that removes repeated duplicates, while keeping a count of how many repeated elements were present. |
| [`DedupWithCount`](#dedupwithcount) | type | An iterator adaptor that removes repeated duplicates, while keeping a count of how many repeated elements were present. |

## Structs

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

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:7-18`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L7-L18)*

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

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:113`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L113)*

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

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:115`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L115)*

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

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:170`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L170)*

#### Trait Implementations

##### `impl Any for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<DP: clone::Clone> Clone for DedupPred2CoalescePred<DP>`

- <span id="deduppred2coalescepred-clone"></span>`fn clone(&self) -> DedupPred2CoalescePred<DP>` — [`DedupPred2CoalescePred`](../index.md#deduppred2coalescepred)

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

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:195`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L195)*

#### Trait Implementations

##### `impl Any for DedupEq`

- <span id="dedupeq-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DedupEq`

- <span id="dedupeq-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DedupEq`

- <span id="dedupeq-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DedupEq`

- <span id="dedupeq-clone"></span>`fn clone(&self) -> DedupEq` — [`DedupEq`](../index.md#dedupeq)

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

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:243`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L243)*

#### Trait Implementations

##### `impl Any for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<DP: clone::Clone> Clone for DedupPredWithCount2CoalescePred<DP>`

- <span id="deduppredwithcount2coalescepred-clone"></span>`fn clone(&self) -> DedupPredWithCount2CoalescePred<DP>` — [`DedupPredWithCount2CoalescePred`](../index.md#deduppredwithcount2coalescepred)

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

## Traits

### `CoalescePredicate<Item, T>`

```rust
trait CoalescePredicate<Item, T> { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:39-41`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L39-L41)*

#### Required Methods

- `fn CoalescePredicate::coalesce_pair(&mut self, t: T, item: Item) -> Result<T, (T, T)>`

#### Implementors

- [`DedupPred2CoalescePred`](../index.md#deduppred2coalescepred)
- [`DedupPredWithCount2CoalescePred`](../index.md#deduppredwithcount2coalescepred)
- `F`

### `CountItem<T>`

```rust
trait CountItem<T> { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:117-120`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L117-L120)*

#### Associated Types

- `type CItem`

#### Required Methods

- `fn CountItem::new(t: T) -> <Self as >::CItem`

#### Implementors

- [`NoCount`](../index.md#nocount)
- [`WithCount`](../index.md#withcount)

### `DedupPredicate<T>`

```rust
trait DedupPredicate<T> { ... }
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:176-179`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L176-L179)*

#### Required Methods

- `fn DedupPredicate::dedup_pair(&mut self, a: &T, b: &T) -> bool`

#### Implementors

- [`DedupEq`](../index.md#dedupeq)
- `F`

## Functions

### `coalesce`

```rust
fn coalesce<I, F>(iter: I, f: F) -> Coalesce<I, F>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:153-162`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L153-L162)*

Create a new `Coalesce`.

### `dedup_by`

```rust
fn dedup_by<I, Pred>(iter: I, dedup_pred: Pred) -> DedupBy<I, Pred>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:210-219`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L210-L219)*

Create a new `DedupBy`.

### `dedup`

```rust
fn dedup<I>(iter: I) -> Dedup<I>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:227-232`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L227-L232)*

Create a new `Dedup`.

### `dedup_by_with_count`

```rust
fn dedup_by_with_count<I, Pred>(iter: I, dedup_pred: Pred) -> DedupByWithCount<I, Pred>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:269-278`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L269-L278)*

Create a new `DedupByWithCount`.

### `dedup_with_count`

```rust
fn dedup_with_count<I>(iter: I) -> DedupWithCount<I>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:281-286`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L281-L286)*

Create a new `DedupWithCount`.

## Type Aliases

### `Coalesce<I, F>`

```rust
type Coalesce<I, F> = CoalesceBy<I, F, NoCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:141`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L141)*

An iterator adaptor that may join together adjacent elements.

See [`.coalesce()`](crate::Itertools::coalesce) for more information.

### `DedupBy<I, Pred>`

```rust
type DedupBy<I, Pred> = CoalesceBy<I, DedupPred2CoalescePred<Pred>, NoCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:167`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L167)*

An iterator adaptor that removes repeated duplicates, determining equality using a comparison function.

See [`.dedup_by()`](crate::Itertools::dedup_by) or [`.dedup()`](crate::Itertools::dedup) for more information.

### `Dedup<I>`

```rust
type Dedup<I> = DedupBy<I, DedupEq>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:224`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L224)*

An iterator adaptor that removes repeated duplicates.

See [`.dedup()`](crate::Itertools::dedup) for more information.

### `DedupByWithCount<I, Pred>`

```rust
type DedupByWithCount<I, Pred> = CoalesceBy<I, DedupPredWithCount2CoalescePred<Pred>, WithCount>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:239-240`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L239-L240)*

An iterator adaptor that removes repeated duplicates, while keeping a count of how many
repeated elements were present. This will determine equality using a comparison function.

See [`.dedup_by_with_count()`](crate::Itertools::dedup_by_with_count) or
[`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information.

### `DedupWithCount<I>`

```rust
type DedupWithCount<I> = DedupByWithCount<I, DedupEq>;
```

*Defined in [`itertools-0.14.0/src/adaptors/coalesce.rs:266`](../../../../.source_1765900590/itertools-0.14.0/src/adaptors/coalesce.rs#L266)*

An iterator adaptor that removes repeated duplicates, while keeping a count of how many
repeated elements were present.

See [`.dedup_with_count()`](crate::Itertools::dedup_with_count) for more information.

