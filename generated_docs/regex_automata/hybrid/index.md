*[regex_automata](../index.md) / [hybrid](index.md)*

---

# Module `hybrid`

A module for building and searching with lazy deterministic finite automata
(DFAs).

Like other modules in this crate, lazy DFAs support a rich regex syntax with
Unicode features. The key feature of a lazy DFA is that it builds itself
incrementally during search, and never uses more than a configured capacity of
memory. Thus, when searching with a lazy DFA, one must supply a mutable "cache"
in which the actual DFA's transition table is stored.

If you're looking for fully compiled DFAs, then please see the top-level
[`dfa` module](crate::dfa).

# Overview

This section gives a brief overview of the primary types in this module:

* A [`regex::Regex`](regex/index.md) provides a way to search for matches of a regular
expression using lazy DFAs. This includes iterating over matches with both the
start and end positions of each match.
* A [`dfa::DFA`](dfa/index.md) provides direct low level access to a lazy DFA.

# Example: basic regex searching

This example shows how to compile a regex using the default configuration
and then use it to find matches in a byte string:

```rust
use regex_automata::{hybrid::regex::Regex, Match};

let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")?;
let mut cache = re.create_cache();

let haystack = "2018-12-24 2016-10-08";
let matches: Vec<Match> = re.find_iter(&mut cache, haystack).collect();
assert_eq!(matches, vec![
    Match::must(0, 0..10),
    Match::must(0, 11..21),
]);
Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: searching with multiple regexes

The lazy DFAs in this module all fully support searching with multiple regexes
simultaneously. You can use this support with standard leftmost-first style
searching to find non-overlapping matches:

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{hybrid::regex::Regex, Match};

let re = Regex::new_many(&[r"\w+", r"\S+"])?;
let mut cache = re.create_cache();

let haystack = "@foo bar";
let matches: Vec<Match> = re.find_iter(&mut cache, haystack).collect();
assert_eq!(matches, vec![
    Match::must(1, 0..4),
    Match::must(0, 5..8),
]);
Ok::<(), Box<dyn std::error::Error>>(())
```

# When should I use this?

Generally speaking, if you can abide the use of mutable state during search,
and you don't need things like capturing groups or Unicode word boundary
support in non-ASCII text, then a lazy DFA is likely a robust choice with
respect to both search speed and memory usage. Note however that its speed
may be worse than a general purpose regex engine if you don't select a good
[prefilter](crate::util::prefilter).

If you know ahead of time that your pattern would result in a very large DFA
if it was fully compiled, it may be better to use an NFA simulation instead
of a lazy DFA. Either that, or increase the cache capacity of your lazy DFA
to something that is big enough to hold the state machine (likely through
experimentation). The issue here is that if the cache is too small, then it
could wind up being reset too frequently and this might decrease searching
speed significantly.

# Differences with fully compiled DFAs

A [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) and a
[`dfa::regex::Regex`](crate::dfa::regex::Regex) both have the same capabilities
(and similarly for their underlying DFAs), but they achieve them through
different means. The main difference is that a hybrid or "lazy" regex builds
its DFA lazily during search, where as a fully compiled regex will build its
DFA at construction time. While building a DFA at search time might sound like
it's slow, it tends to work out where most bytes seen during a search will
reuse pre-built parts of the DFA and thus can be almost as fast as a fully
compiled DFA. The main downside is that searching requires mutable space to
store the DFA, and, in the worst case, a search can result in a new state being
created for each byte seen, which would make searching quite a bit slower.

A fully compiled DFA never has to worry about searches being slower once
it's built. (Aside from, say, the transition table being so large that it
is subject to harsh CPU cache effects.) However, of course, building a full
DFA can be quite time consuming and memory hungry. Particularly when large
Unicode character classes are used, which tend to translate into very large
DFAs.

A lazy DFA strikes a nice balance _in practice_, particularly in the
presence of Unicode mode, by only building what is needed. It avoids the
worst case exponential time complexity of DFA compilation by guaranteeing that
it will only build at most one state per byte searched. While the worst
case here can lead to a very high constant, it will never be exponential.

# Syntax

