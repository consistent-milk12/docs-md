*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [range_trie](index.md)*

---

# Module `range_trie`

## Structs

### `RangeTrie`

```rust
struct RangeTrie {
    states: alloc::vec::Vec<State>,
    free: alloc::vec::Vec<State>,
    iter_stack: core::cell::RefCell<alloc::vec::Vec<NextIter>>,
    iter_ranges: core::cell::RefCell<alloc::vec::Vec<regex_syntax::utf8::Utf8Range>>,
    dupe_stack: alloc::vec::Vec<NextDupe>,
    insert_stack: alloc::vec::Vec<NextInsert>,
}
```

A range trie represents an ordered set of sequences of bytes.

A range trie accepts as input a sequence of byte ranges and merges
them into the existing set such that the trie can produce a sorted
non-overlapping sequence of byte ranges. The sequence emitted corresponds
precisely to the sequence of bytes matched by the given keys, although the
byte ranges themselves may be split at different boundaries.

The order complexity of this data structure seems difficult to analyze.
If the size of a byte is held as a constant, then insertion is clearly
O(n) where n is the number of byte ranges in the input key. However, if
k=256 is our alphabet size, then insertion could be O(k^2 * n). In
particular it seems possible for pathological inputs to cause insertion
to do a lot of work. However, for what we use this data structure for,
there should be no pathological inputs since the ultimate source is always
a sorted set of Unicode scalar value ranges.

Internally, this trie is setup like a finite state machine. Note though
that it is acyclic.

#### Fields

- **`states`**: `alloc::vec::Vec<State>`

  The states in this trie. The first is always the shared final state.
  The second is always the root state. Otherwise, there is no
  particular order.

- **`free`**: `alloc::vec::Vec<State>`

  A free-list of states. When a range trie is cleared, all of its states
  are added to this list. Creating a new state reuses states from this
  list before allocating a new one.

- **`iter_stack`**: `core::cell::RefCell<alloc::vec::Vec<NextIter>>`

  A stack for traversing this trie to yield sequences of byte ranges in
  lexicographic order.

- **`iter_ranges`**: `core::cell::RefCell<alloc::vec::Vec<regex_syntax::utf8::Utf8Range>>`

  A buffer that stores the current sequence during iteration.

- **`dupe_stack`**: `alloc::vec::Vec<NextDupe>`

  A stack used for traversing the trie in order to (deeply) duplicate
  a state. States are recursively duplicated when ranges are split.

- **`insert_stack`**: `alloc::vec::Vec<NextInsert>`

  A stack used for traversing the trie during insertion of a new
  sequence of byte ranges.

#### Implementations

