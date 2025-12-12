*[regex_automata](../index.md) / [meta](index.md)*

---

# Module `meta`

Provides a regex matcher that composes several other regex matchers
automatically.

This module is home to a meta [`Regex`](regex/index.md), which provides a convenient high
level API for executing regular expressions in linear time.

# Comparison with the `regex` crate

A meta `Regex` is the implementation used directly by the `regex` crate.
Indeed, the `regex` crate API is essentially just a light wrapper over a meta
`Regex`. This means that if you need the full flexibility offered by this
API, then you should be able to switch to using this API directly without
any changes in match semantics or syntax. However, there are some API level
differences:

* The `regex` crate API returns match objects that include references to the
haystack itself, which in turn makes it easy to access the matching strings
without having to slice the haystack yourself. In contrast, a meta `Regex`
returns match objects that only have offsets in them.
* At time of writing, a meta `Regex` doesn't have some of the convenience
routines that the `regex` crate has, such as replacements. Note though that
[`Captures::interpolate_string`](crate::util::captures::Captures::interpolate_string)
will handle the replacement string interpolation for you.
* A meta `Regex` supports the [`Input`](crate::Input) abstraction, which
provides a way to configure a search in more ways than is supported by the
`regex` crate. For example, [`Input::anchored`](crate::Input::anchored) can
be used to run an anchored search, regardless of whether the pattern is itself
anchored with a `^`.
* A meta `Regex` supports multi-pattern searching everywhere.
Indeed, every [`Match`](crate::Match) returned by the search APIs
include a [`PatternID`](crate::PatternID) indicating which pattern
matched. In the single pattern case, all matches correspond to
[`PatternID::ZERO`](crate::PatternID::ZERO). In contrast, the `regex` crate
has distinct `Regex` and a `RegexSet` APIs. The former only supports a single
pattern, while the latter supports multiple patterns but cannot report the
offsets of a match.
* A meta `Regex` provides the explicit capability of bypassing its internal
memory pool for automatically acquiring mutable scratch space required by its
internal regex engines. Namely, a [`Cache`](regex/index.md) can be explicitly provided to lower
level routines such as `Regex::search_with`.

## Contents

