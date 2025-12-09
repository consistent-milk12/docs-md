*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [nfa](index.md)*

---

# Module `nfa`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NFA`](#nfa) | struct | A byte oriented Thompson non-deterministic finite automaton (NFA). |
| [`Inner`](#inner) | struct | The "inner" part of the NFA. |
| [`SparseTransitions`](#sparsetransitions) | struct | A sequence of transitions used to represent a sparse state. |
| [`DenseTransitions`](#densetransitions) | struct | A sequence of transitions used to represent a dense state. |
| [`Transition`](#transition) | struct | A single transition to another state. |
| [`PatternIter`](#patterniter) | struct | An iterator over all pattern IDs in an NFA. |
| [`State`](#state) | enum | A state in an NFA. |

## Structs

### `NFA`

```rust
struct NFA(alloc::sync::Arc<Inner>);
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:190-202`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L190-L202)*

A byte oriented Thompson non-deterministic finite automaton (NFA).

A Thompson NFA is a finite state machine that permits unconditional epsilon
transitions, but guarantees that there exists at most one non-epsilon
transition for each element in the alphabet for each state.

An NFA may be used directly for searching, for analysis or to build
a deterministic finite automaton (DFA).

# Cheap clones

Since an NFA is a core data type in this crate that many other regex
engines are based on top of, it is convenient to give ownership of an NFA
to said regex engines. Because of this, an NFA uses reference counting
internally. Therefore, it is cheap to clone and it is encouraged to do so.

# Capabilities

Using an NFA for searching via the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) provides the most amount
of "power" of any regex engine in this crate. Namely, it supports the
following in all cases:

1. Detection of a match.
2. Location of a match, including both the start and end offset, in a
single pass of the haystack.
3. Location of matching capturing groups.
4. Handles multiple patterns, including (1)-(3) when multiple patterns are
present.

# Capturing Groups

Groups refer to parenthesized expressions inside a regex pattern. They look
like this, where `exp` is an arbitrary regex:

* `(exp)` - An unnamed capturing group.
* `(?P<name>exp)` or `(?<name>exp)` - A named capturing group.
* `(?:exp)` - A non-capturing group.
* `(?i:exp)` - A non-capturing group that sets flags.

Only the first two forms are said to be _capturing_. Capturing
means that the last position at which they match is reportable. The
[`Captures`](crate::util::captures::Captures) type provides convenient
access to the match positions of capturing groups, which includes looking
up capturing groups by their name.

# Byte oriented

This NFA is byte oriented, which means that all of its transitions are
defined on bytes. In other words, the alphabet of an NFA consists of the
256 different byte values.

While DFAs nearly demand that they be byte oriented for performance
reasons, an NFA could conceivably be *Unicode codepoint* oriented. Indeed,
a previous version of this NFA supported both byte and codepoint oriented
modes. A codepoint oriented mode can work because an NFA fundamentally uses
a sparse representation of transitions, which works well with the large
sparse space of Unicode codepoints.

Nevertheless, this NFA is only byte oriented. This choice is primarily
driven by implementation simplicity, and also in part memory usage. In
practice, performance between the two is roughly comparable. However,
building a DFA (including a hybrid DFA) really wants a byte oriented NFA.
So if we do have a codepoint oriented NFA, then we also need to generate
byte oriented NFA in order to build an hybrid NFA/DFA. Thus, by only
generating byte oriented NFAs, we can produce one less NFA. In other words,
if we made our NFA codepoint oriented, we'd need to *also* make it support
a byte oriented mode, which is more complicated. But a byte oriented mode
can support everything.

# Differences with DFAs

At the theoretical level, the precise difference between an NFA and a DFA
is that, in a DFA, for every state, an input symbol unambiguously refers
to a single transition _and_ that an input symbol is required for each
transition. At a practical level, this permits DFA implementations to be
implemented at their core with a small constant number of CPU instructions
for each byte of input searched. In practice, this makes them quite a bit
faster than NFAs _in general_. Namely, in order to execute a search for any
Thompson NFA, one needs to keep track of a _set_ of states, and execute
the possible transitions on all of those states for each input symbol.
Overall, this results in much more overhead. To a first approximation, one
can expect DFA searches to be about an order of magnitude faster.

So why use an NFA at all? The main advantage of an NFA is that it takes
linear time (in the size of the pattern string after repetitions have been
expanded) to build and linear memory usage. A DFA, on the other hand, may
take exponential time and/or space to build. Even in non-pathological
cases, DFAs often take quite a bit more memory than their NFA counterparts,
_especially_ if large Unicode character classes are involved. Of course,
an NFA also provides additional capabilities. For example, it can match
Unicode word boundaries on non-ASCII text and resolve the positions of
capturing groups.

Note that a [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) strikes a
good balance between an NFA and a DFA. It avoids the exponential build time
of a DFA while maintaining its fast search time. The downside of a hybrid
NFA/DFA is that in some cases it can be slower at search time than the NFA.
(It also has less functionality than a pure NFA. It cannot handle Unicode
word boundaries on non-ASCII text and cannot resolve capturing groups.)

# Example

This shows how to build an NFA with the default configuration and execute a
search using the Pike VM.

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"foo[0-9]+")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let expected = Some(Match::must(0, 0..8));
re.captures(&mut cache, b"foo12345", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: resolving capturing groups

This example shows how to parse some simple dates and extract the
components of each date via capturing groups.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::captures::Captures,
};

let vm = PikeVM::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})")?;
let mut cache = vm.create_cache();

let haystack = "2012-03-14, 2013-01-01 and 2014-07-05";
let all: Vec<Captures> = vm.captures_iter(
    &mut cache, haystack.as_bytes()
).collect();
// There should be a total of 3 matches.
assert_eq!(3, all.len());
// The year from the second match is '2013'.
let span = all[1].get_group_by_name("y").unwrap();
assert_eq!("2013", &haystack[span]);

Ok::<(), Box<dyn std::error::Error>>(())
```

This example shows that only the last match of a capturing group is
reported, even if it had to match multiple times for an overall match
to occur.

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"([a-z]){4}")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let haystack = b"quux";
re.captures(&mut cache, haystack, &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(3..4)), caps.get_group(1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="nfa-new"></span>`fn new(pattern: &str) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../error/index.md)

