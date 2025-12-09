*[regex_automata](../../index.md) / [meta](../index.md) / [strategy](index.md)*

---

# Module `strategy`

## Contents

- [Structs](#structs)
  - [`Pre`](#pre)
  - [`Core`](#core)
  - [`ReverseAnchored`](#reverseanchored)
  - [`ReverseSuffix`](#reversesuffix)
  - [`ReverseInner`](#reverseinner)
- [Traits](#traits)
  - [`Strategy`](#strategy)
- [Functions](#functions)
  - [`new`](#new)
  - [`copy_match_to_slots`](#copy_match_to_slots)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pre`](#pre) | struct |  |
| [`Core`](#core) | struct |  |
| [`ReverseAnchored`](#reverseanchored) | struct |  |
| [`ReverseSuffix`](#reversesuffix) | struct |  |
| [`ReverseInner`](#reverseinner) | struct |  |
| [`Strategy`](#strategy) | trait | A trait that represents a single meta strategy. |
| [`new`](#new) | fn |  |
| [`copy_match_to_slots`](#copy_match_to_slots) | fn | Copies the offsets in the given match to the corresponding positions in `slots`. |

## Structs

### `Pre<P>`

```rust
struct Pre<P> {
    pre: P,
    group_info: crate::util::captures::GroupInfo,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:189-192`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L189-L192)*

#### Implementations

- <span id="pre-new"></span>`fn new(pre: P) -> Arc<dyn Strategy>` — [`Strategy`](#strategy)

#### Trait Implementations

##### `impl<P: clone::Clone> Clone for Pre<P>`

- <span id="pre-clone"></span>`fn clone(&self) -> Pre<P>` — [`Pre`](#pre)

##### `impl<P: fmt::Debug> Debug for Pre<P>`

- <span id="pre-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<P: PrefilterI> Strategy for Pre<P>`

- <span id="pre-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- <span id="pre-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md)

- <span id="pre-reset-cache"></span>`fn reset_cache(&self, _cache: &mut Cache)` — [`Cache`](../regex/index.md)

- <span id="pre-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="pre-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="pre-search"></span>`fn search(&self, _cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- <span id="pre-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- <span id="pre-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- <span id="pre-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- <span id="pre-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

### `Core`

```rust
struct Core {
    info: crate::meta::regex::RegexInfo,
    pre: Option<crate::util::prefilter::Prefilter>,
    nfa: crate::nfa::thompson::NFA,
    nfarev: Option<crate::nfa::thompson::NFA>,
    pikevm: wrappers::PikeVM,
    backtrack: wrappers::BoundedBacktracker,
    onepass: wrappers::OnePass,
    hybrid: wrappers::Hybrid,
    dfa: wrappers::DFA,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:443-453`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L443-L453)*

#### Implementations

- <span id="core-new"></span>`fn new(info: RegexInfo, pre: Option<Prefilter>, hirs: &[&Hir]) -> Result<Core, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`Core`](#core), [`BuildError`](../error/index.md)

- <span id="core-try-search-mayfail"></span>`fn try_search_mayfail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Result<Option<Match>, RetryFailError>>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="core-search-nofail"></span>`fn search_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- <span id="core-search-half-nofail"></span>`fn search_half_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- <span id="core-search-slots-nofail"></span>`fn search_slots_nofail(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- <span id="core-is-match-nofail"></span>`fn is_match_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- <span id="core-is-capture-search-needed"></span>`fn is_capture_search_needed(&self, slots_len: usize) -> bool`

#### Trait Implementations

##### `impl Debug for Core`

- <span id="core-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Strategy for Core`

- <span id="core-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- <span id="core-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md)

- <span id="core-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- <span id="core-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="core-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="core-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- <span id="core-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- <span id="core-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- <span id="core-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- <span id="core-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

### `ReverseAnchored`

```rust
struct ReverseAnchored {
    core: Core,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:904-906`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L904-L906)*

#### Implementations

- <span id="reverseanchored-new"></span>`fn new(core: Core) -> Result<ReverseAnchored, Core>` — [`Core`](#core), [`ReverseAnchored`](#reverseanchored)

- <span id="reverseanchored-try-search-half-anchored-rev"></span>`fn try_search_half_anchored_rev(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseAnchored`

- <span id="reverseanchored-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Strategy for ReverseAnchored`

- <span id="reverseanchored-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- <span id="reverseanchored-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md)

- <span id="reverseanchored-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- <span id="reverseanchored-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="reverseanchored-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="reverseanchored-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- <span id="reverseanchored-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- <span id="reverseanchored-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- <span id="reverseanchored-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- <span id="reverseanchored-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

### `ReverseSuffix`

```rust
struct ReverseSuffix {
    core: Core,
    pre: crate::util::prefilter::Prefilter,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:1116-1119`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L1116-L1119)*

#### Implementations

- <span id="reversesuffix-new"></span>`fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseSuffix, Core>` — [`Core`](#core), [`ReverseSuffix`](#reversesuffix)

- <span id="reversesuffix-try-search-half-start"></span>`fn try_search_half_start(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- <span id="reversesuffix-try-search-half-fwd"></span>`fn try_search_half_fwd(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="reversesuffix-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut Cache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseSuffix`

- <span id="reversesuffix-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Strategy for ReverseSuffix`

- <span id="reversesuffix-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- <span id="reversesuffix-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md)

- <span id="reversesuffix-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- <span id="reversesuffix-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="reversesuffix-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="reversesuffix-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- <span id="reversesuffix-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- <span id="reversesuffix-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- <span id="reversesuffix-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- <span id="reversesuffix-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

### `ReverseInner`

```rust
struct ReverseInner {
    core: Core,
    preinner: crate::util::prefilter::Prefilter,
    nfarev: crate::nfa::thompson::NFA,
    hybrid: wrappers::ReverseHybrid,
    dfa: wrappers::ReverseDFA,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:1494-1500`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L1494-L1500)*

#### Implementations

- <span id="reverseinner-new"></span>`fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseInner, Core>` — [`Core`](#core), [`ReverseInner`](#reverseinner)

- <span id="reverseinner-try-search-full"></span>`fn try_search_full(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<Match>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md), [`RetryError`](../error/index.md)

- <span id="reverseinner-try-search-half-fwd-stopat"></span>`fn try_search_half_fwd_stopat(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="reverseinner-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut Cache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseInner`

- <span id="reverseinner-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Strategy for ReverseInner`

- <span id="reverseinner-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- <span id="reverseinner-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md)

- <span id="reverseinner-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- <span id="reverseinner-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="reverseinner-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="reverseinner-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- <span id="reverseinner-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- <span id="reverseinner-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- <span id="reverseinner-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- <span id="reverseinner-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

## Traits

### `Strategy`

```rust
trait Strategy: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static { ... }
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:40-76`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L40-L76)*

A trait that represents a single meta strategy. Its main utility is in
providing a way to do dynamic dispatch over a few choices.

Why dynamic dispatch? I actually don't have a super compelling reason, and
importantly, I have not benchmarked it with the main alternative: an enum.
I went with dynamic dispatch initially because the regex engine search code
really can't be inlined into caller code in most cases because it's just
too big. In other words, it is already expected that every regex search
will entail at least the cost of a function call.

I do wonder whether using enums would result in better codegen overall
though. It's a worthwhile experiment to try. Probably the most interesting
benchmark to run in such a case would be one with a high match count. That
is, a benchmark to test the overall latency of a search call.

#### Required Methods

- `fn group_info(&self) -> &GroupInfo`

- `fn create_cache(&self) -> Cache`

- `fn reset_cache(&self, cache: &mut Cache)`

- `fn is_accelerated(&self) -> bool`

- `fn memory_usage(&self) -> usize`

- `fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>`

- `fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>`

- `fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool`

- `fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>`

- `fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)`

#### Implementors

- [`Core`](#core)
- [`Pre`](#pre)
- [`ReverseAnchored`](#reverseanchored)
- [`ReverseInner`](#reverseinner)
- [`ReverseSuffix`](#reversesuffix)

## Functions

### `new`

```rust
fn new(info: &crate::meta::regex::RegexInfo, hirs: &[&regex_syntax::hir::Hir]) -> Result<alloc::sync::Arc<dyn Strategy>, crate::meta::error::BuildError>
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:78-186`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L78-L186)*

### `copy_match_to_slots`

```rust
fn copy_match_to_slots(m: crate::util::search::Match, slots: &mut [Option<crate::util::primitives::NonMaxUsize>])
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:1896-1905`](../../../../.source_1765210505/regex-automata-0.4.13/src/meta/strategy.rs#L1896-L1905)*

Copies the offsets in the given match to the corresponding positions in
`slots`.

In effect, this sets the slots corresponding to the implicit group for the
pattern in the given match. If the indices for the corresponding slots do
not exist, then no slots are set.

This is useful when the caller provides slots (or captures), but you use a
regex engine that doesn't operate on slots (like a lazy DFA). This function
lets you map the match you get back to the slots provided by the caller.

