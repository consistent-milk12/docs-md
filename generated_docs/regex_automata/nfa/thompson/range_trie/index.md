*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [range_trie](index.md)*

---

# Module `range_trie`

## Contents

- [Structs](#structs)
  - [`RangeTrie`](#rangetrie)
  - [`State`](#state)
  - [`Transition`](#transition)
  - [`NextDupe`](#nextdupe)
  - [`NextIter`](#nextiter)
  - [`NextInsert`](#nextinsert)
  - [`Split`](#split)
- [Enums](#enums)
  - [`SplitRange`](#splitrange)
- [Functions](#functions)
  - [`intersects`](#intersects)
- [Constants](#constants)
  - [`FINAL`](#final)
  - [`ROOT`](#root)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RangeTrie`](#rangetrie) | struct | A range trie represents an ordered set of sequences of bytes. |
| [`State`](#state) | struct | A single state in this trie. |
| [`Transition`](#transition) | struct | A transition is a single range of bytes. |
| [`NextDupe`](#nextdupe) | struct | The next state to process during duplication. |
| [`NextIter`](#nextiter) | struct | The next state (and its corresponding transition) that we want to visit during iteration in lexicographic order. |
| [`NextInsert`](#nextinsert) | struct | The next state to process during insertion and any remaining ranges that we want to add for a particular sequence of ranges. |
| [`Split`](#split) | struct | Split represents a partitioning of two ranges into one or more ranges. |
| [`SplitRange`](#splitrange) | enum | A tagged range indicating how it was derived from a pair of ranges. |
| [`intersects`](#intersects) | fn | Returns true if and only if the given ranges intersect. |
| [`FINAL`](#final) | const | There is only one final state in this trie. |
| [`ROOT`](#root) | const | The root state of the trie. |

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:179-199`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L179-L199)*

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

- <span id="rangetrie-new"></span>`fn new() -> RangeTrie` — [`RangeTrie`](#rangetrie)

  Create a new empty range trie.

- <span id="rangetrie-clear"></span>`fn clear(&mut self)`

  Clear this range trie such that it is empty. Clearing a range trie

  and reusing it can beneficial because this may reuse allocations.

- <span id="rangetrie-iter"></span>`fn iter<E, F: FnMut(&[Utf8Range]) -> Result<(), E>>(&self, f: F) -> Result<(), E>`

  Iterate over all of the sequences of byte ranges in this trie, and

  call the provided function for each sequence. Iteration occurs in

  lexicographic order.

- <span id="rangetrie-insert"></span>`fn insert(&mut self, ranges: &[Utf8Range])`

  Inserts a new sequence of ranges into this trie.

  

  The sequence given must be non-empty and must not have a length

  exceeding 4.

- <span id="rangetrie-add-empty"></span>`fn add_empty(&mut self) -> StateID` — [`StateID`](../../../util/primitives/index.md#stateid)

- <span id="rangetrie-duplicate"></span>`fn duplicate(&mut self, old_id: StateID) -> StateID` — [`StateID`](../../../util/primitives/index.md#stateid)

  Performs a deep clone of the given state and returns the duplicate's

  state ID.

  

  A "deep clone" in this context means that the state given along with

  recursively all states that it points to are copied. Once complete,

  the given state ID and the returned state ID share nothing.

  

  This is useful during range trie insertion when a new range overlaps

  with an existing range that is bigger than the new one. The part

  of the existing range that does *not* overlap with the new one is

  duplicated so that adding the new range to the overlap doesn't disturb

  the non-overlapping portion.

  

  There's one exception: if old_id is the final state, then it is not

  duplicated and the same final state is returned. This is because all

  final states in this trie are equivalent.

- <span id="rangetrie-add-transition"></span>`fn add_transition(&mut self, from_id: StateID, range: Utf8Range, next_id: StateID)` — [`StateID`](../../../util/primitives/index.md#stateid)

  Adds the given transition to the given state.

  

  Callers must ensure that all previous transitions in this state

  are lexicographically smaller than the given range.

- <span id="rangetrie-add-transition-at"></span>`fn add_transition_at(&mut self, i: usize, from_id: StateID, range: Utf8Range, next_id: StateID)` — [`StateID`](../../../util/primitives/index.md#stateid)

  Like `add_transition`, except this inserts the transition just before

  the ith transition.

- <span id="rangetrie-set-transition-at"></span>`fn set_transition_at(&mut self, i: usize, from_id: StateID, range: Utf8Range, next_id: StateID)` — [`StateID`](../../../util/primitives/index.md#stateid)

  Overwrites the transition at position i with the given transition.

- <span id="rangetrie-state"></span>`fn state(&self, id: StateID) -> &State` — [`StateID`](../../../util/primitives/index.md#stateid), [`State`](#state)

  Return an immutable borrow for the state with the given ID.

- <span id="rangetrie-state-mut"></span>`fn state_mut(&mut self, id: StateID) -> &mut State` — [`StateID`](../../../util/primitives/index.md#stateid), [`State`](#state)

  Return a mutable borrow for the state with the given ID.

#### Trait Implementations

##### `impl Any for RangeTrie`

- <span id="rangetrie-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RangeTrie`

- <span id="rangetrie-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RangeTrie`

- <span id="rangetrie-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RangeTrie`

- <span id="rangetrie-clone"></span>`fn clone(&self) -> RangeTrie` — [`RangeTrie`](#rangetrie)

##### `impl CloneToUninit for RangeTrie`

- <span id="rangetrie-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RangeTrie`

- <span id="rangetrie-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RangeTrie`

- <span id="rangetrie-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RangeTrie`

- <span id="rangetrie-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RangeTrie`

- <span id="rangetrie-toowned-type-owned"></span>`type Owned = T`

- <span id="rangetrie-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rangetrie-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RangeTrie`

- <span id="rangetrie-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rangetrie-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RangeTrie`

- <span id="rangetrie-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rangetrie-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `State`

```rust
struct State {
    transitions: alloc::vec::Vec<Transition>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:203-207`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L203-L207)*

A single state in this trie.

#### Fields

- **`transitions`**: `alloc::vec::Vec<Transition>`

  A sorted sequence of non-overlapping transitions to other states. Each
  transition corresponds to a single range of bytes.

#### Implementations

- <span id="state-find"></span>`fn find(&self, range: Utf8Range) -> usize`

  Find the position at which the given range should be inserted in this

  state.

  

  The position returned is always in the inclusive range

  [0, transitions.len()]. If 'transitions.len()' is returned, then the

  given range overlaps with no other range in this state *and* is greater

  than all of them.

  

  For all other possible positions, the given range either overlaps

  with the transition at that position or is otherwise less than it

  with no overlap (and is greater than the previous transition). In the

  former case, careful attention must be paid to inserting this range

  as a new transition. In the latter case, the range can be inserted as

  a new transition at the given position without disrupting any other

  transitions.

- <span id="state-clear"></span>`fn clear(&mut self)`

  Clear this state such that it has zero transitions.

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for State`

- <span id="state-toowned-type-owned"></span>`type Owned = T`

- <span id="state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Transition`

```rust
struct Transition {
    range: regex_syntax::utf8::Utf8Range,
    next_id: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:213-218`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L213-L218)*

A transition is a single range of bytes. If a particular byte is in this
range, then the corresponding machine may transition to the state pointed
to by `next_id`.

#### Fields

- **`range`**: `regex_syntax::utf8::Utf8Range`

  The byte range.

- **`next_id`**: `crate::util::primitives::StateID`

  The next state to transition to.

#### Trait Implementations

##### `impl Any for Transition`

- <span id="transition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Transition`

- <span id="transition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Transition`

- <span id="transition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Transition`

- <span id="transition-clone"></span>`fn clone(&self) -> Transition` — [`Transition`](#transition)

##### `impl CloneToUninit for Transition`

- <span id="transition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Transition`

- <span id="transition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Transition`

- <span id="transition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Transition`

- <span id="transition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Transition`

- <span id="transition-toowned-type-owned"></span>`type Owned = T`

- <span id="transition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="transition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Transition`

- <span id="transition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="transition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Transition`

- <span id="transition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="transition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NextDupe`

```rust
struct NextDupe {
    old_id: crate::util::primitives::StateID,
    new_id: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:609-614`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L609-L614)*

The next state to process during duplication.

#### Fields

- **`old_id`**: `crate::util::primitives::StateID`

  The state we want to duplicate.

- **`new_id`**: `crate::util::primitives::StateID`

  The ID of the new state that is a duplicate of old_id.

#### Trait Implementations

##### `impl Any for NextDupe`

- <span id="nextdupe-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NextDupe`

- <span id="nextdupe-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NextDupe`

- <span id="nextdupe-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NextDupe`

- <span id="nextdupe-clone"></span>`fn clone(&self) -> NextDupe` — [`NextDupe`](#nextdupe)

##### `impl CloneToUninit for NextDupe`

- <span id="nextdupe-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NextDupe`

- <span id="nextdupe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NextDupe`

- <span id="nextdupe-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NextDupe`

- <span id="nextdupe-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for NextDupe`

- <span id="nextdupe-toowned-type-owned"></span>`type Owned = T`

- <span id="nextdupe-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nextdupe-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NextDupe`

- <span id="nextdupe-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nextdupe-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NextDupe`

- <span id="nextdupe-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nextdupe-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NextIter`

```rust
struct NextIter {
    state_id: crate::util::primitives::StateID,
    tidx: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:619-622`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L619-L622)*

The next state (and its corresponding transition) that we want to visit
during iteration in lexicographic order.

#### Trait Implementations

##### `impl Any for NextIter`

- <span id="nextiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NextIter`

- <span id="nextiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NextIter`

- <span id="nextiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NextIter`

- <span id="nextiter-clone"></span>`fn clone(&self) -> NextIter` — [`NextIter`](#nextiter)

##### `impl CloneToUninit for NextIter`

- <span id="nextiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NextIter`

- <span id="nextiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NextIter`

- <span id="nextiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NextIter`

- <span id="nextiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for NextIter`

- <span id="nextiter-toowned-type-owned"></span>`type Owned = T`

- <span id="nextiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nextiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NextIter`

- <span id="nextiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nextiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NextIter`

- <span id="nextiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nextiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NextInsert`

```rust
struct NextInsert {
    state_id: crate::util::primitives::StateID,
    ranges: [regex_syntax::utf8::Utf8Range; 4],
    len: u8,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:628-637`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L628-L637)*

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

- <span id="nextinsert-new"></span>`fn new(state_id: StateID, ranges: &[Utf8Range]) -> NextInsert` — [`StateID`](../../../util/primitives/index.md#stateid), [`NextInsert`](#nextinsert)

  Create the next item to visit. The given state ID should correspond

  to the state at which the first range in the given slice should be

  inserted. The slice given must not be empty and it must be no longer

  than 4.

- <span id="nextinsert-push"></span>`fn push(trie: &mut RangeTrie, stack: &mut Vec<NextInsert>, ranges: &[Utf8Range]) -> StateID` — [`RangeTrie`](#rangetrie), [`NextInsert`](#nextinsert), [`StateID`](../../../util/primitives/index.md#stateid)

  Push a new empty state to visit along with any remaining ranges that

  still need to be inserted. The ID of the new empty state is returned.

  

  If ranges is empty, then no new state is created and FINAL is returned.

- <span id="nextinsert-state-id"></span>`fn state_id(&self) -> StateID` — [`StateID`](../../../util/primitives/index.md#stateid)

  Return the ID of the state to visit.

- <span id="nextinsert-ranges"></span>`fn ranges(&self) -> &[Utf8Range]`

  Return the remaining ranges to insert.

#### Trait Implementations

##### `impl Any for NextInsert`

- <span id="nextinsert-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NextInsert`

- <span id="nextinsert-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NextInsert`

- <span id="nextinsert-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NextInsert`

- <span id="nextinsert-clone"></span>`fn clone(&self) -> NextInsert` — [`NextInsert`](#nextinsert)

##### `impl CloneToUninit for NextInsert`

- <span id="nextinsert-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NextInsert`

- <span id="nextinsert-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NextInsert`

- <span id="nextinsert-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NextInsert`

- <span id="nextinsert-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for NextInsert`

- <span id="nextinsert-toowned-type-owned"></span>`type Owned = T`

- <span id="nextinsert-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nextinsert-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NextInsert`

- <span id="nextinsert-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nextinsert-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NextInsert`

- <span id="nextinsert-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nextinsert-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Split`

```rust
struct Split {
    partitions: [SplitRange; 3],
    len: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:771-774`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L771-L774)*

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

- <span id="split-new"></span>`fn new(o: Utf8Range, n: Utf8Range) -> Option<Split>` — [`Split`](#split)

  Create a partitioning of the given ranges.

  

  If the given ranges have an empty intersection, then None is returned.

- <span id="split-parts1"></span>`fn parts1(r1: SplitRange) -> Split` — [`SplitRange`](#splitrange), [`Split`](#split)

  Create a new split with a single partition. This only occurs when two

  ranges are equivalent.

- <span id="split-parts2"></span>`fn parts2(r1: SplitRange, r2: SplitRange) -> Split` — [`SplitRange`](#splitrange), [`Split`](#split)

  Create a new split with two partitions.

- <span id="split-parts3"></span>`fn parts3(r1: SplitRange, r2: SplitRange, r3: SplitRange) -> Split` — [`SplitRange`](#splitrange), [`Split`](#split)

  Create a new split with three partitions.

- <span id="split-as-slice"></span>`fn as_slice(&self) -> &[SplitRange]` — [`SplitRange`](#splitrange)

  Return the partitions in this split as a slice.

#### Trait Implementations

##### `impl Any for Split`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Split`

- <span id="split-clone"></span>`fn clone(&self) -> Split` — [`Split`](#split)

##### `impl CloneToUninit for Split`

- <span id="split-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Split`

- <span id="split-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Split`

##### `impl<T> From for Split`

- <span id="split-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Split`

- <span id="split-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Split`

- <span id="split-partialeq-eq"></span>`fn eq(&self, other: &Split) -> bool` — [`Split`](#split)

##### `impl StructuralPartialEq for Split`

##### `impl ToOwned for Split`

- <span id="split-toowned-type-owned"></span>`type Owned = T`

- <span id="split-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="split-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Split`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Split`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `SplitRange`

```rust
enum SplitRange {
    Old(regex_syntax::utf8::Utf8Range),
    New(regex_syntax::utf8::Utf8Range),
    Both(regex_syntax::utf8::Utf8Range),
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:778-782`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L778-L782)*

A tagged range indicating how it was derived from a pair of ranges.

#### Trait Implementations

##### `impl Any for SplitRange`

- <span id="splitrange-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitRange`

- <span id="splitrange-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitRange`

- <span id="splitrange-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SplitRange`

- <span id="splitrange-clone"></span>`fn clone(&self) -> SplitRange` — [`SplitRange`](#splitrange)

##### `impl CloneToUninit for SplitRange`

- <span id="splitrange-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SplitRange`

##### `impl Debug for SplitRange`

- <span id="splitrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SplitRange`

##### `impl<T> From for SplitRange`

- <span id="splitrange-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SplitRange`

- <span id="splitrange-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SplitRange`

- <span id="splitrange-partialeq-eq"></span>`fn eq(&self, other: &SplitRange) -> bool` — [`SplitRange`](#splitrange)

##### `impl StructuralPartialEq for SplitRange`

##### `impl ToOwned for SplitRange`

- <span id="splitrange-toowned-type-owned"></span>`type Owned = T`

- <span id="splitrange-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="splitrange-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SplitRange`

- <span id="splitrange-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitrange-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitRange`

- <span id="splitrange-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitrange-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `intersects`

```rust
fn intersects(r1: regex_syntax::utf8::Utf8Range, r2: regex_syntax::utf8::Utf8Range) -> bool
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:912-914`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L912-L914)*

Returns true if and only if the given ranges intersect.

## Constants

### `FINAL`
```rust
const FINAL: crate::util::primitives::StateID;
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:154`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L154)*

There is only one final state in this trie. Every sequence of byte ranges
added shares the same final state.

### `ROOT`
```rust
const ROOT: crate::util::primitives::StateID;
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/range_trie.rs:157`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/range_trie.rs#L157)*

The root state of the trie.

