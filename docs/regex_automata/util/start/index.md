*[regex_automata](../../index.md) / [util](../index.md) / [start](index.md)*

---

# Module `start`

Provides helpers for dealing with start state configurations in DFAs.

## Structs

### `Config`

```rust
struct Config {
    look_behind: Option<u8>,
    anchored: crate::util::search::Anchored,
}
```

The configuration used to determine a DFA's start state for a search.

A DFA has a single starting state in the typical textbook description. That
is, it corresponds to the set of all starting states for the NFA that built
it, along with their epsilon closures. In this crate, however, DFAs have
many possible start states due to a few factors:

* DFAs support the ability to run either anchored or unanchored searches.
Each type of search needs its own start state. For example, an unanchored
search requires starting at a state corresponding to a regex with a
`(?s-u:.)*?` prefix, which will match through anything.
* DFAs also optionally support starting an anchored search for any one
specific pattern. Each such pattern requires its own start state.
* If a look-behind assertion like `^` or `\b` is used in the regex, then
the DFA will need to inspect a single byte immediately before the start of
the search to choose the correct start state.

Indeed, this configuration precisely encapsulates all of the above factors.
The `Config::anchored` method sets which kind of anchored search to
perform while the `Config::look_behind` method provides a way to set
the byte that occurs immediately before the start of the search.

Generally speaking, this type is only useful when you want to run searches
without using an [`Input`](../../index.md). In particular, an `Input` wants a haystack
slice, but callers may not have a contiguous sequence of bytes as a
haystack in all cases. This type provides a lower level of control such
that callers can provide their own anchored configuration and look-behind
byte explicitly.

# Example

This shows basic usage that permits running a search with a DFA without
using the `Input` abstraction.

```rust
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

let config = start::Config::new().anchored(Anchored::Yes);
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter() {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
assert!(dfa.is_match_state(state));

Ok::<(), Box<dyn std::error::Error>>(())
```

This example shows how to correctly run a search that doesn't begin at
the start of a haystack. Notice how we set the look-behind byte, and as
a result, the `\b` assertion does not match.

```rust
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

let config = start::Config::new()
    .anchored(Anchored::Yes)
    .look_behind(Some(b'q'));
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter().skip(1) {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
// No match!
assert!(!dfa.is_match_state(state));

Ok::<(), Box<dyn std::error::Error>>(())
```

If we had instead not set a look-behind byte, then the DFA would assume
that it was starting at the beginning of the haystack, and thus `\b` should
match. This in turn would result in erroneously reporting a match:

