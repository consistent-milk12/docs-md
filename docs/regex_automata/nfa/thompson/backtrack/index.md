*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [backtrack](index.md)*

---

# Module `backtrack`

An NFA backed bounded backtracker for executing regex searches with capturing
groups.

This module provides a [`BoundedBacktracker`](regex_automata/nfa/thompson/backtrack/index.md) that works by simulating an NFA
using the classical backtracking algorithm with a twist: it avoids redoing
work that it has done before and thereby avoids worst case exponential time.
In exchange, it can only be used on "short" haystacks. Its advantage is that
is can be faster than the [`PikeVM`](thompson::pikevm::PikeVM) in many cases
because it does less book-keeping.

## Structs

### `Config`

```rust
struct Config {
    // [REDACTED: Private Fields]
}
```

The configuration used for building a bounded backtracker.

A bounded backtracker configuration is a simple data object that is
typically used with [`Builder::configure`](#configure).

#### Implementations

- `fn new() -> Config`
  Return a new default regex configuration.

- `fn prefilter(self: Self, pre: Option<Prefilter>) -> Config`
  Set a prefilter to be used whenever a start state is entered.

- `fn visited_capacity(self: Self, capacity: usize) -> Config`
  Set the visited capacity used to bound backtracking.

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>`
  Returns the prefilter set in this configuration, if one at all.

- `fn get_visited_capacity(self: &Self) -> usize`
  Returns the configured visited capacity.

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

- `fn clone(self: &Self) -> Config`

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

##### `impl Default`

- `fn default() -> Config`

### `Builder`

```rust
struct Builder {
    // [REDACTED: Private Fields]
}
```

A builder for a bounded backtracker.

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the `BoundedBacktracker` construction. This builder
is different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`](#utf8) controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
use regex_automata::{
    nfa::thompson::{self, backtrack::BoundedBacktracker},
    util::syntax,
    Match,
};

let re = BoundedBacktracker::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Ok(Match::must(0, 1..9)));
let got = re.try_find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a BoundedBacktracker Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap()?.range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder`
  Create a new BoundedBacktracker builder with its default configuration.

- `fn build(self: &Self, pattern: &str) -> Result<BoundedBacktracker, BuildError>`
  Build a `BoundedBacktracker` from the given pattern.

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<BoundedBacktracker, BuildError>`
  Build a `BoundedBacktracker` from the given patterns.

- `fn build_from_nfa(self: &Self, nfa: NFA) -> Result<BoundedBacktracker, BuildError>`
  Build a `BoundedBacktracker` directly from its NFA.

- `fn configure(self: &mut Self, config: Config) -> &mut Builder`
  Apply the given `BoundedBacktracker` configuration options to this

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder`
  Set the syntax configuration for this builder using

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder`
  Set the Thompson NFA configuration for this builder using

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

- `fn clone(self: &Self) -> Builder`

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

### `BoundedBacktracker`

```rust
struct BoundedBacktracker {
    // [REDACTED: Private Fields]
}
```

A backtracking regex engine that bounds its execution to avoid exponential
blow-up.

This regex engine only implements leftmost-first match semantics and
only supports leftmost searches. It effectively does the same thing as a
[`PikeVM`](thompson::pikevm::PikeVM), but typically does it faster because
it doesn't have to worry about copying capturing group spans for most NFA
states. Instead, the backtracker can maintain one set of captures (provided
by the caller) and never needs to copy them. In exchange, the backtracker
bounds itself to ensure it doesn't exhibit worst case exponential time.
This results in the backtracker only being able to handle short haystacks
given reasonable memory usage.

# Searches may return an error!

