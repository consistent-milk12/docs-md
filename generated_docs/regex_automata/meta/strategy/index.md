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
  - [`copy_match_to_slots`](#copy-match-to-slots)

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
| [`copy_match_to_slots`](#copy-match-to-slots) | fn | Copies the offsets in the given match to the corresponding positions in `slots`. |

## Structs

### `Pre<P>`

```rust
struct Pre<P> {
    pre: P,
    group_info: crate::util::captures::GroupInfo,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:189-192`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L189-L192)*

#### Implementations

- <span id="pre-new"></span>`fn new(pre: P) -> Arc<dyn Strategy>` — [`Strategy`](#strategy)

#### Trait Implementations

##### `impl Any for Pre<P>`

- <span id="pre-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pre<P>`

- <span id="pre-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pre<P>`

- <span id="pre-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone> Clone for Pre<P>`

- <span id="pre-clone"></span>`fn clone(&self) -> Pre<P>` — [`Pre`](#pre)

##### `impl CloneToUninit for Pre<P>`

- <span id="pre-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug> Debug for Pre<P>`

- <span id="pre-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Pre<P>`

- <span id="pre-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Pre<P>`

- <span id="pre-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<P: PrefilterI> Strategy for Pre<P>`

- <span id="pre-strategy-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md#groupinfo)

- <span id="pre-strategy-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md#cache)

- <span id="pre-strategy-reset-cache"></span>`fn reset_cache(&self, _cache: &mut Cache)` — [`Cache`](../regex/index.md#cache)

- <span id="pre-strategy-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="pre-strategy-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="pre-strategy-search"></span>`fn search(&self, _cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match)

- <span id="pre-strategy-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch)

- <span id="pre-strategy-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input)

- <span id="pre-strategy-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="pre-strategy-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset)

##### `impl ToOwned for Pre<P>`

- <span id="pre-toowned-type-owned"></span>`type Owned = T`

- <span id="pre-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pre-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Pre<P>`

- <span id="pre-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pre-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pre<P>`

- <span id="pre-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pre-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:443-453`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L443-L453)*

#### Implementations

- <span id="core-new"></span>`fn new(info: RegexInfo, pre: Option<Prefilter>, hirs: &[&Hir]) -> Result<Core, BuildError>` — [`RegexInfo`](../regex/index.md#regexinfo), [`Prefilter`](../../util/prefilter/index.md#prefilter), [`Core`](#core), [`BuildError`](../error/index.md#builderror)

- <span id="core-try-search-mayfail"></span>`fn try_search_mayfail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Result<Option<Match>, RetryFailError>>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="core-search-nofail"></span>`fn search_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match)

- <span id="core-search-half-nofail"></span>`fn search_half_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch)

- <span id="core-search-slots-nofail"></span>`fn search_slots_nofail(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="core-is-match-nofail"></span>`fn is_match_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input)

- <span id="core-is-capture-search-needed"></span>`fn is_capture_search_needed(&self, slots_len: usize) -> bool`

#### Trait Implementations

##### `impl Any for Core`

- <span id="core-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Core`

- <span id="core-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Core`

- <span id="core-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Core`

- <span id="core-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Core`

- <span id="core-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Core`

- <span id="core-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Strategy for Core`

- <span id="core-strategy-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md#groupinfo)

- <span id="core-strategy-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md#cache)

- <span id="core-strategy-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md#cache)

- <span id="core-strategy-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="core-strategy-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="core-strategy-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match)

- <span id="core-strategy-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch)

- <span id="core-strategy-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input)

- <span id="core-strategy-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="core-strategy-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset)

##### `impl<U> TryFrom for Core`

- <span id="core-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="core-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Core`

- <span id="core-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="core-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReverseAnchored`

```rust
struct ReverseAnchored {
    core: Core,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:904-906`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L904-L906)*

#### Implementations

- <span id="reverseanchored-new"></span>`fn new(core: Core) -> Result<ReverseAnchored, Core>` — [`Core`](#core), [`ReverseAnchored`](#reverseanchored)

- <span id="reverseanchored-try-search-half-anchored-rev"></span>`fn try_search_half_anchored_rev(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

#### Trait Implementations

##### `impl Any for ReverseAnchored`

- <span id="reverseanchored-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseAnchored`

- <span id="reverseanchored-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseAnchored`

- <span id="reverseanchored-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReverseAnchored`

- <span id="reverseanchored-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseAnchored`

- <span id="reverseanchored-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseAnchored`

- <span id="reverseanchored-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Strategy for ReverseAnchored`

- <span id="reverseanchored-strategy-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md#groupinfo)

- <span id="reverseanchored-strategy-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md#cache)

- <span id="reverseanchored-strategy-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md#cache)

- <span id="reverseanchored-strategy-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="reverseanchored-strategy-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="reverseanchored-strategy-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match)

- <span id="reverseanchored-strategy-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch)

- <span id="reverseanchored-strategy-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input)

- <span id="reverseanchored-strategy-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="reverseanchored-strategy-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset)

##### `impl<U> TryFrom for ReverseAnchored`

- <span id="reverseanchored-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reverseanchored-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseAnchored`

- <span id="reverseanchored-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reverseanchored-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReverseSuffix`

```rust
struct ReverseSuffix {
    core: Core,
    pre: crate::util::prefilter::Prefilter,
}
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:1116-1119`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L1116-L1119)*

#### Implementations

- <span id="reversesuffix-new"></span>`fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseSuffix, Core>` — [`Core`](#core), [`ReverseSuffix`](#reversesuffix)

- <span id="reversesuffix-try-search-half-start"></span>`fn try_search_half_start(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryError`](../error/index.md#retryerror)

- <span id="reversesuffix-try-search-half-fwd"></span>`fn try_search_half_fwd(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, RetryFailError>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="reversesuffix-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut Cache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryError`](../error/index.md#retryerror)

#### Trait Implementations

##### `impl Any for ReverseSuffix`

- <span id="reversesuffix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseSuffix`

- <span id="reversesuffix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseSuffix`

- <span id="reversesuffix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReverseSuffix`

- <span id="reversesuffix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseSuffix`

- <span id="reversesuffix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseSuffix`

- <span id="reversesuffix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Strategy for ReverseSuffix`

- <span id="reversesuffix-strategy-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md#groupinfo)

- <span id="reversesuffix-strategy-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md#cache)

- <span id="reversesuffix-strategy-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md#cache)

- <span id="reversesuffix-strategy-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="reversesuffix-strategy-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="reversesuffix-strategy-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match)

- <span id="reversesuffix-strategy-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch)

- <span id="reversesuffix-strategy-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input)

- <span id="reversesuffix-strategy-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="reversesuffix-strategy-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset)

##### `impl<U> TryFrom for ReverseSuffix`

- <span id="reversesuffix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reversesuffix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseSuffix`

- <span id="reversesuffix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reversesuffix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:1494-1500`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L1494-L1500)*

#### Implementations

- <span id="reverseinner-new"></span>`fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseInner, Core>` — [`Core`](#core), [`ReverseInner`](#reverseinner)

- <span id="reverseinner-try-search-full"></span>`fn try_search_full(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<Match>, RetryError>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match), [`RetryError`](../error/index.md#retryerror)

- <span id="reverseinner-try-search-half-fwd-stopat"></span>`fn try_search_half_fwd_stopat(&self, cache: &mut Cache, input: &Input<'_>) -> Result<Result<HalfMatch, usize>, RetryFailError>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryFailError`](../error/index.md#retryfailerror)

- <span id="reverseinner-try-search-half-rev-limited"></span>`fn try_search_half_rev_limited(&self, cache: &mut Cache, input: &Input<'_>, min_start: usize) -> Result<Option<HalfMatch>, RetryError>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch), [`RetryError`](../error/index.md#retryerror)

#### Trait Implementations

##### `impl Any for ReverseInner`

- <span id="reverseinner-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReverseInner`

- <span id="reverseinner-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReverseInner`

- <span id="reverseinner-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ReverseInner`

- <span id="reverseinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReverseInner`

- <span id="reverseinner-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReverseInner`

- <span id="reverseinner-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Strategy for ReverseInner`

- <span id="reverseinner-strategy-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md#groupinfo)

- <span id="reverseinner-strategy-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](../regex/index.md#cache)

- <span id="reverseinner-strategy-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](../regex/index.md#cache)

- <span id="reverseinner-strategy-is-accelerated"></span>`fn is_accelerated(&self) -> bool`

- <span id="reverseinner-strategy-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="reverseinner-strategy-search"></span>`fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`Match`](../../index.md#match)

- <span id="reverseinner-strategy-search-half"></span>`fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`HalfMatch`](../../index.md#halfmatch)

- <span id="reverseinner-strategy-is-match"></span>`fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input)

- <span id="reverseinner-strategy-search-slots"></span>`fn search_slots(&self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`NonMaxUsize`](../../util/primitives/index.md#nonmaxusize), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="reverseinner-strategy-which-overlapping-matches"></span>`fn which_overlapping_matches(&self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)` — [`Cache`](../regex/index.md#cache), [`Input`](../../index.md#input), [`PatternSet`](../../index.md#patternset)

##### `impl<U> TryFrom for ReverseInner`

- <span id="reverseinner-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="reverseinner-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReverseInner`

- <span id="reverseinner-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="reverseinner-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Strategy`

```rust
trait Strategy: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static { ... }
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:40-76`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L40-L76)*

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

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:78-186`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L78-L186)*

### `copy_match_to_slots`

```rust
fn copy_match_to_slots(m: crate::util::search::Match, slots: &mut [Option<crate::util::primitives::NonMaxUsize>])
```

*Defined in [`regex-automata-0.4.13/src/meta/strategy.rs:1896-1905`](../../../../.source_1765633015/regex-automata-0.4.13/src/meta/strategy.rs#L1896-L1905)*

Copies the offsets in the given match to the corresponding positions in
`slots`.

In effect, this sets the slots corresponding to the implicit group for the
pattern in the given match. If the indices for the corresponding slots do
not exist, then no slots are set.

This is useful when the caller provides slots (or captures), but you use a
regex engine that doesn't operate on slots (like a lazy DFA). This function
lets you map the match you get back to the slots provided by the caller.

