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
| [`CapturesMatches`](#capturesmatches) | struct | An iterator over all non-overlapping leftmost matches, with their capturing |
| [`Cache`](#cache) | struct | A cache represents mutable state that a [`PikeVM`] requires during a |
| [`ActiveStates`](#activestates) | struct | A set of active states used to "simulate" the execution of an NFA via the |
| [`SlotTable`](#slottable) | struct | A table of slots, where each row represent a state in an NFA. |
| [`FollowEpsilon`](#followepsilon) | enum | Represents a stack frame for use while computing an epsilon closure. |
| [`instrument!`](#instrument) | macro | A simple macro for conditionally executing instrumentation logic when |

## Structs

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
}
```

The configuration used for building a [`PikeVM`](#pikevm).

A PikeVM configuration is a simple data object that is typically used with
`Builder::configure`. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with `PikeVM::config`.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../../index.md), [`Config`](#config)

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../../util/prefilter/index.md), [`Config`](#config)

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../../index.md)

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../../util/prefilter/index.md)

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](#config)

#### Trait Implementations

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- <span id="config-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
}
```

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

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../index.md)

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../index.md)

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: NFA) -> Result<PikeVM, BuildError>` — [`NFA`](../index.md), [`PikeVM`](#pikevm), [`BuildError`](../index.md)

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../../util/syntax/index.md), [`Builder`](#builder)

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../index.md), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PikeVM`

```rust
struct PikeVM {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
}
```

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

- <span id="pikevm-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures)` — [`Cache`](#cache), [`Input`](../../../index.md), [`Captures`](../../../util/captures/index.md)

- <span id="pikevm-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](#cache), [`Input`](../../../index.md), [`NonMaxUsize`](../../../util/primitives/index.md), [`PatternID`](../../../index.md)

- <span id="pikevm-search-slots-imp"></span>`fn search_slots_imp(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<HalfMatch>` — [`Cache`](#cache), [`Input`](../../../index.md), [`NonMaxUsize`](../../../util/primitives/index.md), [`HalfMatch`](../../../index.md)

- <span id="pikevm-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](#cache), [`Input`](../../../index.md), [`PatternSet`](../../../index.md)

#### Trait Implementations

##### `impl Clone for PikeVM`

- <span id="pikevm-clone"></span>`fn clone(&self) -> PikeVM` — [`PikeVM`](#pikevm)

##### `impl Debug for PikeVM`

- <span id="pikevm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FindMatches<'r, 'c, 'h>`

```rust
struct FindMatches<'r, 'c, 'h> {
    re: &'r PikeVM,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

An iterator over all non-overlapping matches for a particular search.

The iterator yields a [`Match`](../../../index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `PikeVM::find_iter` method.

#### Trait Implementations

##### `impl<'r, 'c, 'h> Debug for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findmatches-intoiter"></span>`type IntoIter = I`

- <span id="findmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'r, 'c, 'h> Iterator for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-item"></span>`type Item = Match`

- <span id="findmatches-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../../index.md)

### `CapturesMatches<'r, 'c, 'h>`

```rust
struct CapturesMatches<'r, 'c, 'h> {
    re: &'r PikeVM,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

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

##### `impl<'r, 'c, 'h> Debug for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturesmatches-intoiter"></span>`type IntoIter = I`

- <span id="capturesmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'r, 'c, 'h> Iterator for CapturesMatches<'r, 'c, 'h>`

- <span id="capturesmatches-item"></span>`type Item = Captures`

- <span id="capturesmatches-next"></span>`fn next(&mut self) -> Option<Captures>` — [`Captures`](../../../util/captures/index.md)

### `Cache`

```rust
struct Cache {
    stack: alloc::vec::Vec<FollowEpsilon>,
    curr: ActiveStates,
    next: ActiveStates,
}
```

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

- <span id="cache-reset"></span>`fn reset(&mut self, re: &PikeVM)` — [`PikeVM`](#pikevm)

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="cache-setup-search"></span>`fn setup_search(&mut self, captures_slot_len: usize)`

#### Trait Implementations

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl Debug for Cache`

- <span id="cache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ActiveStates`

```rust
struct ActiveStates {
    set: crate::util::sparse_set::SparseSet,
    slot_table: SlotTable,
}
```

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

- <span id="activestates-reset"></span>`fn reset(&mut self, re: &PikeVM)` — [`PikeVM`](#pikevm)

- <span id="activestates-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="activestates-setup-search"></span>`fn setup_search(&mut self, captures_slot_len: usize)`

#### Trait Implementations

##### `impl Clone for ActiveStates`

- <span id="activestates-clone"></span>`fn clone(&self) -> ActiveStates` — [`ActiveStates`](#activestates)

##### `impl Debug for ActiveStates`

- <span id="activestates-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SlotTable`

```rust
struct SlotTable {
    table: alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>,
    slots_per_state: usize,
    slots_for_captures: usize,
}
```

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

- <span id="slottable-reset"></span>`fn reset(&mut self, re: &PikeVM)` — [`PikeVM`](#pikevm)

- <span id="slottable-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="slottable-setup-search"></span>`fn setup_search(&mut self, captures_slot_len: usize)`

- <span id="slottable-for-state"></span>`fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>]` — [`StateID`](../../../util/primitives/index.md), [`NonMaxUsize`](../../../util/primitives/index.md)

- <span id="slottable-all-absent"></span>`fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>]` — [`NonMaxUsize`](../../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for SlotTable`

- <span id="slottable-clone"></span>`fn clone(&self) -> SlotTable` — [`SlotTable`](#slottable)

##### `impl Debug for SlotTable`

- <span id="slottable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl Clone for FollowEpsilon`

- <span id="followepsilon-clone"></span>`fn clone(&self) -> FollowEpsilon` — [`FollowEpsilon`](#followepsilon)

##### `impl Debug for FollowEpsilon`

- <span id="followepsilon-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `instrument!`

A simple macro for conditionally executing instrumentation logic when
the 'trace' log level is enabled. This is a compile-time no-op when the
'internal-instrument-pikevm' feature isn't enabled. The intent here is that
this makes it easier to avoid doing extra work when instrumentation isn't
enabled.

This macro accepts a closure of type `|&mut Counters|`. The closure can
then increment counters (or whatever) in accordance with what one wants
to track.