This module supports the same syntax as the `regex` crate, since they share the
same parser. You can find an exhaustive list of supported syntax in the
[documentation for the `regex` crate](https://docs.rs/regex/1/regex/#syntax).

There are two things that are not supported by the lazy DFAs in this module:

* Capturing groups. The DFAs (and [`Regex`](regex::Regex)es built on top
of them) can only find the offsets of an entire match, but cannot resolve
the offsets of each capturing group. This is because DFAs do not have the
expressive power necessary. Note that it is okay to build a lazy DFA from an
NFA that contains capture groups. The capture groups will simply be ignored.
* Unicode word boundaries. These present particularly difficult challenges for
DFA construction and would result in an explosion in the number of states.
One can enable `dfa::Config::unicode_word_boundary` though, which provides
heuristic support for Unicode word boundaries that only works on ASCII text.
Otherwise, one can use `(?-u:\b)` for an ASCII word boundary, which will work
on any input.

There are no plans to lift either of these limitations.

Note that these restrictions are identical to the restrictions on fully
compiled DFAs.

## Contents

- [Modules](#modules)
  - [`dfa`](#dfa)
  - [`error`](#error)
  - [`id`](#id)
  - [`regex`](#regex)
  - [`search`](#search)
- [Structs](#structs)
  - [`BuildError`](#builderror)
  - [`CacheError`](#cacheerror)
  - [`LazyStateID`](#lazystateid)
- [Enums](#enums)
  - [`StartError`](#starterror)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dfa`](#dfa) | mod | Types and routines specific to lazy DFAs. |
| [`error`](#error) | mod |  |
| [`id`](#id) | mod |  |
| [`regex`](#regex) | mod | A lazy DFA backed `Regex`. |
| [`search`](#search) | mod |  |
| [`BuildError`](#builderror) | struct |  |
| [`CacheError`](#cacheerror) | struct |  |
| [`LazyStateID`](#lazystateid) | struct |  |
| [`StartError`](#starterror) | enum |  |

## Modules

- [`dfa`](dfa/index.md) — Types and routines specific to lazy DFAs.
- [`error`](error/index.md)
- [`id`](id/index.md)
- [`regex`](regex/index.md) — A lazy DFA backed `Regex`.
- [`search`](search/index.md)

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/error.rs:23-25`](../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/error.rs#L23-L25)*

An error that occurs when initial construction of a lazy DFA fails.

A build error can occur when insufficient cache capacity is configured or
if something about the NFA is unsupported. (For example, if one attempts
to build a lazy DFA without heuristic Unicode support but with an NFA that
contains a Unicode word boundary.)

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying
[`nfa::thompson::BuildError`](crate::nfa::thompson::BuildError)
type from its `source` method via the `std::error::Error` trait. This error
only occurs when using convenience routines for building a lazy DFA
directly from a pattern string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="builderror-nfa"></span>`fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../nfa/thompson/error/index.md#builderror)

- <span id="builderror-insufficient-cache-capacity"></span>`fn insufficient_cache_capacity(minimum: usize, given: usize) -> BuildError` — [`BuildError`](error/index.md#builderror)

- <span id="builderror-insufficient-state-id-capacity"></span>`fn insufficient_state_id_capacity(err: LazyStateIDError) -> BuildError` — [`LazyStateIDError`](id/index.md#lazystateiderror), [`BuildError`](error/index.md#builderror)

- <span id="builderror-unsupported-dfa-word-boundary-unicode"></span>`fn unsupported_dfa_word_boundary_unicode() -> BuildError` — [`BuildError`](error/index.md#builderror)

#### Trait Implementations

##### `impl Any for BuildError`

- <span id="builderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildError`

- <span id="builderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildError`

- <span id="builderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](error/index.md#builderror)

##### `impl CloneToUninit for BuildError`

- <span id="builderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildError`

- <span id="builderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- <span id="builderror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> From for BuildError`

- <span id="builderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildError`

- <span id="builderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BuildError`

- <span id="builderror-toowned-type-owned"></span>`type Owned = T`

- <span id="builderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for BuildError`

- <span id="builderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BuildError`

- <span id="builderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildError`

- <span id="builderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CacheError`

```rust
struct CacheError(());
```

*Defined in [`regex-automata-0.4.13/src/hybrid/error.rs:222`](../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/error.rs#L222)*

An error that occurs when cache usage has become inefficient.

One of the weaknesses of a lazy DFA is that it may need to clear its
cache repeatedly if it's not big enough. If this happens too much, then it
can slow searching down significantly. A mitigation to this is to use
heuristics to detect whether the cache is being used efficiently or not.
If not, then a lazy DFA can return a `CacheError`.

The default configuration of a lazy DFA in this crate is
set such that a `CacheError` will never occur. Instead,
callers must opt into this behavior with settings like
[`dfa::Config::minimum_cache_clear_count`](crate::hybrid::dfa::Config::minimum_cache_clear_count)
and
[`dfa::Config::minimum_bytes_per_state`](crate::hybrid::dfa::Config::minimum_bytes_per_state).

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="cacheerror-too-many-cache-clears"></span>`fn too_many_cache_clears() -> CacheError` — [`CacheError`](error/index.md#cacheerror)

- <span id="cacheerror-bad-efficiency"></span>`fn bad_efficiency() -> CacheError` — [`CacheError`](error/index.md#cacheerror)

#### Trait Implementations

##### `impl Any for CacheError`

- <span id="cacheerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CacheError`

- <span id="cacheerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CacheError`

- <span id="cacheerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CacheError`

- <span id="cacheerror-clone"></span>`fn clone(&self) -> CacheError` — [`CacheError`](error/index.md#cacheerror)

##### `impl CloneToUninit for CacheError`

- <span id="cacheerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CacheError`

- <span id="cacheerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CacheError`

- <span id="cacheerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CacheError`

##### `impl<T> From for CacheError`

- <span id="cacheerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CacheError`

- <span id="cacheerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CacheError`

- <span id="cacheerror-toowned-type-owned"></span>`type Owned = T`

- <span id="cacheerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cacheerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for CacheError`

- <span id="cacheerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for CacheError`

- <span id="cacheerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cacheerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CacheError`

- <span id="cacheerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cacheerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LazyStateID`

```rust
struct LazyStateID(u32);
```

*Defined in [`regex-automata-0.4.13/src/hybrid/id.rs:169`](../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/id.rs#L169)*

A state identifier specifically tailored for lazy DFAs.

A lazy state ID logically represents a pointer to a DFA state. In practice,
by limiting the number of DFA states it can address, it reserves some
bits of its representation to encode some additional information. That
additional information is called a "tag." That tag is used to record
whether the state it points to is an unknown, dead, quit, start or match
state.

When implementing a low level search routine with a lazy DFA, it is
necessary to query the type of the current state to know what to do:

* **Unknown** - The state has not yet been computed. The
parameters used to get this state ID must be re-passed to
[`DFA::next_state`](crate::hybrid::dfa::DFA::next_state), which will never
return an unknown state ID.
* **Dead** - A dead state only has transitions to itself. It indicates that
the search cannot do anything else and should stop with whatever result it
has.
* **Quit** - A quit state indicates that the automaton could not answer
whether a match exists or not. Correct search implementations must return a
[`MatchError::quit`](crate::MatchError::quit) when a DFA enters a quit
state.
* **Start** - A start state is a state in which a search can begin.
Lazy DFAs usually have more than one start state. Branching on
this isn't required for correctness, but a common optimization is
to run a prefilter when a search enters a start state. Note that
start states are *not* tagged automatically, and one must enable the
[`Config::specialize_start_states`](crate::hybrid::dfa::Config::specialize_start_states)
setting for start states to be tagged. The reason for this is
that a DFA search loop is usually written to execute a prefilter once it
enters a start state. But if there is no prefilter, this handling can be
quite disastrous as the DFA may ping-pong between the special handling code
and a possible optimized hot path for handling untagged states. When start
states aren't specialized, then they are untagged and remain in the hot
path.
* **Match** - A match state indicates that a match has been found.
Depending on the semantics of your search implementation, it may either
continue until the end of the haystack or a dead state, or it might quit
and return the match immediately.

As an optimization, the [`is_tagged`](LazyStateID::is_tagged) predicate
can be used to determine if a tag exists at all. This is useful to avoid
branching on all of the above types for every byte searched.

# Example

This example shows how `LazyStateID` can be used to implement a correct
search routine with minimal branching. In particular, this search routine
implements "leftmost" matching, which means that it doesn't immediately
stop once a match is found. Instead, it continues until it reaches a dead
state.

Notice also how a correct search implementation deals with
[`CacheError`](crate::hybrid::CacheError)s returned by some of
the lazy DFA routines. When a `CacheError` occurs, it returns
[`MatchError::gave_up`](crate::MatchError::gave_up).

```rust
use regex_automata::{
    hybrid::dfa::{Cache, DFA},
    HalfMatch, MatchError, Input,
};

fn find_leftmost_first(
    dfa: &DFA,
    cache: &mut Cache,
    haystack: &[u8],
) -> Result<Option<HalfMatch>, MatchError> {
    // The start state is determined by inspecting the position and the
    // initial bytes of the haystack. Note that start states can never
    // be match states (since DFAs in this crate delay matches by 1
    // byte), so we don't need to check if the start state is a match.
    let mut sid = dfa.start_state_forward(
        cache,
        &Input::new(haystack),
    )?;
    let mut last_match = None;
    // Walk all the bytes in the haystack. We can quit early if we see
    // a dead or a quit state. The former means the automaton will
    // never transition to any other state. The latter means that the
    // automaton entered a condition in which its search failed.
    for (i, &b) in haystack.iter().enumerate() {
        sid = dfa
            .next_state(cache, sid, b)
            .map_err(|_| MatchError::gave_up(i))?;
        if sid.is_tagged() {
            if sid.is_match() {
                last_match = Some(HalfMatch::new(
                    dfa.match_pattern(cache, sid, 0),
                    i,
                ));
            } else if sid.is_dead() {
                return Ok(last_match);
            } else if sid.is_quit() {
                // It is possible to enter into a quit state after
                // observing a match has occurred. In that case, we
                // should return the match instead of an error.
                if last_match.is_some() {
                    return Ok(last_match);
                }
                return Err(MatchError::quit(b, i));
            }
            // Implementors may also want to check for start states and
            // handle them differently for performance reasons. But it is
            // not necessary for correctness. Note that in order to check
            // for start states, you'll need to enable the
            // 'specialize_start_states' config knob, otherwise start
            // states will not be tagged.
        }
    }
    // Matches are always delayed by 1 byte, so we must explicitly walk
    // the special "EOI" transition at the end of the search.
    sid = dfa
        .next_eoi_state(cache, sid)
        .map_err(|_| MatchError::gave_up(haystack.len()))?;
    if sid.is_match() {
        last_match = Some(HalfMatch::new(
            dfa.match_pattern(cache, sid, 0),
            haystack.len(),
        ));
    }
    Ok(last_match)
}

// We use a greedy '+' operator to show how the search doesn't just stop
// once a match is detected. It continues extending the match. Using
// '[a-z]+?' would also work as expected and stop the search early.
// Greediness is built into the automaton.
let dfa = DFA::new(r"[a-z]+")?;
let mut cache = dfa.create_cache();
let haystack = "123 foobar 4567".as_bytes();
let mat = find_leftmost_first(&dfa, &mut cache, haystack)?.unwrap();
assert_eq!(mat.pattern().as_usize(), 0);
assert_eq!(mat.offset(), 10);

// Here's another example that tests our handling of the special
// EOI transition. This will fail to find a match if we don't call
// 'next_eoi_state' at the end of the search since the match isn't found
// until the final byte in the haystack.
let dfa = DFA::new(r"[0-9]{4}")?;
let mut cache = dfa.create_cache();
let haystack = "123 foobar 4567".as_bytes();
let mat = find_leftmost_first(&dfa, &mut cache, haystack)?.unwrap();
assert_eq!(mat.pattern().as_usize(), 0);
assert_eq!(mat.offset(), 15);

// And note that our search implementation above automatically works
// with multi-DFAs. Namely, `dfa.match_pattern(match_state, 0)` selects
// the appropriate pattern ID for us.
let dfa = DFA::new_many(&[r"[a-z]+", r"[0-9]+"])?;
let mut cache = dfa.create_cache();
let haystack = "123 foobar 4567".as_bytes();
let mat = find_leftmost_first(&dfa, &mut cache, haystack)?.unwrap();
assert_eq!(mat.pattern().as_usize(), 1);
assert_eq!(mat.offset(), 3);
let mat = find_leftmost_first(&dfa, &mut cache, &haystack[3..])?.unwrap();
assert_eq!(mat.pattern().as_usize(), 0);
assert_eq!(mat.offset(), 7);
let mat = find_leftmost_first(&dfa, &mut cache, &haystack[10..])?.unwrap();
assert_eq!(mat.pattern().as_usize(), 1);
assert_eq!(mat.offset(), 5);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="lazystateid-const-max-bit"></span>`const MAX_BIT: usize`

- <span id="lazystateid-const-mask-unknown"></span>`const MASK_UNKNOWN: usize`

- <span id="lazystateid-const-mask-dead"></span>`const MASK_DEAD: usize`

- <span id="lazystateid-const-mask-quit"></span>`const MASK_QUIT: usize`

- <span id="lazystateid-const-mask-start"></span>`const MASK_START: usize`

- <span id="lazystateid-const-mask-match"></span>`const MASK_MATCH: usize`

- <span id="lazystateid-const-max"></span>`const MAX: usize`

- <span id="lazystateid-new"></span>`fn new(id: usize) -> Result<LazyStateID, LazyStateIDError>` — [`LazyStateID`](id/index.md#lazystateid), [`LazyStateIDError`](id/index.md#lazystateiderror)

  Create a new lazy state ID.

  

  If the given identifier exceeds `LazyStateID::MAX`, then this returns

  an error.

- <span id="lazystateid-new-unchecked"></span>`const fn new_unchecked(id: usize) -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

  Create a new lazy state ID without checking whether the given value

  exceeds `LazyStateID::MAX`.

  

  While this is unchecked, providing an incorrect value must never

  sacrifice memory safety.

- <span id="lazystateid-as-usize-untagged"></span>`fn as_usize_untagged(&self) -> usize`

  Return this lazy state ID as an untagged `usize`.

  

  If this lazy state ID is tagged, then the usize returned is the state

  ID without the tag. If the ID was not tagged, then the usize returned

  is equivalent to the state ID.

- <span id="lazystateid-as-usize-unchecked"></span>`const fn as_usize_unchecked(&self) -> usize`

  Return this lazy state ID as its raw internal `usize` value, which may

  be tagged (and thus greater than LazyStateID::MAX).

- <span id="lazystateid-to-unknown"></span>`const fn to_unknown(&self) -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

- <span id="lazystateid-to-dead"></span>`const fn to_dead(&self) -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

- <span id="lazystateid-to-quit"></span>`const fn to_quit(&self) -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

- <span id="lazystateid-to-start"></span>`const fn to_start(&self) -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

  Return this lazy state ID as a state ID that is tagged as a start

  state.

- <span id="lazystateid-to-match"></span>`const fn to_match(&self) -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

  Return this lazy state ID as a lazy state ID that is tagged as a match

  state.

- <span id="lazystateid-is-tagged"></span>`const fn is_tagged(&self) -> bool`

  Return true if and only if this lazy state ID is tagged.

  

  When a lazy state ID is tagged, then one can conclude that it is one

  of a match, start, dead, quit or unknown state.

- <span id="lazystateid-is-unknown"></span>`const fn is_unknown(&self) -> bool`

  Return true if and only if this represents a lazy state ID that is

  "unknown." That is, the state has not yet been created. When a caller

  sees this state ID, it generally means that a state has to be computed

  in order to proceed.

- <span id="lazystateid-is-dead"></span>`const fn is_dead(&self) -> bool`

  Return true if and only if this represents a dead state. A dead state

  is a state that can never transition to any other state except the

  dead state. When a dead state is seen, it generally indicates that a

  search should stop.

- <span id="lazystateid-is-quit"></span>`const fn is_quit(&self) -> bool`

  Return true if and only if this represents a quit state. A quit state

  is a state that is representationally equivalent to a dead state,

  except it indicates the automaton has reached a point at which it can

  no longer determine whether a match exists or not. In general, this

  indicates an error during search and the caller must either pass this

  error up or use a different search technique.

- <span id="lazystateid-is-start"></span>`const fn is_start(&self) -> bool`

  Return true if and only if this lazy state ID has been tagged as a

  start state.

  

  Note that if

  [`Config::specialize_start_states`](crate::hybrid::dfa::Config) is

  disabled (which is the default), then this will always return false

  since start states won't be tagged.

- <span id="lazystateid-is-match"></span>`const fn is_match(&self) -> bool`

  Return true if and only if this lazy state ID has been tagged as a

  match state.

#### Trait Implementations

##### `impl Any for LazyStateID`

- <span id="lazystateid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LazyStateID`

- <span id="lazystateid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LazyStateID`

- <span id="lazystateid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LazyStateID`

- <span id="lazystateid-clone"></span>`fn clone(&self) -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

##### `impl CloneToUninit for LazyStateID`

- <span id="lazystateid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LazyStateID`

##### `impl Debug for LazyStateID`

- <span id="lazystateid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LazyStateID`

- <span id="lazystateid-default"></span>`fn default() -> LazyStateID` — [`LazyStateID`](id/index.md#lazystateid)

##### `impl Eq for LazyStateID`

##### `impl<T> From for LazyStateID`

- <span id="lazystateid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for LazyStateID`

- <span id="lazystateid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LazyStateID`

- <span id="lazystateid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for LazyStateID`

- <span id="lazystateid-ord-cmp"></span>`fn cmp(&self, other: &LazyStateID) -> cmp::Ordering` — [`LazyStateID`](id/index.md#lazystateid)

##### `impl PartialEq for LazyStateID`

- <span id="lazystateid-partialeq-eq"></span>`fn eq(&self, other: &LazyStateID) -> bool` — [`LazyStateID`](id/index.md#lazystateid)

##### `impl PartialOrd for LazyStateID`

- <span id="lazystateid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LazyStateID) -> option::Option<cmp::Ordering>` — [`LazyStateID`](id/index.md#lazystateid)

##### `impl StructuralPartialEq for LazyStateID`

##### `impl ToOwned for LazyStateID`

- <span id="lazystateid-toowned-type-owned"></span>`type Owned = T`

- <span id="lazystateid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="lazystateid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LazyStateID`

- <span id="lazystateid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazystateid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LazyStateID`

- <span id="lazystateid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazystateid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `StartError`

```rust
enum StartError {
    Cache {
        err: CacheError,
    },
    Quit {
        byte: u8,
    },
    UnsupportedAnchored {
        mode: crate::util::search::Anchored,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/error.rs:117-136`](../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/error.rs#L117-L136)*

An error that can occur when computing the start state for a search.

Computing a start state can fail for a few reasons, either
based on incorrect configuration or even based on whether
the look-behind byte triggers a quit state. Typically
one does not need to handle this error if you're using
[`DFA::start_state_forward`](crate::hybrid::dfa::DFA::start_state_forward)
(or its reverse counterpart), as that routine automatically converts
`StartError` to a [`MatchError`](crate::MatchError) for you.

This error may be returned by the
[`DFA::start_state`](crate::hybrid::dfa::DFA::start_state) routine.

This error implements the `std::error::Error` trait when the `std` feature
is enabled.

This error is marked as non-exhaustive. New variants may be added in a
semver compatible release.

#### Variants

- **`Cache`**

  An error that occurs when cache inefficiency has dropped below the
  configured heuristic thresholds.

- **`Quit`**

  An error that occurs when a starting configuration's look-behind byte
  is in this DFA's quit set.

- **`UnsupportedAnchored`**

  An error that occurs when the caller requests an anchored mode that
  isn't supported by the DFA.

#### Implementations

- <span id="starterror-cache"></span>`fn cache(err: CacheError) -> StartError` — [`CacheError`](error/index.md#cacheerror), [`StartError`](error/index.md#starterror)

- <span id="starterror-quit"></span>`fn quit(byte: u8) -> StartError` — [`StartError`](error/index.md#starterror)

- <span id="starterror-unsupported-anchored"></span>`fn unsupported_anchored(mode: Anchored) -> StartError` — [`Anchored`](../index.md#anchored), [`StartError`](error/index.md#starterror)

#### Trait Implementations

##### `impl Any for StartError`

- <span id="starterror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartError`

- <span id="starterror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartError`

- <span id="starterror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartError`

- <span id="starterror-clone"></span>`fn clone(&self) -> StartError` — [`StartError`](error/index.md#starterror)

##### `impl CloneToUninit for StartError`

- <span id="starterror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StartError`

- <span id="starterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StartError`

- <span id="starterror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for StartError`

- <span id="starterror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> From for StartError`

- <span id="starterror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartError`

- <span id="starterror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StartError`

- <span id="starterror-toowned-type-owned"></span>`type Owned = T`

- <span id="starterror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="starterror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StartError`

- <span id="starterror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StartError`

- <span id="starterror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="starterror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartError`

- <span id="starterror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="starterror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

