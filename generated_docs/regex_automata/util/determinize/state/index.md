*[regex_automata](../../../index.md) / [util](../../index.md) / [determinize](../index.md) / [state](index.md)*

---

# Module `state`

This module defines a DFA state representation and builders for constructing
DFA states.

This representation is specifically for use in implementations of NFA-to-DFA
conversion via powerset construction. (Also called "determinization" in this
crate.)

The term "DFA state" is somewhat overloaded in this crate. In some cases, it
refers to the set of transitions over an alphabet for a particular state. In
other cases, it refers to a set of NFA states. The former is really about the
final representation of a state in a DFA's transition table, where as the
latter---what this module is focused on---is closer to an intermediate form
that is used to help eventually build the transition table.

This module exports four types. All four types represent the same idea: an
ordered set of NFA states. This ordered set represents the epsilon closure of a
particular NFA state, where the "epsilon closure" is the set of NFA states that
can be transitioned to without consuming any input. i.e., Follow all of the NFA
state's epsilon transitions. In addition, this implementation of DFA states
cares about two other things: the ordered set of pattern IDs corresponding
to the patterns that match if the state is a match state, and the set of
look-behind assertions that were true when the state was created.

The first, `State`, is a frozen representation of a state that cannot be
modified. It may be cheaply cloned without copying the state itself and can be
accessed safely from multiple threads simultaneously. This type is useful for
when one knows that the DFA state being constructed is distinct from any other
previously constructed states. Namely, powerset construction, in practice,
requires one to keep a cache of previously created DFA states. Otherwise,
the number of DFA states created in memory balloons to an impractically
large number. For this reason, equivalent states should endeavor to have an
equivalent byte-level representation. (In general, "equivalency" here means,
"equivalent assertions, pattern IDs and NFA state IDs." We do not require that
full DFA minimization be implemented here. This form of equivalency is only
surface deep and is more-or-less a practical necessity.)

The other three types represent different phases in the construction of a
DFA state. Internally, these three types (and `State`) all use the same
byte-oriented representation. That means one can use any of the builder types
to check whether the state it represents already exists or not. If it does,
then there is no need to freeze it into a `State` (which requires an alloc and
a copy). Here are the three types described succinctly:

* `StateBuilderEmpty` represents a state with no pattern IDs, no assertions
and no NFA states. Creating a `StateBuilderEmpty` performs no allocs. A
`StateBuilderEmpty` can only be used to query its underlying memory capacity,
or to convert into a builder for recording pattern IDs and/or assertions.

* `StateBuilderMatches` represents a state with zero or more pattern IDs, zero
or more satisfied assertions and zero NFA state IDs. A `StateBuilderMatches`
can only be used for adding pattern IDs and recording assertions.

* `StateBuilderNFA` represents a state with zero or more pattern IDs, zero or
more satisfied assertions and zero or more NFA state IDs. A `StateBuilderNFA`
can only be used for adding NFA state IDs and recording some assertions.

The expected flow here is to use the above builders to construct a candidate
DFA state to check if it already exists. If it does, then there's no need to
freeze it into a `State`. If it doesn't exist, then `StateBuilderNFA::to_state`
can be called to freeze the builder into an immutable `State`. In either
case, `clear` should be called on the builder to turn it back into a
`StateBuilderEmpty` that reuses the underlying memory.

The main purpose for splitting the builder into these distinct types is to
make it impossible to do things like adding a pattern ID after adding an NFA
state ID. Namely, this makes it simpler to use a space-and-time efficient
binary representation for the state. (The format is documented on the `Repr`
type below.) If we just used one type for everything, it would be possible for
callers to use an incorrect interleaving of calls and thus result in a corrupt
representation. I chose to use more type machinery to make this impossible to
do because 1) determinization is itself pretty complex and it wouldn't be too
hard to foul this up and 2) there isn't too much machinery involved and it's
well contained.

