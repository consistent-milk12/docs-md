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

## Contents

- [Structs](#structs)
  - [`PikeVM`](#pikevm)
  - [`PikeVMEngine`](#pikevmengine)
  - [`PikeVMCache`](#pikevmcache)
  - [`BoundedBacktracker`](#boundedbacktracker)
  - [`BoundedBacktrackerEngine`](#boundedbacktrackerengine)
  - [`BoundedBacktrackerCache`](#boundedbacktrackercache)
  - [`OnePass`](#onepass)
  - [`OnePassEngine`](#onepassengine)
  - [`OnePassCache`](#onepasscache)
  - [`Hybrid`](#hybrid)
  - [`HybridEngine`](#hybridengine)
  - [`HybridCache`](#hybridcache)
  - [`DFA`](#dfa)
  - [`DFAEngine`](#dfaengine)
  - [`ReverseHybrid`](#reversehybrid)
  - [`ReverseHybridEngine`](#reversehybridengine)
  - [`ReverseHybridCache`](#reversehybridcache)
  - [`ReverseDFA`](#reversedfa)
  - [`ReverseDFAEngine`](#reversedfaengine)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PikeVM`](#pikevm) | struct |  |
| [`PikeVMEngine`](#pikevmengine) | struct |  |
| [`PikeVMCache`](#pikevmcache) | struct |  |
| [`BoundedBacktracker`](#boundedbacktracker) | struct |  |
| [`BoundedBacktrackerEngine`](#boundedbacktrackerengine) | struct |  |
| [`BoundedBacktrackerCache`](#boundedbacktrackercache) | struct |  |
| [`OnePass`](#onepass) | struct |  |
| [`OnePassEngine`](#onepassengine) | struct |  |
| [`OnePassCache`](#onepasscache) | struct |  |
| [`Hybrid`](#hybrid) | struct |  |
| [`HybridEngine`](#hybridengine) | struct |  |
| [`HybridCache`](#hybridcache) | struct |  |
| [`DFA`](#dfa) | struct |  |
| [`DFAEngine`](#dfaengine) | struct |  |
| [`ReverseHybrid`](#reversehybrid) | struct |  |
| [`ReverseHybridEngine`](#reversehybridengine) | struct |  |
| [`ReverseHybridCache`](#reversehybridcache) | struct |  |
| [`ReverseDFA`](#reversedfa) | struct |  |
| [`ReverseDFAEngine`](#reversedfaengine) | struct |  |

## Structs

### `PikeVM`

```rust
struct PikeVM(PikeVMEngine);
```

#### Implementations

- <span id="pikevm-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<PikeVM, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`PikeVM`](#pikevm), [`BuildError`](../index.md)

- <span id="pikevm-create-cache"></span>`fn create_cache(&self) -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

- <span id="pikevm-get"></span>`fn get(&self) -> &PikeVMEngine` — [`PikeVMEngine`](#pikevmengine)

#### Trait Implementations

##### `impl Debug for PikeVM`

- <span id="pikevm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PikeVMEngine`

```rust
struct PikeVMEngine(pikevm::PikeVM);
```

#### Implementations

- <span id="pikevmengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<PikeVMEngine, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`PikeVMEngine`](#pikevmengine), [`BuildError`](../index.md)

- <span id="pikevmengine-is-match"></span>`fn is_match(&self, cache: &mut PikeVMCache, input: &Input<'_>) -> bool` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md)

- <span id="pikevmengine-search-slots"></span>`fn search_slots(&self, cache: &mut PikeVMCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../index.md)

- <span id="pikevmengine-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut PikeVMCache, input: &Input<'_>, patset: &mut PatternSet)` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md), [`PatternSet`](../../index.md)

#### Trait Implementations

##### `impl Debug for PikeVMEngine`

- <span id="pikevmengine-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PikeVMCache`

```rust
struct PikeVMCache(Option<pikevm::Cache>);
```

#### Implementations

- <span id="pikevmcache-none"></span>`fn none() -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

- <span id="pikevmcache-reset"></span>`fn reset(&mut self, builder: &PikeVM)` — [`PikeVM`](#pikevm)

- <span id="pikevmcache-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="pikevmcache-get"></span>`fn get(&mut self, vm: &pikevm::PikeVM) -> &mut pikevm::Cache` — [`PikeVM`](../../nfa/thompson/pikevm/index.md), [`Cache`](../../nfa/thompson/pikevm/index.md)

#### Trait Implementations

##### `impl Clone for PikeVMCache`

- <span id="pikevmcache-clone"></span>`fn clone(&self) -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

##### `impl Debug for PikeVMCache`

- <span id="pikevmcache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BoundedBacktracker`

```rust
struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
```

#### Implementations

- <span id="boundedbacktracker-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<BoundedBacktracker, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../index.md)

- <span id="boundedbacktracker-create-cache"></span>`fn create_cache(&self) -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

- <span id="boundedbacktracker-get"></span>`fn get(&self, input: &Input<'_>) -> Option<&BoundedBacktrackerEngine>` — [`Input`](../../index.md), [`BoundedBacktrackerEngine`](#boundedbacktrackerengine)

#### Trait Implementations

##### `impl Debug for BoundedBacktracker`

- <span id="boundedbacktracker-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BoundedBacktrackerEngine`

```rust
struct BoundedBacktrackerEngine(backtrack::BoundedBacktracker);
```

#### Implementations

- <span id="boundedbacktrackerengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<Option<BoundedBacktrackerEngine>, BuildError>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`BoundedBacktrackerEngine`](#boundedbacktrackerengine), [`BuildError`](../index.md)

- <span id="boundedbacktrackerengine-is-match"></span>`fn is_match(&self, cache: &mut BoundedBacktrackerCache, input: &Input<'_>) -> bool` — [`BoundedBacktrackerCache`](#boundedbacktrackercache), [`Input`](../../index.md)

- <span id="boundedbacktrackerengine-search-slots"></span>`fn search_slots(&self, cache: &mut BoundedBacktrackerCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`BoundedBacktrackerCache`](#boundedbacktrackercache), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../index.md)

- <span id="boundedbacktrackerengine-max-haystack-len"></span>`fn max_haystack_len(&self) -> usize`

#### Trait Implementations

##### `impl Debug for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BoundedBacktrackerCache`

```rust
struct BoundedBacktrackerCache(Option<backtrack::Cache>);
```

#### Implementations

- <span id="boundedbacktrackercache-none"></span>`fn none() -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

- <span id="boundedbacktrackercache-reset"></span>`fn reset(&mut self, builder: &BoundedBacktracker)` — [`BoundedBacktracker`](#boundedbacktracker)

- <span id="boundedbacktrackercache-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="boundedbacktrackercache-get"></span>`fn get(&mut self, bb: &backtrack::BoundedBacktracker) -> &mut backtrack::Cache` — [`BoundedBacktracker`](../../nfa/thompson/backtrack/index.md), [`Cache`](../../nfa/thompson/backtrack/index.md)

#### Trait Implementations

##### `impl Clone for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-clone"></span>`fn clone(&self) -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

##### `impl Debug for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `OnePass`

```rust
struct OnePass(Option<OnePassEngine>);
```

#### Implementations

- <span id="onepass-new"></span>`fn new(info: &RegexInfo, nfa: &NFA) -> OnePass` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`OnePass`](#onepass)

- <span id="onepass-create-cache"></span>`fn create_cache(&self) -> OnePassCache` — [`OnePassCache`](#onepasscache)

- <span id="onepass-get"></span>`fn get(&self, input: &Input<'_>) -> Option<&OnePassEngine>` — [`Input`](../../index.md), [`OnePassEngine`](#onepassengine)

- <span id="onepass-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Debug for OnePass`

- <span id="onepass-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `OnePassEngine`

```rust
struct OnePassEngine(onepass::DFA);
```

#### Implementations

- <span id="onepassengine-new"></span>`fn new(info: &RegexInfo, nfa: &NFA) -> Option<OnePassEngine>` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`OnePassEngine`](#onepassengine)

- <span id="onepassengine-search-slots"></span>`fn search_slots(&self, cache: &mut OnePassCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`OnePassCache`](#onepasscache), [`Input`](../../index.md), [`NonMaxUsize`](../../util/primitives/index.md), [`PatternID`](../../index.md)

- <span id="onepassengine-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="onepassengine-get-nfa"></span>`fn get_nfa(&self) -> &NFA` — [`NFA`](../../nfa/thompson/index.md)

#### Trait Implementations

##### `impl Debug for OnePassEngine`

- <span id="onepassengine-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `OnePassCache`

```rust
struct OnePassCache(Option<onepass::Cache>);
```

#### Implementations

- <span id="onepasscache-none"></span>`fn none() -> OnePassCache` — [`OnePassCache`](#onepasscache)

- <span id="onepasscache-new"></span>`fn new(builder: &OnePass) -> OnePassCache` — [`OnePass`](#onepass), [`OnePassCache`](#onepasscache)

- <span id="onepasscache-reset"></span>`fn reset(&mut self, builder: &OnePass)` — [`OnePass`](#onepass)

- <span id="onepasscache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for OnePassCache`

- <span id="onepasscache-clone"></span>`fn clone(&self) -> OnePassCache` — [`OnePassCache`](#onepasscache)

##### `impl Debug for OnePassCache`

- <span id="onepasscache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Hybrid`

```rust
struct Hybrid(Option<HybridEngine>);
```

#### Implementations

- <span id="hybrid-none"></span>`fn none() -> Hybrid` — [`Hybrid`](#hybrid)

- <span id="hybrid-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Hybrid` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`Hybrid`](#hybrid)

- <span id="hybrid-create-cache"></span>`fn create_cache(&self) -> HybridCache` — [`HybridCache`](#hybridcache)

- <span id="hybrid-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&HybridEngine>` — [`Input`](../../index.md), [`HybridEngine`](#hybridengine)

- <span id="hybrid-is-some"></span>`fn is_some(&self) -> bool`

#### Trait Implementations

##### `impl Debug for Hybrid`

- <span id="hybrid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HybridEngine`

```rust
struct HybridEngine(hybrid::regex::Regex);
```

#### Implementations

- <span id="hybridengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Option<HybridEngine>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`HybridEngine`](#hybridengine)

- <span id="hybridengine-try-search"></span>`fn try_search(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<Match>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`Match`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="hybridengine-try-search-half-fwd"></span>`fn try_search_half_fwd(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="hybridengine-try-search-half-fwd-stopat"></span>`fn try_search_half_fwd_stopat(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="hybridengine-try-search-half-rev"></span>`fn try_search_half_rev(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="hybridengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut HybridCache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- <span id="hybridengine-try-which-overlapping-matches"></span>`fn try_which_overlapping_matches(&self, cache: &mut HybridCache, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md), [`PatternSet`](../../index.md), [`RetryFailError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for HybridEngine`

- <span id="hybridengine-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `HybridCache`

```rust
struct HybridCache(Option<hybrid::regex::Cache>);
```

#### Implementations

- <span id="hybridcache-none"></span>`fn none() -> HybridCache` — [`HybridCache`](#hybridcache)

- <span id="hybridcache-new"></span>`fn new(builder: &Hybrid) -> HybridCache` — [`Hybrid`](#hybrid), [`HybridCache`](#hybridcache)

- <span id="hybridcache-reset"></span>`fn reset(&mut self, builder: &Hybrid)` — [`Hybrid`](#hybrid)

- <span id="hybridcache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for HybridCache`

- <span id="hybridcache-clone"></span>`fn clone(&self) -> HybridCache` — [`HybridCache`](#hybridcache)

##### `impl Debug for HybridCache`

- <span id="hybridcache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DFA`

```rust
struct DFA(Option<DFAEngine>);
```

#### Implementations

- <span id="dfa-none"></span>`fn none() -> DFA` — [`DFA`](#dfa)

- <span id="dfa-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> DFA` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`DFA`](#dfa)

- <span id="dfa-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&DFAEngine>` — [`Input`](../../index.md), [`DFAEngine`](#dfaengine)

- <span id="dfa-is-some"></span>`fn is_some(&self) -> bool`

- <span id="dfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Debug for DFA`

- <span id="dfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DFAEngine`

```rust
struct DFAEngine(());
```

#### Implementations

- <span id="dfaengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Option<DFAEngine>` — [`RegexInfo`](../regex/index.md), [`Prefilter`](../../util/prefilter/index.md), [`NFA`](../../nfa/thompson/index.md), [`DFAEngine`](#dfaengine)

- <span id="dfaengine-try-search"></span>`fn try_search(&self, input: &Input<'_>) -> Result<Option<Match>, RetryFailError>` — [`Input`](../../index.md), [`Match`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="dfaengine-try-search-half-fwd"></span>`fn try_search_half_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="dfaengine-try-search-half-fwd-stopat"></span>`fn try_search_half_fwd_stopat(&self, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="dfaengine-try-search-half-rev"></span>`fn try_search_half_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="dfaengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- <span id="dfaengine-try-which-overlapping-matches"></span>`fn try_which_overlapping_matches(&self, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), RetryFailError>` — [`Input`](../../index.md), [`PatternSet`](../../index.md), [`RetryFailError`](../error/index.md)

- <span id="dfaengine-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Debug for DFAEngine`

- <span id="dfaengine-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReverseHybrid`

```rust
struct ReverseHybrid(Option<ReverseHybridEngine>);
```

#### Implementations

- <span id="reversehybrid-none"></span>`fn none() -> ReverseHybrid` — [`ReverseHybrid`](#reversehybrid)

- <span id="reversehybrid-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseHybrid`](#reversehybrid)

- <span id="reversehybrid-create-cache"></span>`fn create_cache(&self) -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

- <span id="reversehybrid-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&ReverseHybridEngine>` — [`Input`](../../index.md), [`ReverseHybridEngine`](#reversehybridengine)

#### Trait Implementations

##### `impl Debug for ReverseHybrid`

- <span id="reversehybrid-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReverseHybridEngine`

```rust
struct ReverseHybridEngine(hybrid::dfa::DFA);
```

#### Implementations

- <span id="reversehybridengine-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseHybridEngine>` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseHybridEngine`](#reversehybridengine)

- <span id="reversehybridengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut ReverseHybridCache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`ReverseHybridCache`](#reversehybridcache), [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

#### Trait Implementations

##### `impl Debug for ReverseHybridEngine`

- <span id="reversehybridengine-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReverseHybridCache`

```rust
struct ReverseHybridCache(Option<hybrid::dfa::Cache>);
```

#### Implementations

- <span id="reversehybridcache-none"></span>`fn none() -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

- <span id="reversehybridcache-new"></span>`fn new(builder: &ReverseHybrid) -> ReverseHybridCache` — [`ReverseHybrid`](#reversehybrid), [`ReverseHybridCache`](#reversehybridcache)

- <span id="reversehybridcache-reset"></span>`fn reset(&mut self, builder: &ReverseHybrid)` — [`ReverseHybrid`](#reversehybrid)

- <span id="reversehybridcache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for ReverseHybridCache`

- <span id="reversehybridcache-clone"></span>`fn clone(&self) -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

##### `impl Debug for ReverseHybridCache`

- <span id="reversehybridcache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReverseDFA`

```rust
struct ReverseDFA(Option<ReverseDFAEngine>);
```

#### Implementations

- <span id="reversedfa-none"></span>`fn none() -> ReverseDFA` — [`ReverseDFA`](#reversedfa)

- <span id="reversedfa-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseDFA`](#reversedfa)

- <span id="reversedfa-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&ReverseDFAEngine>` — [`Input`](../../index.md), [`ReverseDFAEngine`](#reversedfaengine)

- <span id="reversedfa-is-some"></span>`fn is_some(&self) -> bool`

- <span id="reversedfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Debug for ReverseDFA`

- <span id="reversedfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ReverseDFAEngine`

```rust
struct ReverseDFAEngine(());
```

#### Implementations

- <span id="reversedfaengine-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseDFAEngine>` — [`RegexInfo`](../regex/index.md), [`NFA`](../../nfa/thompson/index.md), [`ReverseDFAEngine`](#reversedfaengine)

- <span id="reversedfaengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Input`](../../index.md), [`HalfMatch`](../../index.md), [`RetryError`](../error/index.md)

- <span id="reversedfaengine-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Debug for ReverseDFAEngine`

- <span id="reversedfaengine-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