- <span id="nfa-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../error/index.md)

- <span id="nfa-always-match"></span>`fn always_match() -> NFA` — [`NFA`](#nfa)

- <span id="nfa-never-match"></span>`fn never_match() -> NFA` — [`NFA`](#nfa)

- <span id="nfa-config"></span>`fn config() -> Config` — [`Config`](../compiler/index.md)

- <span id="nfa-compiler"></span>`fn compiler() -> Compiler` — [`Compiler`](../compiler/index.md)

- <span id="nfa-patterns"></span>`fn patterns(&self) -> PatternIter<'_>` — [`PatternIter`](#patterniter)

- <span id="nfa-pattern-len"></span>`fn pattern_len(&self) -> usize`

- <span id="nfa-start-anchored"></span>`fn start_anchored(&self) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- <span id="nfa-start-unanchored"></span>`fn start_unanchored(&self) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- <span id="nfa-start-pattern"></span>`fn start_pattern(&self, pid: PatternID) -> Option<StateID>` — [`PatternID`](../../../util/primitives/index.md), [`StateID`](../../../util/primitives/index.md)

- <span id="nfa-byte-class-set"></span>`fn byte_class_set(&self) -> &ByteClassSet` — [`ByteClassSet`](../../../util/alphabet/index.md)

- <span id="nfa-byte-classes"></span>`fn byte_classes(&self) -> &ByteClasses` — [`ByteClasses`](../../../util/alphabet/index.md)

- <span id="nfa-state"></span>`fn state(&self, id: StateID) -> &State` — [`StateID`](../../../util/primitives/index.md), [`State`](#state)

- <span id="nfa-states"></span>`fn states(&self) -> &[State]` — [`State`](#state)

- <span id="nfa-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../../util/captures/index.md)

- <span id="nfa-has-capture"></span>`fn has_capture(&self) -> bool`

- <span id="nfa-has-empty"></span>`fn has_empty(&self) -> bool`

- <span id="nfa-is-utf8"></span>`fn is_utf8(&self) -> bool`

- <span id="nfa-is-reverse"></span>`fn is_reverse(&self) -> bool`

- <span id="nfa-is-always-start-anchored"></span>`fn is_always_start_anchored(&self) -> bool`

- <span id="nfa-look-matcher"></span>`fn look_matcher(&self) -> &LookMatcher` — [`LookMatcher`](../../../util/look/index.md)

- <span id="nfa-look-set-any"></span>`fn look_set_any(&self) -> LookSet` — [`LookSet`](../../../util/look/index.md)

- <span id="nfa-look-set-prefix-any"></span>`fn look_set_prefix_any(&self) -> LookSet` — [`LookSet`](../../../util/look/index.md)

- <span id="nfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for NFA`

- <span id="nfa-clone"></span>`fn clone(&self) -> NFA` — [`NFA`](#nfa)

##### `impl Debug for NFA`

- <span id="nfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Inner`

```rust
struct Inner {
    states: alloc::vec::Vec<State>,
    start_anchored: crate::util::primitives::StateID,
    start_unanchored: crate::util::primitives::StateID,
    start_pattern: alloc::vec::Vec<crate::util::primitives::StateID>,
    group_info: crate::util::captures::GroupInfo,
    byte_class_set: crate::util::alphabet::ByteClassSet,
    byte_classes: crate::util::alphabet::ByteClasses,
    has_capture: bool,
    has_empty: bool,
    utf8: bool,
    reverse: bool,
    look_matcher: crate::util::look::LookMatcher,
    look_set_any: crate::util::look::LookSet,
    look_set_prefix_any: crate::util::look::LookSet,
    memory_extra: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1195-1268`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1195-L1268)*

The "inner" part of the NFA. We split this part out so that we can easily
wrap it in an `Arc` above in the definition of `NFA`.

See builder.rs for the code that actually builds this type. This module
does provide (internal) mutable methods for adding things to this
NFA before finalizing it, but the high level construction process is
controlled by the builder abstraction. (Which is complicated enough to
get its own module.)

#### Fields

- **`states`**: `alloc::vec::Vec<State>`

  The state sequence. This sequence is guaranteed to be indexable by all
  starting state IDs, and it is also guaranteed to contain at most one
  `Match` state for each pattern compiled into this NFA. (A pattern may
  not have a corresponding `Match` state if a `Match` state is impossible
  to reach.)

- **`start_anchored`**: `crate::util::primitives::StateID`

  The anchored starting state of this NFA.

- **`start_unanchored`**: `crate::util::primitives::StateID`

  The unanchored starting state of this NFA.

- **`start_pattern`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The starting states for each individual pattern. Starting at any
  of these states will result in only an anchored search for the
  corresponding pattern. The vec is indexed by pattern ID. When the NFA
  contains a single regex, then `start_pattern[0]` and `start_anchored`
  are always equivalent.

- **`group_info`**: `crate::util::captures::GroupInfo`

  Info about the capturing groups in this NFA. This is responsible for
  mapping groups to slots, mapping groups to names and names to groups.

- **`byte_class_set`**: `crate::util::alphabet::ByteClassSet`

  A representation of equivalence classes over the transitions in this
  NFA. Two bytes in the same equivalence class must not discriminate
  between a match or a non-match. This map can be used to shrink the
  total size of a DFA's transition table with a small match-time cost.
  
  Note that the NFA's transitions are *not* defined in terms of these
  equivalence classes. The NFA's transitions are defined on the original
  byte values. For the most part, this is because they wouldn't really
  help the NFA much since the NFA already uses a sparse representation
  to represent transitions. Byte classes are most effective in a dense
  representation.

- **`byte_classes`**: `crate::util::alphabet::ByteClasses`

  This is generated from `byte_class_set`, and essentially represents the
  same thing but supports different access patterns. Namely, this permits
  looking up the equivalence class of a byte very cheaply.
  
  Ideally we would just store this, but because of annoying code
  structure reasons, we keep both this and `byte_class_set` around for
  now. I think I would prefer that `byte_class_set` were computed in the
  `Builder`, but right now, we compute it as states are added to the
  `NFA`.

- **`has_capture`**: `bool`

  Whether this NFA has a `Capture` state anywhere.

- **`has_empty`**: `bool`

  When the empty string is in the language matched by this NFA.

- **`utf8`**: `bool`

  Whether UTF-8 mode is enabled for this NFA. Briefly, this means that
  all non-empty matches produced by this NFA correspond to spans of valid
  UTF-8, and any empty matches produced by this NFA that split a UTF-8
  encoded codepoint should be filtered out by the corresponding regex
  engine.

- **`reverse`**: `bool`

  Whether this NFA is meant to be matched in reverse or not.

- **`look_matcher`**: `crate::util::look::LookMatcher`

  The matcher to be used for look-around assertions.

- **`look_set_any`**: `crate::util::look::LookSet`

  The union of all look-around assertions that occur anywhere within
  this NFA. If this set is empty, then it means there are precisely zero
  conditional epsilon transitions in the NFA.

- **`look_set_prefix_any`**: `crate::util::look::LookSet`

  The union of all look-around assertions that occur as a zero-length
  prefix for any of the patterns in this NFA.

- **`memory_extra`**: `usize`

  Heap memory used indirectly by NFA states and other things (like the
  various capturing group representations above). Since each state
  might use a different amount of heap, we need to keep track of this
  incrementally.

#### Implementations

- <span id="inner-into-nfa"></span>`fn into_nfa(self) -> NFA` — [`NFA`](#nfa)

- <span id="inner-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../../util/captures/index.md)

- <span id="inner-add"></span>`fn add(&mut self, state: State) -> StateID` — [`State`](#state), [`StateID`](../../../util/primitives/index.md)

- <span id="inner-set-starts"></span>`fn set_starts(&mut self, start_anchored: StateID, start_unanchored: StateID, start_pattern: &[StateID])` — [`StateID`](../../../util/primitives/index.md)

- <span id="inner-set-utf8"></span>`fn set_utf8(&mut self, yes: bool)`

- <span id="inner-set-reverse"></span>`fn set_reverse(&mut self, yes: bool)`

- <span id="inner-set-look-matcher"></span>`fn set_look_matcher(&mut self, m: LookMatcher)` — [`LookMatcher`](../../../util/look/index.md)

- <span id="inner-set-captures"></span>`fn set_captures(&mut self, captures: &[Vec<Option<Arc<str>>>]) -> Result<(), GroupInfoError>` — [`GroupInfoError`](../../../util/captures/index.md)

- <span id="inner-remap"></span>`fn remap(&mut self, old_to_new: &[StateID])` — [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Debug for Inner`

- <span id="inner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Inner`

- <span id="inner-default"></span>`fn default() -> Inner` — [`Inner`](#inner)

### `SparseTransitions`

```rust
struct SparseTransitions {
    pub transitions: alloc::boxed::Box<[Transition]>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1795-1798`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1795-L1798)*

A sequence of transitions used to represent a sparse state.

This is the primary representation of a [`Sparse`](State::Sparse) state.
It corresponds to a sorted sequence of transitions with non-overlapping
byte ranges. If the byte at the current position in the haystack matches
one of the byte ranges, then the finite state machine should take the
corresponding transition.

#### Fields

- **`transitions`**: `alloc::boxed::Box<[Transition]>`

  The sorted sequence of non-overlapping transitions.

#### Implementations

- <span id="sparsetransitions-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md)

- <span id="sparsetransitions-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../../util/alphabet/index.md), [`StateID`](../../../util/primitives/index.md)

- <span id="sparsetransitions-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for SparseTransitions`

- <span id="sparsetransitions-clone"></span>`fn clone(&self) -> SparseTransitions` — [`SparseTransitions`](#sparsetransitions)

##### `impl Debug for SparseTransitions`

- <span id="sparsetransitions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SparseTransitions`

##### `impl PartialEq for SparseTransitions`

- <span id="sparsetransitions-eq"></span>`fn eq(&self, other: &SparseTransitions) -> bool` — [`SparseTransitions`](#sparsetransitions)

##### `impl StructuralPartialEq for SparseTransitions`

### `DenseTransitions`

```rust
struct DenseTransitions {
    pub transitions: alloc::boxed::Box<[crate::util::primitives::StateID]>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1882-1886`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1882-L1886)*

A sequence of transitions used to represent a dense state.

This is the primary representation of a [`Dense`](State::Dense) state. It
provides constant time matching. That is, given a byte in a haystack and
a `DenseTransitions`, one can determine if the state matches in constant
time.

This is in contrast to `SparseTransitions`, whose time complexity is
necessarily bigger than constant time. Also in contrast, `DenseTransitions`
usually requires (much) more heap memory.

#### Fields

- **`transitions`**: `alloc::boxed::Box<[crate::util::primitives::StateID]>`

  A dense representation of this state's transitions on the heap. This
  always has length 256.

#### Implementations

- <span id="densetransitions-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md)

- <span id="densetransitions-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../../util/alphabet/index.md), [`StateID`](../../../util/primitives/index.md)

- <span id="densetransitions-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md)

- <span id="densetransitions-iter"></span>`fn iter(&self) -> impl Iterator<Item = Transition> + '_` — [`Transition`](#transition)

#### Trait Implementations

##### `impl Clone for DenseTransitions`

- <span id="densetransitions-clone"></span>`fn clone(&self) -> DenseTransitions` — [`DenseTransitions`](#densetransitions)

##### `impl Debug for DenseTransitions`

- <span id="densetransitions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DenseTransitions`

##### `impl PartialEq for DenseTransitions`

- <span id="densetransitions-eq"></span>`fn eq(&self, other: &DenseTransitions) -> bool` — [`DenseTransitions`](#densetransitions)

##### `impl StructuralPartialEq for DenseTransitions`

### `Transition`

```rust
struct Transition {
    pub start: u8,
    pub end: u8,
    pub next: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1965-1972`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1965-L1972)*

A single transition to another state.

This transition may only be followed if the current byte in the haystack
falls in the inclusive range of bytes specified.

#### Fields

- **`start`**: `u8`

  The inclusive start of the byte range.

- **`end`**: `u8`

  The inclusive end of the byte range.

- **`next`**: `crate::util::primitives::StateID`

  The identifier of the state to transition to.

#### Implementations

- <span id="transition-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> bool`

- <span id="transition-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> bool` — [`Unit`](../../../util/alphabet/index.md)

- <span id="transition-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for Transition`

- <span id="transition-clone"></span>`fn clone(&self) -> Transition` — [`Transition`](#transition)

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- <span id="transition-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Transition`

##### `impl Hash for Transition`

- <span id="transition-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Transition`

- <span id="transition-eq"></span>`fn eq(&self, other: &Transition) -> bool` — [`Transition`](#transition)

##### `impl StructuralPartialEq for Transition`

### `PatternIter<'a>`

```rust
struct PatternIter<'a> {
    it: crate::util::primitives::PatternIDIter,
    _marker: core::marker::PhantomData<&'a ()>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:2023-2031`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L2023-L2031)*

An iterator over all pattern IDs in an NFA.

This iterator is created by `NFA::patterns`.

The lifetime parameter `'a` refers to the lifetime of the NFA from which
this pattern iterator was created.

#### Fields

- **`_marker`**: `core::marker::PhantomData<&'a ()>`

  We explicitly associate a lifetime with this iterator even though we
  don't actually borrow anything from the NFA. We do this for backward
  compatibility purposes. If we ever do need to borrow something from
  the NFA, then we can and just get rid of this marker without breaking
  the public API.

#### Trait Implementations

##### `impl Debug for PatternIter<'a>`

- <span id="patterniter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for PatternIter<'a>`

- <span id="patterniter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patterniter-type-intoiter"></span>`type IntoIter = I`

- <span id="patterniter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternIter<'a>`

- <span id="patterniter-type-item"></span>`type Item = PatternID`

- <span id="patterniter-next"></span>`fn next(&mut self) -> Option<PatternID>` — [`PatternID`](../../../util/primitives/index.md)

## Enums

### `State`

```rust
enum State {
    ByteRange {
        trans: Transition,
    },
    Sparse(SparseTransitions),
    Dense(DenseTransitions),
    Look {
        look: crate::util::look::Look,
        next: crate::util::primitives::StateID,
    },
    Union {
        alternates: alloc::boxed::Box<[crate::util::primitives::StateID]>,
    },
    BinaryUnion {
        alt1: crate::util::primitives::StateID,
        alt2: crate::util::primitives::StateID,
    },
    Capture {
        next: crate::util::primitives::StateID,
        pattern_id: crate::util::primitives::PatternID,
        group_index: crate::util::primitives::SmallIndex,
        slot: crate::util::primitives::SmallIndex,
    },
    Fail,
    Match {
        pattern_id: crate::util::primitives::PatternID,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1514-1621`](../../../../../.source_1765210505/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1514-L1621)*

A state in an NFA.

In theory, it can help to conceptualize an `NFA` as a graph consisting of
`State`s. Each `State` contains its complete set of outgoing transitions.

In practice, it can help to conceptualize an `NFA` as a sequence of
instructions for a virtual machine. Each `State` says what to do and where
to go next.

Strictly speaking, the practical interpretation is the most correct one,
because of the [`Capture`](State::Capture) state. Namely, a `Capture`
state always forwards execution to another state unconditionally. Its only
purpose is to cause a side effect: the recording of the current input
position at a particular location in memory. In this sense, an `NFA`
has more power than a theoretical non-deterministic finite automaton.

For most uses of this crate, it is likely that one may never even need to
be aware of this type at all. The main use cases for looking at `State`s
directly are if you need to write your own search implementation or if you
need to do some kind of analysis on the NFA.

#### Variants

- **`ByteRange`**

  A state with a single transition that can only be taken if the current
  input symbol is in a particular range of bytes.

- **`Sparse`**

  A state with possibly many transitions represented in a sparse fashion.
  Transitions are non-overlapping and ordered lexicographically by input
  range.
  
  In practice, this is used for encoding UTF-8 automata. Its presence is
  primarily an optimization that avoids many additional unconditional
  epsilon transitions (via [`Union`](State::Union) states), and thus
  decreases the overhead of traversing the NFA. This can improve both
  matching time and DFA construction time.

- **`Dense`**

  A dense representation of a state with multiple transitions.

- **`Look`**

  A conditional epsilon transition satisfied via some sort of
  look-around. Look-around is limited to anchor and word boundary
  assertions.
  
  Look-around states are meant to be evaluated while performing epsilon
  closure (computing the set of states reachable from a particular state
  via only epsilon transitions). If the current position in the haystack
  satisfies the look-around assertion, then you're permitted to follow
  that epsilon transition.

- **`Union`**

  An alternation such that there exists an epsilon transition to all
  states in `alternates`, where matches found via earlier transitions
  are preferred over later transitions.

- **`BinaryUnion`**

  An alternation such that there exists precisely two unconditional
  epsilon transitions, where matches found via `alt1` are preferred over
  matches found via `alt2`.
  
  This state exists as a common special case of Union where there are
  only two alternates. In this case, we don't need any allocations to
  represent the state. This saves a bit of memory and also saves an
  additional memory access when traversing the NFA.

- **`Capture`**

  An empty state that records a capture location.
  
  From the perspective of finite automata, this is precisely equivalent
  to an unconditional epsilon transition, but serves the purpose of
  instructing NFA simulations to record additional state when the finite
  state machine passes through this epsilon transition.
  
  `slot` in this context refers to the specific capture group slot
  offset that is being recorded. Each capturing group has two slots
  corresponding to the start and end of the matching portion of that
  group.
  
  The pattern ID and capture group index are also included in this state
  in case they are useful. But mostly, all you'll need is `next` and
  `slot`.

- **`Fail`**

  A state that cannot be transitioned out of. This is useful for cases
  where you want to prevent matching from occurring. For example, if your
  regex parser permits empty character classes, then one could choose
  a `Fail` state to represent them. (An empty character class can be
  thought of as an empty set. Since nothing is in an empty set, they can
  never match anything.)

- **`Match`**

  A match state. There is at least one such occurrence of this state for
  each regex that can match that is in this NFA.

#### Implementations

- <span id="state-is-epsilon"></span>`fn is_epsilon(&self) -> bool`

- <span id="state-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="state-remap"></span>`fn remap(&mut self, remap: &[StateID])` — [`StateID`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Debug for State`

- <span id="state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for State`

##### `impl PartialEq for State`

- <span id="state-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

