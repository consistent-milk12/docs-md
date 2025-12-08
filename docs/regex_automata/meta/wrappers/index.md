*[regex_automata](../../index.md) / [meta](../index.md) / [wrappers](index.md)*

---

# Module `wrappers`

This module contains a boat load of wrappers around each of our internal regex
engines. They encapsulate a few things:

1. The wrappers manage the conditional existence of the regex engine. Namely,
the PikeVM is the only required regex engine. The rest are optional. These
wrappers present a uniform API regardless of which engines are available. And
availability might be determined by compile time features or by dynamic
configuration via `meta::Config`. Encapsulating the conditional compilation
features is in particular a huge simplification for the higher level code that
composes these engines.
2. The wrappers manage construction of each engine, including skipping it if
the engine is unavailable or configured to not be used.
3. The wrappers manage whether an engine *can* be used for a particular
search configuration. For example, `BoundedBacktracker::get` only returns a
backtracking engine when the haystack is bigger than the maximum supported
length. The wrappers also sometimes take a position on when an engine *ought*
to be used, but only in cases where the logic is extremely local to the engine
itself. Otherwise, things like "choose between the backtracker and the one-pass
DFA" are managed by the higher level meta strategy code.

There are also corresponding wrappers for the various `Cache` types for each
regex engine that needs them. If an engine is unavailable or not used, then a
cache for it will *not* actually be allocated.

## Structs

### `PikeVM`

```rust
struct PikeVM(PikeVMEngine);
```

#### Implementations

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<PikeVM, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`PikeVM`](#pikevm), [`BuildError`](../index.md)

- `fn create_cache(self: &Self) -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

- `fn get(self: &Self) -> &PikeVMEngine` — [`PikeVMEngine`](#pikevmengine)

#### Trait Implementations

##### `impl Debug for PikeVM`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PikeVMEngine`

```rust
struct PikeVMEngine(pikevm::PikeVM);
```

#### Implementations

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<PikeVMEngine, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`PikeVMEngine`](#pikevmengine), [`BuildError`](../index.md)

- `fn is_match(self: &Self, cache: &mut PikeVMCache, input: &Input<'_>) -> bool` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md)

- `fn search_slots(self: &Self, cache: &mut PikeVMCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../index.md)

- `fn which_overlapping_matches(self: &Self, cache: &mut PikeVMCache, input: &Input<'_>, patset: &mut PatternSet)` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md), [`PatternSet`](../../index.md)

#### Trait Implementations

##### `impl Debug for PikeVMEngine`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PikeVMCache`

```rust
struct PikeVMCache(Option<pikevm::Cache>);
```

#### Implementations

- `fn none() -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

- `fn reset(self: &mut Self, builder: &PikeVM)` — [`PikeVM`](#pikevm)

- `fn memory_usage(self: &Self) -> usize`

- `fn get(self: &mut Self, vm: &pikevm::PikeVM) -> &mut pikevm::Cache` — [`PikeVM`](../../nfa/thompson/pikevm/index.md), [`Cache`](../../nfa/thompson/pikevm/index.md)

#### Trait Implementations

##### `impl Clone for PikeVMCache`

- `fn clone(self: &Self) -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

##### `impl Debug for PikeVMCache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BoundedBacktracker`

```rust
struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
```

#### Implementations

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<BoundedBacktracker, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../index.md)