By design, this backtracking regex engine is bounded. This bound is
implemented by not visiting any combination of NFA state ID and position
in a haystack more than once. Thus, the total memory required to bound
backtracking is proportional to `haystack.len() * nfa.states().len()`.
This can obviously get quite large, since large haystacks aren't terribly
uncommon. To avoid using exorbitant memory, the capacity is bounded by
a fixed limit set via [`Config::visited_capacity`](#visited-capacity). Thus, if the total
capacity required for a particular regex and a haystack exceeds this
capacity, then the search routine will return an error.

Unlike other regex engines that may return an error at search time (like
the DFA or the hybrid NFA/DFA), there is no way to guarantee that a bounded
backtracker will work for every haystack. Therefore, this regex engine
_only_ exposes fallible search routines to avoid the footgun of panicking
when running a search on a haystack that is too big.

If one wants to use the fallible search APIs without handling the
error, the only way to guarantee an error won't occur from the
haystack length is to ensure the haystack length does not exceed
[`BoundedBacktracker::max_haystack_len`](#max-haystack-len).

# Example: Unicode word boundaries

This example shows that the bounded backtracker implements Unicode word
boundaries correctly by default.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Ok(Match::must(0, 0..12))), it.next());
assert_eq!(Some(Ok(Match::must(0, 13..23))), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: multiple regex patterns

The bounded backtracker supports searching for multiple patterns
simultaneously, just like other regex engines. Note though that because it
uses a backtracking strategy, this regex engine is unlikely to scale well
as more patterns are added. But then again, as more patterns are added, the
maximum haystack length allowed will also shorten (assuming the visited
capacity remains invariant).

```
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new_many(&["[a-z]+", "[0-9]+"])?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "abc 1 foo 4567 0 quux");
assert_eq!(Some(Ok(Match::must(0, 0..3))), it.next());
assert_eq!(Some(Ok(Match::must(1, 4..5))), it.next());
assert_eq!(Some(Ok(Match::must(0, 6..9))), it.next());
assert_eq!(Some(Ok(Match::must(1, 10..14))), it.next());
assert_eq!(Some(Ok(Match::must(1, 15..16))), it.next());
assert_eq!(Some(Ok(Match::must(0, 17..21))), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError>`
  Parse the given regular expression using the default configuration and

- `fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<BoundedBacktracker, BuildError>`
  Like `new`, but parses multiple patterns into a single "multi regex."

- `fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError>`
  # Example

- `fn always_match() -> Result<BoundedBacktracker, BuildError>`
  Create a new `BoundedBacktracker` that matches every input.

- `fn never_match() -> Result<BoundedBacktracker, BuildError>`
  Create a new `BoundedBacktracker` that never matches any input.

- `fn config() -> Config`
  Return a default configuration for a `BoundedBacktracker`.

- `fn builder() -> Builder`
  Return a builder for configuring the construction of a

- `fn create_cache(self: &Self) -> Cache`
  Create a new cache for this regex.

- `fn create_captures(self: &Self) -> Captures`
  Create a new empty set of capturing groups that is guaranteed to be

- `fn reset_cache(self: &Self, cache: &mut Cache)`
  Reset the given cache such that it can be used for searching with the

- `fn pattern_len(self: &Self) -> usize`
  Returns the total number of patterns compiled into this

- `fn get_config(self: &Self) -> &Config`
  Return the config for this `BoundedBacktracker`.

- `fn get_nfa(self: &Self) -> &NFA`
  Returns a reference to the underlying NFA.

- `fn max_haystack_len(self: &Self) -> usize`
  Returns the maximum haystack length supported by this backtracker.

- `fn try_is_match<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> Result<bool, MatchError>`
  Returns true if and only if this regex matches the given haystack.

- `fn try_find<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> Result<Option<Match>, MatchError>`
  Executes a leftmost forward search and returns a `Match` if one exists.

- `fn try_captures<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I, caps: &mut Captures) -> Result<(), MatchError>`
  Executes a leftmost forward search and writes the spans of capturing

- `fn try_find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(self: &'r Self, cache: &'c mut Cache, input: I) -> TryFindMatches<'r, 'c, 'h>`
  Returns an iterator over all non-overlapping leftmost matches in the

- `fn try_captures_iter<'r, 'c, 'h, I: Into<Input<'h>>>(self: &'r Self, cache: &'c mut Cache, input: I) -> TryCapturesMatches<'r, 'c, 'h>`
  Returns an iterator over all non-overlapping `Captures` values. If no

- `fn try_search(self: &Self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures) -> Result<(), MatchError>`
  Executes a leftmost forward search and writes the spans of capturing

- `fn try_search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Result<Option<PatternID>, MatchError>`
  Executes a leftmost forward search and writes the spans of capturing

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

- `fn clone(self: &Self) -> BoundedBacktracker`

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

### `TryFindMatches<'r, 'c, 'h>`

```rust
struct TryFindMatches<'r, 'c, 'h> {
    // [REDACTED: Private Fields]
}
```

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`BoundedBacktracker::try_find_iter`](#try-find-iter)
method.

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

##### `impl Iterator<'r, 'c, 'h>`

- `type Item = Result<Match, MatchError>`

- `fn next(self: &mut Self) -> Option<Result<Match, MatchError>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'c, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TryCapturesMatches<'r, 'c, 'h>`

```rust
struct TryCapturesMatches<'r, 'c, 'h> {
    // [REDACTED: Private Fields]
}
```

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the
[`BoundedBacktracker::try_captures_iter`](#try-captures-iter) method.

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

##### `impl Iterator<'r, 'c, 'h>`

- `type Item = Result<Captures, MatchError>`

- `fn next(self: &mut Self) -> Option<Result<Captures, MatchError>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'c, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Cache`

```rust
struct Cache {
    // [REDACTED: Private Fields]
}
```

A cache represents mutable state that a [`BoundedBacktracker`](regex_automata/nfa/thompson/backtrack/index.md) requires
during a search.

For a given [`BoundedBacktracker`](regex_automata/nfa/thompson/backtrack/index.md), its corresponding cache may be created
either via [`BoundedBacktracker::create_cache`](#create-cache), or via [`Cache::new`](#new).
They are equivalent in every way, except the former does not require
explicitly importing `Cache`.

A particular `Cache` is coupled with the [`BoundedBacktracker`](regex_automata/nfa/thompson/backtrack/index.md) from which
it was created. It may only be used with that `BoundedBacktracker`. A cache
and its allocations may be re-purposed via [`Cache::reset`](#reset), in which case,
it can only be used with the new `BoundedBacktracker` (and not the old
one).

#### Implementations

- `fn new(re: &BoundedBacktracker) -> Cache`
  Create a new [`BoundedBacktracker`] cache.

- `fn reset(self: &mut Self, re: &BoundedBacktracker)`
  Reset this cache such that it can be used for searching with different

- `fn memory_usage(self: &Self) -> usize`
  Returns the heap memory usage, in bytes, of this cache.

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

- `fn clone(self: &Self) -> Cache`

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

## Functions

### `min_visited_capacity`

```rust
fn min_visited_capacity(nfa: &crate::nfa::thompson::NFA, input: &crate::util::search::Input<'_>) -> usize
```

Returns the minimum visited capacity for the given haystack.

This function can be used as the argument to [`Config::visited_capacity`](#visited-capacity)
in order to guarantee that a backtracking search for the given `input`
won't return an error when using a [`BoundedBacktracker`](regex_automata/nfa/thompson/backtrack/index.md) built from the
given `NFA`.

This routine exists primarily as a way to test that the bounded backtracker
works correctly when its capacity is set to the smallest possible amount.
Still, it may be useful in cases where you know you want to use the bounded
backtracker for a specific input, and just need to know what visited
capacity to provide to make it work.

Be warned that this number could be quite large as it is multiplicative in
the size the given NFA and haystack.

