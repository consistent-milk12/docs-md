*[aho_corasick](../index.md) / [automaton](index.md)*

---

# Module `automaton`

Provides [`Automaton`](aho_corasick/automaton/index.md) trait for abstracting over Aho-Corasick automata.

The `Automaton` trait provides a way to write generic code over any
Aho-Corasick automaton. It also provides access to lower level APIs that
permit walking the state transitions of an Aho-Corasick automaton manually.

## Structs

### `Prefilter`

```rust
struct Prefilter {
}
```

A prefilter for accelerating a search.

This crate uses prefilters in the core search implementations to accelerate
common cases. They typically only apply to cases where there are a small
number of patterns (less than 100 or so), but when they do, thoughput can
be boosted considerably, perhaps by an order of magnitude. When a prefilter
is active, it is used whenever a search enters an automaton's start state.

Currently, prefilters cannot be constructed by
callers. A `Prefilter` can only be accessed via the
[`Automaton::prefilter`](crate::automaton::Automaton::prefilter)
method and used to execute a search. In other words, a prefilter can be
used to optimize your own search implementation if necessary, but cannot do
much else. If you have a use case for more APIs, please submit an issue.

#### Implementations

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate`
  Execute a search in the haystack within the span given. If a match or

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Prefilter`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StateID`

```rust
struct StateID();
```

The identifier of a finite automaton state.

It is represented by a `u32` even on 64-bit systems in order to conserve
space. Namely, on all targets, this type guarantees that its value will
fit in a `u32`, `i32`, `usize` and an `isize`. This means that on 16-bit
targets, for example, this type's maximum value will never overflow an
`isize`, which means it will never overflow a `i16` even though its
internal representation is still a `u32`.

# Safety

While a `StateID` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `StateID` with an
invalid value can be done in entirely safe code. This may in turn result in
panics or silent logical errors.

#### Implementations

- `const MAX: StateID`

- `const LIMIT: usize`

- `const ZERO: StateID`

- `const SIZE: usize`

- `fn new(value: usize) -> Result<StateID, StateIDError>`
  Create a new value that is represented by a "small index."

- `const fn new_unchecked(value: usize) -> StateID`
  Create a new value without checking whether the given argument

- `const fn from_u32_unchecked(index: u32) -> StateID`
  Create a new value from a `u32` without checking whether the

- `fn must(value: usize) -> StateID`
  Like `new`, but panics if the given value is not valid.

- `const fn as_usize(self: &Self) -> usize`
  Return the internal value as a `usize`. This is guaranteed to

- `const fn as_u64(self: &Self) -> u64`
  Return the internal value as a `u64`. This is guaranteed to

- `const fn as_u32(self: &Self) -> u32`
  Return the internal value as a `u32`. This is guaranteed to

- `const fn as_i32(self: &Self) -> i32`
  Return the internal value as a `i32`. This is guaranteed to

- `fn one_more(self: &Self) -> usize`
  Returns one more than this value as a usize.

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>`
  Decode this value from the bytes given using the native endian

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID`
  Decode this value from the bytes given using the native endian

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`
  Return the underlying integer as raw bytes in native endian

#### Trait Implementations

##### `impl From`

- `fn from(value: u8) -> StateID`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> StateID`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &StateID) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StateID) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &StateID) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: u32) -> Result<StateID, StateIDError>`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: u16) -> Result<StateID, StateIDError>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: u64) -> Result<StateID, StateIDError>`

##### `impl TryFrom`

- `type Error = StateIDError`

- `fn try_from(value: usize) -> Result<StateID, StateIDError>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> StateID`

### `StateIDError`

```rust
struct StateIDError();
```

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`
  Returns the value that could not be converted to an ID.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> StateIDError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StateIDError) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OverlappingState`

```rust
struct OverlappingState {
}
```

Represents the current state of an overlapping search.

This is used for overlapping searches since they need to know something
about the previous search. For example, when multiple patterns match at the
same position, this state tracks the last reported pattern so that the next
search knows whether to report another matching pattern or continue with
the search at the next position. Additionally, it also tracks which state
the last search call terminated in and the current offset of the search
in the haystack.

This type provides limited introspection capabilities. The only thing a
caller can do is construct it and pass it around to permit search routines
to use it to track state, and to ask whether a match has been found.

Callers should always provide a fresh state constructed via
[`OverlappingState::start`](#start) when starting a new search. That same state
should be reused for subsequent searches on the same `Input`. The state
given will advance through the haystack itself. Callers can detect the end
of a search when neither an error nor a match is returned.

# Example

This example shows how to manually iterate over all overlapping matches. If
you need this, you might consider using
[`AhoCorasick::find_overlapping_iter`](crate::AhoCorasick::find_overlapping_iter)
instead, but this shows how to correctly use an `OverlappingState`.

```
use aho_corasick::{
    automaton::OverlappingState,
    AhoCorasick, Input, Match,
};