- `fn create_cache(self: &Self) -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

- `fn get(self: &Self, input: &Input<'_>) -> Option<&BoundedBacktrackerEngine>` — [`Input`](../../index.md), [`BoundedBacktrackerEngine`](#boundedbacktrackerengine)

#### Trait Implementations

##### `impl Debug for BoundedBacktracker`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BoundedBacktrackerEngine`

```rust
struct BoundedBacktrackerEngine(backtrack::BoundedBacktracker);
```

#### Implementations

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<Option<BoundedBacktrackerEngine>, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`BoundedBacktrackerEngine`](#boundedbacktrackerengine), [`BuildError`](../index.md)

- `fn is_match(self: &Self, cache: &mut BoundedBacktrackerCache, input: &Input<'_>) -> bool` — [`BoundedBacktrackerCache`](#boundedbacktrackercache), [`Input`](../../index.md)

- `fn search_slots(self: &Self, cache: &mut BoundedBacktrackerCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`BoundedBacktrackerCache`](#boundedbacktrackercache), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../index.md)

- `fn max_haystack_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug for BoundedBacktrackerEngine`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BoundedBacktrackerCache`

```rust
struct BoundedBacktrackerCache(Option<backtrack::Cache>);
```

#### Implementations

- `fn none() -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

- `fn reset(self: &mut Self, builder: &BoundedBacktracker)` — [`BoundedBacktracker`](#boundedbacktracker)

- `fn memory_usage(self: &Self) -> usize`

- `fn get(self: &mut Self, bb: &backtrack::BoundedBacktracker) -> &mut backtrack::Cache` — [`BoundedBacktracker`](../../nfa/thompson/backtrack/index.md), [`Cache`](../../nfa/thompson/backtrack/index.md)

#### Trait Implementations

##### `impl Clone for BoundedBacktrackerCache`

- `fn clone(self: &Self) -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

##### `impl Debug for BoundedBacktrackerCache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OnePass`

```rust
struct OnePass(Option<OnePassEngine>);
```

#### Implementations

- `fn new(info: &RegexInfo, nfa: &NFA) -> OnePass` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`OnePass`](#onepass)

- `fn create_cache(self: &Self) -> OnePassCache` — [`OnePassCache`](#onepasscache)

- `fn get(self: &Self, input: &Input<'_>) -> Option<&OnePassEngine>` — [`Input`](../../index.md), [`OnePassEngine`](#onepassengine)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug for OnePass`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OnePassEngine`

```rust
struct OnePassEngine(onepass::DFA);
```

#### Implementations

- `fn new(info: &RegexInfo, nfa: &NFA) -> Option<OnePassEngine>` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`OnePassEngine`](#onepassengine)

- `fn search_slots(self: &Self, cache: &mut OnePassCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`OnePassCache`](#onepasscache), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn get_nfa(self: &Self) -> &NFA` — [`NFA`](../../nfa/thompson/index.md)

#### Trait Implementations

##### `impl Debug for OnePassEngine`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OnePassCache`

```rust
struct OnePassCache(Option<onepass::Cache>);
```

#### Implementations

- `fn none() -> OnePassCache` — [`OnePassCache`](#onepasscache)

- `fn new(builder: &OnePass) -> OnePassCache` — [`OnePass`](#onepass), [`OnePassCache`](#onepasscache)

- `fn reset(self: &mut Self, builder: &OnePass)` — [`OnePass`](#onepass)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for OnePassCache`

- `fn clone(self: &Self) -> OnePassCache` — [`OnePassCache`](#onepasscache)

##### `impl Debug for OnePassCache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Hybrid`

```rust
struct Hybrid(Option<HybridEngine>);
```

#### Implementations

- `fn none() -> Hybrid` — [`Hybrid`](#hybrid)

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Hybrid` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`Hybrid`](#hybrid)

- `fn create_cache(self: &Self) -> HybridCache` — [`HybridCache`](#hybridcache)

- `fn get(self: &Self, _input: &Input<'_>) -> Option<&HybridEngine>` — [`Input`](../../index.md), [`HybridEngine`](#hybridengine)

- `fn is_some(self: &Self) -> bool`

#### Trait Implementations

##### `impl Debug for Hybrid`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HybridEngine`

```rust
struct HybridEngine(hybrid::regex::Regex);
```

#### Implementations

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Option<HybridEngine>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`HybridEngine`](#hybridengine)

- `fn try_search(self: &Self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<Match>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`Match`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_fwd(self: &Self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_fwd_stopat(self: &Self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_rev(self: &Self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_rev_limited(self: &Self, cache: &mut HybridCache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- `fn try_which_overlapping_matches(self: &Self, cache: &mut HybridCache, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`PatternSet`](../../index.md), [`RetryFailError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for HybridEngine`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `HybridCache`

```rust
struct HybridCache(Option<hybrid::regex::Cache>);
```

#### Implementations

- `fn none() -> HybridCache` — [`HybridCache`](#hybridcache)

- `fn new(builder: &Hybrid) -> HybridCache` — [`Hybrid`](#hybrid), [`HybridCache`](#hybridcache)

- `fn reset(self: &mut Self, builder: &Hybrid)` — [`Hybrid`](#hybrid)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for HybridCache`

- `fn clone(self: &Self) -> HybridCache` — [`HybridCache`](#hybridcache)

##### `impl Debug for HybridCache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DFA`

```rust
struct DFA(Option<DFAEngine>);
```

#### Implementations

- `fn none() -> DFA` — [`DFA`](#dfa)

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> DFA` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`DFA`](#dfa)

- `fn get(self: &Self, _input: &Input<'_>) -> Option<&DFAEngine>` — [`Input`](../../index.md), [`DFAEngine`](#dfaengine)

- `fn is_some(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug for DFA`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DFAEngine`

```rust
struct DFAEngine(());
```

#### Implementations

- `fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Option<DFAEngine>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`DFAEngine`](#dfaengine)

- `fn try_search(self: &Self, input: &Input<'_>) -> Result<Option<Match>, RetryFailError>` — [`Input`](../../index.md), [`Match`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_fwd(self: &Self, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_fwd_stopat(self: &Self, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_rev(self: &Self, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn try_search_half_rev_limited(self: &Self, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- `fn try_which_overlapping_matches(self: &Self, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), RetryFailError>` — [`Input`](../../index.md), [`PatternSet`](../../index.md), [`RetryFailError`](../error/index.md)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug for DFAEngine`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReverseHybrid`

```rust
struct ReverseHybrid(Option<ReverseHybridEngine>);
```

#### Implementations

- `fn none() -> ReverseHybrid` — [`ReverseHybrid`](#reversehybrid)

- `fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseHybrid`](#reversehybrid)

- `fn create_cache(self: &Self) -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

- `fn get(self: &Self, _input: &Input<'_>) -> Option<&ReverseHybridEngine>` — [`Input`](../../index.md), [`ReverseHybridEngine`](#reversehybridengine)

#### Trait Implementations

##### `impl Debug for ReverseHybrid`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReverseHybridEngine`

```rust
struct ReverseHybridEngine(hybrid::dfa::DFA);
```

#### Implementations

- `fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseHybridEngine>` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseHybridEngine`](#reversehybridengine)

- `fn try_search_half_rev_limited(self: &Self, cache: &mut ReverseHybridCache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`ReverseHybridCache`](#reversehybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseHybridEngine`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReverseHybridCache`

```rust
struct ReverseHybridCache(Option<hybrid::dfa::Cache>);
```

#### Implementations

- `fn none() -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

- `fn new(builder: &ReverseHybrid) -> ReverseHybridCache` — [`ReverseHybrid`](#reversehybrid), [`ReverseHybridCache`](#reversehybridcache)

- `fn reset(self: &mut Self, builder: &ReverseHybrid)` — [`ReverseHybrid`](#reversehybrid)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for ReverseHybridCache`

- `fn clone(self: &Self) -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

##### `impl Debug for ReverseHybridCache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReverseDFA`

```rust
struct ReverseDFA(Option<ReverseDFAEngine>);
```

#### Implementations

- `fn none() -> ReverseDFA` — [`ReverseDFA`](#reversedfa)

- `fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseDFA`](#reversedfa)

- `fn get(self: &Self, _input: &Input<'_>) -> Option<&ReverseDFAEngine>` — [`Input`](../../index.md), [`ReverseDFAEngine`](#reversedfaengine)

- `fn is_some(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug for ReverseDFA`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ReverseDFAEngine`

```rust
struct ReverseDFAEngine(());
```

#### Implementations

- `fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseDFAEngine>` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseDFAEngine`](#reversedfaengine)

- `fn try_search_half_rev_limited(self: &Self, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Debug for ReverseDFAEngine`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

