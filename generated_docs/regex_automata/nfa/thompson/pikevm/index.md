*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [pikevm](index.md)*

---

# Module `pikevm`

An NFA backed Pike VM for executing regex searches with capturing groups.

This module provides a [`PikeVM`](#pikevm) that works by simulating an NFA and
resolving all spans of capturing groups that participate in a match.

## Contents

- [Structs](#structs)
  - [`Config`](#config)
  - [`Builder`](#builder)
  - [`PikeVM`](#pikevm)
  - [`FindMatches`](#findmatches)
  - [`CapturesMatches`](#capturesmatches)
  - [`Cache`](#cache)
  - [`ActiveStates`](#activestates)
  - [`SlotTable`](#slottable)
- [Enums](#enums)
  - [`FollowEpsilon`](#followepsilon)
- [Macros](#macros)
  - [`instrument!`](#instrument)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration used for building a [`PikeVM`]. |
| [`Builder`](#builder) | struct | A builder for a `PikeVM`. |
| [`PikeVM`](#pikevm) | struct | A virtual machine for executing regex searches with capturing groups. |
| [`FindMatches`](#findmatches) | struct | An iterator over all non-overlapping matches for a particular search. |
| [`CapturesMatches`](#capturesmatches) | struct | An iterator over all non-overlapping leftmost matches, with their capturing groups, for a particular search. |
| [`Cache`](#cache) | struct | A cache represents mutable state that a [`PikeVM`] requires during a search. |
| [`ActiveStates`](#activestates) | struct | A set of active states used to "simulate" the execution of an NFA via the PikeVM. |
| [`SlotTable`](#slottable) | struct | A table of slots, where each row represent a state in an NFA. |
| [`FollowEpsilon`](#followepsilon) | enum | Represents a stack frame for use while computing an epsilon closure. |
| [`instrument!`](#instrument) | macro | A simple macro for conditionally executing instrumentation logic when the 'trace' log level is enabled. |

## Structs

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:66-69`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L66-L69)*

The configuration used for building a [`PikeVM`](#pikevm).

A PikeVM configuration is a simple data object that is typically used with
`Builder::configure`. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with `PikeVM::config`.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

  Return a new default PikeVM configuration.

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../../index.md#matchkind), [`Config`](#config)

  Set the desired match semantics.

  

  The default is [`MatchKind::LeftmostFirst`](../../../index.md), which corresponds to the

  match semantics of Perl-like regex engines. That is, when multiple

  patterns would match at the same leftmost position, the pattern that

  appears first in the concrete syntax is chosen.

  

  Currently, the only other kind of match semantics supported is

  [`MatchKind::All`](../../../index.md). This corresponds to "classical DFA" construction

  where all possible matches are visited in the NFA by the `PikeVM`.

  

  Typically, `All` is used when one wants to execute an overlapping

  search and `LeftmostFirst` otherwise. In particular, it rarely makes

  sense to use `All` with the various "leftmost" find routines, since the

  leftmost routines depend on the `LeftmostFirst` automata construction

  strategy. Specifically, `LeftmostFirst` results in the `PikeVM`

  simulating dead states as a way to terminate the search and report a

  match. `LeftmostFirst` also supports non-greedy matches using this

  strategy where as `All` does not.

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

      nfa::thompson::pikevm::PikeVM,

      util::prefilter::Prefilter,

      Input, Match, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["foo", "bar"]);

  let re = PikeVM::builder()

      .configure(PikeVM::config().prefilter(pre))

      .build(r"(foo|bar)[a-z]+")?;

  let mut cache = re.create_cache();

  let input = Input::new("foo1 barfox bar");

  assert_eq!(Some(Match::must(0, 5..11)), re.find(&mut cache, input));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  Be warned though that an incorrect prefilter can lead to incorrect

  results!

  

  ```rust

  use regex_automata::{

      nfa::thompson::pikevm::PikeVM,

      util::prefilter::Prefilter,

      Input, HalfMatch, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["foo", "car"]);

  let re = PikeVM::builder()

      .configure(PikeVM::config().prefilter(pre))

      .build(r"(foo|bar)[a-z]+")?;

  let mut cache = re.create_cache();

  let input = Input::new("foo1 barfox bar");

  // No match reported even though there clearly is one!

  assert_eq!(None, re.find(&mut cache, input));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../../index.md#matchkind)

  Returns the match semantics set in this configuration.

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../../util/prefilter/index.md#prefilter)

  Returns the prefilter set in this configuration, if one at all.

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:239-243`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L239-L243)*

A builder for a `PikeVM`.

This builder permits configuring options for the syntax of a pattern,
the NFA construction and the `PikeVM` construction. This builder is
different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`util::syntax::Config::utf8`](crate::util::syntax::Config::utf8)
controls whether the pattern itself can contain sub-expressions that match
invalid UTF-8.
* `thompson::Config::utf8` controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```rust
use regex_automata::{
    nfa::thompson::{self, pikevm::PikeVM},
    util::syntax,
    Match,
};

let re = PikeVM::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a PikeVM Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new PikeVM builder with its default configuration.

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Build a `PikeVM` from the given pattern.

  

  If there was a problem parsing or compiling the pattern, then an error

  is returned.

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Build a `PikeVM` from the given patterns.

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: NFA) -> Result<PikeVM, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Build a `PikeVM` directly from its NFA.

  

  Note that when using this method, any configuration that applies to the

  construction of the NFA itself will of course be ignored, since the NFA

  given here is already built.

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

  Apply the given `PikeVM` configuration options to this builder.

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../../util/syntax/index.md#config), [`Builder`](#builder)

  Set the syntax configuration for this builder using

  [`syntax::Config`](crate::util::syntax::Config).

  

  This permits setting things like case insensitivity, Unicode and multi

  line mode.

  

  These settings only apply when constructing a PikeVM directly from a

  pattern.

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../compiler/index.md#config), [`Builder`](#builder)

  Set the Thompson NFA configuration for this builder using

  [`nfa::thompson::Config`](crate::nfa::thompson::Config).

  

  This permits setting things like if additional time should be spent

  shrinking the size of the NFA.

  

  These settings only apply when constructing a PikeVM directly from a

  pattern.

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

### `PikeVM`

```rust
struct PikeVM {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:387-390`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L387-L390)*

A virtual machine for executing regex searches with capturing groups.

# Infallible APIs

Unlike most other regex engines in this crate, a `PikeVM` never returns an
error at search time. It supports all [`Anchored`](../../../index.md) configurations, never
quits and works on haystacks of arbitrary length.

There are two caveats to mention though:

* If an invalid pattern ID is given to a search via [`Anchored::Pattern`](../../../index.md),
then the PikeVM will report "no match." This is consistent with all other
regex engines in this crate.
* When using `PikeVM::which_overlapping_matches` with a [`PatternSet`](../../../index.md)
that has insufficient capacity to store all valid pattern IDs, then if a
match occurs for a `PatternID` that cannot be inserted, it is silently
dropped as if it did not match.

# Advice

The `PikeVM` is generally the most "powerful" regex engine in this crate.
"Powerful" in this context means that it can handle any regular expression
that is parseable by `regex-syntax` and any size haystack. Regrettably,
the `PikeVM` is also simultaneously often the _slowest_ regex engine in
practice. This results in an annoying situation where one generally tries
to pick any other regex engine (or perhaps none at all) before being
forced to fall back to a `PikeVM`.

For example, a common strategy for dealing with capturing groups is to
actually look for the overall match of the regex using a faster regex
engine, like a [lazy DFA](crate::hybrid::regex::Regex). Once the overall
match is found, one can then run the `PikeVM` on just the match span to
find the spans of the capturing groups. In this way, the faster regex
engine does the majority of the work, while the `PikeVM` only lends its
power in a more limited role.

Unfortunately, this isn't always possible because the faster regex engines
don't support all of the regex features in `regex-syntax`. This notably
includes (and is currently limited to) Unicode word boundaries. So if
your pattern has Unicode word boundaries, you typically can't use a
DFA-based regex engine at all (unless you [enable heuristic support for
it](crate::hybrid::dfa::Config::unicode_word_boundary)). (The [one-pass
DFA](crate::dfa::onepass::DFA) can handle Unicode word boundaries for
anchored searches only, but in a cruel sort of joke, many Unicode features
tend to result in making the regex _not_ one-pass.)

# Example

This example shows that the `PikeVM` implements Unicode word boundaries
correctly by default.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Match::must(0, 0..12)), it.next());
assert_eq!(Some(Match::must(0, 13..23)), it.next());
assert_eq!(None, it.next());
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="pikevm-new"></span>`fn new(pattern: &str) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Parse the given regular expression using the default configuration and

  return the corresponding `PikeVM`.

  

  If you want a non-default configuration, then use the [`Builder`](#builder) to

  set your own configuration.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re = PikeVM::new("foo[0-9]+bar")?;

  let mut cache = re.create_cache();

  assert_eq!(

      Some(Match::must(0, 3..14)),

      re.find_iter(&mut cache, "zzzfoo12345barzzz").next(),

  );

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Like `new`, but parses multiple patterns into a single "multi regex."

  This similarly uses the default regex configuration.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re = PikeVM::new_many(&["[a-z]+", "[0-9]+"])?;

  let mut cache = re.create_cache();

  

  let mut it = re.find_iter(&mut cache, "abc 1 foo 4567 0 quux");

  assert_eq!(Some(Match::must(0, 0..3)), it.next());

  assert_eq!(Some(Match::must(1, 4..5)), it.next());

  assert_eq!(Some(Match::must(0, 6..9)), it.next());

  assert_eq!(Some(Match::must(1, 10..14)), it.next());

  assert_eq!(Some(Match::must(1, 15..16)), it.next());

  assert_eq!(Some(Match::must(0, 17..21)), it.next());

  assert_eq!(None, it.next());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-new-from-nfa"></span>`fn new_from_nfa(nfa: NFA) -> Result<PikeVM, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Like `new`, but builds a PikeVM directly from an NFA. This is useful

  if you already have an NFA, or even if you hand-assembled the NFA.

  

  # Example

  

  This shows how to hand assemble a regular expression via its HIR,

  compile an NFA from it and build a PikeVM from the NFA.

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

  

  let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![

      ClassBytesRange::new(b'0', b'9'),

      ClassBytesRange::new(b'A', b'Z'),

      ClassBytesRange::new(b'_', b'_'),

      ClassBytesRange::new(b'a', b'z'),

  ])));

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  let expected = Some(Match::must(0, 3..4));

  re.captures(&mut cache, "!@#A#@!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-always-match"></span>`fn always_match() -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Create a new `PikeVM` that matches every input.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re = PikeVM::always_match()?;

  let mut cache = re.create_cache();

  

  let expected = Match::must(0, 0..0);

  assert_eq!(Some(expected), re.find_iter(&mut cache, "").next());

  assert_eq!(Some(expected), re.find_iter(&mut cache, "foo").next());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-never-match"></span>`fn never_match() -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

  Create a new `PikeVM` that never matches any input.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::pikevm::PikeVM;

  

  let re = PikeVM::never_match()?;

  let mut cache = re.create_cache();

  

  assert_eq!(None, re.find_iter(&mut cache, "").next());

  assert_eq!(None, re.find_iter(&mut cache, "foo").next());

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-config"></span>`fn config() -> Config` — [`Config`](#config)

  Return a default configuration for a `PikeVM`.

  

  This is a convenience routine to avoid needing to import the `Config`

  type when customizing the construction of a `PikeVM`.

  

  # Example

  

  This example shows how to disable UTF-8 mode. When UTF-8 mode is

  disabled, zero-width matches that split a codepoint are allowed.

  Otherwise they are never reported.

  

  In the code below, notice that `""` is permitted to match positions

  that split the encoding of a codepoint.

  

  ```rust

  use regex_automata::{nfa::thompson::{self, pikevm::PikeVM}, Match};

  

  let re = PikeVM::builder()

      .thompson(thompson::Config::new().utf8(false))

      .build(r"")?;

  let mut cache = re.create_cache();

  

  let haystack = "a☃z";

  let mut it = re.find_iter(&mut cache, haystack);

  assert_eq!(Some(Match::must(0, 0..0)), it.next());

  assert_eq!(Some(Match::must(0, 1..1)), it.next());

  assert_eq!(Some(Match::must(0, 2..2)), it.next());

  assert_eq!(Some(Match::must(0, 3..3)), it.next());

  assert_eq!(Some(Match::must(0, 4..4)), it.next());

  assert_eq!(Some(Match::must(0, 5..5)), it.next());

  assert_eq!(None, it.next());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  Return a builder for configuring the construction of a `PikeVM`.

  

  This is a convenience routine to avoid needing to import the

  [`Builder`](#builder) type in common cases.

  

  # Example

  

  This example shows how to use the builder to disable UTF-8 mode

  everywhere.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, pikevm::PikeVM},

      util::syntax,

      Match,

  };

  

  let re = PikeVM::builder()

      .syntax(syntax::Config::new().utf8(false))

      .thompson(thompson::Config::new().utf8(false))

      .build(r"foo(?-u:[^b])ar.*")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";

  let expected = Some(Match::must(0, 1..9));

  re.captures(&mut cache, haystack, &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-create-captures"></span>`fn create_captures(&self) -> Captures` — [`Captures`](../../../util/captures/index.md#captures)

  Create a new empty set of capturing groups that is guaranteed to be

  valid for the search APIs on this `PikeVM`.

  

  A `Captures` value created for a specific `PikeVM` cannot be used with

  any other `PikeVM`.

  

  This is a convenience function for `Captures::all`. See the

  [`Captures`](../../../util/captures/index.md) documentation for an explanation of its alternative

  constructors that permit the `PikeVM` to do less work during a search,

  and thus might make it faster.

- <span id="pikevm-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](#cache)

  Create a new cache for this `PikeVM`.

  

  The cache returned should only be used for searches for this

  `PikeVM`. If you want to reuse the cache for another `PikeVM`, then

  you must call `Cache::reset` with that `PikeVM` (or, equivalently,

  `PikeVM::reset_cache`).

- <span id="pikevm-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](#cache)

  Reset the given cache such that it can be used for searching with the

  this `PikeVM` (and only this `PikeVM`).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different `PikeVM`.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different `PikeVM`.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re1 = PikeVM::new(r"\w")?;

  let re2 = PikeVM::new(r"\W")?;

  

  let mut cache = re1.create_cache();

  assert_eq!(

      Some(Match::must(0, 0..2)),

      re1.find_iter(&mut cache, "Δ").next(),

  );

  

  // Using 'cache' with re2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the PikeVM we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 're1' is also not

  // allowed.

  re2.reset_cache(&mut cache);

  assert_eq!(

      Some(Match::must(0, 0..3)),

      re2.find_iter(&mut cache, "☃").next(),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the total number of patterns compiled into this `PikeVM`.

  

  In the case of a `PikeVM` that contains no patterns, this returns `0`.

  

  # Example

  

  This example shows the pattern length for a `PikeVM` that never

  matches:

  

  ```rust

  use regex_automata::nfa::thompson::pikevm::PikeVM;

  

  let re = PikeVM::never_match()?;

  assert_eq!(re.pattern_len(), 0);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  And another example for a `PikeVM` that matches at every position:

  

  ```rust

  use regex_automata::nfa::thompson::pikevm::PikeVM;

  

  let re = PikeVM::always_match()?;

  assert_eq!(re.pattern_len(), 1);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  And finally, a `PikeVM` that was constructed from multiple patterns:

  

  ```rust

  use regex_automata::nfa::thompson::pikevm::PikeVM;

  

  let re = PikeVM::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"])?;

  assert_eq!(re.pattern_len(), 3);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="pikevm-get-config"></span>`fn get_config(&self) -> &Config` — [`Config`](#config)

  Return the config for this `PikeVM`.

- <span id="pikevm-get-nfa"></span>`fn get_nfa(&self) -> &NFA` — [`NFA`](../nfa/index.md#nfa)

  Returns a reference to the underlying NFA.

#### Trait Implementations

##### `impl Any for PikeVM`

- <span id="pikevm-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PikeVM`

- <span id="pikevm-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PikeVM`

- <span id="pikevm-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PikeVM`

- <span id="pikevm-clone"></span>`fn clone(&self) -> PikeVM` — [`PikeVM`](#pikevm)

##### `impl CloneToUninit for PikeVM`

- <span id="pikevm-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PikeVM`

- <span id="pikevm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PikeVM`

- <span id="pikevm-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PikeVM`

- <span id="pikevm-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PikeVM`

- <span id="pikevm-toowned-type-owned"></span>`type Owned = T`

- <span id="pikevm-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pikevm-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PikeVM`

- <span id="pikevm-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pikevm-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PikeVM`

- <span id="pikevm-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pikevm-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FindMatches<'r, 'c, 'h>`

```rust
struct FindMatches<'r, 'c, 'h> {
    re: &'r PikeVM,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:1799-1804`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L1799-L1804)*

An iterator over all non-overlapping matches for a particular search.

The iterator yields a [`Match`](../../../index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `PikeVM::find_iter` method.

#### Trait Implementations

##### `impl Any for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="findmatches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-iterator-type-item"></span>`type Item = Match`

- <span id="findmatches-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../../index.md#match)

##### `impl<U> TryFrom for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CapturesMatches<'r, 'c, 'h>`

```rust
struct CapturesMatches<'r, 'c, 'h> {
    re: &'r PikeVM,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:1837-1842`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L1837-L1842)*

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a particular search.

The iterator yields a [`Captures`](../../../util/captures/index.md) value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `PikeVM::captures_iter` method.

#### Trait Implementations

##### `impl Any for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturesmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturesmatches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-iterator-type-item"></span>`type Item = Captures`

- <span id="capturesmatches-iterator-next"></span>`fn next(&mut self) -> Option<Captures>` — [`Captures`](../../../util/captures/index.md#captures)

##### `impl<U> TryFrom for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturesmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturesmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Cache`

```rust
struct Cache {
    stack: alloc::vec::Vec<FollowEpsilon>,
    curr: ActiveStates,
    next: ActiveStates,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:1878-1889`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L1878-L1889)*

A cache represents mutable state that a [`PikeVM`](#pikevm) requires during a
search.

For a given [`PikeVM`](#pikevm), its corresponding cache may be created either via
`PikeVM::create_cache`, or via `Cache::new`. They are equivalent in
every way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the [`PikeVM`](#pikevm) from which it
was created. It may only be used with that `PikeVM`. A cache and its
allocations may be re-purposed via `Cache::reset`, in which case, it can
only be used with the new `PikeVM` (and not the old one).

#### Fields

- **`stack`**: `alloc::vec::Vec<FollowEpsilon>`

  Stack used while computing epsilon closure. This effectively lets us
  move what is more naturally expressed through recursion to a stack
  on the heap.

- **`curr`**: `ActiveStates`

  The current active states being explored for the current byte in the
  haystack.

- **`next`**: `ActiveStates`

  The next set of states we're building that will be explored for the
  next byte in the haystack.

#### Implementations

- <span id="cache-new"></span>`fn new(re: &PikeVM) -> Cache` — [`PikeVM`](#pikevm), [`Cache`](#cache)

  Create a new [`PikeVM`](#pikevm) cache.

  

  A potentially more convenient routine to create a cache is

  `PikeVM::create_cache`, as it does not require also importing the

  `Cache` type.

  

  If you want to reuse the returned `Cache` with some other `PikeVM`,

  then you must call `Cache::reset` with the desired `PikeVM`.

- <span id="cache-reset"></span>`fn reset(&mut self, re: &PikeVM)` — [`PikeVM`](#pikevm)

  Reset this cache such that it can be used for searching with a

  different [`PikeVM`](#pikevm).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different `PikeVM`.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different `PikeVM`.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re1 = PikeVM::new(r"\w")?;

  let re2 = PikeVM::new(r"\W")?;

  

  let mut cache = re1.create_cache();

  assert_eq!(

      Some(Match::must(0, 0..2)),

      re1.find_iter(&mut cache, "Δ").next(),

  );

  

  // Using 'cache' with re2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the PikeVM we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 're1' is also not

  // allowed.

  cache.reset(&re2);

  assert_eq!(

      Some(Match::must(0, 0..3)),

      re2.find_iter(&mut cache, "☃").next(),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, of this cache.

  

  This does **not** include the stack size used up by this cache. To

  compute that, use `std::mem::size_of::<Cache>()`.

- <span id="cache-setup-search"></span>`fn setup_search(&mut self, captures_slot_len: usize)`

  Clears this cache. This should be called at the start of every search

  to ensure we start with a clean slate.

  

  This also sets the length of the capturing groups used in the current

  search. This permits an optimization where by 'SlotTable::for_state'

  only returns the number of slots equivalent to the number of slots

  given in the 'Captures' value. This may be less than the total number

  of possible slots, e.g., when one only wants to track overall match

  offsets. This in turn permits less copying of capturing group spans

  in the PikeVM.

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

### `ActiveStates`

```rust
struct ActiveStates {
    set: crate::util::sparse_set::SparseSet,
    slot_table: SlotTable,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:1996-2005`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L1996-L2005)*

A set of active states used to "simulate" the execution of an NFA via the
PikeVM.

There are two sets of these used during NFA simulation. One set corresponds
to the "current" set of states being traversed for the current position
in a haystack. The other set corresponds to the "next" set of states being
built, which will become the new "current" set for the next position in the
haystack. These two sets correspond to CLIST and NLIST in Thompson's
original paper regexes: https://dl.acm.org/doi/pdf/10.1145/363347.363387

In addition to representing a set of NFA states, this also maintains slot
values for each state. These slot values are what turn the NFA simulation
into the "Pike VM." Namely, they track capturing group values for each
state. During the computation of epsilon closure, we copy slot values from
states in the "current" set to the "next" set. Eventually, once a match
is found, the slot values for that match state are what we write to the
caller provided 'Captures' value.

#### Fields

- **`set`**: `crate::util::sparse_set::SparseSet`

  The set of active NFA states. This set preserves insertion order, which
  is critical for simulating the match semantics of backtracking regex
  engines.

- **`slot_table`**: `SlotTable`

  The slots for every NFA state, where each slot stores a (possibly
  absent) offset. Every capturing group has two slots. One for a start
  offset and one for an end offset.

#### Implementations

- <span id="activestates-new"></span>`fn new(re: &PikeVM) -> ActiveStates` — [`PikeVM`](#pikevm), [`ActiveStates`](#activestates)

  Create a new set of active states for the given PikeVM. The active

  states returned may only be used with the given PikeVM. (Use 'reset'

  to re-purpose the allocation for a different PikeVM.)

- <span id="activestates-reset"></span>`fn reset(&mut self, re: &PikeVM)` — [`PikeVM`](#pikevm)

  Reset this set of active states such that it can be used with the given

  PikeVM (and only that PikeVM).

- <span id="activestates-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Return the heap memory usage, in bytes, used by this set of active

  states.

  

  This does not include the stack size of this value.

- <span id="activestates-setup-search"></span>`fn setup_search(&mut self, captures_slot_len: usize)`

  Setup this set of active states for a new search. The given slot

  length should be the number of slots in a caller provided 'Captures'

  (and may be zero).

#### Trait Implementations

##### `impl Any for ActiveStates`

- <span id="activestates-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ActiveStates`

- <span id="activestates-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ActiveStates`

- <span id="activestates-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ActiveStates`

- <span id="activestates-clone"></span>`fn clone(&self) -> ActiveStates` — [`ActiveStates`](#activestates)

##### `impl CloneToUninit for ActiveStates`

- <span id="activestates-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ActiveStates`

- <span id="activestates-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ActiveStates`

- <span id="activestates-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ActiveStates`

- <span id="activestates-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ActiveStates`

- <span id="activestates-toowned-type-owned"></span>`type Owned = T`

- <span id="activestates-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="activestates-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ActiveStates`

- <span id="activestates-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="activestates-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ActiveStates`

- <span id="activestates-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="activestates-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SlotTable`

```rust
struct SlotTable {
    table: alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>,
    slots_per_state: usize,
    slots_for_captures: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:2065-2075`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L2065-L2075)*

A table of slots, where each row represent a state in an NFA. Thus, the
table has room for storing slots for every single state in an NFA.

This table is represented with a single contiguous allocation. In general,
the notion of "capturing group" doesn't really exist at this level of
abstraction, hence the name "slot" instead. (Indeed, every capturing group
maps to a pair of slots, one for the start offset and one for the end
offset.) Slots are indexed by the 'Captures' NFA state.

N.B. Not every state actually needs a row of slots. Namely, states that
only have epsilon transitions currently never have anything written to
their rows in this table. Thus, the table is somewhat wasteful in its heap
usage. However, it is important to maintain fast random access by state
ID, which means one giant table tends to work well. RE2 takes a different
approach here and allocates each row as its own reference counted thing.
I explored such a strategy at one point here, but couldn't get it to work
well using entirely safe code. (To the ambitious reader: I encourage you to
re-litigate that experiment.) I very much wanted to stick to safe code, but
could be convinced otherwise if there was a solid argument and the safety
was encapsulated well.

#### Fields

- **`table`**: `alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>`

  The actual table of offsets.

- **`slots_per_state`**: `usize`

  The number of slots per state, i.e., the table's stride or the length
  of each row.

- **`slots_for_captures`**: `usize`

  The number of slots in the caller-provided 'Captures' value for the
  current search. Setting this to 'slots_per_state' is always correct,
  but may be wasteful.

#### Implementations

- <span id="slottable-new"></span>`fn new() -> SlotTable` — [`SlotTable`](#slottable)

  Create a new slot table.

  

  One should call 'reset' with the corresponding PikeVM before use.

- <span id="slottable-reset"></span>`fn reset(&mut self, re: &PikeVM)` — [`PikeVM`](#pikevm)

  Reset this slot table such that it can be used with the given PikeVM

  (and only that PikeVM).

- <span id="slottable-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Return the heap memory usage, in bytes, used by this slot table.

  

  This does not include the stack size of this value.

- <span id="slottable-setup-search"></span>`fn setup_search(&mut self, captures_slot_len: usize)`

  Perform any per-search setup for this slot table.

  

  In particular, this sets the length of the number of slots used in the

  'Captures' given by the caller (if any at all). This number may be

  smaller than the total number of slots available, e.g., when the caller

  is only interested in tracking the overall match and not the spans of

  every matching capturing group. Only tracking the overall match can

  save a substantial amount of time copying capturing spans during a

  search.

- <span id="slottable-for-state"></span>`fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>]` — [`StateID`](../../../util/primitives/index.md#stateid), [`NonMaxUsize`](../../../util/primitives/index.md#nonmaxusize)

  Return a mutable slice of the slots for the given state.

  

  Note that the length of the slice returned may be less than the total

  number of slots available for this state. In particular, the length

  always matches the number of slots indicated via 'setup_search'.

- <span id="slottable-all-absent"></span>`fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>]` — [`NonMaxUsize`](../../../util/primitives/index.md#nonmaxusize)

  Return a slice of slots of appropriate length where every slot offset

  is guaranteed to be absent. This is useful in cases where you need to

  compute an epsilon closure outside of the user supplied regex, and thus

  never want it to have any capturing slots set.

#### Trait Implementations

##### `impl Any for SlotTable`

- <span id="slottable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SlotTable`

- <span id="slottable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SlotTable`

- <span id="slottable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SlotTable`

- <span id="slottable-clone"></span>`fn clone(&self) -> SlotTable` — [`SlotTable`](#slottable)

##### `impl CloneToUninit for SlotTable`

- <span id="slottable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SlotTable`

- <span id="slottable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SlotTable`

- <span id="slottable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SlotTable`

- <span id="slottable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SlotTable`

- <span id="slottable-toowned-type-owned"></span>`type Owned = T`

- <span id="slottable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slottable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SlotTable`

- <span id="slottable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slottable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SlotTable`

- <span id="slottable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slottable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `FollowEpsilon`

```rust
enum FollowEpsilon {
    Explore(crate::util::primitives::StateID),
    RestoreCapture {
        slot: crate::util::primitives::SmallIndex,
        offset: Option<crate::util::primitives::NonMaxUsize>,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:2199-2204`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L2199-L2204)*

Represents a stack frame for use while computing an epsilon closure.

(An "epsilon closure" refers to the set of reachable NFA states from a
single state without consuming any input. That is, the set of all epsilon
transitions not only from that single state, but from every other state
reachable by an epsilon transition as well. This is why it's called a
"closure." Computing an epsilon closure is also done during DFA
determinization! Compare and contrast the epsilon closure here in this
PikeVM and the one used for determinization in crate::util::determinize.)

Computing the epsilon closure in a Thompson NFA proceeds via a depth
first traversal over all epsilon transitions from a particular state.
(A depth first traversal is important because it emulates the same priority
of matches that is typically found in backtracking regex engines.) This
depth first traversal is naturally expressed using recursion, but to avoid
a call stack size proportional to the size of a regex, we put our stack on
the heap instead.

This stack thus consists of call frames. The typical call frame is
`Explore`, which instructs epsilon closure to explore the epsilon
transitions from that state. (Subsequent epsilon transitions are then
pushed on to the stack as more `Explore` frames.) If the state ID being
explored has no epsilon transitions, then the capturing group slots are
copied from the original state that sparked the epsilon closure (from the
'step' routine) to the state ID being explored. This way, capturing group
slots are forwarded from the previous state to the next.

The other stack frame, `RestoreCaptures`, instructs the epsilon closure to
set the position for a particular slot back to some particular offset. This
frame is pushed when `Explore` sees a `Capture` transition. `Explore` will
set the offset of the slot indicated in `Capture` to the current offset,
and then push the old offset on to the stack as a `RestoreCapture` frame.
Thus, the new offset is only used until the epsilon closure reverts back to
the `RestoreCapture` frame. In effect, this gives the `Capture` epsilon
transition its "scope" to only states that come "after" it during depth
first traversal.

#### Variants

- **`Explore`**

  Explore the epsilon transitions from a state ID.

- **`RestoreCapture`**

  Reset the given `slot` to the given `offset` (which might be `None`).

#### Trait Implementations

##### `impl Any for FollowEpsilon`

- <span id="followepsilon-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FollowEpsilon`

- <span id="followepsilon-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FollowEpsilon`

- <span id="followepsilon-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FollowEpsilon`

- <span id="followepsilon-clone"></span>`fn clone(&self) -> FollowEpsilon` — [`FollowEpsilon`](#followepsilon)

##### `impl CloneToUninit for FollowEpsilon`

- <span id="followepsilon-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FollowEpsilon`

- <span id="followepsilon-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FollowEpsilon`

- <span id="followepsilon-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FollowEpsilon`

- <span id="followepsilon-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for FollowEpsilon`

- <span id="followepsilon-toowned-type-owned"></span>`type Owned = T`

- <span id="followepsilon-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="followepsilon-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FollowEpsilon`

- <span id="followepsilon-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="followepsilon-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FollowEpsilon`

- <span id="followepsilon-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="followepsilon-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Macros

### `instrument!`

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/pikevm.rs:36-44`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/pikevm.rs#L36-L44)*

A simple macro for conditionally executing instrumentation logic when
the 'trace' log level is enabled. This is a compile-time no-op when the
'internal-instrument-pikevm' feature isn't enabled. The intent here is that
this makes it easier to avoid doing extra work when instrumentation isn't
enabled.

This macro accepts a closure of type `|&mut Counters|`. The closure can
then increment counters (or whatever) in accordance with what one wants
to track.

