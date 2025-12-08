*[regex_automata](../../index.md) / [meta](../index.md) / [strategy](index.md)*

---

# Module `strategy`

## Structs

### `Pre<P>`

```rust
struct Pre<P> {
    pre: P,
    group_info: crate::util::captures::GroupInfo,
}
```

#### Implementations

- `fn new(pre: P) -> Arc<dyn Strategy>` — [`Strategy`](#strategy)

#### Trait Implementations

##### `impl<P: $crate::clone::Clone> Clone for Pre<P>`

- `fn clone(self: &Self) -> Pre<P>` — [`Pre`](#pre)

##### `impl<P: $crate::fmt::Debug> Debug for Pre<P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<P: PrefilterI> Strategy for Pre<P>`

- `fn group_info(self: &Self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- `fn create_cache(self: &Self) -> Cache` — [`Cache`](../regex/index.md)

- `fn reset_cache(self: &Self, _cache: &mut Cache)` — [`Cache`](../regex/index.md)

- `fn is_accelerated(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn search(self: &Self, _cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- `fn search_half(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- `fn is_match(self: &Self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- `fn search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

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

#### Implementations

- `fn new(info: RegexInfo, pre: Option<Prefilter>, hirs: &[&Hir]) -> Result<Core, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`Core`](#core), [`BuildError`](../error/index.md)

- `fn try_search_mayfail(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Result<Option<Match>, RetryFailError>>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn search_nofail(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- `fn search_half_nofail(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- `fn search_slots_nofail(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn is_match_nofail(self: &Self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- `fn is_capture_search_needed(self: &Self, slots_len: usize) -> bool`

#### Trait Implementations

##### `impl Debug for Core`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Strategy for Core`

- `fn group_info(self: &Self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- `fn create_cache(self: &Self) -> Cache` — [`Cache`](../regex/index.md)

- `fn reset_cache(self: &Self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- `fn is_accelerated(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn search(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- `fn search_half(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- `fn is_match(self: &Self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- `fn search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

### `ReverseAnchored`

```rust
struct ReverseAnchored {
    core: Core,
}
```

#### Implementations

- `fn new(core: Core) -> Result<ReverseAnchored, Core>` — [`Core`](#core), [`ReverseAnchored`](#reverseanchored)

- `fn try_search_half_anchored_rev(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseAnchored`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Strategy for ReverseAnchored`

- `fn group_info(self: &Self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- `fn create_cache(self: &Self) -> Cache` — [`Cache`](../regex/index.md)

- `fn reset_cache(self: &Self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- `fn is_accelerated(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn search(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- `fn search_half(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- `fn is_match(self: &Self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- `fn search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

### `ReverseSuffix`

```rust
struct ReverseSuffix {
    core: Core,
    pre: crate::util::prefilter::Prefilter,
}
```

#### Implementations

- `fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseSuffix, Core>` — [`Core`](#core), [`ReverseSuffix`](#reversesuffix)

- `fn try_search_half_start(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- `fn try_search_half_fwd(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_rev_limited(self: &Self, cache: &mut Cache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseSuffix`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Strategy for ReverseSuffix`

- `fn group_info(self: &Self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- `fn create_cache(self: &Self) -> Cache` — [`Cache`](../regex/index.md)

- `fn reset_cache(self: &Self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- `fn is_accelerated(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn search(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- `fn search_half(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- `fn is_match(self: &Self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- `fn search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

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

#### Implementations

- `fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseInner, Core>` — [`Core`](#core), [`ReverseInner`](#reverseinner)

- `fn try_search_full(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<Match>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md), [`RetryError`](../error/index.md)

- `fn try_search_half_fwd_stopat(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_rev_limited(self: &Self, cache: &mut Cache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseInner`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Strategy for ReverseInner`

- `fn group_info(self: &Self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- `fn create_cache(self: &Self) -> Cache` — [`Cache`](../regex/index.md)

- `fn reset_cache(self: &Self, cache: &mut Cache)` — [`Cache`](../regex/index.md)

- `fn is_accelerated(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn search(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`Match`](../../index.md)

- `fn search_half(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`HalfMatch`](../../index.md)

- `fn is_match(self: &Self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md), [`Input`](../../index.md)

- `fn search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md), [`Input`](../../index.md), [`PatternSet`](../../index.md)

## Traits

### `Strategy`

```rust
trait Strategy: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static { ... }
```

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

- `fn group_info(self: &Self) -> &GroupInfo`

- `fn create_cache(self: &Self) -> Cache`

- `fn reset_cache(self: &Self, cache: &mut Cache)`

- `fn is_accelerated(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn search(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>`

- `fn search_half(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>`

- `fn is_match(self: &Self, cache: &mut Cache, input: &Input<'_>) -> bool`

- `fn search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>`

- `fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)`

## Functions

### `new`

```rust
fn new(info: &crate::meta::regex::RegexInfo, hirs: &[&regex_syntax::hir::Hir]) -> Result<alloc::sync::Arc<dyn Strategy>, crate::meta::error::BuildError>
```

### `copy_match_to_slots`

```rust
fn copy_match_to_slots(m: crate::util::search::Match, slots: &mut [Option<crate::util::primitives::NonMaxUsize>])
```

Copies the offsets in the given match to the corresponding positions in
`slots`.

In effect, this sets the slots corresponding to the implicit group for the
pattern in the given match. If the indices for the corresponding slots do
not exist, then no slots are set.

This is useful when the caller provides slots (or captures), but you use a
regex engine that doesn't operate on slots (like a lazy DFA). This function
lets you map the match you get back to the slots provided by the caller.

