*[regex_automata](../../index.md) / [util](../index.md) / [start](index.md)*

---

# Module `start`

Provides helpers for dealing with start state configurations in DFAs.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration used to determine a DFA's start state for a search. |
| [`StartByteMap`](#startbytemap) | struct | A map from every possible byte value to its corresponding starting configuration. |
| [`Start`](#start) | enum | Represents the six possible starting configurations of a DFA search. |

## Structs

### `Config`

```rust
struct Config {
    look_behind: Option<u8>,
    anchored: crate::util::search::Anchored,
}
```

*Defined in [`regex-automata-0.4.13/src/util/start.rs:121-124`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/start.rs#L121-L124)*

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

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

  Create a new default start configuration.

  

  The default is an unanchored search that starts at the beginning of the

  haystack.

- <span id="config-from-input-forward"></span>`fn from_input_forward(input: &Input<'_>) -> Config` — [`Input`](../../index.md#input), [`Config`](#config)

  A convenience routine for building a start configuration from an

  [`Input`](../../index.md) for a forward search.

  

  This automatically sets the look-behind byte to the byte immediately

  preceding the start of the search. If the start of the search is at

  offset `0`, then no look-behind byte is set.

- <span id="config-from-input-reverse"></span>`fn from_input_reverse(input: &Input<'_>) -> Config` — [`Input`](../../index.md#input), [`Config`](#config)

  A convenience routine for building a start configuration from an

  [`Input`](../../index.md) for a reverse search.

  

  This automatically sets the look-behind byte to the byte immediately

  following the end of the search. If the end of the search is at

  offset `haystack.len()`, then no look-behind byte is set.

- <span id="config-look-behind"></span>`fn look_behind(self, byte: Option<u8>) -> Config` — [`Config`](#config)

  Set the look-behind byte at the start of a search.

  

  Unless the search is intended to logically start at the beginning of a

  haystack, this should _always_ be set to the byte immediately preceding

  the start of the search. If no look-behind byte is set, then the start

  configuration will assume it is at the beginning of the haystack. For

  example, the anchor `^` will match.

  

  The default is that no look-behind byte is set.

- <span id="config-anchored"></span>`fn anchored(self, mode: Anchored) -> Config` — [`Anchored`](../../index.md#anchored), [`Config`](#config)

  Set the anchored mode of a search.

  

  The default is an unanchored search.

- <span id="config-get-look-behind"></span>`fn get_look_behind(&self) -> Option<u8>`

  Return the look-behind byte in this configuration, if one exists.

- <span id="config-get-anchored"></span>`fn get_anchored(&self) -> Anchored` — [`Anchored`](../../index.md#anchored)

  Return the anchored mode in this configuration.

#### Trait Implementations

##### `impl Any for Config`

- <span id="config-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Config`

- <span id="config-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Config`

- <span id="config-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl CloneToUninit for Config`

- <span id="config-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Config`

- <span id="config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Config`

- <span id="config-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Config`

- <span id="config-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Config`

- <span id="config-toowned-type-owned"></span>`type Owned = T`

- <span id="config-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="config-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Config`

- <span id="config-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="config-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Config`

- <span id="config-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="config-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StartByteMap`

```rust
struct StartByteMap {
    map: [Start; 256],
}
```

*Defined in [`regex-automata-0.4.13/src/util/start.rs:208-210`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/start.rs#L208-L210)*

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

- <span id="startbytemap-new"></span>`fn new(lookm: &LookMatcher) -> StartByteMap` — [`LookMatcher`](../look/index.md#lookmatcher), [`StartByteMap`](#startbytemap)

  Create a new map from byte values to their corresponding starting

  configurations. The map is determined, in part, by how look-around

  assertions are matched via the matcher given.

- <span id="startbytemap-get"></span>`fn get(&self, byte: u8) -> Start` — [`Start`](#start)

  Return the starting configuration for the given look-behind byte.

  

  If no look-behind exists, callers should use `Start::Text`.

- <span id="startbytemap-from-bytes"></span>`fn from_bytes(slice: &[u8]) -> Result<(StartByteMap, usize), DeserializeError>` — [`StartByteMap`](#startbytemap), [`DeserializeError`](../wire/index.md#deserializeerror)

  Deserializes a byte class map from the given slice. If the slice is of

  insufficient length or otherwise contains an impossible mapping, then

  an error is returned. Upon success, the number of bytes read along with

  the map are returned. The number of bytes read is always a multiple of

  8.

- <span id="startbytemap-write-to"></span>`fn write_to(&self, dst: &mut [u8]) -> Result<usize, SerializeError>` — [`SerializeError`](../wire/index.md#serializeerror)

  Writes this map to the given byte buffer. if the given buffer is too

  small, then an error is returned. Upon success, the total number of

  bytes written is returned. The number of bytes written is guaranteed to

  be a multiple of 8.

- <span id="startbytemap-write-to-len"></span>`fn write_to_len(&self) -> usize`

  Returns the total number of bytes written by `write_to`.

#### Trait Implementations

##### `impl Any for StartByteMap`

- <span id="startbytemap-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartByteMap`

- <span id="startbytemap-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartByteMap`

- <span id="startbytemap-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartByteMap`

- <span id="startbytemap-clone"></span>`fn clone(&self) -> StartByteMap` — [`StartByteMap`](#startbytemap)

##### `impl CloneToUninit for StartByteMap`

- <span id="startbytemap-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StartByteMap`

- <span id="startbytemap-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for StartByteMap`

- <span id="startbytemap-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartByteMap`

- <span id="startbytemap-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StartByteMap`

- <span id="startbytemap-toowned-type-owned"></span>`type Owned = T`

- <span id="startbytemap-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="startbytemap-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StartByteMap`

- <span id="startbytemap-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="startbytemap-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartByteMap`

- <span id="startbytemap-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="startbytemap-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/util/start.rs:344-369`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/start.rs#L344-L369)*

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

- <span id="start-from-usize"></span>`fn from_usize(n: usize) -> Option<Start>` — [`Start`](#start)

  Return the starting state corresponding to the given integer. If no

  starting state exists for the given integer, then None is returned.

- <span id="start-len"></span>`fn len() -> usize`

  Returns the total number of starting state configurations.

- <span id="start-as-u8"></span>`fn as_u8(&self) -> u8`

  Return this starting configuration as `u8` integer. It is guaranteed to

  be less than `Start::len()`.

- <span id="start-as-usize"></span>`fn as_usize(&self) -> usize`

  Return this starting configuration as a `usize` integer. It is

  guaranteed to be less than `Start::len()`.

#### Trait Implementations

##### `impl Any for Start`

- <span id="start-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Start`

- <span id="start-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Start`

- <span id="start-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Start`

- <span id="start-clone"></span>`fn clone(&self) -> Start` — [`Start`](#start)

##### `impl CloneToUninit for Start`

- <span id="start-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Start`

##### `impl Debug for Start`

- <span id="start-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Start`

##### `impl<T> From for Start`

- <span id="start-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Start`

- <span id="start-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Start`

- <span id="start-partialeq-eq"></span>`fn eq(&self, other: &Start) -> bool` — [`Start`](#start)

##### `impl StructuralPartialEq for Start`

##### `impl ToOwned for Start`

- <span id="start-toowned-type-owned"></span>`type Owned = T`

- <span id="start-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="start-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Start`

- <span id="start-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="start-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Start`

- <span id="start-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="start-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

