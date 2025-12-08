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

## Structs

### `State`

```rust
struct State(alloc::sync::Arc<[u8]>);
```

A DFA state that, at its core, is represented by an ordered set of NFA
states.

This type is intended to be used only in NFA-to-DFA conversion via powerset
construction.

It may be cheaply cloned and accessed safely from multiple threads
simultaneously.

#### Implementations

- `fn dead() -> State` — [`State`](#state)

- `fn is_match(self: &Self) -> bool`

- `fn is_from_word(self: &Self) -> bool`

- `fn is_half_crlf(self: &Self) -> bool`

- `fn look_have(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn look_need(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn match_len(self: &Self) -> usize`

- `fn match_pattern(self: &Self, index: usize) -> PatternID` — [`PatternID`](../../../index.md)

- `fn match_pattern_ids(self: &Self) -> Option<Vec<PatternID>>` — [`PatternID`](../../../index.md)

- `fn iter_nfa_state_ids<F: FnMut(StateID)>(self: &Self, f: F)`

- `fn memory_usage(self: &Self) -> usize`

- `fn repr(self: &Self) -> Repr<'_>` — [`Repr`](#repr)

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for State`

##### `impl Hash for State`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord for State`

- `fn cmp(self: &Self, other: &State) -> $crate::cmp::Ordering` — [`State`](#state)

##### `impl PartialEq for State`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](#state)

##### `impl PartialOrd for State`

- `fn partial_cmp(self: &Self, other: &State) -> $crate::option::Option<$crate::cmp::Ordering>` — [`State`](#state)

##### `impl StructuralPartialEq for State`

### `StateBuilderEmpty`

```rust
struct StateBuilderEmpty(alloc::vec::Vec<u8>);
```

A state builder that represents an empty state.

This is a useful "initial condition" for state construction. It has no
NFA state IDs, no assertions set and no pattern IDs. No allocations are
made when new() is called. Its main use is for being converted into a
builder that can capture assertions and pattern IDs.

#### Implementations

- `fn new() -> StateBuilderEmpty` — [`StateBuilderEmpty`](#statebuilderempty)

- `fn into_matches(self: Self) -> StateBuilderMatches` — [`StateBuilderMatches`](#statebuildermatches)

- `fn clear(self: &mut Self)`

- `fn capacity(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for StateBuilderEmpty`

- `fn clone(self: &Self) -> StateBuilderEmpty` — [`StateBuilderEmpty`](#statebuilderempty)

##### `impl Debug for StateBuilderEmpty`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StateBuilderMatches`

```rust
struct StateBuilderMatches(alloc::vec::Vec<u8>);
```

A state builder that collects assertions and pattern IDs.

When collecting pattern IDs is finished, this can be converted into a
builder that collects NFA state IDs.

#### Implementations

- `fn into_nfa(self: Self) -> StateBuilderNFA` — [`StateBuilderNFA`](#statebuildernfa)

- `fn set_is_from_word(self: &mut Self)`

- `fn set_is_half_crlf(self: &mut Self)`

- `fn look_have(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn set_look_have(self: &mut Self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md)

- `fn add_match_pattern_id(self: &mut Self, pid: PatternID)` — [`PatternID`](../../../index.md)

- `fn repr(self: &Self) -> Repr<'_>` — [`Repr`](#repr)

- `fn repr_vec(self: &mut Self) -> ReprVec<'_>` — [`ReprVec`](#reprvec)

#### Trait Implementations

##### `impl Clone for StateBuilderMatches`

- `fn clone(self: &Self) -> StateBuilderMatches` — [`StateBuilderMatches`](#statebuildermatches)

##### `impl Debug for StateBuilderMatches`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `StateBuilderNFA`

```rust
struct StateBuilderNFA {
    repr: alloc::vec::Vec<u8>,
    prev_nfa_state_id: crate::util::primitives::StateID,
}
```

A state builder that collects some assertions and NFA state IDs.

When collecting NFA state IDs is finished, this can be used to build a
`State` if necessary.

When dont with building a state (regardless of whether it got kept or not),
it's usually a good idea to call `clear` to get an empty builder back so
that it can be reused to build the next state.

#### Implementations

- `fn to_state(self: &Self) -> State` — [`State`](#state)

- `fn clear(self: Self) -> StateBuilderEmpty` — [`StateBuilderEmpty`](#statebuilderempty)

- `fn look_need(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn set_look_have(self: &mut Self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md)

- `fn set_look_need(self: &mut Self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md)

- `fn add_nfa_state_id(self: &mut Self, sid: StateID)` — [`StateID`](../../primitives/index.md)

- `fn as_bytes(self: &Self) -> &[u8]`

- `fn repr(self: &Self) -> Repr<'_>` — [`Repr`](#repr)

- `fn repr_vec(self: &mut Self) -> ReprVec<'_>` — [`ReprVec`](#reprvec)

#### Trait Implementations

##### `impl Clone for StateBuilderNFA`

- `fn clone(self: &Self) -> StateBuilderNFA` — [`StateBuilderNFA`](#statebuildernfa)

##### `impl Debug for StateBuilderNFA`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Repr<'a>`

```rust
struct Repr<'a>(&'a [u8]);
```

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

- `fn is_match(self: &Self) -> bool`

- `fn has_pattern_ids(self: &Self) -> bool`

- `fn is_from_word(self: &Self) -> bool`

- `fn is_half_crlf(self: &Self) -> bool`

- `fn look_have(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn look_need(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn match_len(self: &Self) -> usize`

- `fn match_pattern(self: &Self, index: usize) -> PatternID` — [`PatternID`](../../../index.md)

- `fn match_pattern_ids(self: &Self) -> Option<Vec<PatternID>>` — [`PatternID`](../../../index.md)

- `fn iter_match_pattern_ids<F: FnMut(PatternID)>(self: &Self, f: F)`

- `fn iter_nfa_state_ids<F: FnMut(StateID)>(self: &Self, f: F)`

- `fn pattern_offset_end(self: &Self) -> usize`

- `fn encoded_pattern_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl<'a> Debug for Repr<'a>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `ReprVec<'a>`

```rust
struct ReprVec<'a>(&'a mut alloc::vec::Vec<u8>);
```

ReprVec is a write-only view into the representation of a DFA state.

See Repr for more details on the purpose of this type and also the format.

Note that not all possible combinations of methods may be called. This is
precisely what the various StateBuilder types encapsulate: they only
permit valid combinations via Rust's linear typing.

#### Implementations

- `fn set_is_match(self: &mut Self)`

- `fn set_has_pattern_ids(self: &mut Self)`

- `fn set_is_from_word(self: &mut Self)`

- `fn set_is_half_crlf(self: &mut Self)`

- `fn look_have(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn look_need(self: &Self) -> LookSet` — [`LookSet`](../../look/index.md)

- `fn set_look_have(self: &mut Self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md)

- `fn set_look_need(self: &mut Self, set: impl FnMut(LookSet) -> LookSet)` — [`LookSet`](../../look/index.md)

- `fn add_match_pattern_id(self: &mut Self, pid: PatternID)` — [`PatternID`](../../../index.md)

- `fn close_match_pattern_ids(self: &mut Self)`

- `fn add_nfa_state_id(self: &mut Self, prev: &mut StateID, sid: StateID)` — [`StateID`](../../primitives/index.md)

- `fn repr(self: &Self) -> Repr<'_>` — [`Repr`](#repr)

## Functions

### `write_vari32`

```rust
fn write_vari32(data: &mut alloc::vec::Vec<u8>, n: i32)
```

Write a signed 32-bit integer using zig-zag encoding.

https://developers.google.com/protocol-buffers/docs/encoding#varints

### `read_vari32`

```rust
fn read_vari32(data: &[u8]) -> (i32, usize)
```

Read a signed 32-bit integer using zig-zag encoding. Also, return the
number of bytes read.

https://developers.google.com/protocol-buffers/docs/encoding#varints

### `write_varu32`

```rust
fn write_varu32(data: &mut alloc::vec::Vec<u8>, n: u32)
```

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

Read an unsigned 32-bit varint. Also, return the number of bytes read.

https://developers.google.com/protocol-buffers/docs/encoding#varints

### `write_u32`

```rust
fn write_u32(dst: &mut alloc::vec::Vec<u8>, n: u32)
```

Push a native-endian encoded `n` on to `dst`.

