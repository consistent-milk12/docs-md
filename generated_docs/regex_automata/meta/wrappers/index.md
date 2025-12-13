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

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:49`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L49)*

#### Implementations

- <span id="pikevm-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<PikeVM, BuildError>` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`PikeVM`](#pikevm), [`BuildError`](../error/index.md#builderror)

- <span id="pikevm-create-cache"></span>`fn create_cache(&self) -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

- <span id="pikevm-get"></span>`fn get(&self) -> &PikeVMEngine` — [`PikeVMEngine`](#pikevmengine)

#### Trait Implementations

##### `impl Any for PikeVM`

- <span id="pikevm-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PikeVM`

- <span id="pikevm-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PikeVM`

- <span id="pikevm-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

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

##### `impl<U> TryFrom for PikeVM`

- <span id="pikevm-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pikevm-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PikeVM`

- <span id="pikevm-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pikevm-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PikeVMEngine`

```rust
struct PikeVMEngine(pikevm::PikeVM);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:71`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L71)*

#### Implementations

- <span id="pikevmengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<PikeVMEngine, BuildError>` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`PikeVMEngine`](#pikevmengine), [`BuildError`](../error/index.md#builderror)

- <span id="pikevmengine-is-match"></span>`fn is_match(&self, cache: &mut PikeVMCache, input: &Input<'_>) -> bool` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md#input)

- <span id="pikevmengine-search-slots"></span>`fn search_slots(&self, cache: &mut PikeVMCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="pikevmengine-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut PikeVMCache, input: &Input<'_>, patset: &mut PatternSet)` — [`PikeVMCache`](#pikevmcache), [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset)

#### Trait Implementations

##### `impl Any for PikeVMEngine`

- <span id="pikevmengine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PikeVMEngine`

- <span id="pikevmengine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PikeVMEngine`

- <span id="pikevmengine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PikeVMEngine`

- <span id="pikevmengine-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PikeVMEngine`

- <span id="pikevmengine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PikeVMEngine`

- <span id="pikevmengine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for PikeVMEngine`

- <span id="pikevmengine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pikevmengine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PikeVMEngine`

- <span id="pikevmengine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pikevmengine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PikeVMCache`

```rust
struct PikeVMCache(Option<pikevm::Cache>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:121`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L121)*

#### Implementations

- <span id="pikevmcache-none"></span>`fn none() -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

- <span id="pikevmcache-reset"></span>`fn reset(&mut self, builder: &PikeVM)` — [`PikeVM`](#pikevm)

- <span id="pikevmcache-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="pikevmcache-get"></span>`fn get(&mut self, vm: &pikevm::PikeVM) -> &mut pikevm::Cache` — [`PikeVM`](../../nfa/thompson/pikevm/index.md#pikevm), [`Cache`](../../nfa/thompson/pikevm/index.md#cache)

#### Trait Implementations

##### `impl Any for PikeVMCache`

- <span id="pikevmcache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PikeVMCache`

- <span id="pikevmcache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PikeVMCache`

- <span id="pikevmcache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PikeVMCache`

- <span id="pikevmcache-clone"></span>`fn clone(&self) -> PikeVMCache` — [`PikeVMCache`](#pikevmcache)

##### `impl CloneToUninit for PikeVMCache`

- <span id="pikevmcache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PikeVMCache`

- <span id="pikevmcache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PikeVMCache`

- <span id="pikevmcache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PikeVMCache`

- <span id="pikevmcache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PikeVMCache`

- <span id="pikevmcache-toowned-type-owned"></span>`type Owned = T`

- <span id="pikevmcache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pikevmcache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PikeVMCache`

- <span id="pikevmcache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pikevmcache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PikeVMCache`

- <span id="pikevmcache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pikevmcache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoundedBacktracker`

```rust
struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:142`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L142)*

#### Implementations

- <span id="boundedbacktracker-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<BoundedBacktracker, BuildError>` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="boundedbacktracker-create-cache"></span>`fn create_cache(&self) -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

- <span id="boundedbacktracker-get"></span>`fn get(&self, input: &Input<'_>) -> Option<&BoundedBacktrackerEngine>` — [`Input`](../../index.md#input), [`BoundedBacktrackerEngine`](#boundedbacktrackerengine)

#### Trait Implementations

##### `impl Any for BoundedBacktracker`

- <span id="boundedbacktracker-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoundedBacktracker`

- <span id="boundedbacktracker-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoundedBacktracker`

- <span id="boundedbacktracker-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

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

##### `impl<U> TryFrom for BoundedBacktracker`

- <span id="boundedbacktracker-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boundedbacktracker-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoundedBacktracker`

- <span id="boundedbacktracker-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boundedbacktracker-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoundedBacktrackerEngine`

```rust
struct BoundedBacktrackerEngine(backtrack::BoundedBacktracker);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:188-191`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L188-L191)*

