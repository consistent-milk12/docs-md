*[regex_automata](../../index.md) / [dfa](../index.md) / [onepass](index.md)*

---

# Module `onepass`

A DFA that can return spans for matching capturing groups.

This module is the home of a [one-pass DFA](DFA).

This module also contains a [`Builder`](../../nfa/thompson/builder/index.md) and a [`Config`](#config) for building and
configuring a one-pass DFA.

## Structs

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
```

The configuration used for building a [one-pass DFA](DFA).

A one-pass DFA configuration is a simple data object that is typically used
with `Builder::configure`. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with `DFA::config`.

#### Implementations

- `fn new() -> Config` — [`Config`](#config)

- `fn match_kind(self: Self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md), [`Config`](#config)

- `fn starts_for_each_pattern(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn byte_classes(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn size_limit(self: Self, limit: Option<usize>) -> Config` — [`Config`](#config)

- `fn get_match_kind(self: &Self) -> MatchKind` — [`MatchKind`](../../index.md)

- `fn get_starts_for_each_pattern(self: &Self) -> bool`

- `fn get_byte_classes(self: &Self) -> bool`

- `fn get_size_limit(self: &Self) -> Option<usize>`

- `fn overwrite(self: &Self, o: Config) -> Config` — [`Config`](#config)

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Config`

- `fn default() -> Config` — [`Config`](#config)

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
}
```

A builder for a [one-pass DFA](DFA).

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the DFA construction. This builder is different from a
general purpose regex builder in that it permits fine grain configuration
of the construction process. The trade off for this is complexity, and
the possibility of setting a configuration that might not make sense. For
example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* `thompson::Config::utf8` controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the NFA.
This is generally what you want for matching on arbitrary bytes.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    nfa::thompson,
    util::syntax,
    Match,
};

let re = DFA::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

let haystack = b"foo\xFFarzz\xE2\x98\xFF\n";
re.captures(&mut cache, haystack, &mut caps);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a one-pass DFA Config,
//  since that only impacts regexes that can
// produce matches of length 0.
assert_eq!(Some(Match::must(0, 0..8)), caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder` — [`Builder`](#builder)

- `fn build(self: &Self, pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

- `fn build_from_nfa(self: &Self, nfa: NFA) -> Result<DFA, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md), [`DFA`](#dfa), [`BuildError`](#builderror)

- `fn configure(self: &mut Self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md), [`Builder`](#builder)

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/compiler/index.md), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DFA`

```rust
struct DFA {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
    table: alloc::vec::Vec<Transition>,
    starts: alloc::vec::Vec<crate::util::primitives::StateID>,
    min_match_id: crate::util::primitives::StateID,
    classes: crate::util::alphabet::ByteClasses,
    alphabet_len: usize,
    stride2: usize,
    pateps_offset: usize,
    explicit_slot_start: usize,
}
```

A one-pass DFA for executing a subset of anchored regex searches while
resolving capturing groups.

A one-pass DFA can be built from an NFA that is one-pass. An NFA is
one-pass when there is never any ambiguity about how to continue a search.
For example, `a*a` is not one-pass because during a search, it's not
possible to know whether to continue matching the `a*` or to move on to
the single `a`. However, `a*b` is one-pass, because for every byte in the
input, it's always clear when to move on from `a*` to `b`.

# Only anchored searches are supported

In this crate, especially for DFAs, unanchored searches are implemented by
treating the pattern as if it had a `(?s-u:.)*?` prefix. While the prefix
is one-pass on its own, adding anything after it, e.g., `(?s-u:.)*?a` will
make the overall pattern not one-pass. Why? Because the `(?s-u:.)` matches
any byte, and there is therefore ambiguity as to when the prefix should
stop matching and something else should start matching.

Therefore, one-pass DFAs do not support unanchored searches. In addition
to many regexes simply not being one-pass, it implies that one-pass DFAs
have limited utility. With that said, when a one-pass DFA can be used, it
can potentially provide a dramatic speed up over alternatives like the
[`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)
and the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). In particular,
a one-pass DFA is the only DFA capable of reporting the spans of matching
capturing groups.

To clarify, when we say that unanchored searches are not supported, what
that actually means is:

* The high level routines, `DFA::is_match` and `DFA::captures`, always
do anchored searches.
* Since iterators are most useful in the context of unanchored searches,
there is no `DFA::captures_iter` method.
* For lower level routines like `DFA::try_search`, an error will be
returned if the given [`Input`](../../index.md) is configured to do an unanchored search or
search for an invalid pattern ID. (Note that an [`Input`](../../index.md) is configured to
do an unanchored search by default, so just giving a `Input::new` is
guaranteed to return an error.)

# Other limitations

In addition to the [configurable heap limit](Config::size_limit) and
the requirement that a regex pattern be one-pass, there are some other
limitations:

* There is an internal limit on the total number of explicit capturing
groups that appear across all patterns. It is somewhat small and there is
no way to configure it. If your pattern(s) exceed this limit, then building
a one-pass DFA will fail.
* If the number of patterns exceeds an internal unconfigurable limit, then
building a one-pass DFA will fail. This limit is quite large and you're
unlikely to hit it.
* If the total number of states exceeds an internal unconfigurable limit,
then building a one-pass DFA will fail. This limit is quite large and
you're unlikely to hit it.

# Other examples of regexes that aren't one-pass

One particularly unfortunate example is that enabling Unicode can cause
regexes that were one-pass to no longer be one-pass. Consider the regex
`(?-u)\w*\s` for example. It is one-pass because there is exactly no
overlap between the ASCII definitions of `\w` and `\s`. But `\w*\s`
(i.e., with Unicode enabled) is *not* one-pass because `\w` and `\s` get
translated to UTF-8 automatons. And while the *codepoints* in `\w` and `\s`
do not overlap, the underlying UTF-8 encodings do. Indeed, because of the
overlap between UTF-8 automata, the use of Unicode character classes will
tend to vastly increase the likelihood of a regex not being one-pass.

# How does one know if a regex is one-pass or not?

At the time of writing, the only way to know is to try and build a one-pass
DFA. The one-pass property is checked while constructing the DFA.

This does mean that you might potentially waste some CPU cycles and memory
by optimistically trying to build a one-pass DFA. But this is currently the
only way. In the future, building a one-pass DFA might be able to use some
heuristics to detect common violations of the one-pass property and bail
more quickly.

# Resource usage

Unlike a general DFA, a one-pass DFA has stricter bounds on its resource
usage. Namely, construction of a one-pass DFA has a time and space
complexity of `O(n)`, where `n ~ nfa.states().len()`. (A general DFA's time
and space complexity is `O(2^n)`.) This smaller time bound is achieved
because there is at most one DFA state created for each NFA state. If
additional DFA states would be required, then the pattern is not one-pass
and construction will fail.

Note though that currently, this DFA uses a fully dense representation.
This means that while its space complexity is no worse than an NFA, it may
in practice use more memory because of higher constant factors. The reason
for this trade off is two-fold. Firstly, a dense representation makes the
search faster. Secondly, the bigger an NFA, the more unlikely it is to be
one-pass. Therefore, most one-pass DFAs are usually pretty small.

# Example

This example shows that the one-pass DFA implements Unicode word boundaries
correctly while simultaneously reporting spans for capturing groups that
participate in a match. (This is the only DFA that implements full support
for Unicode word boundaries.)

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{dfa::onepass::DFA, Match, Span};

let re = DFA::new(r"\b(?P<first>\w+)[[:space:]]+(?P<last>\w+)\b")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "Шерлок Холмс", &mut caps);
assert_eq!(Some(Match::must(0, 0..23)), caps.get_match());
assert_eq!(Some(Span::from(0..12)), caps.get_group_by_name("first"));
assert_eq!(Some(Span::from(13..23)), caps.get_group_by_name("last"));
Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: iteration

Unlike other regex engines in this crate, this one does not provide
iterator search functions. This is because a one-pass DFA only supports
anchored searches, and so iterator functions are generally not applicable.

However, if you know that all of your matches are
directly adjacent, then an iterator can be used. The
[`util::iter::Searcher`](crate::util::iter::Searcher) type can be used for
this purpose:

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    util::iter::Searcher,
    Anchored, Input, Span,
};

let re = DFA::new(r"\w(\d)\w")?;
let (mut cache, caps) = (re.create_cache(), re.create_captures());
let input = Input::new("a1zb2yc3x").anchored(Anchored::Yes);

let mut it = Searcher::new(input).into_captures_iter(caps, |input, caps| {
    Ok(re.try_search(&mut cache, input, caps)?)
}).infallible();
let caps0 = it.next().unwrap();
assert_eq!(Some(Span::from(1..2)), caps0.get_group(1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`config`**: `Config`

  The configuration provided by the caller.

- **`nfa`**: `crate::nfa::thompson::NFA`

  The NFA used to build this DFA.
  
  NOTE: We probably don't need to store the NFA here, but we use enough
  bits from it that it's convenient to do so. And there really isn't much
  cost to doing so either, since an NFA is reference counted internally.

- **`table`**: `alloc::vec::Vec<Transition>`

  The transition table. Given a state ID 's' and a byte of haystack 'b',
  the next state is `table[sid + classes[byte]]`.
  
  The stride of this table (i.e., the number of columns) is always
  a power of 2, even if the alphabet length is smaller. This makes
  converting between state IDs and state indices very cheap.
  
  Note that the stride always includes room for one extra "transition"
  that isn't actually a transition. It is a 'PatternEpsilons' that is
  used for match states only. Because of this, the maximum number of
  active columns in the transition table is 257, which means the maximum
  stride is 512 (the next power of 2 greater than or equal to 257).

- **`starts`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The DFA state IDs of the starting states.
  
  `starts[0]` is always present and corresponds to the starting state
  when searching for matches of any pattern in the DFA.
  
  `starts[i]` where i>0 corresponds to the starting state for the pattern
  ID 'i-1'. These starting states are optional.

- **`min_match_id`**: `crate::util::primitives::StateID`

  Every state ID >= this value corresponds to a match state.
  
  This is what a search uses to detect whether a state is a match state
  or not. It requires only a simple comparison instead of bit-unpacking
  the PatternEpsilons from every state.

- **`classes`**: `crate::util::alphabet::ByteClasses`

  The alphabet of this DFA, split into equivalence classes. Bytes in the
  same equivalence class can never discriminate between a match and a
  non-match.

- **`alphabet_len`**: `usize`

  The number of elements in each state in the transition table. This may
  be less than the stride, since the stride is always a power of 2 and
  the alphabet length can be anything up to and including 256.

- **`stride2`**: `usize`

  The number of columns in the transition table, expressed as a power of
  2.

- **`pateps_offset`**: `usize`

  The offset at which the PatternEpsilons for a match state is stored in
  the transition table.
  
  PERF: One wonders whether it would be better to put this in a separate
  allocation, since only match states have a non-empty PatternEpsilons
  and the number of match states tends be dwarfed by the number of
  non-match states. So this would save '8*len(non_match_states)' for each
  DFA. The question is whether moving this to a different allocation will
  lead to a perf hit during searches. You might think dealing with match
  states is rare, but some regexes spend a lot of time in match states
  gobbling up input. But... match state handling is already somewhat
  expensive, so maybe this wouldn't do much? Either way, it's worth
  experimenting.

- **`explicit_slot_start`**: `usize`

  The first explicit slot index. This refers to the first slot appearing
  immediately after the last implicit slot. It is always 'patterns.len()
  * 2'.
  
  We record this because we only store the explicit slots in our DFA
  transition table that need to be saved. Implicit slots are handled
  automatically as part of the search.

#### Implementations

- `fn search_imp(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Result<Option<PatternID>, MatchError>` — [`Cache`](#cache), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md), [`MatchError`](../../index.md)

- `fn find_match(self: &Self, cache: &mut Cache, input: &Input<'_>, at: usize, sid: StateID, slots: &mut [Option<NonMaxUsize>], matched_pid: &mut Option<PatternID>) -> bool` — [`Cache`](#cache), [`Input`](../../index.md), [`StateID`](../../util/primitives/index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for DFA`

- `fn clone(self: &Self) -> DFA` — [`DFA`](#dfa)

##### `impl Debug for DFA`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Remappable for crate::dfa::onepass::DFA`

- `fn state_len(self: &Self) -> usize`

- `fn stride2(self: &Self) -> usize`

- `fn swap_states(self: &mut Self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md)

- `fn remap(self: &mut Self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md)

### `Cache`

```rust
struct Cache {
    explicit_slots: alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>,
    explicit_slot_len: usize,
}
```

A cache represents mutable state that a one-pass [`DFA`](../../meta/wrappers/index.md) requires during a
search.

For a given one-pass DFA, its corresponding cache may be created either via
`DFA::create_cache`, or via `Cache::new`. They are equivalent in every
way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the one-pass DFA from which it was
created. It may only be used with that one-pass DFA. A cache and its
allocations may be re-purposed via `Cache::reset`, in which case, it can
only be used with the new one-pass DFA (and not the old one).

#### Fields

- **`explicit_slots`**: `alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>`

  Scratch space used to store slots during a search. Basically, we use
  the caller provided slots to store slots known when a match occurs.
  But after a match occurs, we might continue a search but ultimately
  fail to extend the match. When continuing the search, we need some
  place to store candidate capture offsets without overwriting the slot
  offsets recorded for the most recently seen match.

- **`explicit_slot_len`**: `usize`

  The number of slots in the caller-provided 'Captures' value for the
  current search. This is always at most 'explicit_slots.len()', but
  might be less than it, if the caller provided fewer slots to fill.

#### Implementations

- `fn new(re: &DFA) -> Cache` — [`DFA`](#dfa), [`Cache`](#cache)

- `fn reset(self: &mut Self, re: &DFA)` — [`DFA`](#dfa)

- `fn memory_usage(self: &Self) -> usize`

- `fn explicit_slots(self: &mut Self) -> &mut [Option<NonMaxUsize>]` — [`NonMaxUsize`](../../util/primitives/index.md)

- `fn setup_search(self: &mut Self, explicit_slot_len: usize)`

#### Trait Implementations

##### `impl Clone for Cache`

- `fn clone(self: &Self) -> Cache` — [`Cache`](#cache)

##### `impl Debug for Cache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

An error that occurred during the construction of a one-pass DFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying `thompson::BuildError` type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building a one-pass DFA directly from a pattern
string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- `fn nfa(err: crate::nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md)

- `fn word(err: UnicodeWordBoundaryError) -> BuildError` — [`UnicodeWordBoundaryError`](../../util/look/index.md), [`BuildError`](#builderror)

- `fn too_many_states(limit: u64) -> BuildError` — [`BuildError`](#builderror)

- `fn too_many_patterns(limit: u64) -> BuildError` — [`BuildError`](#builderror)

- `fn unsupported_look(look: Look) -> BuildError` — [`Look`](../../util/look/index.md), [`BuildError`](#builderror)

- `fn exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](#builderror)

- `fn not_one_pass(msg: &'static str) -> BuildError` — [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- `fn clone(self: &Self) -> BuildError` — [`BuildError`](#builderror)

##### `impl Debug for BuildError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for BuildError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for BuildError`

- `fn to_string(self: &Self) -> String`