- `fn new() -> RangeTrie` — [`RangeTrie`](#rangetrie)

- `fn clear(self: &mut Self)`

- `fn iter<E, F: FnMut(&[Utf8Range]) -> Result<(), E>>(self: &Self, f: F) -> Result<(), E>`

- `fn insert(self: &mut Self, ranges: &[Utf8Range])`

- `fn add_empty(self: &mut Self) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `fn duplicate(self: &mut Self, old_id: StateID) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `fn add_transition(self: &mut Self, from_id: StateID, range: Utf8Range, next_id: StateID)` — [`StateID`](../../../util/primitives/index.md)

- `fn add_transition_at(self: &mut Self, i: usize, from_id: StateID, range: Utf8Range, next_id: StateID)` — [`StateID`](../../../util/primitives/index.md)

- `fn set_transition_at(self: &mut Self, i: usize, from_id: StateID, range: Utf8Range, next_id: StateID)` — [`StateID`](../../../util/primitives/index.md)

- `fn state(self: &Self, id: StateID) -> &State` — [`StateID`](../../../util/primitives/index.md), [`State`](#state)

- `fn state_mut(self: &mut Self, id: StateID) -> &mut State` — [`StateID`](../../../util/primitives/index.md), [`State`](#state)

#### Trait Implementations

##### `impl Clone for RangeTrie`

- `fn clone(self: &Self) -> RangeTrie` — [`RangeTrie`](#rangetrie)

##### `impl Debug for RangeTrie`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `State`

```rust
struct State {
    transitions: alloc::vec::Vec<Transition>,
}
```

A single state in this trie.

#### Fields

- **`transitions`**: `alloc::vec::Vec<Transition>`

  A sorted sequence of non-overlapping transitions to other states. Each
  transition corresponds to a single range of bytes.

#### Implementations

- `fn find(self: &Self, range: Utf8Range) -> usize`

- `fn clear(self: &mut Self)`

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Transition`

```rust
struct Transition {
    range: regex_syntax::utf8::Utf8Range,
    next_id: crate::util::primitives::StateID,
}
```

A transition is a single range of bytes. If a particular byte is in this
range, then the corresponding machine may transition to the state pointed
to by `next_id`.

#### Fields

- **`range`**: `regex_syntax::utf8::Utf8Range`

  The byte range.

- **`next_id`**: `crate::util::primitives::StateID`

  The next state to transition to.

#### Trait Implementations

##### `impl Clone for Transition`

- `fn clone(self: &Self) -> Transition` — [`Transition`](#transition)

##### `impl Debug for Transition`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NextDupe`

```rust
struct NextDupe {
    old_id: crate::util::primitives::StateID,
    new_id: crate::util::primitives::StateID,
}
```

The next state to process during duplication.

#### Fields

- **`old_id`**: `crate::util::primitives::StateID`

  The state we want to duplicate.

- **`new_id`**: `crate::util::primitives::StateID`

  The ID of the new state that is a duplicate of old_id.

#### Trait Implementations

##### `impl Clone for NextDupe`

- `fn clone(self: &Self) -> NextDupe` — [`NextDupe`](#nextdupe)

##### `impl Debug for NextDupe`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `NextIter`

```rust
struct NextIter {
    state_id: crate::util::primitives::StateID,
    tidx: usize,
}
```

The next state (and its corresponding transition) that we want to visit
during iteration in lexicographic order.

#### Trait Implementations

##### `impl Clone for NextIter`

- `fn clone(self: &Self) -> NextIter` — [`NextIter`](#nextiter)

##### `impl Debug for NextIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `NextInsert`

```rust
struct NextInsert {
    state_id: crate::util::primitives::StateID,
    ranges: [regex_syntax::utf8::Utf8Range; 4],
    len: u8,
}
```

The next state to process during insertion and any remaining ranges that we
want to add for a particular sequence of ranges. The first such instance
is always the root state along with all ranges given.

#### Fields

- **`state_id`**: `crate::util::primitives::StateID`

  The next state to begin inserting ranges. This state should be the
  state at which `ranges[0]` should be inserted.

- **`ranges`**: `[regex_syntax::utf8::Utf8Range; 4]`

  The ranges to insert. We used a fixed-size array here to avoid an
  allocation.

- **`len`**: `u8`

  The number of valid ranges in the above array.

#### Implementations

- `fn new(state_id: StateID, ranges: &[Utf8Range]) -> NextInsert` — [`StateID`](../../../util/primitives/index.md), [`NextInsert`](#nextinsert)

- `fn push(trie: &mut RangeTrie, stack: &mut Vec<NextInsert>, ranges: &[Utf8Range]) -> StateID` — [`RangeTrie`](#rangetrie), [`NextInsert`](#nextinsert), [`StateID`](../../../util/primitives/index.md)

- `fn state_id(self: &Self) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `fn ranges(self: &Self) -> &[Utf8Range]`

#### Trait Implementations

##### `impl Clone for NextInsert`

- `fn clone(self: &Self) -> NextInsert` — [`NextInsert`](#nextinsert)

##### `impl Debug for NextInsert`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Split`

```rust
struct Split {
    partitions: [SplitRange; 3],
    len: usize,
}
```

Split represents a partitioning of two ranges into one or more ranges. This
is the secret sauce that makes a range trie work, as it's what tells us
how to deal with two overlapping but unequal ranges during insertion.

Essentially, either two ranges overlap or they don't. If they don't, then
handling insertion is easy: just insert the new range into its
lexicographically correct position. Since it does not overlap with anything
else, no other transitions are impacted by the new range.

If they do overlap though, there are generally three possible cases to
handle:

1. The part where the two ranges actually overlap. i.e., The intersection.
2. The part of the existing range that is not in the new range.
3. The part of the new range that is not in the old range.

(1) is guaranteed to always occur since all overlapping ranges have a
non-empty intersection. If the two ranges are not equivalent, then at
least one of (2) or (3) is guaranteed to occur as well. In some cases,
e.g., `[0-4]` and `[4-9]`, all three cases will occur.

This `Split` type is responsible for providing (1), (2) and (3) for any
possible pair of byte ranges.

As for insertion, for the overlap in (1), the remaining ranges to insert
should be added by following the corresponding transition. However, this
should only be done for the overlapping parts of the range. If there was
a part of the existing range that was not in the new range, then that
existing part must be split off from the transition and duplicated. The
remaining parts of the overlap can then be added to using the new ranges
without disturbing the existing range.

Handling the case for the part of a new range that is not in an existing
range is seemingly easy. Just treat it as if it were a non-overlapping
range. The problem here is that if this new non-overlapping range occurs
after both (1) and (2), then it's possible that it can overlap with the
next transition in the current state. If it does, then the whole process
must be repeated!

# Details of the 3 cases

The following details the various cases that are implemented in code
below. It's plausible that the number of cases is not actually minimal,
but it's important for this code to remain at least somewhat readable.

Given [a,b] and [x,y], where a <= b, x <= y, b < 256 and y < 256, we define
the follow distinct relationships where at least one must apply. The order
of these matters, since multiple can match. The first to match applies.

  1. b < x <=> [a,b] < [x,y]
  2. y < a <=> [x,y] < [a,b]

In the case of (1) and (2), these are the only cases where there is no
overlap. Or otherwise, the intersection of [a,b] and [x,y] is empty. In
order to compute the intersection, one can do [max(a,x), min(b,y)]. The
intersection in all of the following cases is non-empty.

   3. a = x && b = y <=> [a,b] == [x,y]
   4. a = x && b < y <=> [x,y] right-extends [a,b]
   5. b = y && a > x <=> [x,y] left-extends [a,b]
   6. x = a && y < b <=> [a,b] right-extends [x,y]
   7. y = b && x > a <=> [a,b] left-extends [x,y]
   8. a > x && b < y <=> [x,y] covers [a,b]
   9. x > a && y < b <=> [a,b] covers [x,y]
  10. b = x && a < y <=> [a,b] is left-adjacent to [x,y]
  11. y = a && x < b <=> [x,y] is left-adjacent to [a,b]
  12. b > x && b < y <=> [a,b] left-overlaps [x,y]
  13. y > a && y < b <=> [x,y] left-overlaps [a,b]

In cases 3-13, we can form rules that partition the ranges into a
non-overlapping ordered sequence of ranges:

   3. [a,b]
   4. [a,b], [b+1,y]
   5. [x,a-1], [a,b]
   6. [x,y], [y+1,b]
   7. [a,x-1], [x,y]
   8. [x,a-1], [a,b], [b+1,y]
   9. [a,x-1], [x,y], [y+1,b]
  10. [a,b-1], [b,b], [b+1,y]
  11. [x,y-1], [y,y], [y+1,b]
  12. [a,x-1], [x,b], [b+1,y]
  13. [x,a-1], [a,y], [y+1,b]

In the code below, we go a step further and identify each of the above
outputs as belonging either to the overlap of the two ranges or to one
of [a,b] or [x,y] exclusively.

#### Implementations

- `fn new(o: Utf8Range, n: Utf8Range) -> Option<Split>` — [`Split`](#split)

- `fn parts1(r1: SplitRange) -> Split` — [`SplitRange`](#splitrange), [`Split`](#split)

- `fn parts2(r1: SplitRange, r2: SplitRange) -> Split` — [`SplitRange`](#splitrange), [`Split`](#split)

- `fn parts3(r1: SplitRange, r2: SplitRange, r3: SplitRange) -> Split` — [`SplitRange`](#splitrange), [`Split`](#split)

- `fn as_slice(self: &Self) -> &[SplitRange]` — [`SplitRange`](#splitrange)

#### Trait Implementations

##### `impl Clone for Split`

- `fn clone(self: &Self) -> Split` — [`Split`](#split)

##### `impl Debug for Split`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Split`

##### `impl PartialEq for Split`

- `fn eq(self: &Self, other: &Split) -> bool` — [`Split`](#split)

##### `impl StructuralPartialEq for Split`

## Enums

### `SplitRange`

```rust
enum SplitRange {
    Old(regex_syntax::utf8::Utf8Range),
    New(regex_syntax::utf8::Utf8Range),
    Both(regex_syntax::utf8::Utf8Range),
}
```

A tagged range indicating how it was derived from a pair of ranges.

#### Trait Implementations

##### `impl Clone for SplitRange`

- `fn clone(self: &Self) -> SplitRange` — [`SplitRange`](#splitrange)

##### `impl Copy for SplitRange`

##### `impl Debug for SplitRange`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SplitRange`

##### `impl PartialEq for SplitRange`

- `fn eq(self: &Self, other: &SplitRange) -> bool` — [`SplitRange`](#splitrange)

##### `impl StructuralPartialEq for SplitRange`

## Functions

### `intersects`

```rust
fn intersects(r1: regex_syntax::utf8::Utf8Range, r2: regex_syntax::utf8::Utf8Range) -> bool
```

Returns true if and only if the given ranges intersect.

## Constants

### `FINAL`

```rust
const FINAL: crate::util::primitives::StateID;
```

There is only one final state in this trie. Every sequence of byte ranges
added shares the same final state.

### `ROOT`

```rust
const ROOT: crate::util::primitives::StateID;
```

The root state of the trie.

