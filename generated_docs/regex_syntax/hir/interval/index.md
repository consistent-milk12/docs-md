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

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:34-54`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/interval.rs#L34-L54)*

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

- <span id="intervalset-push"></span>`fn push(&mut self, interval: I)`

- <span id="intervalset-iter"></span>`fn iter(&self) -> IntervalSetIter<'_, I>` — [`IntervalSetIter`](#intervalsetiter)

- <span id="intervalset-intervals"></span>`fn intervals(&self) -> &[I]`

- <span id="intervalset-case-fold-simple"></span>`fn case_fold_simple(&mut self) -> Result<(), unicode::CaseFoldError>` — [`CaseFoldError`](../../unicode/index.md)

- <span id="intervalset-union"></span>`fn union(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- <span id="intervalset-intersect"></span>`fn intersect(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- <span id="intervalset-difference"></span>`fn difference(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- <span id="intervalset-symmetric-difference"></span>`fn symmetric_difference(&mut self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- <span id="intervalset-negate"></span>`fn negate(&mut self)`

- <span id="intervalset-canonicalize"></span>`fn canonicalize(&mut self)`

- <span id="intervalset-is-canonical"></span>`fn is_canonical(&self) -> bool`

#### Trait Implementations

##### `impl<I: clone::Clone> Clone for IntervalSet<I>`

- <span id="intervalset-clone"></span>`fn clone(&self) -> IntervalSet<I>` — [`IntervalSet`](#intervalset)

##### `impl<I: fmt::Debug> Debug for IntervalSet<I>`

- <span id="intervalset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I: Interval> Eq for IntervalSet<I>`

##### `impl<I: Interval> PartialEq for IntervalSet<I>`

- <span id="intervalset-eq"></span>`fn eq(&self, other: &IntervalSet<I>) -> bool` — [`IntervalSet`](#intervalset)

### `IntervalSetIter<'a, I>`

```rust
struct IntervalSetIter<'a, I>(slice::Iter<'a, I>);
```

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:386`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/interval.rs#L386)*

An iterator over intervals.

#### Trait Implementations

##### `impl<'a, I: fmt::Debug> Debug for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intervalsetiter-type-intoiter"></span>`type IntoIter = I`

- <span id="intervalsetiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a, I> Iterator for IntervalSetIter<'a, I>`

- <span id="intervalsetiter-type-item"></span>`type Item = &'a I`

- <span id="intervalsetiter-next"></span>`fn next(&mut self) -> Option<&'a I>`

## Traits

### `Interval`

```rust
trait Interval: Clone + Copy + Debug + Default + Eq + PartialEq + PartialOrd + Ord { ... }
```

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:396-508`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/interval.rs#L396-L508)*

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

- [`ClassBytesRange`](../index.md)
- [`ClassUnicodeRange`](../index.md)

### `Bound`

```rust
trait Bound: Copy + Clone + Debug + Eq + PartialEq + PartialOrd + Ord { ... }
```

*Defined in [`regex-syntax-0.8.8/src/hir/interval.rs:510-518`](../../../../.source_1765210505/regex-syntax-0.8.8/src/hir/interval.rs#L510-L518)*

#### Required Methods

- `fn min_value() -> Self`

- `fn max_value() -> Self`

- `fn as_u32(self) -> u32`

- `fn increment(self) -> Self`

- `fn decrement(self) -> Self`

#### Implementors

- `char`
- `u8`

