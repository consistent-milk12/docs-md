*[regex_automata](../../index.md) / [util](../index.md) / [determinize](index.md)*

---

# Module `determinize`

This module contains types and routines for implementing determinization.

In this crate, there are at least two places where we implement
determinization: fully ahead-of-time compiled DFAs in the `dfa` module and
lazily compiled DFAs in the `hybrid` module. The stuff in this module
corresponds to the things that are in common between these implementations.

There are three broad things that our implementations of determinization have
in common, as defined by this module:

* The classification of start states. That is, whether we're dealing with
word boundaries, line boundaries, etc., is all the same. This also includes
the look-behind assertions that are satisfied by each starting state
classification.
* The representation of DFA states as sets of NFA states, including
convenience types for building these DFA states that are amenable to reusing
allocations.
* Routines for the "classical" parts of determinization: computing the
epsilon closure, tracking match states (with corresponding pattern IDs, since
we support multi-pattern finite automata) and, of course, computing the
transition function between states for units of input.

I did consider a couple of alternatives to this particular form of code reuse:

1. Don't do any code reuse. The problem here is that we *really* want both
forms of determinization to do exactly identical things when it comes to
their handling of NFA states. While our tests generally ensure this, the code
is tricky and large enough where not reusing code is a pretty big bummer.

2. Implement all of determinization once and make it generic over fully
compiled DFAs and lazily compiled DFAs. While I didn't actually try this
approach, my instinct is that it would be more complex than is needed here.
And the interface required would be pretty hairy. Instead, I think splitting
it into logical sub-components works better.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`state`](#state) | mod | This module defines a DFA state representation and builders for constructing DFA states. |
| [`next`](#next) | fn | Compute the set of all reachable NFA states, including the full epsilon closure, from a DFA state for a single unit of input. |
| [`epsilon_closure`](#epsilon_closure) | fn | Compute the epsilon closure for the given NFA state. |
| [`add_nfa_states`](#add_nfa_states) | fn | Add the NFA state IDs in the given `set` to the given DFA builder state. |
| [`set_lookbehind_from_start`](#set_lookbehind_from_start) | fn | Sets the appropriate look-behind assertions on the given state based on this starting configuration. |

## Modules

- [`state`](state/index.md) — This module defines a DFA state representation and builders for constructing

## Functions

### `next`

```rust
fn next(nfa: &thompson::NFA, match_kind: crate::util::search::MatchKind, sparses: &mut crate::util::sparse_set::SparseSets, stack: &mut alloc::vec::Vec<crate::util::primitives::StateID>, state: &self::state::State, unit: alphabet::Unit, empty_builder: self::state::StateBuilderEmpty) -> self::state::StateBuilderNFA
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/mod.rs:92-353`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/determinize/mod.rs#L92-L353)*

Compute the set of all reachable NFA states, including the full epsilon
closure, from a DFA state for a single unit of input. The set of reachable
states is returned as a `StateBuilderNFA`. The `StateBuilderNFA` returned
also includes any look-behind assertions satisfied by `unit`, in addition
to whether it is a match state. For multi-pattern DFAs, the builder will
also include the pattern IDs that match (in the order seen).

`nfa` must be able to resolve any NFA state in `state` and any NFA state
reachable via the epsilon closure of any NFA state in `state`. `sparses`
must have capacity equivalent to `nfa.len()`.

`match_kind` should correspond to the match semantics implemented by the
DFA being built. Generally speaking, for leftmost-first match semantics,
states that appear after the first NFA match state will not be included in
the `StateBuilderNFA` returned since they are impossible to visit.

`sparses` is used as scratch space for NFA traversal. Other than their
capacity requirements (detailed above), there are no requirements on what's
contained within them (if anything). Similarly, what's inside of them once
this routine returns is unspecified.

`stack` must have length 0. It is used as scratch space for depth first
traversal. After returning, it is guaranteed that `stack` will have length
0.

`state` corresponds to the current DFA state on which one wants to compute
the transition for the input `unit`.

`empty_builder` corresponds to the builder allocation to use to produce a
complete `StateBuilderNFA` state. If the state is not needed (or is already
cached), then it can be cleared and reused without needing to create a new
`State`. The `StateBuilderNFA` state returned is final and ready to be
turned into a `State` if necessary.

### `epsilon_closure`

```rust
fn epsilon_closure(nfa: &thompson::NFA, start_nfa_id: crate::util::primitives::StateID, look_have: crate::util::look::LookSet, stack: &mut alloc::vec::Vec<crate::util::primitives::StateID>, set: &mut crate::util::sparse_set::SparseSet)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/mod.rs:369-428`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/determinize/mod.rs#L369-L428)*

Compute the epsilon closure for the given NFA state. The epsilon closure
consists of all NFA state IDs, including `start_nfa_id`, that can be
reached from `start_nfa_id` without consuming any input. These state IDs
are written to `set` in the order they are visited, but only if they are
not already in `set`. `start_nfa_id` must be a valid state ID for the NFA
given.

`look_have` consists of the satisfied assertions at the current
position. For conditional look-around epsilon transitions, these are
only followed if they are satisfied by `look_have`.

`stack` must have length 0. It is used as scratch space for depth first
traversal. After returning, it is guaranteed that `stack` will have length
0.

### `add_nfa_states`

```rust
fn add_nfa_states(nfa: &thompson::NFA, set: &crate::util::sparse_set::SparseSet, builder: &mut self::state::StateBuilderNFA)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/mod.rs:448-579`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/determinize/mod.rs#L448-L579)*

Add the NFA state IDs in the given `set` to the given DFA builder state.
The order in which states are added corresponds to the order in which they
were added to `set`.

The DFA builder state given should already have its complete set of match
pattern IDs added (if any) and any look-behind assertions (StartLF, Start
and whether this state is being generated for a transition over a word byte
when applicable) that are true immediately prior to transitioning into this
state (via `builder.look_have()`). The match pattern IDs should correspond
to matches that occurred on the previous transition, since all matches are
delayed by one byte. The things that should _not_ be set are look-ahead
assertions (EndLF, End and whether the next byte is a word byte or not).
The builder state should also not have anything in `look_need` set, as this
routine will compute that for you.

The given NFA should be able to resolve all identifiers in `set` to a
particular NFA state. Additionally, `set` must have capacity equivalent
to `nfa.len()`.

### `set_lookbehind_from_start`

```rust
fn set_lookbehind_from_start(nfa: &thompson::NFA, start: &crate::util::start::Start, builder: &mut self::state::StateBuilderMatches)
```

*Defined in [`regex-automata-0.4.13/src/util/determinize/mod.rs:583-682`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/determinize/mod.rs#L583-L682)*

Sets the appropriate look-behind assertions on the given state based on
this starting configuration.