```rust
use regex_automata::{
    dfa::{Automaton, dense},
    util::start,
    Anchored,
};

let dfa = dense::DFA::new(r"(?-u)\b\w+\b")?;
let haystack = "quartz";

// Whoops, forgot the look-behind byte...
let config = start::Config::new().anchored(Anchored::Yes);
let mut state = dfa.start_state(&config)?;
for &b in haystack.as_bytes().iter().skip(1) {
    state = dfa.next_state(state, b);
}
state = dfa.next_eoi_state(state);
// And now we get a match unexpectedly.
assert!(dfa.is_match_state(state));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Config` — [`Config`](#config)

- `fn from_input_forward(input: &Input<'_>) -> Config` — [`Input`](../../index.md), [`Config`](#config)

- `fn from_input_reverse(input: &Input<'_>) -> Config` — [`Input`](../../index.md), [`Config`](#config)

- `fn look_behind(self: Self, byte: Option<u8>) -> Config` — [`Config`](#config)

- `fn anchored(self: Self, mode: Anchored) -> Config` — [`Anchored`](../../index.md), [`Config`](#config)

- `fn get_look_behind(self: &Self) -> Option<u8>`

- `fn get_anchored(self: &Self) -> Anchored` — [`Anchored`](../../index.md)

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StartByteMap`

```rust
struct StartByteMap {
    map: [Start; 256],
}
```

A map from every possible byte value to its corresponding starting
configuration.

This map is used in order to lookup the start configuration for a particular
position in a haystack. This start configuration is then used in
combination with things like the anchored mode and pattern ID to fully
determine the start state.

Generally speaking, this map is only used for fully compiled DFAs and lazy
DFAs. For NFAs (including the one-pass DFA), the start state is generally
selected by virtue of traversing the NFA state graph. DFAs do the same
thing, but at build time and not search time. (Well, technically the lazy
DFA does it at search time, but it does enough work to cache the full
result of the epsilon closure that the NFA engines tend to need to do.)

#### Implementations

- `fn new(lookm: &LookMatcher) -> StartByteMap` — [`LookMatcher`](../look/index.md), [`StartByteMap`](#startbytemap)

- `fn get(self: &Self, byte: u8) -> Start` — [`Start`](#start)

- `fn from_bytes(slice: &[u8]) -> Result<(StartByteMap, usize), DeserializeError>` — [`StartByteMap`](#startbytemap), [`DeserializeError`](../wire/index.md)

- `fn write_to(self: &Self, dst: &mut [u8]) -> Result<usize, SerializeError>` — [`SerializeError`](../wire/index.md)

- `fn write_to_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for StartByteMap`

- `fn clone(self: &Self) -> StartByteMap` — [`StartByteMap`](#startbytemap)

##### `impl Debug for StartByteMap`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Enums

### `Start`

```rust
enum Start {
    NonWordByte,
    WordByte,
    Text,
    LineLF,
    LineCR,
    CustomLineTerminator,
}
```

Represents the six possible starting configurations of a DFA search.

The starting configuration is determined by inspecting the beginning
of the haystack (up to 1 byte). Ultimately, this along with a pattern ID
(if specified) and the type of search (anchored or not) is what selects the
start state to use in a DFA.

As one example, if a DFA only supports unanchored searches and does not
support anchored searches for each pattern, then it will have at most 6
distinct start states. (Some start states may be reused if determinization
can determine that they will be equivalent.) If the DFA supports both
anchored and unanchored searches, then it will have a maximum of 12
distinct start states. Finally, if the DFA also supports anchored searches
for each pattern, then it can have up to `12 + (N * 6)` start states, where
`N` is the number of patterns.

Handling each of these starting configurations in the context of DFA
determinization can be *quite* tricky and subtle. But the code is small
and can be found at `crate::util::determinize::set_lookbehind_from_start`.

#### Variants

- **`NonWordByte`**

  This occurs when the starting position is not any of the ones below.

- **`WordByte`**

  This occurs when the byte immediately preceding the start of the search
  is an ASCII word byte.

- **`Text`**

  This occurs when the starting position of the search corresponds to the
  beginning of the haystack.

- **`LineLF`**

  This occurs when the byte immediately preceding the start of the search
  is a line terminator. Specifically, `\n`.

- **`LineCR`**

  This occurs when the byte immediately preceding the start of the search
  is a line terminator. Specifically, `\r`.

- **`CustomLineTerminator`**

  This occurs when a custom line terminator has been set via a
  `LookMatcher`, and when that line terminator is neither a `\r` or a
  `\n`.
  
  If the custom line terminator is a word byte, then this start
  configuration is still selected. DFAs that implement word boundary
  assertions will likely need to check whether the custom line terminator
  is a word byte, in which case, it should behave as if the byte
  satisfies `\b` in addition to multi-line anchors.

#### Implementations

- `fn from_usize(n: usize) -> Option<Start>` — [`Start`](#start)

- `fn len() -> usize`

- `fn as_u8(self: &Self) -> u8`

- `fn as_usize(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for Start`

- `fn clone(self: &Self) -> Start` — [`Start`](#start)

##### `impl Copy for Start`

##### `impl Debug for Start`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Start`

##### `impl PartialEq for Start`

- `fn eq(self: &Self, other: &Start) -> bool` — [`Start`](#start)

##### `impl StructuralPartialEq for Start`

