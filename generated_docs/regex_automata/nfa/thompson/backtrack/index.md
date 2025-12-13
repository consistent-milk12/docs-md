*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [backtrack](index.md)*

---

# Module `backtrack`

An NFA backed bounded backtracker for executing regex searches with capturing
groups.

This module provides a [`BoundedBacktracker`](#boundedbacktracker) that works by simulating an NFA
using the classical backtracking algorithm with a twist: it avoids redoing
work that it has done before and thereby avoids worst case exponential time.
In exchange, it can only be used on "short" haystacks. Its advantage is that
is can be faster than the [`PikeVM`](thompson::pikevm::PikeVM) in many cases
because it does less book-keeping.

## Contents

- [Structs](#structs)
  - [`Config`](#config)
  - [`Builder`](#builder)
  - [`BoundedBacktracker`](#boundedbacktracker)
  - [`TryFindMatches`](#tryfindmatches)
  - [`TryCapturesMatches`](#trycapturesmatches)
  - [`Cache`](#cache)
  - [`Visited`](#visited)
- [Enums](#enums)
  - [`Frame`](#frame)
- [Functions](#functions)
  - [`min_visited_capacity`](#min-visited-capacity)
  - [`div_ceil`](#div-ceil)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration used for building a bounded backtracker. |
| [`Builder`](#builder) | struct | A builder for a bounded backtracker. |
| [`BoundedBacktracker`](#boundedbacktracker) | struct | A backtracking regex engine that bounds its execution to avoid exponential blow-up. |
| [`TryFindMatches`](#tryfindmatches) | struct | An iterator over all non-overlapping matches for a fallible search. |
| [`TryCapturesMatches`](#trycapturesmatches) | struct | An iterator over all non-overlapping leftmost matches, with their capturing groups, for a fallible search. |
| [`Cache`](#cache) | struct | A cache represents mutable state that a [`BoundedBacktracker`] requires during a search. |
| [`Visited`](#visited) | struct | A bitset that keeps track of whether a particular (StateID, offset) has been considered during backtracking. |
| [`Frame`](#frame) | enum | Represents a stack frame on the heap while doing backtracking. |
| [`min_visited_capacity`](#min-visited-capacity) | fn | Returns the minimum visited capacity for the given haystack. |
| [`div_ceil`](#div-ceil) | fn | Integer division, but rounds up instead of down. |

## Structs

### `Config`

```rust
struct Config {
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
    visited_capacity: Option<usize>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:50-53`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L50-L53)*

The configuration used for building a bounded backtracker.

A bounded backtracker configuration is a simple data object that is
typically used with `Builder::configure`.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

  Return a new default regex configuration.

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../../util/prefilter/index.md#prefilter), [`Config`](#config)

  Set a prefilter to be used whenever a start state is entered.

  

  A [`Prefilter`](../../../util/prefilter/index.md) in this context is meant to accelerate searches by

  looking for literal prefixes that every match for the corresponding

  pattern (or patterns) must start with. Once a prefilter produces a

  match, the underlying search routine continues on to try and confirm

  the match.

  

  Be warned that setting a prefilter does not guarantee that the search

  will be faster. While it's usually a good bet, if the prefilter

  produces a lot of false positive candidates (i.e., positions matched

  by the prefilter but not by the regex), then the overall result can

  be slower than if you had just executed the regex engine without any

  prefilters.

  

  By default no prefilter is set.

  

  # Example

  

  ```rust

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      util::prefilter::Prefilter,

      Input, Match, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["foo", "bar"]);

  let re = BoundedBacktracker::builder()

      .configure(BoundedBacktracker::config().prefilter(pre))

      .build(r"(foo|bar)[a-z]+")?;

  let mut cache = re.create_cache();

  let input = Input::new("foo1 barfox bar");

  assert_eq!(

      Some(Match::must(0, 5..11)),

      re.try_find(&mut cache, input)?,

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  Be warned though that an incorrect prefilter can lead to incorrect

  results!

  

  ```rust

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      util::prefilter::Prefilter,

      Input, HalfMatch, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["foo", "car"]);

  let re = BoundedBacktracker::builder()

      .configure(BoundedBacktracker::config().prefilter(pre))

      .build(r"(foo|bar)[a-z]+")?;

  let mut cache = re.create_cache();

  let input = Input::new("foo1 barfox bar");

  // No match reported even though there clearly is one!

  assert_eq!(None, re.try_find(&mut cache, input)?);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-visited-capacity"></span>`fn visited_capacity(self, capacity: usize) -> Config` — [`Config`](#config)

  Set the visited capacity used to bound backtracking.

  

  The visited capacity represents the amount of heap memory (in bytes) to

  allocate toward tracking which parts of the backtracking search have

  been done before. The heap memory needed for any particular search is

  proportional to `haystack.len() * nfa.states().len()`, which an be

  quite large. Therefore, the bounded backtracker is typically only able

  to run on shorter haystacks.

  

  For a given regex, increasing the visited capacity means that the

  maximum haystack length that can be searched is increased. The

  `BoundedBacktracker::max_haystack_len` method returns that maximum.

  

  The default capacity is a reasonable but empirically chosen size.

  

  # Example

  

  As with other regex engines, Unicode is what tends to make the bounded

  backtracker less useful by making the maximum haystack length quite

  small. If necessary, increasing the visited capacity using this routine

  will increase the maximum haystack length at the cost of using more

  memory.

  

  Note though that the specific maximum values here are not an API

  guarantee. The default visited capacity is subject to change and not

  covered by semver.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::backtrack::BoundedBacktracker;

  

  // Unicode inflates the size of the underlying NFA quite a bit, and

  // thus means that the backtracker can only handle smaller haystacks,

  // assuming that the visited capacity remains unchanged.

  let re = BoundedBacktracker::new(r"\w+")?;

  assert!(re.max_haystack_len() <= 7_000);

  // But we can increase the visited capacity to handle bigger haystacks!

  let re = BoundedBacktracker::builder()

      .configure(BoundedBacktracker::config().visited_capacity(1<<20))

      .build(r"\w+")?;

  assert!(re.max_haystack_len() >= 25_000);

  assert!(re.max_haystack_len() <= 28_000);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../../util/prefilter/index.md#prefilter)

  Returns the prefilter set in this configuration, if one at all.

- <span id="config-get-visited-capacity"></span>`fn get_visited_capacity(&self) -> usize`

  Returns the configured visited capacity.

  

  Note that the actual capacity used may be slightly bigger than the

  configured capacity.

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](#config)

  Overwrite the default configuration such that the options in `o` are

  always used. If an option in `o` is not set, then the corresponding

  option in `self` is used. If it's not set in `self` either, then it

  remains not set.

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

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

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

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:256-260`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L256-L260)*

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
* `thompson::Config::utf8` controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```rust
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

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new BoundedBacktracker builder with its default configuration.

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  Build a `BoundedBacktracker` from the given pattern.

  

  If there was a problem parsing or compiling the pattern, then an error

  is returned.

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  Build a `BoundedBacktracker` from the given patterns.

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: NFA) -> Result<BoundedBacktracker, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  Build a `BoundedBacktracker` directly from its NFA.

  

  Note that when using this method, any configuration that applies to the

  construction of the NFA itself will of course be ignored, since the NFA

  given here is already built.

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

  Apply the given `BoundedBacktracker` configuration options to this

  builder.

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../../util/syntax/index.md#config), [`Builder`](#builder)

  Set the syntax configuration for this builder using

  [`syntax::Config`](crate::util::syntax::Config).

  

  This permits setting things like case insensitivity, Unicode and multi

  line mode.

  

  These settings only apply when constructing a `BoundedBacktracker`

  directly from a pattern.

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../compiler/index.md#config), [`Builder`](#builder)

  Set the Thompson NFA configuration for this builder using

  [`nfa::thompson::Config`](crate::nfa::thompson::Config).

  

  This permits setting things like if additional time should be spent

  shrinking the size of the NFA.

  

  These settings only apply when constructing a `BoundedBacktracker`

  directly from a pattern.

#### Trait Implementations

##### `impl Any for Builder`

- <span id="builder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Builder`

- <span id="builder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Builder`

- <span id="builder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl CloneToUninit for Builder`

- <span id="builder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Builder`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Builder`

- <span id="builder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Builder`

- <span id="builder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Builder`

- <span id="builder-toowned-type-owned"></span>`type Owned = T`

- <span id="builder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Builder`

- <span id="builder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Builder`

- <span id="builder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoundedBacktracker`

```rust
struct BoundedBacktracker {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:427-430`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L427-L430)*

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
a fixed limit set via `Config::visited_capacity`. Thus, if the total
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
`BoundedBacktracker::max_haystack_len`.

# Example: Unicode word boundaries

This example shows that the bounded backtracker implements Unicode word
boundaries correctly by default.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Ok(Match::must(0, 0..12))), it.next());
assert_eq!(Some(Ok(Match::must(0, 13..23))), it.next());
assert_eq!(None, it.next());
Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: multiple regex patterns

The bounded backtracker supports searching for multiple patterns
simultaneously, just like other regex engines. Note though that because it
uses a backtracking strategy, this regex engine is unlikely to scale well
as more patterns are added. But then again, as more patterns are added, the
maximum haystack length allowed will also shorten (assuming the visited
capacity remains invariant).

```rust
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
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="boundedbacktracker-new"></span>`fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  Parse the given regular expression using the default configuration and

  return the corresponding `BoundedBacktracker`.

  

  If you want a non-default configuration, then use the [`Builder`](#builder) to

  set your own configuration.

  

  # Example

  

  ```rust

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      Match,

  };

  

  let re = BoundedBacktracker::new("foo[0-9]+bar")?;

  let mut cache = re.create_cache();

  assert_eq!(

      Some(Ok(Match::must(0, 3..14))),

      re.try_find_iter(&mut cache, "zzzfoo12345barzzz").next(),

  );

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  Like `new`, but parses multiple patterns into a single "multi regex."

  This similarly uses the default regex configuration.

  

  # Example

  

  ```rust

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      Match,

  };

  

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

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-new-from-nfa"></span>`fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  # Example

  

  This shows how to hand assemble a regular expression via its HIR,

  compile an NFA from it and build a BoundedBacktracker from the NFA.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{NFA, backtrack::BoundedBacktracker},

      Match,

  };

  use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

  

  let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![

      ClassBytesRange::new(b'0', b'9'),

      ClassBytesRange::new(b'A', b'Z'),

      ClassBytesRange::new(b'_', b'_'),

      ClassBytesRange::new(b'a', b'z'),

  ])));

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

  

  let re = BoundedBacktracker::new_from_nfa(nfa)?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  let expected = Some(Match::must(0, 3..4));

  re.try_captures(&mut cache, "!@#A#@!", &mut caps)?;

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-always-match"></span>`fn always_match() -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  Create a new `BoundedBacktracker` that matches every input.

  

  # Example

  

  ```rust

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      Match,

  };

  

  let re = BoundedBacktracker::always_match()?;

  let mut cache = re.create_cache();

  

  let expected = Some(Ok(Match::must(0, 0..0)));

  assert_eq!(expected, re.try_find_iter(&mut cache, "").next());

  assert_eq!(expected, re.try_find_iter(&mut cache, "foo").next());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-never-match"></span>`fn never_match() -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

  Create a new `BoundedBacktracker` that never matches any input.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::backtrack::BoundedBacktracker;

  

  let re = BoundedBacktracker::never_match()?;

  let mut cache = re.create_cache();

  

  assert_eq!(None, re.try_find_iter(&mut cache, "").next());

  assert_eq!(None, re.try_find_iter(&mut cache, "foo").next());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-config"></span>`fn config() -> Config` — [`Config`](#config)

  Return a default configuration for a `BoundedBacktracker`.

  

  This is a convenience routine to avoid needing to import the `Config`

  type when customizing the construction of a `BoundedBacktracker`.

  

  # Example

  

  This example shows how to disable UTF-8 mode. When UTF-8 mode is

  disabled, zero-width matches that split a codepoint are allowed.

  Otherwise they are never reported.

  

  In the code below, notice that `""` is permitted to match positions

  that split the encoding of a codepoint.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, backtrack::BoundedBacktracker},

      Match,

  };

  

  let re = BoundedBacktracker::builder()

      .thompson(thompson::Config::new().utf8(false))

      .build(r"")?;

  let mut cache = re.create_cache();

  

  let haystack = "a☃z";

  let mut it = re.try_find_iter(&mut cache, haystack);

  assert_eq!(Some(Ok(Match::must(0, 0..0))), it.next());

  assert_eq!(Some(Ok(Match::must(0, 1..1))), it.next());

  assert_eq!(Some(Ok(Match::must(0, 2..2))), it.next());

  assert_eq!(Some(Ok(Match::must(0, 3..3))), it.next());

  assert_eq!(Some(Ok(Match::must(0, 4..4))), it.next());

  assert_eq!(Some(Ok(Match::must(0, 5..5))), it.next());

  assert_eq!(None, it.next());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  Return a builder for configuring the construction of a

  `BoundedBacktracker`.

  

  This is a convenience routine to avoid needing to import the

  [`Builder`](#builder) type in common cases.

  

  # Example

  

  This example shows how to use the builder to disable UTF-8 mode

  everywhere.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      nfa::thompson::{self, backtrack::BoundedBacktracker},

      util::syntax,

      Match,

  };

  

  let re = BoundedBacktracker::builder()

      .syntax(syntax::Config::new().utf8(false))

      .thompson(thompson::Config::new().utf8(false))

      .build(r"foo(?-u:[^b])ar.*")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";

  let expected = Some(Match::must(0, 1..9));

  re.try_captures(&mut cache, haystack, &mut caps)?;

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](#cache)

  Create a new cache for this regex.

  

  The cache returned should only be used for searches for this

  regex. If you want to reuse the cache for another regex, then you

  must call `Cache::reset` with that regex (or, equivalently,

  `BoundedBacktracker::reset_cache`).

- <span id="boundedbacktracker-create-captures"></span>`fn create_captures(&self) -> Captures` — [`Captures`](../../../util/captures/index.md#captures)

  Create a new empty set of capturing groups that is guaranteed to be

  valid for the search APIs on this `BoundedBacktracker`.

  

  A `Captures` value created for a specific `BoundedBacktracker` cannot

  be used with any other `BoundedBacktracker`.

  

  This is a convenience function for `Captures::all`. See the

  [`Captures`](../../../util/captures/index.md) documentation for an explanation of its alternative

  constructors that permit the `BoundedBacktracker` to do less work

  during a search, and thus might make it faster.

- <span id="boundedbacktracker-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](#cache)

  Reset the given cache such that it can be used for searching with the

  this `BoundedBacktracker` (and only this `BoundedBacktracker`).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different `BoundedBacktracker`.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different

  `BoundedBacktracker`.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      Match,

  };

  

  let re1 = BoundedBacktracker::new(r"\w")?;

  let re2 = BoundedBacktracker::new(r"\W")?;

  

  let mut cache = re1.create_cache();

  assert_eq!(

      Some(Ok(Match::must(0, 0..2))),

      re1.try_find_iter(&mut cache, "Δ").next(),

  );

  

  // Using 'cache' with re2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the BoundedBacktracker we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 're1' is also not

  // allowed.

  cache.reset(&re2);

  assert_eq!(

      Some(Ok(Match::must(0, 0..3))),

      re2.try_find_iter(&mut cache, "☃").next(),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the total number of patterns compiled into this

  `BoundedBacktracker`.

  

  In the case of a `BoundedBacktracker` that contains no patterns, this

  returns `0`.

  

  # Example

  

  This example shows the pattern length for a `BoundedBacktracker` that

  never matches:

  

  ```rust

  use regex_automata::nfa::thompson::backtrack::BoundedBacktracker;

  

  let re = BoundedBacktracker::never_match()?;

  assert_eq!(re.pattern_len(), 0);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  And another example for a `BoundedBacktracker` that matches at every

  position:

  

  ```rust

  use regex_automata::nfa::thompson::backtrack::BoundedBacktracker;

  

  let re = BoundedBacktracker::always_match()?;

  assert_eq!(re.pattern_len(), 1);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  And finally, a `BoundedBacktracker` that was constructed from multiple

  patterns:

  

  ```rust

  use regex_automata::nfa::thompson::backtrack::BoundedBacktracker;

  

  let re = BoundedBacktracker::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"])?;

  assert_eq!(re.pattern_len(), 3);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="boundedbacktracker-get-config"></span>`fn get_config(&self) -> &Config` — [`Config`](#config)

  Return the config for this `BoundedBacktracker`.

- <span id="boundedbacktracker-get-nfa"></span>`fn get_nfa(&self) -> &NFA` — [`NFA`](../nfa/index.md#nfa)

  Returns a reference to the underlying NFA.

- <span id="boundedbacktracker-max-haystack-len"></span>`fn max_haystack_len(&self) -> usize`

  Returns the maximum haystack length supported by this backtracker.

  

  This routine is a function of both `Config::visited_capacity` and the

  internal size of the backtracker's NFA.

  

  # Example

  

  This example shows how the maximum haystack length can vary depending

  on the size of the regex itself. Note though that the specific maximum

  values here are not an API guarantee. The default visited capacity is

  subject to change and not covered by semver.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      Match, MatchError,

  };

  

  // If you're only using ASCII, you get a big budget.

  let re = BoundedBacktracker::new(r"(?-u)\w+")?;

  let mut cache = re.create_cache();

  assert_eq!(re.max_haystack_len(), 299_592);

  // Things work up to the max.

  let mut haystack = "a".repeat(299_592);

  let expected = Some(Ok(Match::must(0, 0..299_592)));

  assert_eq!(expected, re.try_find_iter(&mut cache, &haystack).next());

  // But you'll get an error if you provide a haystack that's too big.

  // Notice that we use the 'try_find_iter' routine instead, which

  // yields Result<Match, MatchError> instead of Match.

  haystack.push('a');

  let expected = Some(Err(MatchError::haystack_too_long(299_593)));

  assert_eq!(expected, re.try_find_iter(&mut cache, &haystack).next());

  

  // Unicode inflates the size of the underlying NFA quite a bit, and

  // thus means that the backtracker can only handle smaller haystacks,

  // assuming that the visited capacity remains unchanged.

  let re = BoundedBacktracker::new(r"\w+")?;

  assert!(re.max_haystack_len() <= 7_000);

  // But we can increase the visited capacity to handle bigger haystacks!

  let re = BoundedBacktracker::builder()

      .configure(BoundedBacktracker::config().visited_capacity(1<<20))

      .build(r"\w+")?;

  assert!(re.max_haystack_len() >= 25_000);

  assert!(re.max_haystack_len() <= 28_000);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

#### Trait Implementations

##### `impl Any for BoundedBacktracker`

- <span id="boundedbacktracker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoundedBacktracker`

- <span id="boundedbacktracker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoundedBacktracker`

- <span id="boundedbacktracker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BoundedBacktracker`

- <span id="boundedbacktracker-clone"></span>`fn clone(&self) -> BoundedBacktracker` — [`BoundedBacktracker`](#boundedbacktracker)

##### `impl CloneToUninit for BoundedBacktracker`

- <span id="boundedbacktracker-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BoundedBacktracker`

- <span id="boundedbacktracker-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BoundedBacktracker`

- <span id="boundedbacktracker-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BoundedBacktracker`

- <span id="boundedbacktracker-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BoundedBacktracker`

- <span id="boundedbacktracker-toowned-type-owned"></span>`type Owned = T`

- <span id="boundedbacktracker-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="boundedbacktracker-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BoundedBacktracker`

- <span id="boundedbacktracker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boundedbacktracker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoundedBacktracker`

- <span id="boundedbacktracker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boundedbacktracker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryFindMatches<'r, 'c, 'h>`

```rust
struct TryFindMatches<'r, 'c, 'h> {
    re: &'r BoundedBacktracker,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1572-1577`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1572-L1577)*

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `BoundedBacktracker::try_find_iter`
method.

#### Trait Implementations

##### `impl Any for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tryfindmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tryfindmatches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-iterator-type-item"></span>`type Item = Result<Match, MatchError>`

- <span id="tryfindmatches-iterator-next"></span>`fn next(&mut self) -> Option<Result<Match, MatchError>>` — [`Match`](../../../index.md#match), [`MatchError`](../../../index.md#matcherror)

##### `impl<U> TryFrom for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfindmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfindmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryCapturesMatches<'r, 'c, 'h>`

```rust
struct TryCapturesMatches<'r, 'c, 'h> {
    re: &'r BoundedBacktracker,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1610-1615`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1610-L1615)*

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the
`BoundedBacktracker::try_captures_iter` method.

#### Trait Implementations

##### `impl Any for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="trycapturesmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="trycapturesmatches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-iterator-type-item"></span>`type Item = Result<Captures, MatchError>`

- <span id="trycapturesmatches-iterator-next"></span>`fn next(&mut self) -> Option<Result<Captures, MatchError>>` — [`Captures`](../../../util/captures/index.md#captures), [`MatchError`](../../../index.md#matcherror)

##### `impl<U> TryFrom for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="trycapturesmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="trycapturesmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cache`

```rust
struct Cache {
    stack: alloc::vec::Vec<Frame>,
    visited: Visited,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1653-1664`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1653-L1664)*

A cache represents mutable state that a [`BoundedBacktracker`](#boundedbacktracker) requires
during a search.

For a given [`BoundedBacktracker`](#boundedbacktracker), its corresponding cache may be created
either via `BoundedBacktracker::create_cache`, or via `Cache::new`.
They are equivalent in every way, except the former does not require
explicitly importing `Cache`.

A particular `Cache` is coupled with the [`BoundedBacktracker`](#boundedbacktracker) from which
it was created. It may only be used with that `BoundedBacktracker`. A cache
and its allocations may be re-purposed via `Cache::reset`, in which case,
it can only be used with the new `BoundedBacktracker` (and not the old
one).

#### Fields

- **`stack`**: `alloc::vec::Vec<Frame>`

  Stack used on the heap for doing backtracking instead of the
  traditional recursive approach. We don't want recursion because then
  we're likely to hit a stack overflow for bigger regexes.

- **`visited`**: `Visited`

  The set of (StateID, HaystackOffset) pairs that have been visited
  by the backtracker within a single search. If such a pair has been
  visited, then we avoid doing the work for that pair again. This is
  what "bounds" the backtracking and prevents it from having worst case
  exponential time.

#### Implementations

- <span id="cache-new"></span>`fn new(re: &BoundedBacktracker) -> Cache` — [`BoundedBacktracker`](#boundedbacktracker), [`Cache`](#cache)

  Create a new [`BoundedBacktracker`](#boundedbacktracker) cache.

  

  A potentially more convenient routine to create a cache is

  `BoundedBacktracker::create_cache`, as it does not require also

  importing the `Cache` type.

  

  If you want to reuse the returned `Cache` with some other

  `BoundedBacktracker`, then you must call `Cache::reset` with the

  desired `BoundedBacktracker`.

- <span id="cache-reset"></span>`fn reset(&mut self, re: &BoundedBacktracker)` — [`BoundedBacktracker`](#boundedbacktracker)

  Reset this cache such that it can be used for searching with different

  [`BoundedBacktracker`](#boundedbacktracker).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different `BoundedBacktracker`.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different

  `BoundedBacktracker`.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      nfa::thompson::backtrack::BoundedBacktracker,

      Match,

  };

  

  let re1 = BoundedBacktracker::new(r"\w")?;

  let re2 = BoundedBacktracker::new(r"\W")?;

  

  let mut cache = re1.create_cache();

  assert_eq!(

      Some(Ok(Match::must(0, 0..2))),

      re1.try_find_iter(&mut cache, "Δ").next(),

  );

  

  // Using 'cache' with re2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the BoundedBacktracker we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 're1' is also not

  // allowed.

  cache.reset(&re2);

  assert_eq!(

      Some(Ok(Match::must(0, 0..3))),

      re2.try_find_iter(&mut cache, "☃").next(),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, of this cache.

  

  This does **not** include the stack size used up by this cache. To

  compute that, use `std::mem::size_of::<Cache>()`.

- <span id="cache-setup-search"></span>`fn setup_search(&mut self, re: &BoundedBacktracker, input: &Input<'_>) -> Result<(), MatchError>` — [`BoundedBacktracker`](#boundedbacktracker), [`Input`](../../../index.md#input), [`MatchError`](../../../index.md#matcherror)

  Clears this cache. This should be called at the start of every search

  to ensure we start with a clean slate.

  

  This also sets the length of the capturing groups used in the current

  search. This permits an optimization where by 'SlotTable::for_state'

  only returns the number of slots equivalent to the number of slots

  given in the 'Captures' value. This may be less than the total number

  of possible slots, e.g., when one only wants to track overall match

  offsets. This in turn permits less copying of capturing group spans

  in the BoundedBacktracker.

#### Trait Implementations

##### `impl Any for Cache`

- <span id="cache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cache`

- <span id="cache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cache`

- <span id="cache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl CloneToUninit for Cache`

- <span id="cache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Cache`

- <span id="cache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Cache`

- <span id="cache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cache`

- <span id="cache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Cache`

- <span id="cache-toowned-type-owned"></span>`type Owned = T`

- <span id="cache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Cache`

- <span id="cache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cache`

- <span id="cache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Visited`

```rust
struct Visited {
    bitset: alloc::vec::Vec<usize>,
    stride: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1779-1801`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1779-L1801)*

A bitset that keeps track of whether a particular (StateID, offset) has
been considered during backtracking. If it has already been visited, then
backtracking skips it. This is what gives backtracking its "bound."

#### Fields

- **`bitset`**: `alloc::vec::Vec<usize>`

  The actual underlying bitset. Each element in the bitset corresponds
  to a particular (StateID, offset) pair. States correspond to the rows
  and the offsets correspond to the columns.
  
  If our underlying NFA has N states and the haystack we're searching
  has M bytes, then we have N*(M+1) entries in our bitset table. The
  M+1 occurs because our matches are delayed by one byte (to support
  look-around), and so we need to handle the end position itself rather
  than stopping just before the end. (If there is no end position, then
  it's treated as "end-of-input," which is matched by things like '$'.)
  
  Given BITS=N*(M+1), we wind up with div_ceil(BITS, sizeof(usize))
  blocks.
  
  We use 'usize' to represent our blocks because it makes some of the
  arithmetic in 'insert' a bit nicer. For example, if we used 'u32' for
  our block, we'd either need to cast u32s to usizes or usizes to u32s.

- **`stride`**: `usize`

  The stride represents one plus length of the haystack we're searching
  (as described above). The stride must be initialized for each search.

#### Implementations

- <span id="visited-const-block-size"></span>`const BLOCK_SIZE: usize`

- <span id="visited-new"></span>`fn new(re: &BoundedBacktracker) -> Visited` — [`BoundedBacktracker`](#boundedbacktracker), [`Visited`](#visited)

  Create a new visited set for the given backtracker.

  

  The set is ready to use, but must be setup at the beginning of each

  search by calling `setup_search`.

- <span id="visited-insert"></span>`fn insert(&mut self, sid: StateID, at: usize) -> bool` — [`StateID`](../../../util/primitives/index.md#stateid)

  Insert the given (StateID, offset) pair into this set. If it already

  exists, then this is a no-op and it returns false. Otherwise this

  returns true.

- <span id="visited-reset"></span>`fn reset(&mut self, _: &BoundedBacktracker)` — [`BoundedBacktracker`](#boundedbacktracker)

  Reset this visited set to work with the given bounded backtracker.

- <span id="visited-setup-search"></span>`fn setup_search(&mut self, re: &BoundedBacktracker, input: &Input<'_>) -> Result<(), MatchError>` — [`BoundedBacktracker`](#boundedbacktracker), [`Input`](../../../index.md#input), [`MatchError`](../../../index.md#matcherror)

  Setup this visited set to work for a search using the given NFA

  and input configuration. The NFA must be the same NFA used by the

  BoundedBacktracker given to Visited::reset. Failing to call this might

  result in panics or silently incorrect search behavior.

- <span id="visited-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Return the heap memory usage, in bytes, of this visited set.

#### Trait Implementations

##### `impl Any for Visited`

- <span id="visited-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Visited`

- <span id="visited-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Visited`

- <span id="visited-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Visited`

- <span id="visited-clone"></span>`fn clone(&self) -> Visited` — [`Visited`](#visited)

##### `impl CloneToUninit for Visited`

- <span id="visited-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Visited`

- <span id="visited-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Visited`

- <span id="visited-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Visited`

- <span id="visited-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Visited`

- <span id="visited-toowned-type-owned"></span>`type Owned = T`

- <span id="visited-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="visited-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Visited`

- <span id="visited-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="visited-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Visited`

- <span id="visited-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="visited-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Frame`

```rust
enum Frame {
    Step {
        sid: crate::util::primitives::StateID,
        at: usize,
    },
    RestoreCapture {
        slot: crate::util::primitives::SmallIndex,
        offset: Option<crate::util::primitives::NonMaxUsize>,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1761-1773`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1761-L1773)*

Represents a stack frame on the heap while doing backtracking.

Instead of using explicit recursion for backtracking, we use a stack on
the heap to keep track of things that we want to explore if the current
backtracking branch turns out to not lead to a match.

#### Variants

- **`Step`**

  Look for a match starting at `sid` and the given position in the
  haystack.

- **`RestoreCapture`**

  Reset the given `slot` to the given `offset` (which might be `None`).
  This effectively gives a "scope" to capturing groups, such that an
  offset for a particular group only gets returned if the match goes
  through that capturing group. If backtracking ends up going down a
  different branch that results in a different offset (or perhaps none at
  all), then this "restore capture" frame will cause the offset to get
  reset.

#### Trait Implementations

##### `impl Any for Frame`

- <span id="frame-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Frame`

- <span id="frame-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Frame`

- <span id="frame-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Frame`

- <span id="frame-clone"></span>`fn clone(&self) -> Frame` — [`Frame`](#frame)

##### `impl CloneToUninit for Frame`

- <span id="frame-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Frame`

- <span id="frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Frame`

- <span id="frame-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Frame`

- <span id="frame-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Frame`

- <span id="frame-toowned-type-owned"></span>`type Owned = T`

- <span id="frame-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="frame-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Frame`

- <span id="frame-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="frame-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Frame`

- <span id="frame-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="frame-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `min_visited_capacity`

```rust
fn min_visited_capacity(nfa: &crate::nfa::thompson::NFA, input: &crate::util::search::Input<'_>) -> usize
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:41-43`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L41-L43)*

Returns the minimum visited capacity for the given haystack.

This function can be used as the argument to `Config::visited_capacity`
in order to guarantee that a backtracking search for the given `input`
won't return an error when using a [`BoundedBacktracker`](#boundedbacktracker) built from the
given `NFA`.

This routine exists primarily as a way to test that the bounded backtracker
works correctly when its capacity is set to the smallest possible amount.
Still, it may be useful in cases where you know you want to use the bounded
backtracker for a specific input, and just need to know what visited
capacity to provide to make it work.

Be warned that this number could be quite large as it is multiplicative in
the size the given NFA and haystack.

### `div_ceil`

```rust
fn div_ceil(lhs: usize, rhs: usize) -> usize
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1881-1887`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1881-L1887)*

Integer division, but rounds up instead of down.

