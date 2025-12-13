*[regex_syntax](../../index.md) / [hir](../index.md) / [interval](index.md)*

---

# Module `interval`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntervalSet`](#intervalset) | struct |  |
| [`IntervalSetIter`](#intervalsetiter) | struct | An iterator over intervals. |
| [`Interval`](#interval) | trait |  |
| [`Bound`](#bound) | trait |  |

## Structs

### `IntervalSet<I>`

```rust
struct IntervalSet<I> {
    ranges: alloc::vec::Vec<I>,
    folded: bool,
}
```

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:34-54`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/interval.rs#L34-L54)*

#### Fields

- **`ranges`**: `alloc::vec::Vec<I>`

  A sorted set of non-overlapping ranges.

- **`folded`**: `bool`

  While not required at all for correctness, we keep track of whether an
  interval set has been case folded or not. This helps us avoid doing
  redundant work if, for example, a set has already been cased folded.
  And note that whether a set is folded or not is preserved through
  all of the pairwise set operations. That is, if both interval sets
  have been case folded, then any of difference, union, intersection or
  symmetric difference all produce a case folded set.
  
  Note that when this is true, it *must* be the case that the set is case
  folded. But when it's false, the set *may* be case folded. In other
  words, we only set this to true when we know it to be case, but we're
  okay with it being false if it would otherwise be costly to determine
  whether it should be true. This means code cannot assume that a false
  value necessarily indicates that the set is not case folded.
  
  Bottom line: this is a performance optimization.

#### Implementations

- <span id="intervalset-new"></span>`fn new<T: IntoIterator<Item = I>>(intervals: T) -> IntervalSet<I>` — [`IntervalSet`](#intervalset)

  Create a new set from a sequence of intervals. Each interval is

  specified as a pair of bounds, where both bounds are inclusive.

  

  The given ranges do not need to be in any specific order, and ranges

  may overlap.

- <span id="intervalset-push"></span>`fn push(&mut self, interval: I)`

  Add a new interval to this set.

- <span id="intervalset-iter"></span>`fn iter(&self) -> IntervalSetIter<'_, I>` — [`IntervalSetIter`](#intervalsetiter)

  Return an iterator over all intervals in this set.

  

  The iterator yields intervals in ascending order.

- <span id="intervalset-intervals"></span>`fn intervals(&self) -> &[I]`

  Return an immutable slice of intervals in this set.

  

  The sequence returned is in canonical ordering.

- <span id="intervalset-case-fold-simple"></span>`fn case_fold_simple(&mut self) -> Result<(), unicode::CaseFoldError>` — [`CaseFoldError`](../../unicode/index.md#casefolderror)

  Expand this interval set such that it contains all case folded

  characters. For example, if this class consists of the range `a-z`,

  then applying case folding will result in the class containing both the

  ranges `a-z` and `A-Z`.

  

  This returns an error if the necessary case mapping data is not

  available.

- <span id="intervalset-union"></span>`fn union(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

  Union this set with the given set, in place.

- <span id="intervalset-intersect"></span>`fn intersect(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

  Intersect this set with the given set, in place.

- <span id="intervalset-difference"></span>`fn difference(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

  Subtract the given set from this set, in place.

- <span id="intervalset-symmetric-difference"></span>`fn symmetric_difference(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

  Compute the symmetric difference of the two sets, in place.

  

  This computes the symmetric difference of two interval sets. This

  removes all elements in this set that are also in the given set,

  but also adds all elements from the given set that aren't in this

  set. That is, the set will contain all elements in either set,

  but will not contain any elements that are in both sets.

- <span id="intervalset-negate"></span>`fn negate(&mut self)`

  Negate this interval set.

  

  For all `x` where `x` is any element, if `x` was in this set, then it

  will not be in this set after negation.

- <span id="intervalset-canonicalize"></span>`fn canonicalize(&mut self)`

  Converts this set into a canonical ordering.

- <span id="intervalset-is-canonical"></span>`fn is_canonical(&self) -> bool`

  Returns true if and only if this class is in a canonical ordering.

#### Trait Implementations

##### `impl Any for IntervalSet<I>`

- <span id="intervalset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntervalSet<I>`

- <span id="intervalset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntervalSet<I>`

- <span id="intervalset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: clone::Clone> Clone for IntervalSet<I>`

- <span id="intervalset-clone"></span>`fn clone(&self) -> IntervalSet<I>` — [`IntervalSet`](#intervalset)

##### `impl CloneToUninit for IntervalSet<I>`

- <span id="intervalset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I: fmt::Debug> Debug for IntervalSet<I>`

- <span id="intervalset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Interval> Eq for IntervalSet<I>`

##### `impl<T> From for IntervalSet<I>`

- <span id="intervalset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntervalSet<I>`

- <span id="intervalset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I: Interval> PartialEq for IntervalSet<I>`

- <span id="intervalset-partialeq-eq"></span>`fn eq(&self, other: &IntervalSet<I>) -> bool` — [`IntervalSet`](#intervalset)

##### `impl ToOwned for IntervalSet<I>`

- <span id="intervalset-toowned-type-owned"></span>`type Owned = T`

- <span id="intervalset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="intervalset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for IntervalSet<I>`

- <span id="intervalset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intervalset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntervalSet<I>`

- <span id="intervalset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intervalset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `IntervalSetIter<'a, I>`

```rust
struct IntervalSetIter<'a, I>(slice::Iter<'a, I>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:386`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/interval.rs#L386)*

An iterator over intervals.

#### Trait Implementations

##### `impl Any for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I: fmt::Debug> Debug for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intervalsetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intervalsetiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<I> Iterator for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-iterator-type-item"></span>`type Item = &'a I`

- <span id="intervalsetiter-iterator-next"></span>`fn next(&mut self) -> Option<&'a I>`

##### `impl<U> TryFrom for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="intervalsetiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="intervalsetiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Interval`

```rust
trait Interval: Clone + Copy + Debug + Default + Eq + PartialEq + PartialOrd + Ord { ... }
```

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:396-508`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/interval.rs#L396-L508)*

#### Associated Types

- `type Bound: 1`

#### Required Methods

- `fn lower(&self) -> <Self as >::Bound`

- `fn upper(&self) -> <Self as >::Bound`

- `fn set_lower(&mut self, bound: <Self as >::Bound)`

- `fn set_upper(&mut self, bound: <Self as >::Bound)`

- `fn case_fold_simple(&self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError>`

#### Provided Methods

- `fn create(lower: <Self as >::Bound, upper: <Self as >::Bound) -> Self`

  Create a new interval.

- `fn union(&self, other: &Self) -> Option<Self>`

  Union the given overlapping range into this range.

- `fn intersect(&self, other: &Self) -> Option<Self>`

  Intersect this range with the given range and return the result.

- `fn difference(&self, other: &Self) -> (Option<Self>, Option<Self>)`

  Subtract the given range from this range and return the resulting

- `fn is_contiguous(&self, other: &Self) -> bool`

  Returns true if and only if the two ranges are contiguous. Two ranges

- `fn is_intersection_empty(&self, other: &Self) -> bool`

  Returns true if and only if the intersection of this range and the

- `fn is_subset(&self, other: &Self) -> bool`

  Returns true if and only if this range is a subset of the other range.

#### Implementors

- [`ClassBytesRange`](../index.md#classbytesrange)
- [`ClassUnicodeRange`](../index.md#classunicoderange)

### `Bound`

```rust
trait Bound: Copy + Clone + Debug + Eq + PartialEq + PartialOrd + Ord { ... }
```

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:510-518`](../../../../.source_1765521767/regex-syntax-0.8.8/src/hir/interval.rs#L510-L518)*

#### Required Methods

- `fn min_value() -> Self`

- `fn max_value() -> Self`

- `fn as_u32(self) -> u32`

- `fn increment(self) -> Self`

- `fn decrement(self) -> Self`

#### Implementors

- `char`
- `u8`

