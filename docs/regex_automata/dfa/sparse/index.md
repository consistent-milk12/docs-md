*[regex_automata](../../index.md) / [dfa](../index.md) / [sparse](index.md)*

---

# Module `sparse`

Types and routines specific to sparse DFAs.

This module is the home of [`sparse::DFA`](DFA).

Unlike the [`dense`](../dense/index.md) module, this module does not contain a builder or
configuration specific for sparse DFAs. Instead, the intended way to build a
sparse DFA is either by using a default configuration with its constructor
[`sparse::DFA::new`](DFA::new), or by first configuring the construction of a
dense DFA with [`dense::Builder`](../dense/index.md) and then calling `dense::DFA::to_sparse`.
For example, this configures a sparse DFA to do an overlapping search:

```rust
use regex_automata::{
    dfa::{Automaton, OverlappingState, dense},
    HalfMatch, Input, MatchKind,
};

let dense_re = dense::Builder::new()
    .configure(dense::Config::new().match_kind(MatchKind::All))
    .build(r"Samwise|Sam")?;
let sparse_re = dense_re.to_sparse()?;

// Setup our haystack and initial start state.
let input = Input::new("Samwise");
let mut state = OverlappingState::start();

// First, 'Sam' will match.
sparse_re.try_search_overlapping_fwd(&input, &mut state)?;
assert_eq!(Some(HalfMatch::must(0, 3)), state.get_match());

// And now 'Samwise' will match.
sparse_re.try_search_overlapping_fwd(&input, &mut state)?;
assert_eq!(Some(HalfMatch::must(0, 7)), state.get_match());
Ok::<(), Box<dyn std::error::Error>>(())
```

## Structs

### `DFA<T>`

```rust
struct DFA<T> {
    tt: Transitions<T>,
    st: StartTable<T>,
    special: crate::dfa::special::Special,
    pre: Option<crate::util::prefilter::Prefilter>,
    quitset: crate::util::alphabet::ByteSet,
    flags: crate::dfa::dense::Flags,
}
```

A sparse deterministic finite automaton (DFA) with variable sized states.

In contrast to a [dense::DFA], a sparse DFA uses a more space efficient
representation for its transitions. Consequently, sparse DFAs may use much
less memory than dense DFAs, but this comes at a price. In particular,
reading the more space efficient transitions takes more work, and
consequently, searching using a sparse DFA is typically slower than a dense
DFA.

A sparse DFA can be built using the default configuration via the
`DFA::new` constructor. Otherwise, one can configure various aspects of a
dense DFA via [`dense::Builder`](../dense/index.md), and then convert a dense DFA to a sparse
DFA using `dense::DFA::to_sparse`.

In general, a sparse DFA supports all the same search operations as a dense
DFA.

Making the choice between a dense and sparse DFA depends on your specific
work load. If you can sacrifice a bit of search time performance, then a
sparse DFA might be the best choice. In particular, while sparse DFAs are
probably always slower than dense DFAs, you may find that they are easily
fast enough for your purposes!

# Type parameters

A `DFA` has one type parameter, `T`, which is used to represent the parts
of a sparse DFA. `T` is typically a `Vec<u8>` or a `&[u8]`.

# The `Automaton` trait

This type implements the [`Automaton`](../automaton/index.md) trait, which means it can be used
for searching. For example:

```rust
use regex_automata::{dfa::{Automaton, sparse::DFA}, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let expected = Some(HalfMatch::must(0, 8));
assert_eq!(expected, dfa.try_search_fwd(&Input::new("foo12345"))?);
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn from_bytes(slice: &'a [u8]) -> Result<(DFA<&'a [u8]>, usize), DeserializeError>` — [`DFA`](#dfa), [`DeserializeError`](../../util/wire/index.md)

- `unsafe fn from_bytes_unchecked(slice: &'a [u8]) -> Result<(DFA<&'a [u8]>, usize), DeserializeError>` — [`DFA`](#dfa), [`DeserializeError`](../../util/wire/index.md)

#### Trait Implementations

##### `impl<T: AsRef<[u8]>> Automaton for DFA<T>`

- `fn is_special_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_dead_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_quit_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_match_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_start_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_accel_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn next_state(self: &Self, current: StateID, input: u8) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `unsafe fn next_state_unchecked(self: &Self, current: StateID, input: u8) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `fn next_eoi_state(self: &Self, current: StateID) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `fn pattern_len(self: &Self) -> usize`

- `fn match_len(self: &Self, id: StateID) -> usize` — [`StateID`](../../util/primitives/index.md)

- `fn match_pattern(self: &Self, id: StateID, match_index: usize) -> PatternID` — [`StateID`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn has_empty(self: &Self) -> bool`

- `fn is_utf8(self: &Self) -> bool`

- `fn is_always_start_anchored(self: &Self) -> bool`

- `fn start_state(self: &Self, config: &start::Config) -> Result<StateID, StartError>` — [`Config`](../../util/start/index.md), [`StateID`](../../util/primitives/index.md), [`StartError`](../automaton/index.md)

- `fn universal_start_state(self: &Self, mode: Anchored) -> Option<StateID>` — [`Anchored`](../../index.md), [`StateID`](../../util/primitives/index.md)

- `fn accelerator(self: &Self, id: StateID) -> &[u8]` — [`StateID`](../../util/primitives/index.md)

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md)

##### `impl<T: $crate::clone::Clone> Clone for DFA<T>`

- `fn clone(self: &Self) -> DFA<T>` — [`DFA`](#dfa)

##### `impl<T: AsRef<[u8]>> Debug for DFA<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