#### Implementations

- <span id="boundedbacktrackerengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA) -> Result<Option<BoundedBacktrackerEngine>, BuildError>` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`BoundedBacktrackerEngine`](#boundedbacktrackerengine), [`BuildError`](../error/index.md#builderror)

- <span id="boundedbacktrackerengine-is-match"></span>`fn is_match(&self, cache: &mut BoundedBacktrackerCache, input: &Input<'_>) -> bool` — [`BoundedBacktrackerCache`](#boundedbacktrackercache), [`Input`](../../index.md#input)

- <span id="boundedbacktrackerengine-search-slots"></span>`fn search_slots(&self, cache: &mut BoundedBacktrackerCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`BoundedBacktrackerCache`](#boundedbacktrackercache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="boundedbacktrackerengine-max-haystack-len"></span>`fn max_haystack_len(&self) -> usize`

#### Trait Implementations

##### `impl Any for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boundedbacktrackerengine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoundedBacktrackerEngine`

- <span id="boundedbacktrackerengine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boundedbacktrackerengine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BoundedBacktrackerCache`

```rust
struct BoundedBacktrackerCache(Option<backtrack::Cache>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:282-285`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L282-L285)*

#### Implementations

- <span id="boundedbacktrackercache-none"></span>`fn none() -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

- <span id="boundedbacktrackercache-reset"></span>`fn reset(&mut self, builder: &BoundedBacktracker)` — [`BoundedBacktracker`](#boundedbacktracker)

- <span id="boundedbacktrackercache-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="boundedbacktrackercache-get"></span>`fn get(&mut self, bb: &backtrack::BoundedBacktracker) -> &mut backtrack::Cache` — [`BoundedBacktracker`](../../nfa/thompson/backtrack/index.md#boundedbacktracker), [`Cache`](../../nfa/thompson/backtrack/index.md#cache)

#### Trait Implementations

##### `impl Any for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-clone"></span>`fn clone(&self) -> BoundedBacktrackerCache` — [`BoundedBacktrackerCache`](#boundedbacktrackercache)

##### `impl CloneToUninit for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-toowned-type-owned"></span>`type Owned = T`

- <span id="boundedbacktrackercache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="boundedbacktrackercache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="boundedbacktrackercache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BoundedBacktrackerCache`