- [Modules](#modules)
  - [`error`](#error)
  - [`limited`](#limited)
  - [`literal`](#literal)
  - [`regex`](#regex)
  - [`reverse_inner`](#reverse-inner)
  - [`stopat`](#stopat)
  - [`strategy`](#strategy)
  - [`wrappers`](#wrappers)
- [Structs](#structs)
  - [`BuildError`](#builderror)
  - [`Builder`](#builder)
  - [`Cache`](#cache)
  - [`CapturesMatches`](#capturesmatches)
  - [`Config`](#config)
  - [`FindMatches`](#findmatches)
  - [`Regex`](#regex)
  - [`Split`](#split)
  - [`SplitN`](#splitn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`error`](#error) | mod |  |
| [`limited`](#limited) | mod | This module defines two bespoke reverse DFA searching routines. |
| [`literal`](#literal) | mod |  |
| [`regex`](#regex) | mod |  |
| [`reverse_inner`](#reverse-inner) | mod | A module dedicated to plucking inner literals out of a regex pattern, and then constructing a prefilter for them. |
| [`stopat`](#stopat) | mod | This module defines two bespoke forward DFA search routines. |
| [`strategy`](#strategy) | mod |  |
| [`wrappers`](#wrappers) | mod | This module contains a boat load of wrappers around each of our internal regex engines. |
| [`BuildError`](#builderror) | struct |  |
| [`Builder`](#builder) | struct |  |
| [`Cache`](#cache) | struct |  |
| [`CapturesMatches`](#capturesmatches) | struct |  |
| [`Config`](#config) | struct |  |
| [`FindMatches`](#findmatches) | struct |  |
| [`Regex`](#regex) | struct |  |
| [`Split`](#split) | struct |  |
| [`SplitN`](#splitn) | struct |  |

## Modules

- [`error`](error/index.md)
- [`limited`](limited/index.md) — This module defines two bespoke reverse DFA searching routines. (One for the
- [`literal`](literal/index.md)
- [`regex`](regex/index.md)
- [`reverse_inner`](reverse_inner/index.md) — A module dedicated to plucking inner literals out of a regex pattern, and
- [`stopat`](stopat/index.md) — This module defines two bespoke forward DFA search routines. One for the lazy
- [`strategy`](strategy/index.md)
- [`wrappers`](wrappers/index.md) — This module contains a boat load of wrappers around each of our internal regex

## Structs

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/error.rs:27-29`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/error.rs#L27-L29)*

An error that occurs when construction of a `Regex` fails.

A build error is generally a result of one of two possible failure
modes. First is a parse or syntax error in the concrete syntax of a
pattern. Second is that the construction of the underlying regex matcher
fails, usually because it gets too big with respect to limits like
[`Config::nfa_size_limit`](crate::meta::Config::nfa_size_limit).

This error provides very little introspection capabilities. You can:

* Ask for the [`PatternID`](../util/primitives/index.md) of the pattern that caused an error, if one
is available. This is available for things like syntax errors, but not for
cases where build limits are exceeded.
* Ask for the underlying syntax error, but only if the error is a syntax
error.
* Ask for a human readable message corresponding to the underlying error.
* The `BuildError::source` method (from the `std::error::Error`
trait implementation) may be used to query for an underlying error if one
exists. There are no API guarantees about which error is returned.

When the `std` feature is enabled, this implements `std::error::Error`.

#### Implementations

- <span id="builderror-pattern"></span>`fn pattern(&self) -> Option<PatternID>` — [`PatternID`](../util/primitives/index.md#patternid)

- <span id="builderror-size-limit"></span>`fn size_limit(&self) -> Option<usize>`

- <span id="builderror-syntax-error"></span>`fn syntax_error(&self) -> Option<&regex_syntax::Error>`

- <span id="builderror-ast"></span>`fn ast(pid: PatternID, err: ast::Error) -> BuildError` — [`PatternID`](../util/primitives/index.md#patternid), [`BuildError`](error/index.md#builderror)

- <span id="builderror-hir"></span>`fn hir(pid: PatternID, err: hir::Error) -> BuildError` — [`PatternID`](../util/primitives/index.md#patternid), [`BuildError`](error/index.md#builderror)

- <span id="builderror-nfa"></span>`fn nfa(err: nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../nfa/thompson/error/index.md#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](error/index.md#builderror)

##### `impl Debug for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- <span id="builderror-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl ToString for BuildError`

- <span id="builderror-to-string"></span>`fn to_string(&self) -> String`

### `Builder`

```rust
struct Builder {
    config: Config,
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:3380-3384`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L3380-L3384)*

A builder for configuring and constructing a `Regex`.

The builder permits configuring two different aspects of a `Regex`:

* `Builder::configure` will set high-level configuration options as
described by a [`Config`](regex/index.md).
* `Builder::syntax` will set the syntax level configuration options
as described by a [`util::syntax::Config`](crate::util::syntax::Config).
This only applies when building a `Regex` from pattern strings.

Once configured, the builder can then be used to construct a `Regex` from
one of 4 different inputs:

* `Builder::build` creates a regex from a single pattern string.
* `Builder::build_many` creates a regex from many pattern strings.
* `Builder::build_from_hir` creates a regex from a
[`regex-syntax::Hir`](Hir) expression.
* `Builder::build_many_from_hir` creates a regex from many
[`regex-syntax::Hir`](Hir) expressions.

The latter two methods in particular provide a way to construct a fully
feature regular expression matcher directly from an `Hir` expression
without having to first convert it to a string. (This is in contrast to the
top-level `regex` crate which intentionally provides no such API in order
to avoid making `regex-syntax` a public dependency.)

As a convenience, this builder may be created via `Regex::builder`, which
may help avoid an extra import.

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change the
line terminator to the NUL byte:

```rust
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: disable UTF-8 requirement

By default, regex patterns are required to match UTF-8. This includes
regex patterns that can produce matches of length zero. In the case of an
empty match, by default, matches will not appear between the code units of
a UTF-8 encoded codepoint.

However, it can be useful to disable this requirement, particularly if
you're searching things like `&[u8]` that are not known to be valid UTF-8.

```rust
use regex_automata::{meta::Regex, util::syntax, Match};

let mut builder = Regex::builder();
// Disables the requirement that non-empty matches match UTF-8.
builder.syntax(syntax::Config::new().utf8(false));
// Disables the requirement that empty matches match UTF-8 boundaries.
builder.configure(Regex::config().utf8_empty(false));

// We can match raw bytes via \xZZ syntax, but we need to disable
// Unicode mode to do that. We could disable it everywhere, or just
// selectively, as shown here.
let re = builder.build(r"(?-u:\xFF)foo(?-u:\xFF)")?;
let hay = b"\xFFfoo\xFF";
assert_eq!(Some(Match::must(0, 0..5)), re.find(hay));

// We can also match between code units.
let re = builder.build(r"")?;
let hay = "☃";
assert_eq!(re.find_iter(hay).collect::<Vec<Match>>(), vec![
    Match::must(0, 0..0),
    Match::must(0, 1..1),
    Match::must(0, 2..2),
    Match::must(0, 3..3),
]);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](regex/index.md#builder)

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](regex/index.md#regex), [`BuildError`](error/index.md#builderror)

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](regex/index.md#regex), [`BuildError`](error/index.md#builderror)

- <span id="builder-build-from-hir"></span>`fn build_from_hir(&self, hir: &Hir) -> Result<Regex, BuildError>` — [`Regex`](regex/index.md#regex), [`BuildError`](error/index.md#builderror)

- <span id="builder-build-many-from-hir"></span>`fn build_many_from_hir<H: Borrow<Hir>>(&self, hirs: &[H]) -> Result<Regex, BuildError>` — [`Regex`](regex/index.md#regex), [`BuildError`](error/index.md#builderror)

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](regex/index.md#config), [`Builder`](regex/index.md#builder)

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../util/syntax/index.md#config), [`Builder`](regex/index.md#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](regex/index.md#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Cache`

```rust
struct Cache {
    capmatches: crate::util::captures::Captures,
    pikevm: wrappers::PikeVMCache,
    backtrack: wrappers::BoundedBacktrackerCache,
    onepass: wrappers::OnePassCache,
    hybrid: wrappers::HybridCache,
    revhybrid: wrappers::ReverseHybridCache,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2353-2360`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2353-L2360)*

Represents mutable scratch space used by regex engines during a search.

Most of the regex engines in this crate require some kind of
mutable state in order to execute a search. This mutable state is
explicitly separated from the core regex object (such as a
[`thompson::NFA`](crate::nfa::thompson::NFA)) so that the read-only regex
object can be shared across multiple threads simultaneously without any
synchronization. Conversely, a `Cache` must either be duplicated if using
the same `Regex` from multiple threads, or else there must be some kind of
synchronization that guarantees exclusive access while it's in use by one
thread.

A `Regex` attempts to do this synchronization for you by using a thread
pool internally. Its size scales roughly with the number of simultaneous
regex searches.

For cases where one does not want to rely on a `Regex`'s internal thread
pool, lower level routines such as `Regex::search_with` are provided
that permit callers to pass a `Cache` into the search routine explicitly.

General advice is that the thread pool is often more than good enough.
However, it may be possible to observe the effects of its latency,
especially when searching many small haystacks from many threads
simultaneously.

Caches can be created from their corresponding `Regex` via
`Regex::create_cache`. A cache can only be used with either the `Regex`
that created it, or the `Regex` that was most recently used to reset it
with `Cache::reset`. Using a cache with any other `Regex` may result in
panics or incorrect results.

# Example

```rust
use regex_automata::{meta::Regex, Input, Match};

let re = Regex::new(r"(?-u)m\w+\s+m\w+")?;
let mut cache = re.create_cache();
let input = Input::new("crazy janey and her mission man");
assert_eq!(
    Some(Match::must(0, 20..31)),
    re.search_with(&mut cache, &input),
);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="cache-new"></span>`fn new(re: &Regex) -> Cache` — [`Regex`](regex/index.md#regex), [`Cache`](regex/index.md#cache)

- <span id="cache-reset"></span>`fn reset(&mut self, re: &Regex)` — [`Regex`](regex/index.md#regex)

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](regex/index.md#cache)

##### `impl Debug for Cache`

- <span id="cache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CapturesMatches<'r, 'h>`

```rust
struct CapturesMatches<'r, 'h> {
    re: &'r Regex,
    cache: crate::util::pool::PoolGuard<'r, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2138-2143`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2138-L2143)*

An iterator over all non-overlapping leftmost matches with their capturing
groups.

The iterator yields a [`Captures`](../util/captures/index.md) value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::captures_iter` method.

#### Implementations

- <span id="capturesmatches-regex"></span>`fn regex(&self) -> &'r Regex` — [`Regex`](regex/index.md#regex)

- <span id="capturesmatches-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../index.md#input)

#### Trait Implementations

##### `impl Debug for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for CapturesMatches<'r, 'h>`

##### `impl IntoIterator for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturesmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturesmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CapturesMatches<'r, 'h>`

- <span id="capturesmatches-iterator-type-item"></span>`type Item = Captures`

- <span id="capturesmatches-next"></span>`fn next(&mut self) -> Option<Captures>` — [`Captures`](../util/captures/index.md#captures)

- <span id="capturesmatches-count"></span>`fn count(self) -> usize`

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    utf8_empty: Option<bool>,
    autopre: Option<bool>,
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
    which_captures: Option<crate::nfa::thompson::WhichCaptures>,
    nfa_size_limit: Option<Option<usize>>,
    onepass_size_limit: Option<Option<usize>>,
    hybrid_cache_capacity: Option<usize>,
    hybrid: Option<bool>,
    dfa: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    dfa_state_limit: Option<Option<usize>>,
    onepass: Option<bool>,
    backtrack: Option<bool>,
    byte_classes: Option<bool>,
    line_terminator: Option<u8>,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2453-2477`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2453-L2477)*

An object describing the configuration of a `Regex`.

This configuration only includes options for the
non-syntax behavior of a `Regex`, and can be applied via the
`Builder::configure` method. For configuring the syntax options, see
[`util::syntax::Config`](crate::util::syntax::Config).

# Example: lower the NFA size limit

In some cases, the default size limit might be too big. The size limit can
be lowered, which will prevent large regex patterns from compiling.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::meta::Regex;

let result = Regex::builder()
    .configure(Regex::config().nfa_size_limit(Some(20 * (1<<10))))
    // Not even 20KB is enough to build a single large Unicode class!
    .build(r"\pL");
assert!(result.is_err());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](regex/index.md#config)

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../index.md#matchkind), [`Config`](regex/index.md#config)

- <span id="config-utf8-empty"></span>`fn utf8_empty(self, yes: bool) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-auto-prefilter"></span>`fn auto_prefilter(self, yes: bool) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../util/prefilter/index.md#prefilter), [`Config`](regex/index.md#config)

- <span id="config-which-captures"></span>`fn which_captures(self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](../nfa/thompson/compiler/index.md#whichcaptures), [`Config`](regex/index.md#config)

- <span id="config-nfa-size-limit"></span>`fn nfa_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-onepass-size-limit"></span>`fn onepass_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-hybrid-cache-capacity"></span>`fn hybrid_cache_capacity(self, limit: usize) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-dfa-size-limit"></span>`fn dfa_size_limit(self, limit: Option<usize>) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-dfa-state-limit"></span>`fn dfa_state_limit(self, limit: Option<usize>) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-byte-classes"></span>`fn byte_classes(self, yes: bool) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-line-terminator"></span>`fn line_terminator(self, byte: u8) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-hybrid"></span>`fn hybrid(self, yes: bool) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-dfa"></span>`fn dfa(self, yes: bool) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-onepass"></span>`fn onepass(self, yes: bool) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-backtrack"></span>`fn backtrack(self, yes: bool) -> Config` — [`Config`](regex/index.md#config)

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../index.md#matchkind)

- <span id="config-get-utf8-empty"></span>`fn get_utf8_empty(&self) -> bool`

- <span id="config-get-auto-prefilter"></span>`fn get_auto_prefilter(&self) -> bool`

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../util/prefilter/index.md#prefilter)

- <span id="config-get-which-captures"></span>`fn get_which_captures(&self) -> WhichCaptures` — [`WhichCaptures`](../nfa/thompson/compiler/index.md#whichcaptures)

- <span id="config-get-nfa-size-limit"></span>`fn get_nfa_size_limit(&self) -> Option<usize>`

- <span id="config-get-onepass-size-limit"></span>`fn get_onepass_size_limit(&self) -> Option<usize>`

- <span id="config-get-hybrid-cache-capacity"></span>`fn get_hybrid_cache_capacity(&self) -> usize`

- <span id="config-get-dfa-size-limit"></span>`fn get_dfa_size_limit(&self) -> Option<usize>`

- <span id="config-get-dfa-state-limit"></span>`fn get_dfa_state_limit(&self) -> Option<usize>`

- <span id="config-get-byte-classes"></span>`fn get_byte_classes(&self) -> bool`

- <span id="config-get-line-terminator"></span>`fn get_line_terminator(&self) -> u8`

- <span id="config-get-hybrid"></span>`fn get_hybrid(&self) -> bool`

- <span id="config-get-dfa"></span>`fn get_dfa(&self) -> bool`

- <span id="config-get-onepass"></span>`fn get_onepass(&self) -> bool`

- <span id="config-get-backtrack"></span>`fn get_backtrack(&self) -> bool`

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](regex/index.md#config)

#### Trait Implementations

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](regex/index.md#config)

##### `impl Debug for Config`

- <span id="config-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](regex/index.md#config)

### `FindMatches<'r, 'h>`

```rust
struct FindMatches<'r, 'h> {
    re: &'r Regex,
    cache: crate::util::pool::PoolGuard<'r, Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2075-2079`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2075-L2079)*

An iterator over all non-overlapping matches.

The iterator yields a [`Match`](../index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::find_iter` method.

#### Implementations

- <span id="findmatches-regex"></span>`fn regex(&self) -> &'r Regex` — [`Regex`](regex/index.md#regex)

- <span id="findmatches-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../index.md#input)

#### Trait Implementations

##### `impl Debug for FindMatches<'r, 'h>`

- <span id="findmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for FindMatches<'r, 'h>`

##### `impl IntoIterator for FindMatches<'r, 'h>`

- <span id="findmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="findmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindMatches<'r, 'h>`

- <span id="findmatches-iterator-type-item"></span>`type Item = Match`

- <span id="findmatches-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../index.md#match)

- <span id="findmatches-count"></span>`fn count(self) -> usize`

### `Regex`

```rust
struct Regex {
    imp: alloc::sync::Arc<RegexI>,
    pool: crate::util::pool::Pool<Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:235-252`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L235-L252)*

A regex matcher that works by composing several other regex matchers
automatically.

In effect, a meta regex papers over a lot of the quirks or performance
problems in each of the regex engines in this crate. Its goal is to provide
an infallible and simple API that "just does the right thing" in the common
case.

A meta regex is the implementation of a `Regex` in the `regex` crate.
Indeed, the `regex` crate API is essentially just a light wrapper over
this type. This includes the `regex` crate's `RegexSet` API!

# Composition

This is called a "meta" matcher precisely because it uses other regex
matchers to provide a convenient high level regex API. Here are some
examples of how other regex matchers are composed:

* When calling `Regex::captures`, instead of immediately
running a slower but more capable regex engine like the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM), the meta regex engine
will usually first look for the bounds of a match with a higher throughput
regex engine like a [lazy DFA](crate::hybrid). Only when a match is found
is a slower engine like `PikeVM` used to find the matching span for each
capture group.
* While higher throughout engines like the lazy DFA cannot handle
Unicode word boundaries in general, they can still be used on pure ASCII
haystacks by pretending that Unicode word boundaries are just plain ASCII
word boundaries. However, if a haystack is not ASCII, the meta regex engine
will automatically switch to a (possibly slower) regex engine that supports
Unicode word boundaries in general.
* In some cases where a regex pattern is just a simple literal or a small
set of literals, an actual regex engine won't be used at all. Instead,
substring or multi-substring search algorithms will be employed.

There are many other forms of composition happening too, but the above
should give a general idea. In particular, it may perhaps be surprising
that *multiple* regex engines might get executed for a single search. That
is, the decision of what regex engine to use is not _just_ based on the
pattern, but also based on the dynamic execution of the search itself.

The primary reason for this composition is performance. The fundamental
tension is that the faster engines tend to be less capable, and the more
capable engines tend to be slower.

Note that the forms of composition that are allowed are determined by
compile time crate features and configuration. For example, if the `hybrid`
feature isn't enabled, or if `Config::hybrid` has been disabled, then the
meta regex engine will never use a lazy DFA.

# Synchronization and cloning

Most of the regex engines in this crate require some kind of mutable
"scratch" space to read and write from while performing a search. Since
a meta regex composes these regex engines, a meta regex also requires
mutable scratch space. This scratch space is called a [`Cache`](regex/index.md).

Most regex engines _also_ usually have a read-only component, typically
a [Thompson `NFA`](crate::nfa::thompson::NFA).

In order to make the `Regex` API convenient, most of the routines hide
the fact that a `Cache` is needed at all. To achieve this, a [memory
pool](crate::util::pool::Pool) is used internally to retrieve `Cache`
values in a thread safe way that also permits reuse. This in turn implies
that every such search call requires some form of synchronization. Usually
this synchronization is fast enough to not notice, but in some cases, it
can be a bottleneck. This typically occurs when all of the following are
true:

* The same `Regex` is shared across multiple threads simultaneously,
usually via a [`util::lazy::Lazy`](crate::util::lazy::Lazy) or something
similar from the `once_cell` or `lazy_static` crates.
* The primary unit of work in each thread is a regex search.
* Searches are run on very short haystacks.

This particular case can lead to high contention on the pool used by a
`Regex` internally, which can in turn increase latency to a noticeable
effect. This cost can be mitigated in one of the following ways:

* Use a distinct copy of a `Regex` in each thread, usually by cloning it.
Cloning a `Regex` _does not_ do a deep copy of its read-only component.
But it does lead to each `Regex` having its own memory pool, which in
turn eliminates the problem of contention. In general, this technique should
not result in any additional memory usage when compared to sharing the same
`Regex` across multiple threads simultaneously.
* Use lower level APIs, like `Regex::search_with`, which permit passing
a `Cache` explicitly. In this case, it is up to you to determine how best
to provide a `Cache`. For example, you might put a `Cache` in thread-local
storage if your use case allows for it.

Overall, this is an issue that happens rarely in practice, but it can
happen.

# Warning: spin-locks may be used in alloc-only mode

When this crate is built without the `std` feature and the high level APIs
on a `Regex` are used, then a spin-lock will be used to synchronize access
to an internal pool of `Cache` values. This may be undesirable because
a spin-lock is [effectively impossible to implement correctly in user
space][spinlocks-are-bad]. That is, more concretely, the spin-lock could
result in a deadlock.

If one wants to avoid the use of spin-locks when the `std` feature is
disabled, then you must use APIs that accept a `Cache` value explicitly.
For example, `Regex::search_with`.

# Example

```rust
use regex_automata::meta::Regex;

let re = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$")?;
assert!(re.is_match("2010-03-14"));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: anchored search

This example shows how to use `Input::anchored` to run an anchored
search, even when the regex pattern itself isn't anchored. An anchored
search guarantees that if a match is found, then the start offset of the
match corresponds to the offset at which the search was started.

```rust
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"\bfoo\b")?;
let input = Input::new("xx foo xx").range(3..).anchored(Anchored::Yes);
// The offsets are in terms of the original haystack.
assert_eq!(Some(Match::must(0, 3..6)), re.find(input));

// Notice that no match occurs here, because \b still takes the
// surrounding context into account, even if it means looking back
// before the start of your search.
let hay = "xxfoo xx";
let input = Input::new(hay).range(2..).anchored(Anchored::Yes);
assert_eq!(None, re.find(input));
// Indeed, you cannot achieve the above by simply slicing the
// haystack itself, since the regex engine can't see the
// surrounding context. This is why 'Input' permits setting
// the bounds of a search!
let input = Input::new(&hay[2..]).anchored(Anchored::Yes);
// WRONG!
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: earliest search

This example shows how to use `Input::earliest` to run a search that
might stop before finding the typical leftmost match.

```rust
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"[a-z]{3}|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 1..2)), re.find(input));

// Note that "earliest" isn't really a match semantic unto itself.
// Instead, it is merely an instruction to whatever regex engine
// gets used internally to quit as soon as it can. For example,
// this regex uses a different search technique, and winds up
// producing a different (but valid) match!
let re = Regex::new(r"abc|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change
the line terminator to the NUL byte:

```rust
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`imp`**: `alloc::sync::Arc<RegexI>`

  The actual regex implementation.

- **`pool`**: `crate::util::pool::Pool<Cache, alloc::boxed::Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>>`

  A thread safe pool of caches.
  
  For the higher level search APIs, a `Cache` is automatically plucked
  from this pool before running a search. The lower level `with` methods
  permit the caller to provide their own cache, thereby bypassing
  accesses to this pool.
  
  Note that we put this outside the `Arc` so that cloning a `Regex`
  results in creating a fresh `CachePool`. This in turn permits callers
  to clone regexes into separate threads where each such regex gets
  the pool's "thread owner" optimization. Otherwise, if one shares the
  `Regex` directly, then the pool will go through a slower mutex path for
  all threads except for the "owner."

#### Implementations

- <span id="regex-new"></span>`fn new(pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](regex/index.md#regex), [`BuildError`](error/index.md#builderror)

- <span id="regex-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](regex/index.md#regex), [`BuildError`](error/index.md#builderror)

- <span id="regex-config"></span>`fn config() -> Config` — [`Config`](regex/index.md#config)

- <span id="regex-builder"></span>`fn builder() -> Builder` — [`Builder`](regex/index.md#builder)

#### Trait Implementations

##### `impl Clone for Regex`

- <span id="regex-clone"></span>`fn clone(&self) -> Regex` — [`Regex`](regex/index.md#regex)

##### `impl Debug for Regex`

- <span id="regex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Split<'r, 'h>`

```rust
struct Split<'r, 'h> {
    finder: FindMatches<'r, 'h>,
    last: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2206-2209`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2206-L2209)*

Yields all substrings delimited by a regular expression match.

The spans correspond to the offsets between matches.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::split` method.

#### Implementations

- <span id="split-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../index.md#input)

#### Trait Implementations

##### `impl Debug for Split<'r, 'h>`

- <span id="split-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for Split<'r, 'h>`

##### `impl IntoIterator for Split<'r, 'h>`

- <span id="split-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="split-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Split<'r, 'h>`

- <span id="split-iterator-type-item"></span>`type Item = Span`

- <span id="split-next"></span>`fn next(&mut self) -> Option<Span>` — [`Span`](../index.md#span)

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    splits: Split<'r, 'h>,
    limit: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/regex.rs:2260-2263`](../../../.source_1765521767/regex-automata-0.4.13/src/meta/regex.rs#L2260-L2263)*

Yields at most `N` spans delimited by a regular expression match.

The spans correspond to the offsets between matches. The last span will be
whatever remains after splitting.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::splitn` method.

#### Implementations

- <span id="splitn-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../index.md#input)

#### Trait Implementations

##### `impl Debug for SplitN<'r, 'h>`

- <span id="splitn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for SplitN<'r, 'h>`

##### `impl IntoIterator for SplitN<'r, 'h>`

- <span id="splitn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splitn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splitn-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SplitN<'r, 'h>`

- <span id="splitn-iterator-type-item"></span>`type Item = Span`

- <span id="splitn-next"></span>`fn next(&mut self) -> Option<Span>` — [`Span`](../index.md#span)

- <span id="splitn-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