As an optimization, sometimes states won't have certain things set. For
example, if the underlying NFA has no word boundary assertions, then there is
no reason to set a state's look-behind assertion as to whether it was generated
from a word byte or not. Similarly, if a state has no NFA states corresponding
to look-around assertions, then there is no reason to set `look_have` to a
non-empty set. Finally, callers usually omit unconditional epsilon transitions
when adding NFA state IDs since they aren't discriminatory.

Finally, the binary representation used by these states is, thankfully, not
serialized anywhere. So any kind of change can be made with reckless abandon,
as long as everything in this module agrees.

## Contents

- [Structs](#structs)
  - [`State`](#state)
  - [`StateBuilderEmpty`](#statebuilderempty)
  - [`StateBuilderMatches`](#statebuildermatches)
  - [`StateBuilderNFA`](#statebuildernfa)
  - [`Repr`](#repr)
  - [`ReprVec`](#reprvec)
- [Functions](#functions)
  - [`write_vari32`](#write-vari32)
  - [`read_vari32`](#read-vari32)
  - [`write_varu32`](#write-varu32)
  - [`read_varu32`](#read-varu32)
  - [`write_u32`](#write-u32)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`State`](#state) | struct | A DFA state that, at its core, is represented by an ordered set of NFA states. |
| [`StateBuilderEmpty`](#statebuilderempty) | struct | A state builder that represents an empty state. |
| [`StateBuilderMatches`](#statebuildermatches) | struct | A state builder that collects assertions and pattern IDs. |
| [`StateBuilderNFA`](#statebuildernfa) | struct | A state builder that collects some assertions and NFA state IDs. |
| [`Repr`](#repr) | struct | Repr is a read-only view into the representation of a DFA state. |
| [`ReprVec`](#reprvec) | struct | ReprVec is a write-only view into the representation of a DFA state. |
| [`write_vari32`](#write-vari32) | fn | Write a signed 32-bit integer using zig-zag encoding. |
| [`read_vari32`](#read-vari32) | fn | Read a signed 32-bit integer using zig-zag encoding. |
| [`write_varu32`](#write-varu32) | fn | Write an unsigned 32-bit integer as a varint. |
| [`read_varu32`](#read-varu32) | fn | Read an unsigned 32-bit varint. |
| [`write_u32`](#write-u32) | fn | Push a native-endian encoded `n` on to `dst`. |

## Structs

### `State`

```rust
struct State(alloc::sync::Arc<[u8]>);
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:109`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L109)*

A DFA state that, at its core, is represented by an ordered set of NFA
states.

This type is intended to be used only in NFA-to-DFA conversion via powerset
construction.

It may be cheaply cloned and accessed safely from multiple threads
simultaneously.

#### Implementations

- <span id="state-dead"></span>`fn dead() -> State` — [`State`](#state)

- <span id="state-is-match"></span>`fn is_match(&self) -> bool`

- <span id="state-is-from-word"></span>`fn is_from_word(&self) -> bool`

- <span id="state-is-half-crlf"></span>`fn is_half_crlf(&self) -> bool`

- <span id="state-look-have"></span>`fn look_have(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

- <span id="state-look-need"></span>`fn look_need(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

- <span id="state-match-len"></span>`fn match_len(&self) -> usize`

- <span id="state-match-pattern"></span>`fn match_pattern(&self, index: usize) -> PatternID` — [`PatternID`](../../primitives/index.md#patternid)

- <span id="state-match-pattern-ids"></span>`fn match_pattern_ids(&self) -> Option<Vec<PatternID>>` — [`PatternID`](../../primitives/index.md#patternid)

- <span id="state-iter-nfa-state-ids"></span>`fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F)`

- <span id="state-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="state-repr"></span>`fn repr(&self) -> Repr<'_>` — [`Repr`](#repr)

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

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for State`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for State`

- <span id="state-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for State`

- <span id="state-ord-cmp"></span>`fn cmp(&self, other: &State) -> cmp::Ordering` — [`State`](#state)

##### `impl PartialEq for State`

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl PartialOrd for State`

- <span id="state-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &State) -> option::Option<cmp::Ordering>` — [`State`](#state)

##### `impl StructuralPartialEq for State`

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

### `StateBuilderEmpty`

```rust
struct StateBuilderEmpty(alloc::vec::Vec<u8>);
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:191`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L191)*

A state builder that represents an empty state.

This is a useful "initial condition" for state construction. It has no
NFA state IDs, no assertions set and no pattern IDs. No allocations are
made when new() is called. Its main use is for being converted into a
builder that can capture assertions and pattern IDs.

#### Implementations

- <span id="statebuilderempty-new"></span>`fn new() -> StateBuilderEmpty` — [`StateBuilderEmpty`](#statebuilderempty)

- <span id="statebuilderempty-into-matches"></span>`fn into_matches(self) -> StateBuilderMatches` — [`StateBuilderMatches`](#statebuildermatches)

- <span id="statebuilderempty-clear"></span>`fn clear(&mut self)`

- <span id="statebuilderempty-capacity"></span>`fn capacity(&self) -> usize`

#### Trait Implementations

##### `impl Any for StateBuilderEmpty`

- <span id="statebuilderempty-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateBuilderEmpty`

- <span id="statebuilderempty-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateBuilderEmpty`

- <span id="statebuilderempty-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateBuilderEmpty`

- <span id="statebuilderempty-clone"></span>`fn clone(&self) -> StateBuilderEmpty` — [`StateBuilderEmpty`](#statebuilderempty)

##### `impl CloneToUninit for StateBuilderEmpty`

- <span id="statebuilderempty-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StateBuilderEmpty`

- <span id="statebuilderempty-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StateBuilderEmpty`

- <span id="statebuilderempty-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StateBuilderEmpty`

- <span id="statebuilderempty-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StateBuilderEmpty`

- <span id="statebuilderempty-toowned-type-owned"></span>`type Owned = T`

- <span id="statebuilderempty-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="statebuilderempty-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StateBuilderEmpty`

- <span id="statebuilderempty-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statebuilderempty-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateBuilderEmpty`

- <span id="statebuilderempty-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statebuilderempty-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StateBuilderMatches`

```rust
struct StateBuilderMatches(alloc::vec::Vec<u8>);
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:218`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L218)*

A state builder that collects assertions and pattern IDs.

When collecting pattern IDs is finished, this can be converted into a
builder that collects NFA state IDs.

#### Implementations

- <span id="statebuildermatches-into-nfa"></span>`fn into_nfa(self) -> StateBuilderNFA` — [`StateBuilderNFA`](#statebuildernfa)

- <span id="statebuildermatches-set-is-from-word"></span>`fn set_is_from_word(&mut self)`

- <span id="statebuildermatches-set-is-half-crlf"></span>`fn set_is_half_crlf(&mut self)`

- <span id="statebuildermatches-look-have"></span>`fn look_have(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

- <span id="statebuildermatches-set-look-have"></span>`fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md#lookset)

- <span id="statebuildermatches-add-match-pattern-id"></span>`fn add_match_pattern_id(&mut self, pid: PatternID)` — [`PatternID`](../../primitives/index.md#patternid)

- <span id="statebuildermatches-repr"></span>`fn repr(&self) -> Repr<'_>` — [`Repr`](#repr)

- <span id="statebuildermatches-repr-vec"></span>`fn repr_vec(&mut self) -> ReprVec<'_>` — [`ReprVec`](#reprvec)

#### Trait Implementations

##### `impl Any for StateBuilderMatches`

- <span id="statebuildermatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateBuilderMatches`

- <span id="statebuildermatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateBuilderMatches`

- <span id="statebuildermatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateBuilderMatches`

- <span id="statebuildermatches-clone"></span>`fn clone(&self) -> StateBuilderMatches` — [`StateBuilderMatches`](#statebuildermatches)

##### `impl CloneToUninit for StateBuilderMatches`

- <span id="statebuildermatches-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StateBuilderMatches`

- <span id="statebuildermatches-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for StateBuilderMatches`

- <span id="statebuildermatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StateBuilderMatches`

- <span id="statebuildermatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StateBuilderMatches`

- <span id="statebuildermatches-toowned-type-owned"></span>`type Owned = T`

- <span id="statebuildermatches-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="statebuildermatches-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StateBuilderMatches`

- <span id="statebuildermatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statebuildermatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateBuilderMatches`

- <span id="statebuildermatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statebuildermatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StateBuilderNFA`

```rust
struct StateBuilderNFA {
    repr: alloc::vec::Vec<u8>,
    prev_nfa_state_id: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:274-277`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L274-L277)*

A state builder that collects some assertions and NFA state IDs.

When collecting NFA state IDs is finished, this can be used to build a
`State` if necessary.

When dont with building a state (regardless of whether it got kept or not),
it's usually a good idea to call `clear` to get an empty builder back so
that it can be reused to build the next state.

#### Implementations

- <span id="statebuildernfa-to-state"></span>`fn to_state(&self) -> State` — [`State`](#state)

- <span id="statebuildernfa-clear"></span>`fn clear(self) -> StateBuilderEmpty` — [`StateBuilderEmpty`](#statebuilderempty)

- <span id="statebuildernfa-look-need"></span>`fn look_need(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

- <span id="statebuildernfa-set-look-have"></span>`fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md#lookset)

- <span id="statebuildernfa-set-look-need"></span>`fn set_look_need(&mut self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md#lookset)

- <span id="statebuildernfa-add-nfa-state-id"></span>`fn add_nfa_state_id(&mut self, sid: StateID)` — [`StateID`](../../primitives/index.md#stateid)

- <span id="statebuildernfa-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

- <span id="statebuildernfa-repr"></span>`fn repr(&self) -> Repr<'_>` — [`Repr`](#repr)

- <span id="statebuildernfa-repr-vec"></span>`fn repr_vec(&mut self) -> ReprVec<'_>` — [`ReprVec`](#reprvec)

#### Trait Implementations

##### `impl Any for StateBuilderNFA`

- <span id="statebuildernfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateBuilderNFA`

- <span id="statebuildernfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateBuilderNFA`

- <span id="statebuildernfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateBuilderNFA`

- <span id="statebuildernfa-clone"></span>`fn clone(&self) -> StateBuilderNFA` — [`StateBuilderNFA`](#statebuildernfa)

##### `impl CloneToUninit for StateBuilderNFA`

- <span id="statebuildernfa-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StateBuilderNFA`

- <span id="statebuildernfa-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for StateBuilderNFA`

- <span id="statebuildernfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StateBuilderNFA`

- <span id="statebuildernfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StateBuilderNFA`

- <span id="statebuildernfa-toowned-type-owned"></span>`type Owned = T`

- <span id="statebuildernfa-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="statebuildernfa-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StateBuilderNFA`

- <span id="statebuildernfa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statebuildernfa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateBuilderNFA`

- <span id="statebuildernfa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statebuildernfa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Repr<'a>`

```rust
struct Repr<'a>(&'a [u8]);
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:386`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L386)*

Repr is a read-only view into the representation of a DFA state.

Primarily, a Repr is how we achieve DRY: we implement decoding the format
in one place, and then use a Repr to implement the various methods on the
public state types.

The format is as follows:

The first three bytes correspond to bitsets.

Byte 0 is a bitset corresponding to miscellaneous flags associated with the
state. Bit 0 is set to 1 if the state is a match state. Bit 1 is set to 1
if the state has pattern IDs explicitly written to it. (This is a flag that
is not meant to be set by determinization, but rather, is used as part of
an internal space-saving optimization.) Bit 2 is set to 1 if the state was
generated by a transition over a "word" byte. (Callers may not always set
this. For example, if the NFA has no word boundary assertion, then needing
to track whether a state came from a word byte or not is superfluous and
wasteful.) Bit 3 is set to 1 if the state was generated by a transition
from a `\r` (forward search) or a `\n` (reverse search) when CRLF mode is
enabled.

Bytes 1..5 correspond to the look-behind assertions that were satisfied
by the transition that created this state. (Look-ahead assertions are not
tracked as part of states. Instead, these are applied by re-computing the
epsilon closure of a state when computing the transition function. See
`next` in the parent module.)

Bytes 5..9 correspond to the set of look-around assertions (including both
look-behind and look-ahead) that appear somewhere in this state's set of
NFA state IDs. This is used to determine whether this state's epsilon
closure should be re-computed when computing the transition function.
Namely, look-around assertions are "just" conditional epsilon transitions,
so if there are new assertions available when computing the transition
function, we should only re-compute the epsilon closure if those new
assertions are relevant to this particular state.

Bytes 9..13 correspond to a 32-bit native-endian encoded integer
corresponding to the number of patterns encoded in this state. If the state
is not a match state (byte 0 bit 0 is 0) or if it's only pattern ID is
PatternID::ZERO, then no integer is encoded at this position. Instead, byte
offset 3 is the position at which the first NFA state ID is encoded.

For a match state with at least one non-ZERO pattern ID, the next bytes
correspond to a sequence of 32-bit native endian encoded integers that
represent each pattern ID, in order, that this match state represents.

After the pattern IDs (if any), NFA state IDs are delta encoded as
varints.[1] The first NFA state ID is encoded as itself, and each
subsequent NFA state ID is encoded as the difference between itself and the
previous NFA state ID.

[1] - https://developers.google.com/protocol-buffers/docs/encoding#varints

#### Implementations

- <span id="repr-is-match"></span>`fn is_match(&self) -> bool`

  Returns true if and only if this is a match state.

  

  If callers have added pattern IDs to this state, then callers MUST set

  this state as a match state explicitly. However, as a special case,

  states that are marked as match states but with no pattern IDs, then

  the state is treated as if it had a single pattern ID equivalent to

  PatternID::ZERO.

- <span id="repr-has-pattern-ids"></span>`fn has_pattern_ids(&self) -> bool`

  Returns true if and only if this state has had at least one pattern

  ID added to it.

  

  This is an internal-only flag that permits the representation to save

  space in the common case of an NFA with one pattern in it. In that

  case, a match state can only ever have exactly one pattern ID:

  PatternID::ZERO. So there's no need to represent it.

- <span id="repr-is-from-word"></span>`fn is_from_word(&self) -> bool`

  Returns true if and only if this state is marked as having been created

  from a transition over a word byte. This is useful for checking whether

  a word boundary assertion is true or not, which requires look-behind

  (whether the current state came from a word byte or not) and look-ahead

  (whether the transition byte is a word byte or not).

  

  Since states with this set are distinct from states that don't have

  this set (even if they are otherwise equivalent), callers should not

  set this assertion unless the underlying NFA has at least one word

  boundary assertion somewhere. Otherwise, a superfluous number of states

  may be created.

- <span id="repr-is-half-crlf"></span>`fn is_half_crlf(&self) -> bool`

  Returns true if and only if this state is marked as being inside of a

  CRLF terminator. In the forward direction, this means the state was

  created after seeing a `\r`. In the reverse direction, this means the

  state was created after seeing a `\n`.

- <span id="repr-look-have"></span>`fn look_have(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

  The set of look-behind assertions that were true in the transition that

  created this state.

  

  Generally, this should be empty if 'look_need' is empty, since there is

  no reason to track which look-behind assertions are true if the state

  has no conditional epsilon transitions.

  

  Satisfied look-ahead assertions are not tracked in states. Instead,

  these are re-computed on demand via epsilon closure when computing the

  transition function.

- <span id="repr-look-need"></span>`fn look_need(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

  The set of look-around (both behind and ahead) assertions that appear

  at least once in this state's set of NFA states.

  

  This is used to determine whether the epsilon closure needs to be

  re-computed when computing the transition function. Namely, if the

  state has no conditional epsilon transitions, then there is no need

  to re-compute the epsilon closure.

- <span id="repr-match-len"></span>`fn match_len(&self) -> usize`

  Returns the total number of match pattern IDs in this state.

  

  If this state is not a match state, then this always returns 0.

- <span id="repr-match-pattern"></span>`fn match_pattern(&self, index: usize) -> PatternID` — [`PatternID`](../../primitives/index.md#patternid)

  Returns the pattern ID for this match state at the given index.

  

  If the given index is greater than or equal to `match_len()` for this

  state, then this could panic or return incorrect results.

- <span id="repr-match-pattern-ids"></span>`fn match_pattern_ids(&self) -> Option<Vec<PatternID>>` — [`PatternID`](../../primitives/index.md#patternid)

  Returns a copy of all match pattern IDs in this state. If this state

  is not a match state, then this returns None.

- <span id="repr-iter-match-pattern-ids"></span>`fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, f: F)`

  Calls the given function on every pattern ID in this state.

- <span id="repr-iter-nfa-state-ids"></span>`fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F)`

  Calls the given function on every NFA state ID in this state.

- <span id="repr-pattern-offset-end"></span>`fn pattern_offset_end(&self) -> usize`

  Returns the offset into this state's representation where the pattern

  IDs end and the NFA state IDs begin.

- <span id="repr-encoded-pattern-len"></span>`fn encoded_pattern_len(&self) -> usize`

  Returns the total number of *encoded* pattern IDs in this state.

  

  This may return 0 even when this is a match state, since the pattern

  ID `PatternID::ZERO` is not encoded when it's the only pattern ID in

  the match state (the overwhelming common case).

#### Trait Implementations

##### `impl Any for Repr<'a>`

- <span id="repr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Repr<'a>`

- <span id="repr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Repr<'a>`

- <span id="repr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Repr<'a>`

- <span id="repr-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Repr<'a>`

- <span id="repr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Repr<'a>`

- <span id="repr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Repr<'a>`

- <span id="repr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="repr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Repr<'a>`

- <span id="repr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="repr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReprVec<'a>`

```rust
struct ReprVec<'a>(&'a mut alloc::vec::Vec<u8>);
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:588`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L588)*

ReprVec is a write-only view into the representation of a DFA state.

See Repr for more details on the purpose of this type and also the format.

Note that not all possible combinations of methods may be called. This is
precisely what the various StateBuilder types encapsulate: they only
permit valid combinations via Rust's linear typing.

#### Implementations

- <span id="reprvec-set-is-match"></span>`fn set_is_match(&mut self)`

  Set this state as a match state.

  

  This should not be exposed explicitly outside of this module. It is

  set automatically when a pattern ID is added.

- <span id="reprvec-set-has-pattern-ids"></span>`fn set_has_pattern_ids(&mut self)`

  Set that this state has pattern IDs explicitly written to it.

  

  This should not be exposed explicitly outside of this module. This is

  used internally as a space saving optimization. Namely, if the state

  is a match state but does not have any pattern IDs written to it,

  then it is automatically inferred to have a pattern ID of ZERO.

- <span id="reprvec-set-is-from-word"></span>`fn set_is_from_word(&mut self)`

  Set this state as being built from a transition over a word byte.

  

  Setting this is only necessary when one needs to deal with word

  boundary assertions. Therefore, if the underlying NFA has no word

  boundary assertions, callers should not set this.

- <span id="reprvec-set-is-half-crlf"></span>`fn set_is_half_crlf(&mut self)`

  Set this state as having seen half of a CRLF terminator.

  

  In the forward direction, this should be set when a `\r` has been seen.

  In the reverse direction, this should be set when a `\n` has been seen.

- <span id="reprvec-look-have"></span>`fn look_have(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

  The set of look-behind assertions that were true in the transition that

  created this state.

- <span id="reprvec-look-need"></span>`fn look_need(&self) -> LookSet` — [`LookSet`](../../look/index.md#lookset)

  The set of look-around (both behind and ahead) assertions that appear

  at least once in this state's set of NFA states.

- <span id="reprvec-set-look-have"></span>`fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md#lookset)

  Mutate the set of look-behind assertions that were true in the

  transition that created this state.

- <span id="reprvec-set-look-need"></span>`fn set_look_need(&mut self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md#lookset)

  Mutate the set of look-around (both behind and ahead) assertions that

  appear at least once in this state's set of NFA states.

- <span id="reprvec-add-match-pattern-id"></span>`fn add_match_pattern_id(&mut self, pid: PatternID)` — [`PatternID`](../../primitives/index.md#patternid)

  Add a pattern ID to this state. All match states must have at least

  one pattern ID associated with it.

  

  Callers must never add duplicative pattern IDs.

  

  The order in which patterns are added must correspond to the order

  in which patterns are reported as matches.

- <span id="reprvec-close-match-pattern-ids"></span>`fn close_match_pattern_ids(&mut self)`

  Indicate that no more pattern IDs will be added to this state.

  

  Once this is called, callers must not call it or 'add_match_pattern_id'

  again.

  

  This should not be exposed explicitly outside of this module. It

  should be called only when converting a StateBuilderMatches into a

  StateBuilderNFA.

- <span id="reprvec-add-nfa-state-id"></span>`fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID)` — [`StateID`](../../primitives/index.md#stateid)

  Add an NFA state ID to this state. The order in which NFA states are

  added matters. It is the caller's responsibility to ensure that

  duplicate NFA state IDs are not added.

- <span id="reprvec-repr"></span>`fn repr(&self) -> Repr<'_>` — [`Repr`](#repr)

  Return a read-only view of this state's representation.

#### Trait Implementations

##### `impl Any for ReprVec<'a>`

- <span id="reprvec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReprVec<'a>`

- <span id="reprvec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReprVec<'a>`

- <span id="reprvec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for ReprVec<'a>`

- <span id="reprvec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReprVec<'a>`

- <span id="reprvec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReprVec<'a>`

- <span id="reprvec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reprvec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReprVec<'a>`

- <span id="reprvec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reprvec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `write_vari32`

```rust
fn write_vari32(data: &mut alloc::vec::Vec<u8>, n: i32)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:735-741`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L735-L741)*

Write a signed 32-bit integer using zig-zag encoding.

https://developers.google.com/protocol-buffers/docs/encoding#varints

### `read_vari32`

```rust
fn read_vari32(data: &[u8]) -> (i32, usize)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:747-754`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L747-L754)*

Read a signed 32-bit integer using zig-zag encoding. Also, return the
number of bytes read.

https://developers.google.com/protocol-buffers/docs/encoding#varints

### `write_varu32`

```rust
fn write_varu32(data: &mut alloc::vec::Vec<u8>, n: u32)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:763-769`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L763-L769)*

Write an unsigned 32-bit integer as a varint. In essence, `n` is written
as a sequence of bytes where all bytes except for the last one have the
most significant bit set. The least significant 7 bits correspond to the
actual bits of `n`. So in the worst case, a varint uses 5 bytes, but in
very common cases, it uses fewer than 4.

https://developers.google.com/protocol-buffers/docs/encoding#varints

### `read_varu32`

```rust
fn read_varu32(data: &[u8]) -> (u32, usize)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:774-788`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L774-L788)*

Read an unsigned 32-bit varint. Also, return the number of bytes read.

https://developers.google.com/protocol-buffers/docs/encoding#varints

### `write_u32`

```rust
fn write_u32(dst: &mut alloc::vec::Vec<u8>, n: u32)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/state.rs:791-797`](../../../../../.source_1765633015/regex-automata-0.4.13/src/util/determinize/state.rs#L791-L797)*

Push a native-endian encoded `n` on to `dst`.