- <span id="boundedbacktrackercache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="boundedbacktrackercache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OnePass`

```rust
struct OnePass(Option<OnePassEngine>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:327`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L327)*

#### Implementations

- <span id="onepass-new"></span>`fn new(info: &RegexInfo, nfa: &NFA) -> OnePass` — [`RegexInfo`](../regex/index.md#regexinfo), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`OnePass`](#onepass)

- <span id="onepass-create-cache"></span>`fn create_cache(&self) -> OnePassCache` — [`OnePassCache`](#onepasscache)

- <span id="onepass-get"></span>`fn get(&self, input: &Input<'_>) -> Option<&OnePassEngine>` — [`Input`](../../index.md#input), [`OnePassEngine`](#onepassengine)

- <span id="onepass-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for OnePass`

- <span id="onepass-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnePass`

- <span id="onepass-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnePass`

- <span id="onepass-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for OnePass`

- <span id="onepass-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OnePass`

- <span id="onepass-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OnePass`

- <span id="onepass-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OnePass`

- <span id="onepass-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="onepass-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OnePass`

- <span id="onepass-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="onepass-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OnePassEngine`

```rust
struct OnePassEngine(onepass::DFA);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:355-358`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L355-L358)*

#### Implementations

- <span id="onepassengine-new"></span>`fn new(info: &RegexInfo, nfa: &NFA) -> Option<OnePassEngine>` — [`RegexInfo`](../regex/index.md#regexinfo), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`OnePassEngine`](#onepassengine)

- <span id="onepassengine-search-slots"></span>`fn search_slots(&self, cache: &mut OnePassCache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`OnePassCache`](#onepasscache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="onepassengine-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="onepassengine-get-nfa"></span>`fn get_nfa(&self) -> &NFA` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa)

#### Trait Implementations

##### `impl Any for OnePassEngine`

- <span id="onepassengine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnePassEngine`

- <span id="onepassengine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnePassEngine`

- <span id="onepassengine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for OnePassEngine`

- <span id="onepassengine-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OnePassEngine`

- <span id="onepassengine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OnePassEngine`

- <span id="onepassengine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for OnePassEngine`

- <span id="onepassengine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="onepassengine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OnePassEngine`

- <span id="onepassengine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="onepassengine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OnePassCache`

```rust
struct OnePassCache(Option<onepass::Cache>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:461-464`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L461-L464)*

#### Implementations

- <span id="onepasscache-none"></span>`fn none() -> OnePassCache` — [`OnePassCache`](#onepasscache)

- <span id="onepasscache-new"></span>`fn new(builder: &OnePass) -> OnePassCache` — [`OnePass`](#onepass), [`OnePassCache`](#onepasscache)

- <span id="onepasscache-reset"></span>`fn reset(&mut self, builder: &OnePass)` — [`OnePass`](#onepass)

- <span id="onepasscache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for OnePassCache`

- <span id="onepasscache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OnePassCache`

- <span id="onepasscache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OnePassCache`

- <span id="onepasscache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OnePassCache`

- <span id="onepasscache-clone"></span>`fn clone(&self) -> OnePassCache` — [`OnePassCache`](#onepasscache)

##### `impl CloneToUninit for OnePassCache`

- <span id="onepasscache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for OnePassCache`

- <span id="onepasscache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for OnePassCache`

- <span id="onepasscache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OnePassCache`

- <span id="onepasscache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for OnePassCache`

- <span id="onepasscache-toowned-type-owned"></span>`type Owned = T`

- <span id="onepasscache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="onepasscache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OnePassCache`

- <span id="onepasscache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="onepasscache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OnePassCache`

- <span id="onepasscache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="onepasscache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Hybrid`

```rust
struct Hybrid(Option<HybridEngine>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:509`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L509)*

#### Implementations

- <span id="hybrid-none"></span>`fn none() -> Hybrid` — [`Hybrid`](#hybrid)

- <span id="hybrid-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Hybrid` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`Hybrid`](#hybrid)

- <span id="hybrid-create-cache"></span>`fn create_cache(&self) -> HybridCache` — [`HybridCache`](#hybridcache)

- <span id="hybrid-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&HybridEngine>` — [`Input`](../../index.md#input), [`HybridEngine`](#hybridengine)

- <span id="hybrid-is-some"></span>`fn is_some(&self) -> bool`

#### Trait Implementations

##### `impl Any for Hybrid`

- <span id="hybrid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Hybrid`

- <span id="hybrid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Hybrid`

- <span id="hybrid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Hybrid`

- <span id="hybrid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Hybrid`

- <span id="hybrid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Hybrid`

- <span id="hybrid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Hybrid`

- <span id="hybrid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hybrid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Hybrid`

- <span id="hybrid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hybrid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HybridEngine`

```rust
struct HybridEngine(hybrid::regex::Regex);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:541-544`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L541-L544)*

#### Implementations

- <span id="hybridengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Option<HybridEngine>` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`HybridEngine`](#hybridengine)

- <span id="hybridengine-try-search"></span>`fn try_search(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<Match>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md#input), [`Match`](../../index.md#match), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="hybridengine-try-search-half-fwd"></span>`fn try_search_half_fwd(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="hybridengine-try-search-half-fwd-stopat"></span>`fn try_search_half_fwd_stopat(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="hybridengine-try-search-half-rev"></span>`fn try_search_half_rev(&self, cache: &mut HybridCache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="hybridengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut HybridCache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryError`](../error/index.md#retryerror)

- <span id="hybridengine-try-which-overlapping-matches"></span>`fn try_which_overlapping_matches(&self, cache: &mut HybridCache, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), RetryFailError>` — [`HybridCache`](#hybridcache), [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset), [`RetryFailError`](../error/index.md#retryfailerror)

#### Trait Implementations

##### `impl Any for HybridEngine`

- <span id="hybridengine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HybridEngine`

- <span id="hybridengine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HybridEngine`

- <span id="hybridengine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HybridEngine`

- <span id="hybridengine-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HybridEngine`

- <span id="hybridengine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HybridEngine`

- <span id="hybridengine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HybridEngine`

- <span id="hybridengine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hybridengine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HybridEngine`

- <span id="hybridengine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hybridengine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HybridCache`

```rust
struct HybridCache(Option<hybrid::regex::Cache>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:753-756`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L753-L756)*

#### Implementations

- <span id="hybridcache-none"></span>`fn none() -> HybridCache` — [`HybridCache`](#hybridcache)

- <span id="hybridcache-new"></span>`fn new(builder: &Hybrid) -> HybridCache` — [`Hybrid`](#hybrid), [`HybridCache`](#hybridcache)

- <span id="hybridcache-reset"></span>`fn reset(&mut self, builder: &Hybrid)` — [`Hybrid`](#hybrid)

- <span id="hybridcache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for HybridCache`

- <span id="hybridcache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HybridCache`

- <span id="hybridcache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HybridCache`

- <span id="hybridcache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for HybridCache`

- <span id="hybridcache-clone"></span>`fn clone(&self) -> HybridCache` — [`HybridCache`](#hybridcache)

##### `impl CloneToUninit for HybridCache`

- <span id="hybridcache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for HybridCache`

- <span id="hybridcache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HybridCache`

- <span id="hybridcache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HybridCache`

- <span id="hybridcache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for HybridCache`

- <span id="hybridcache-toowned-type-owned"></span>`type Owned = T`

- <span id="hybridcache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="hybridcache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for HybridCache`

- <span id="hybridcache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="hybridcache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HybridCache`

- <span id="hybridcache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="hybridcache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DFA`

```rust
struct DFA(Option<DFAEngine>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:801`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L801)*

#### Implementations

- <span id="dfa-none"></span>`fn none() -> DFA` — [`DFA`](#dfa)

- <span id="dfa-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> DFA` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`DFA`](#dfa)

- <span id="dfa-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&DFAEngine>` — [`Input`](../../index.md#input), [`DFAEngine`](#dfaengine)

- <span id="dfa-is-some"></span>`fn is_some(&self) -> bool`

- <span id="dfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for DFA`

- <span id="dfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DFA`

- <span id="dfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DFA`

- <span id="dfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DFA`

- <span id="dfa-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DFA`

- <span id="dfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DFA`

- <span id="dfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DFA`

- <span id="dfa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dfa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DFA`

- <span id="dfa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dfa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DFAEngine`

```rust
struct DFAEngine(());
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:833-836`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L833-L836)*

#### Implementations

- <span id="dfaengine-new"></span>`fn new(info: &RegexInfo, pre: Option<Prefilter>, nfa: &NFA, nfarev: &NFA) -> Option<DFAEngine>` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`DFAEngine`](#dfaengine)

- <span id="dfaengine-try-search"></span>`fn try_search(&self, input: &Input<'_>) -> Result<Option<Match>, RetryFailError>` — [`Input`](../../index.md#input), [`Match`](../../index.md#match), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="dfaengine-try-search-half-fwd"></span>`fn try_search_half_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="dfaengine-try-search-half-fwd-stopat"></span>`fn try_search_half_fwd_stopat(&self, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="dfaengine-try-search-half-rev"></span>`fn try_search_half_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="dfaengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryError`](../error/index.md#retryerror)

- <span id="dfaengine-try-which-overlapping-matches"></span>`fn try_which_overlapping_matches(&self, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), RetryFailError>` — [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="dfaengine-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for DFAEngine`

- <span id="dfaengine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DFAEngine`

- <span id="dfaengine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DFAEngine`

- <span id="dfaengine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DFAEngine`

- <span id="dfaengine-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DFAEngine`

- <span id="dfaengine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DFAEngine`

- <span id="dfaengine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DFAEngine`

- <span id="dfaengine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dfaengine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DFAEngine`

- <span id="dfaengine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dfaengine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReverseHybrid`

```rust
struct ReverseHybrid(Option<ReverseHybridEngine>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:1059`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L1059)*

#### Implementations

- <span id="reversehybrid-none"></span>`fn none() -> ReverseHybrid` — [`ReverseHybrid`](#reversehybrid)

- <span id="reversehybrid-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid` — [`RegexInfo`](../regex/index.md#regexinfo), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`ReverseHybrid`](#reversehybrid)

- <span id="reversehybrid-create-cache"></span>`fn create_cache(&self) -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

- <span id="reversehybrid-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&ReverseHybridEngine>` — [`Input`](../../index.md#input), [`ReverseHybridEngine`](#reversehybridengine)

#### Trait Implementations

##### `impl Any for ReverseHybrid`

- <span id="reversehybrid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseHybrid`

- <span id="reversehybrid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseHybrid`

- <span id="reversehybrid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReverseHybrid`

- <span id="reversehybrid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseHybrid`

- <span id="reversehybrid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseHybrid`

- <span id="reversehybrid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReverseHybrid`

- <span id="reversehybrid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reversehybrid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseHybrid`

- <span id="reversehybrid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reversehybrid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReverseHybridEngine`

```rust
struct ReverseHybridEngine(hybrid::dfa::DFA);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:1085-1088`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L1085-L1088)*

#### Implementations

- <span id="reversehybridengine-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseHybridEngine>` — [`RegexInfo`](../regex/index.md#regexinfo), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`ReverseHybridEngine`](#reversehybridengine)

- <span id="reversehybridengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut ReverseHybridCache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`ReverseHybridCache`](#reversehybridcache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryError`](../error/index.md#retryerror)

#### Trait Implementations

##### `impl Any for ReverseHybridEngine`

- <span id="reversehybridengine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseHybridEngine`

- <span id="reversehybridengine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseHybridEngine`

- <span id="reversehybridengine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReverseHybridEngine`

- <span id="reversehybridengine-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseHybridEngine`

- <span id="reversehybridengine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseHybridEngine`

- <span id="reversehybridengine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReverseHybridEngine`

- <span id="reversehybridengine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reversehybridengine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseHybridEngine`

- <span id="reversehybridengine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reversehybridengine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReverseHybridCache`

```rust
struct ReverseHybridCache(Option<hybrid::dfa::Cache>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:1158-1161`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L1158-L1161)*

#### Implementations

- <span id="reversehybridcache-none"></span>`fn none() -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

- <span id="reversehybridcache-new"></span>`fn new(builder: &ReverseHybrid) -> ReverseHybridCache` — [`ReverseHybrid`](#reversehybrid), [`ReverseHybridCache`](#reversehybridcache)

- <span id="reversehybridcache-reset"></span>`fn reset(&mut self, builder: &ReverseHybrid)` — [`ReverseHybrid`](#reversehybrid)

- <span id="reversehybridcache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for ReverseHybridCache`

- <span id="reversehybridcache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseHybridCache`

- <span id="reversehybridcache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseHybridCache`

- <span id="reversehybridcache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ReverseHybridCache`

- <span id="reversehybridcache-clone"></span>`fn clone(&self) -> ReverseHybridCache` — [`ReverseHybridCache`](#reversehybridcache)

##### `impl CloneToUninit for ReverseHybridCache`

- <span id="reversehybridcache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ReverseHybridCache`

- <span id="reversehybridcache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseHybridCache`

- <span id="reversehybridcache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseHybridCache`

- <span id="reversehybridcache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ReverseHybridCache`

- <span id="reversehybridcache-toowned-type-owned"></span>`type Owned = T`

- <span id="reversehybridcache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="reversehybridcache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ReverseHybridCache`

- <span id="reversehybridcache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reversehybridcache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseHybridCache`

- <span id="reversehybridcache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reversehybridcache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReverseDFA`

```rust
struct ReverseDFA(Option<ReverseDFAEngine>);
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:1206`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L1206)*

#### Implementations

- <span id="reversedfa-none"></span>`fn none() -> ReverseDFA` — [`ReverseDFA`](#reversedfa)

- <span id="reversedfa-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA` — [`RegexInfo`](../regex/index.md#regexinfo), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`ReverseDFA`](#reversedfa)

- <span id="reversedfa-get"></span>`fn get(&self, _input: &Input<'_>) -> Option<&ReverseDFAEngine>` — [`Input`](../../index.md#input), [`ReverseDFAEngine`](#reversedfaengine)

- <span id="reversedfa-is-some"></span>`fn is_some(&self) -> bool`

- <span id="reversedfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for ReverseDFA`

- <span id="reversedfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseDFA`

- <span id="reversedfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseDFA`

- <span id="reversedfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReverseDFA`

- <span id="reversedfa-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseDFA`

- <span id="reversedfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseDFA`

- <span id="reversedfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReverseDFA`

- <span id="reversedfa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reversedfa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseDFA`

- <span id="reversedfa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reversedfa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReverseDFAEngine`

```rust
struct ReverseDFAEngine(());
```

*Defined in [`regex-automata-0.4.13/src/meta/wrappers.rs:1233-1236`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/wrappers.rs#L1233-L1236)*

#### Implementations

- <span id="reversedfaengine-new"></span>`fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseDFAEngine>` — [`RegexInfo`](../regex/index.md#regexinfo), [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`ReverseDFAEngine`](#reversedfaengine)

- <span id="reversedfaengine-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryError`](../error/index.md#retryerror)

- <span id="reversedfaengine-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for ReverseDFAEngine`

- <span id="reversedfaengine-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseDFAEngine`

- <span id="reversedfaengine-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseDFAEngine`

- <span id="reversedfaengine-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReverseDFAEngine`

- <span id="reversedfaengine-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseDFAEngine`

- <span id="reversedfaengine-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseDFAEngine`

- <span id="reversedfaengine-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for ReverseDFAEngine`

- <span id="reversedfaengine-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reversedfaengine-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseDFAEngine`

- <span id="reversedfaengine-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reversedfaengine-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

