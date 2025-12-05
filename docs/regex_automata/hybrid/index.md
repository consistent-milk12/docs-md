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
* A `dfa::DFA` provides direct low level access to a lazy DFA.

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

## Modules

- [`dfa`](dfa/index.md) - Types and routines specific to lazy DFAs.
- [`regex`](regex/index.md) - A lazy DFA backed `Regex`.

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

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

- `fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md)

- `fn insufficient_cache_capacity(minimum: usize, given: usize) -> BuildError` — [`BuildError`](../../hybrid/error/index.md)

- `fn insufficient_state_id_capacity(err: LazyStateIDError) -> BuildError` — [`LazyStateIDError`](../../hybrid/id/index.md), [`BuildError`](../../hybrid/error/index.md)

- `fn unsupported_dfa_word_boundary_unicode() -> BuildError` — [`BuildError`](../../hybrid/error/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> BuildError` — [`BuildError`](../../hybrid/error/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `CacheError`

```rust
struct CacheError(());
```

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

- `fn too_many_cache_clears() -> CacheError` — [`CacheError`](../../hybrid/error/index.md)

- `fn bad_efficiency() -> CacheError` — [`CacheError`](../../hybrid/error/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> CacheError` — [`CacheError`](../../hybrid/error/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `LazyStateID`

```rust
struct LazyStateID(u32);
```

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

- `const MAX_BIT: usize`

- `const MASK_UNKNOWN: usize`

- `const MASK_DEAD: usize`

- `const MASK_QUIT: usize`

- `const MASK_START: usize`

- `const MASK_MATCH: usize`

- `const MAX: usize`

- `fn new(id: usize) -> Result<LazyStateID, LazyStateIDError>` — [`LazyStateID`](../../hybrid/id/index.md), [`LazyStateIDError`](../../hybrid/id/index.md)

- `const fn new_unchecked(id: usize) -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

- `fn as_usize_untagged(self: &Self) -> usize`

- `const fn as_usize_unchecked(self: &Self) -> usize`

- `const fn to_unknown(self: &Self) -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

- `const fn to_dead(self: &Self) -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

- `const fn to_quit(self: &Self) -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

- `const fn to_start(self: &Self) -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

- `const fn to_match(self: &Self) -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

- `const fn is_tagged(self: &Self) -> bool`

- `const fn is_unknown(self: &Self) -> bool`

- `const fn is_dead(self: &Self) -> bool`

- `const fn is_quit(self: &Self) -> bool`

- `const fn is_start(self: &Self) -> bool`

- `const fn is_match(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> LazyStateID` — [`LazyStateID`](../../hybrid/id/index.md)

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &LazyStateID) -> $crate::cmp::Ordering` — [`LazyStateID`](../../hybrid/id/index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &LazyStateID) -> bool` — [`LazyStateID`](../../hybrid/id/index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &LazyStateID) -> $crate::option::Option<$crate::cmp::Ordering>` — [`LazyStateID`](../../hybrid/id/index.md)

##### `impl StructuralPartialEq`

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

- `fn cache(err: CacheError) -> StartError` — [`CacheError`](../../hybrid/error/index.md), [`StartError`](../../hybrid/error/index.md)

- `fn quit(byte: u8) -> StartError` — [`StartError`](../../hybrid/error/index.md)

- `fn unsupported_anchored(mode: Anchored) -> StartError` — [`Anchored`](../../util/search/index.md), [`StartError`](../../hybrid/error/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> StartError` — [`StartError`](../../hybrid/error/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