let patterns = &["append", "appendage", "app"];
let haystack = "append the app to the appendage";

let ac = AhoCorasick::new(patterns).unwrap();
let mut state = OverlappingState::start();
let mut matches = vec![];

loop {
    ac.find_overlapping(haystack, &mut state);
    let mat = match state.get_match() {
        None => break,
        Some(mat) => mat,
    };
    matches.push(mat);
}
let expected = vec![
    Match::must(2, 0..3),
    Match::must(0, 0..6),
    Match::must(2, 11..14),
    Match::must(2, 22..25),
    Match::must(0, 22..28),
    Match::must(1, 22..31),
];
assert_eq!(expected, matches);
```

#### Implementations

- `fn start() -> OverlappingState`
  Create a new overlapping state that begins at the start state.

- `fn get_match(self: &Self) -> Option<Match>`
  Return the match result of the most recent search to execute with this

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> OverlappingState`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FindIter<'a, 'h, A>`

```rust
struct FindIter<'a, 'h, A> {
}
```

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`](#matchkind) used by this
automaton.

This iterator is constructed via the [`Automaton::try_find_iter`](#try-find-iter) method.

The type variable `A` refers to the implementation of the [`Automaton`](aho_corasick/automaton/index.md)
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`](aho_corasick/automaton/index.md)
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'a, 'h, A: Automaton>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, 'h, A: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FindOverlappingIter<'a, 'h, A>`

```rust
struct FindOverlappingIter<'a, 'h, A> {
}
```

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the
[`Automaton::try_find_overlapping_iter`](#try-find-overlapping-iter) method.

The type variable `A` refers to the implementation of the [`Automaton`](aho_corasick/automaton/index.md)
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`](aho_corasick/automaton/index.md)
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'a, 'h, A: Automaton>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, 'h, A: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StreamFindIter<'a, A, R>`

```rust
struct StreamFindIter<'a, A, R> {
}
```

An iterator that reports matches in a stream.

This iterator yields elements of type `io::Result<Match>`, where an error
is reported if there was a problem reading from the underlying stream.
The iterator terminates only when the underlying stream reaches `EOF`.

This iterator is constructed via the [`Automaton::try_stream_find_iter`](#try-stream-find-iter)
method.

The type variable `A` refers to the implementation of the [`Automaton`](aho_corasick/automaton/index.md)
trait used to execute the search.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the [`Automaton`](aho_corasick/automaton/index.md)
implementation.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'a, A: Automaton, R: std::io::Read>`

- `type Item = Result<Match, Error>`

- `fn next(self: &mut Self) -> Option<std::io::Result<Match>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, A: $crate::fmt::Debug, R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `Candidate`

```rust
enum Candidate {
    None,
    Match(crate::util::search::Match),
    PossibleStartOfMatch(usize),
}
```

A candidate is the result of running a prefilter on a haystack at a
particular position.

The result is either no match, a confirmed match or a possible match.

When no match is returned, the prefilter is guaranteeing that no possible
match can be found in the haystack, and the caller may trust this. That is,
all correct prefilters must never report false negatives.

In some cases, a prefilter can confirm a match very quickly, in which case,
the caller may use this to stop what it's doing and report the match. In
this case, prefilter implementations must never report a false positive.
In other cases, the prefilter can only report a potential match, in which
case the callers must attempt to confirm the match. In this case, prefilter
implementations are permitted to return false positives.

#### Variants

- **`None`**

  No match was found. Since false negatives are not possible, this means
  the search can quit as it is guaranteed not to find another match.

- **`Match`**

  A confirmed match was found. Callers do not need to confirm it.

- **`PossibleStartOfMatch`**

  The start of a possible match was found. Callers must confirm it before
  reporting it as a match.

#### Implementations

- `fn into_option(self: Self) -> Option<usize>`
  Convert this candidate into an option. This is useful when callers

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Candidate`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Automaton`

```rust
trait Automaton: private::Sealed { ... }
```

A trait that abstracts over Aho-Corasick automata.

This trait primarily exists for niche use cases such as:

* Using an NFA or DFA directly, bypassing the top-level
[`AhoCorasick`](crate::AhoCorasick) searcher. Currently, these include
[`noncontiguous::NFA`](crate::nfa::noncontiguous::NFA),
[`contiguous::NFA`](crate::nfa::contiguous::NFA) and
[`dfa::DFA`](crate::dfa::DFA).
* Implementing your own custom search routine by walking the automaton
yourself. This might be useful for implementing search on non-contiguous
strings or streams.

For most use cases, it is not expected that users will need
to use or even know about this trait. Indeed, the top level
[`AhoCorasick`](crate::AhoCorasick) searcher does not expose any details
about this trait, nor does it implement it itself.

Note that this trait defines a number of default methods, such as
[`Automaton::try_find`](#try-find) and [`Automaton::try_find_iter`](#try-find-iter), which implement
higher level search routines in terms of the lower level automata API.

# Sealed

Currently, this trait is sealed. That means users of this crate can write
generic routines over this trait but cannot implement it themselves. This
restriction may be lifted in the future, but sealing the trait permits
adding new required methods in a backwards compatible fashion.

# Special states

This trait encodes a notion of "special" states in an automaton. Namely,
a state is treated as special if it is a dead, match or start state:

* A dead state is a state that cannot be left once entered. All transitions
on a dead state lead back to itself. The dead state is meant to be treated
as a sentinel indicating that the search should stop and return a match if
one has been found, and nothing otherwise.
* A match state is a state that indicates one or more patterns have
matched. Depending on the [`MatchKind`](#matchkind) of the automaton, a search may
stop once a match is seen, or it may continue looking for matches until
it enters a dead state or sees the end of the haystack.
* A start state is a state that a search begins in. It is useful to know
when a search enters a start state because it may mean that a prefilter can
be used to skip ahead and quickly look for candidate matches. Unlike dead
and match states, it is never necessary to explicitly handle start states
for correctness. Indeed, in this crate, implementations of `Automaton`
will only treat start states as "special" when a prefilter is enabled and
active. Otherwise, treating it as special has no purpose and winds up
slowing down the overall search because it results in ping-ponging between
the main state transition and the "special" state logic.

Since checking whether a state is special by doing three different
checks would be too expensive inside a fast search loop, the
[`Automaton::is_special`](#is-special) method is provided for quickly checking whether
the state is special. The `Automaton::is_dead`, `Automaton::is_match` and
`Automaton::is_start` predicates can then be used to determine which kind
of special state it is.

# Panics

Most of the APIs on this trait should panic or give incorrect results
if invalid inputs are given to it. For example, `Automaton::next_state`
has unspecified behavior if the state ID given to it is not a valid
state ID for the underlying automaton. Valid state IDs can only be
retrieved in one of two ways: calling `Automaton::start_state` or calling
`Automaton::next_state` with a valid state ID.

# Safety

This trait is not safe to implement so that code may rely on the
correctness of implementations of this trait to avoid undefined behavior.
The primary correctness guarantees are:

* `Automaton::start_state` always returns a valid state ID or an error or
panics.
* `Automaton::next_state`, when given a valid state ID, always returns
a valid state ID for all values of `anchored` and `byte`, or otherwise
panics.

In general, the rest of the methods on `Automaton` need to uphold their
contracts as well. For example, `Automaton::is_dead` should only returns
true if the given state ID is actually a dead state.

Note that currently this crate does not rely on the safety property defined
here to avoid undefined behavior. Instead, this was done to make it
_possible_ to do in the future.

# Example

This example shows how one might implement a basic but correct search
routine. We keep things simple by not using prefilters or worrying about
anchored searches, but do make sure our search is correct for all possible
[`MatchKind`](#matchkind) semantics. (The comments in the code below note the parts
that are needed to support certain `MatchKind` semantics.)

```
use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
    Anchored, Match, MatchError, MatchKind,
};

// Run an unanchored search for 'aut' in 'haystack'. Return the first match
// seen according to the automaton's match semantics. This returns an error
// if the given automaton does not support unanchored searches.
fn find<A: Automaton>(
    aut: A,
    haystack: &[u8](#u8)
,
) -> Result<Option<Match>, MatchError> {
    let mut sid = aut.start_state(Anchored::No)?;
    let mut at = 0;
    let mut mat = None;
    let get_match = |sid, at| {
        let pid = aut.match_pattern(sid, 0);
        let len = aut.pattern_len(pid);
        Match::new(pid, (at - len)..at)
    };
    // Start states can be match states!
    if aut.is_match(sid) {
        mat = Some(get_match(sid, at));
        // Standard semantics require matches to be reported as soon as
        // they're seen. Otherwise, we continue until we see a dead state
        // or the end of the haystack.
        if matches!(aut.match_kind(), MatchKind::Standard) {
            return Ok(mat);
        }
    }
    while at < haystack.len() {
        sid = aut.next_state(Anchored::No, sid, haystack[at](#at)
);
        if aut.is_special(sid) {
            if aut.is_dead(sid) {
                return Ok(mat);
            } else if aut.is_match(sid) {
                mat = Some(get_match(sid, at + 1));
                // As above, standard semantics require that we return
                // immediately once a match is found.
                if matches!(aut.match_kind(), MatchKind::Standard) {
                    return Ok(mat);
                }
            }
        }
        at += 1;
    }
    Ok(mat)
}

// Show that it works for standard searches.
let nfa = NFA::new(&["samwise", "sam"]).unwrap();
assert_eq!(Some(Match::must(1, 0..3)), find(&nfa, b"samwise")?);

// But also works when using leftmost-first. Notice how the match result
// has changed!
let nfa = NFA::builder()
    .match_kind(MatchKind::LeftmostFirst)
    .build(&["samwise", "sam"])
    .unwrap();
assert_eq!(Some(Match::must(0, 0..7)), find(&nfa, b"samwise")?);

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Required Methods

- `fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError>`

  Returns the starting state for the given anchor mode.

- `fn next_state(self: &Self, anchored: Anchored, sid: StateID, byte: u8) -> StateID`

  Performs a state transition from `sid` for `byte` and returns the next

- `fn is_special(self: &Self, sid: StateID) -> bool`

  Returns true if the given ID represents a "special" state. A special

- `fn is_dead(self: &Self, sid: StateID) -> bool`

  Returns true if the given ID represents a dead state.

- `fn is_match(self: &Self, sid: StateID) -> bool`

  Returns true if the given ID represents a match state.

- `fn is_start(self: &Self, sid: StateID) -> bool`

  Returns true if the given ID represents a start state.

- `fn match_kind(self: &Self) -> MatchKind`

  Returns the match semantics that this automaton was built with.

- `fn match_len(self: &Self, sid: StateID) -> usize`

  Returns the total number of matches for the given state ID.

- `fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID`

  Returns the pattern ID for the match state given by `sid` at the

- `fn patterns_len(self: &Self) -> usize`

  Returns the total number of patterns compiled into this automaton.

- `fn pattern_len(self: &Self, pid: PatternID) -> usize`

  Returns the length of the pattern for the given ID.

- `fn min_pattern_len(self: &Self) -> usize`

  Returns the length, in bytes, of the shortest pattern in this

- `fn max_pattern_len(self: &Self) -> usize`

  Returns the length, in bytes, of the longest pattern in this automaton.

- `fn memory_usage(self: &Self) -> usize`

  Returns the heap memory usage, in bytes, used by this automaton.

- `fn prefilter(self: &Self) -> Option<&Prefilter>`

  Returns a prefilter, if available, that can be used to accelerate

- `fn try_find(self: &Self, input: &Input<'_>) -> Result<Option<Match>, MatchError>`

  Executes a non-overlapping search with this automaton using the given

- `fn try_find_overlapping(self: &Self, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>`

  Executes a overlapping search with this automaton using the given

- `fn try_find_iter<'a, 'h>(self: &'a Self, input: Input<'h>) -> Result<FindIter<'a, 'h, Self>, MatchError>`

  Returns an iterator of non-overlapping matches with this automaton

- `fn try_find_overlapping_iter<'a, 'h>(self: &'a Self, input: Input<'h>) -> Result<FindOverlappingIter<'a, 'h, Self>, MatchError>`

  Returns an iterator of overlapping matches with this automaton

- `fn try_replace_all<B>(self: &Self, haystack: &str, replace_with: &[B]) -> Result<String, MatchError>`

  Replaces all non-overlapping matches in `haystack` with

- `fn try_replace_all_bytes<B>(self: &Self, haystack: &[u8], replace_with: &[B]) -> Result<Vec<u8>, MatchError>`

  Replaces all non-overlapping matches in `haystack` with

- `fn try_replace_all_with<F>(self: &Self, haystack: &str, dst: &mut String, replace_with: F) -> Result<(), MatchError>`

  Replaces all non-overlapping matches in `haystack` by calling the

- `fn try_replace_all_with_bytes<F>(self: &Self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F) -> Result<(), MatchError>`

  Replaces all non-overlapping matches in `haystack` by calling the

- `fn try_stream_find_iter<'a, R: std::io::Read>(self: &'a Self, rdr: R) -> Result<StreamFindIter<'a, Self, R>, MatchError>`

  Returns an iterator of non-overlapping matches with this automaton

- `fn try_stream_replace_all<R, W, B>(self: &Self, rdr: R, wtr: W, replace_with: &[B]) -> std::io::Result<()>`

  Replaces all non-overlapping matches in `rdr` with strings from

- `fn try_stream_replace_all_with<R, W, F>(self: &Self, rdr: R, wtr: W, replace_with: F) -> std::io::Result<()>`

  Replaces all non-overlapping matches in `rdr` by calling the

