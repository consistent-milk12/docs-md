*[regex_syntax](../../index.md) / [hir](../index.md) / [interval](index.md)*

---

# Module `interval`

## Structs

### `IntervalSet<I>`

```rust
struct IntervalSet<I> {
    ranges: alloc::vec::Vec<I>,
    folded: bool,
}
```

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

- `fn new<T: IntoIterator<Item = I>>(intervals: T) -> IntervalSet<I>` — [`IntervalSet`](#intervalset)

- `fn push(self: &mut Self, interval: I)`

- `fn iter(self: &Self) -> IntervalSetIter<'_, I>` — [`IntervalSetIter`](#intervalsetiter)

- `fn intervals(self: &Self) -> &[I]`

- `fn case_fold_simple(self: &mut Self) -> Result<(), unicode::CaseFoldError>` — [`CaseFoldError`](../../unicode/index.md)

- `fn union(self: &mut Self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- `fn intersect(self: &mut Self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- `fn difference(self: &mut Self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- `fn symmetric_difference(self: &mut Self, other: &IntervalSet<I>)` — [`IntervalSet`](#intervalset)

- `fn negate(self: &mut Self)`

- `fn canonicalize(self: &mut Self)`

- `fn is_canonical(self: &Self) -> bool`

#### Trait Implementations

##### `impl<I: $crate::clone::Clone> Clone for IntervalSet<I>`

- `fn clone(self: &Self) -> IntervalSet<I>` — [`IntervalSet`](#intervalset)

##### `impl<I: $crate::fmt::Debug> Debug for IntervalSet<I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I: Interval> Eq for IntervalSet<I>`

##### `impl<I: Interval> PartialEq for IntervalSet<I>`

- `fn eq(self: &Self, other: &IntervalSet<I>) -> bool` — [`IntervalSet`](#intervalset)

### `IntervalSetIter<'a, I>`

```rust
struct IntervalSetIter<'a, I>(slice::Iter<'a, I>);
```

An iterator over intervals.

#### Trait Implementations

##### `impl<'a, I: $crate::fmt::Debug> Debug for IntervalSetIter<'a, I>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for IntervalSetIter<'a, I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, I> Iterator for IntervalSetIter<'a, I>`

- `type Item = &'a I`

- `fn next(self: &mut Self) -> Option<&'a I>`

## Traits

### `Interval`

```rust
trait Interval: Clone + Copy + Debug + Default + Eq + PartialEq + PartialOrd + Ord { ... }
```

#### Required Methods

- `type Bound: 1`

- `fn lower(self: &Self) -> <Self as >::Bound`

- `fn upper(self: &Self) -> <Self as >::Bound`

- `fn set_lower(self: &mut Self, bound: <Self as >::Bound)`

- `fn set_upper(self: &mut Self, bound: <Self as >::Bound)`

- `fn case_fold_simple(self: &Self, intervals: &mut Vec<Self>) -> Result<(), unicode::CaseFoldError>`

- `fn create(lower: <Self as >::Bound, upper: <Self as >::Bound) -> Self`

  Create a new interval.

- `fn union(self: &Self, other: &Self) -> Option<Self>`

  Union the given overlapping range into this range.

- `fn intersect(self: &Self, other: &Self) -> Option<Self>`

  Intersect this range with the given range and return the result.

- `fn difference(self: &Self, other: &Self) -> (Option<Self>, Option<Self>)`

  Subtract the given range from this range and return the resulting

- `fn is_contiguous(self: &Self, other: &Self) -> bool`

  Returns true if and only if the two ranges are contiguous. Two ranges

- `fn is_intersection_empty(self: &Self, other: &Self) -> bool`

  Returns true if and only if the intersection of this range and the

- `fn is_subset(self: &Self, other: &Self) -> bool`

  Returns true if and only if this range is a subset of the other range.

### `Bound`

```rust
trait Bound: Copy + Clone + Debug + Eq + PartialEq + PartialOrd + Ord { ... }
```

#### Required Methods

- `fn min_value() -> Self`

- `fn max_value() -> Self`

- `fn as_u32(self: Self) -> u32`

- `fn increment(self: Self) -> Self`

- `fn decrement(self: Self) -> Self`

