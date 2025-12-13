*[memchr](../../index.md) / [memmem](../index.md) / [searcher](index.md)*

---

# Module `searcher`

## Contents

- [Structs](#structs)
  - [`Searcher`](#searcher)
  - [`TwoWayWithPrefilter`](#twowaywithprefilter)
  - [`SearcherRev`](#searcherrev)
  - [`Prefilter`](#prefilter)
  - [`PrefilterState`](#prefilterstate)
  - [`Pre`](#pre)
- [Enums](#enums)
  - [`SearcherRevKind`](#searcherrevkind)
  - [`PrefilterConfig`](#prefilterconfig)
- [Functions](#functions)
  - [`searcher_kind_empty`](#searcher-kind-empty)
  - [`searcher_kind_one_byte`](#searcher-kind-one-byte)
  - [`searcher_kind_two_way`](#searcher-kind-two-way)
  - [`searcher_kind_two_way_with_prefilter`](#searcher-kind-two-way-with-prefilter)
  - [`searcher_kind_sse2`](#searcher-kind-sse2)
  - [`searcher_kind_avx2`](#searcher-kind-avx2)
  - [`prefilter_kind_fallback`](#prefilter-kind-fallback)
  - [`prefilter_kind_sse2`](#prefilter-kind-sse2)
  - [`prefilter_kind_avx2`](#prefilter-kind-avx2)
  - [`do_packed_search`](#do-packed-search)
- [Type Aliases](#type-aliases)
  - [`SearcherKindFn`](#searcherkindfn)
  - [`PrefilterKindFn`](#prefilterkindfn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Searcher`](#searcher) | struct | A "meta" substring searcher. |
| [`TwoWayWithPrefilter`](#twowaywithprefilter) | struct | A two-way substring searcher with a prefilter. |
| [`SearcherRev`](#searcherrev) | struct | A reverse substring searcher. |
| [`Prefilter`](#prefilter) | struct | The implementation of a prefilter. |
| [`PrefilterState`](#prefilterstate) | struct | PrefilterState tracks state associated with the effectiveness of a prefilter. |
| [`Pre`](#pre) | struct | A combination of prefilter effectiveness state and the prefilter itself. |
| [`SearcherRevKind`](#searcherrevkind) | enum | The kind of the reverse searcher. |
| [`PrefilterConfig`](#prefilterconfig) | enum | Prefilter controls whether heuristics are used to accelerate searching. |
| [`searcher_kind_empty`](#searcher-kind-empty) | fn | Reads from the `empty` field of `SearcherKind` to handle the case of searching for the empty needle. |
| [`searcher_kind_one_byte`](#searcher-kind-one-byte) | fn | Reads from the `one_byte` field of `SearcherKind` to handle the case of searching for a single byte needle. |
| [`searcher_kind_two_way`](#searcher-kind-two-way) | fn | Reads from the `two_way` field of `SearcherKind` to handle the case of searching for an arbitrary needle without prefilter acceleration. |
| [`searcher_kind_two_way_with_prefilter`](#searcher-kind-two-way-with-prefilter) | fn | Reads from the `two_way_with_prefilter` field of `SearcherKind` to handle the case of searching for an arbitrary needle with prefilter acceleration. |
| [`searcher_kind_sse2`](#searcher-kind-sse2) | fn | Reads from the `sse2` field of `SearcherKind` to execute the x86_64 SSE2 vectorized substring search implementation. |
| [`searcher_kind_avx2`](#searcher-kind-avx2) | fn | Reads from the `avx2` field of `SearcherKind` to execute the x86_64 AVX2 vectorized substring search implementation. |
| [`prefilter_kind_fallback`](#prefilter-kind-fallback) | fn | Reads from the `fallback` field of `PrefilterKind` to execute the fallback prefilter. |
| [`prefilter_kind_sse2`](#prefilter-kind-sse2) | fn | Reads from the `sse2` field of `PrefilterKind` to execute the x86_64 SSE2 prefilter. |
| [`prefilter_kind_avx2`](#prefilter-kind-avx2) | fn | Reads from the `avx2` field of `PrefilterKind` to execute the x86_64 AVX2 prefilter. |
| [`do_packed_search`](#do-packed-search) | fn | Returns true if the needle has the right characteristics for a vector algorithm to handle the entirety of substring search. |
| [`SearcherKindFn`](#searcherkindfn) | type | The type of a substring search function. |
| [`PrefilterKindFn`](#prefilterkindfn) | type | The type of a prefilter function. |

## Structs

### `Searcher`

```rust
struct Searcher {
    call: fn(&Searcher, &mut PrefilterState, &[u8], &[u8]) -> Option<usize>,
    kind: SearcherKind,
    rabinkarp: rabinkarp::Finder,
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:33-37`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L33-L37)*

A "meta" substring searcher.

To a first approximation, this chooses what it believes to be the "best"
substring search implemnetation based on the needle at construction time.
Then, every call to `find` will execute that particular implementation. To
a second approximation, multiple substring search algorithms may be used,
depending on the haystack. For example, for supremely short haystacks,
Rabin-Karp is typically used.

See the documentation on `Prefilter` for an explanation of the dispatching
mechanism. The quick summary is that an enum has too much overhead and
we can't use dynamic dispatch via traits because we need to work in a
core-only environment. (Dynamic dispatch works in core-only, but you
need `&dyn Trait` and we really need a `Box<dyn Trait>` here. The latter
requires `alloc`.) So instead, we use a union and an appropriately paired
free function to read from the correct field on the union and execute the
chosen substring search implementation.

#### Implementations

- <span id="searcher-new"></span>`fn new<R: HeuristicFrequencyRank>(prefilter: PrefilterConfig, ranker: R, needle: &[u8]) -> Searcher` — [`PrefilterConfig`](#prefilterconfig), [`Searcher`](#searcher)

  Creates a new "meta" substring searcher that attempts to choose the

  best algorithm based on the needle, heuristics and what the current

  target supports.

- <span id="searcher-twoway"></span>`fn twoway(needle: &[u8], rabinkarp: rabinkarp::Finder, prestrat: Option<Prefilter>) -> Searcher` — [`Finder`](../../arch/all/rabinkarp/index.md#finder), [`Prefilter`](#prefilter), [`Searcher`](#searcher)

  Creates a new searcher that always uses the Two-Way algorithm. This is

  typically used when vector algorithms are unavailable or inappropriate.

  (For example, when the needle is "too long.")

  

  If a prefilter is given, then the searcher returned will be accelerated

  by the prefilter.

- <span id="searcher-find"></span>`fn find(&self, prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>` — [`PrefilterState`](#prefilterstate)

  Searches the given haystack for the given needle. The needle given

  should be the same as the needle that this finder was initialized

  with.

  

  Inlining this can lead to big wins for latency, and #[`inline`](../../../compact_str/repr/inline/index.md) doesn't

  seem to be enough in some cases.

#### Trait Implementations

##### `impl Any for Searcher`

- <span id="searcher-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Searcher`

- <span id="searcher-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Searcher`

- <span id="searcher-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Searcher`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher` — [`Searcher`](#searcher)

##### `impl CloneToUninit for Searcher`

- <span id="searcher-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Searcher`

- <span id="searcher-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Searcher`

- <span id="searcher-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Searcher`

- <span id="searcher-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Searcher`

- <span id="searcher-toowned-type-owned"></span>`type Owned = T`

- <span id="searcher-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searcher-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Searcher`

- <span id="searcher-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searcher-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Searcher`

- <span id="searcher-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searcher-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TwoWayWithPrefilter`

```rust
struct TwoWayWithPrefilter {
    finder: twoway::Finder,
    prestrat: Prefilter,
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:262-265`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L262-L265)*

A two-way substring searcher with a prefilter.

#### Trait Implementations

##### `impl Any for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-clone"></span>`fn clone(&self) -> TwoWayWithPrefilter` — [`TwoWayWithPrefilter`](#twowaywithprefilter)

##### `impl CloneToUninit for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for TwoWayWithPrefilter`

##### `impl Debug for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-toowned-type-owned"></span>`type Owned = T`

- <span id="twowaywithprefilter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="twowaywithprefilter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="twowaywithprefilter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TwoWayWithPrefilter`

- <span id="twowaywithprefilter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="twowaywithprefilter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SearcherRev`

```rust
struct SearcherRev {
    kind: SearcherRevKind,
    rabinkarp: rabinkarp::FinderRev,
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:441-444`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L441-L444)*

A reverse substring searcher.

#### Implementations

- <span id="searcherrev-new"></span>`fn new(needle: &[u8]) -> SearcherRev` — [`SearcherRev`](#searcherrev)

  Creates a new searcher for finding occurrences of the given needle in

  reverse. That is, it reports the last (instead of the first) occurrence

  of a needle in a haystack.

- <span id="searcherrev-rfind"></span>`fn rfind(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

  Searches the given haystack for the last occurrence of the given

  needle. The needle given should be the same as the needle that this

  finder was initialized with.

#### Trait Implementations

##### `impl Any for SearcherRev`

- <span id="searcherrev-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SearcherRev`

- <span id="searcherrev-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SearcherRev`

- <span id="searcherrev-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SearcherRev`

- <span id="searcherrev-clone"></span>`fn clone(&self) -> SearcherRev` — [`SearcherRev`](#searcherrev)

##### `impl CloneToUninit for SearcherRev`

- <span id="searcherrev-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SearcherRev`

- <span id="searcherrev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SearcherRev`

- <span id="searcherrev-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SearcherRev`

- <span id="searcherrev-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SearcherRev`

- <span id="searcherrev-toowned-type-owned"></span>`type Owned = T`

- <span id="searcherrev-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searcherrev-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SearcherRev`

- <span id="searcherrev-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searcherrev-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SearcherRev`

- <span id="searcherrev-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searcherrev-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Prefilter`

```rust
struct Prefilter {
    call: fn(&Prefilter, &[u8]) -> Option<usize>,
    kind: PrefilterKind,
    rarest_byte: u8,
    rarest_offset: u8,
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:604-609`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L604-L609)*

The implementation of a prefilter.

This type encapsulates dispatch to one of several possible choices for a
prefilter. Generally speaking, all prefilters have the same approximate
algorithm: they choose a couple of bytes from the needle that are believed
to be rare, use a fast vector algorithm to look for those bytes and return
positions as candidates for some substring search algorithm (currently only
Two-Way) to confirm as a match or not.

The differences between the algorithms are actually at the vector
implementation level. Namely, we need different routines based on both
which target architecture we're on and what CPU features are supported.

The straight-forwardly obvious approach here is to use an enum, and make
`Prefilter::find` do case analysis to determine which algorithm was
selected and invoke it. However, I've observed that this leads to poor
codegen in some cases, especially in latency sensitive benchmarks. That is,
this approach comes with overhead that I wasn't able to eliminate.

The second obvious approach is to use dynamic dispatch with traits. Doing
that in this context where `Prefilter` owns the selection generally
requires heap allocation, and this code is designed to run in core-only
environments.

So we settle on using a union (that's `PrefilterKind`) and a function
pointer (that's `PrefilterKindFn`). We select the right function pointer
based on which field in the union we set, and that function in turn
knows which field of the union to access. The downside of this approach
is that it forces us to think about safety, but the upside is that
there are some nice latency improvements to benchmarks. (Especially the
`memmem/sliceslice/short` benchmark.)

In cases where we've selected a vector algorithm and the haystack given
is too short, we fallback to the scalar version of `memchr` on the
`rarest_byte`. (The scalar version of `memchr` is still better than a naive
byte-at-a-time loop because it will read in `usize`-sized chunks at a
time.)

#### Implementations

- <span id="prefilter-fallback"></span>`fn fallback<R: HeuristicFrequencyRank>(ranker: R, pair: Pair, needle: &[u8]) -> Option<Prefilter>` — [`Pair`](../../arch/all/packedpair/index.md#pair), [`Prefilter`](#prefilter)

  Return a "fallback" prefilter, but only if it is believed to be

  effective.

- <span id="prefilter-sse2"></span>`fn sse2(finder: sse2::Finder, needle: &[u8]) -> Prefilter` — [`Finder`](../../arch/x86_64/sse2/packedpair/index.md#finder), [`Prefilter`](#prefilter)

  Return a prefilter using a x86_64 SSE2 vector algorithm.

- <span id="prefilter-avx2"></span>`fn avx2(finder: avx2::Finder, needle: &[u8]) -> Prefilter` — [`Finder`](../../arch/x86_64/avx2/packedpair/index.md#finder), [`Prefilter`](#prefilter)

  Return a prefilter using a x86_64 AVX2 vector algorithm.

- <span id="prefilter-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return a *candidate* position for a match.

  

  When this returns an offset, it implies that a match could begin at

  that offset, but it may not. That is, it is possible for a false

  positive to be returned.

  

  When `None` is returned, then it is guaranteed that there are no

  matches for the needle in the given haystack. That is, it is impossible

  for a false negative to be returned.

  

  The purpose of this routine is to look for candidate matching positions

  as quickly as possible before running a (likely) slower confirmation

  step.

- <span id="prefilter-find-simple"></span>`fn find_simple(&self, haystack: &[u8]) -> Option<usize>`

  A "simple" prefilter that just looks for the occurrence of the rarest

  byte from the needle. This is generally only used for very small

  haystacks.

#### Trait Implementations

##### `impl Any for Prefilter`

- <span id="prefilter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Prefilter`

- <span id="prefilter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Prefilter`

- <span id="prefilter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Prefilter`

- <span id="prefilter-clone"></span>`fn clone(&self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl CloneToUninit for Prefilter`

- <span id="prefilter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Prefilter`

##### `impl Debug for Prefilter`

- <span id="prefilter-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Prefilter`

- <span id="prefilter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Prefilter`

- <span id="prefilter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Prefilter`

- <span id="prefilter-toowned-type-owned"></span>`type Owned = T`

- <span id="prefilter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prefilter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Prefilter`

- <span id="prefilter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prefilter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Prefilter`

- <span id="prefilter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prefilter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PrefilterState`

```rust
struct PrefilterState {
    skips: u32,
    skipped: u32,
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:878-888`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L878-L888)*

PrefilterState tracks state associated with the effectiveness of a
prefilter. It is used to track how many bytes, on average, are skipped by
the prefilter. If this average dips below a certain threshold over time,
then the state renders the prefilter inert and stops using it.

A prefilter state should be created for each search. (Where creating an
iterator is treated as a single search.) A prefilter state should only be
created from a `Freqy`. e.g., An inert `Freqy` will produce an inert
`PrefilterState`.

#### Fields

- **`skips`**: `u32`

  The number of skips that has been executed. This is always 1 greater
  than the actual number of skips. The special sentinel value of 0
  indicates that the prefilter is inert. This is useful to avoid
  additional checks to determine whether the prefilter is still
  "effective." Once a prefilter becomes inert, it should no longer be
  used (according to our heuristics).

- **`skipped`**: `u32`

  The total number of bytes that have been skipped.

#### Implementations

- <span id="prefilterstate-const-min-skips"></span>`const MIN_SKIPS: u32`

- <span id="prefilterstate-const-min-skip-bytes"></span>`const MIN_SKIP_BYTES: u32`

- <span id="prefilterstate-new"></span>`fn new() -> PrefilterState` — [`PrefilterState`](#prefilterstate)

  Create a fresh prefilter state.

- <span id="prefilterstate-update"></span>`fn update(&mut self, skipped: usize)`

  Update this state with the number of bytes skipped on the last

  invocation of the prefilter.

- <span id="prefilterstate-is-effective"></span>`fn is_effective(&mut self) -> bool`

  Return true if and only if this state indicates that a prefilter is

  still effective.

- <span id="prefilterstate-is-inert"></span>`fn is_inert(&self) -> bool`

  Returns true if the prefilter this state represents should no longer

  be used.

- <span id="prefilterstate-skips"></span>`fn skips(&self) -> u32`

  Returns the total number of times the prefilter has been used.

#### Trait Implementations

##### `impl Any for PrefilterState`

- <span id="prefilterstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrefilterState`

- <span id="prefilterstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrefilterState`

- <span id="prefilterstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PrefilterState`

- <span id="prefilterstate-clone"></span>`fn clone(&self) -> PrefilterState` — [`PrefilterState`](#prefilterstate)

##### `impl CloneToUninit for PrefilterState`

- <span id="prefilterstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PrefilterState`

##### `impl Debug for PrefilterState`

- <span id="prefilterstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PrefilterState`

- <span id="prefilterstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PrefilterState`

- <span id="prefilterstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PrefilterState`

- <span id="prefilterstate-toowned-type-owned"></span>`type Owned = T`

- <span id="prefilterstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prefilterstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PrefilterState`

- <span id="prefilterstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prefilterstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrefilterState`

- <span id="prefilterstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prefilterstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Pre<'a>`

```rust
struct Pre<'a> {
    prestate: &'a mut PrefilterState,
    prestrat: &'a Prefilter,
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:960-965`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L960-L965)*

A combination of prefilter effectiveness state and the prefilter itself.

#### Fields

- **`prestate`**: `&'a mut PrefilterState`

  State that tracks the effectiveness of a prefilter.

- **`prestrat`**: `&'a Prefilter`

  The actual prefilter.

#### Implementations

- <span id="pre-find"></span>`fn find(&mut self, haystack: &[u8]) -> Option<usize>`

  Call this prefilter on the given haystack with the given needle.

- <span id="pre-is-effective"></span>`fn is_effective(&mut self) -> bool`

  Return true if and only if this prefilter should be used.

#### Trait Implementations

##### `impl Any for Pre<'a>`

- <span id="pre-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pre<'a>`

- <span id="pre-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pre<'a>`

- <span id="pre-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Pre<'a>`

- <span id="pre-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Pre<'a>`

- <span id="pre-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Pre<'a>`

- <span id="pre-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Pre<'a>`

- <span id="pre-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pre-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pre<'a>`

- <span id="pre-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pre-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `SearcherRevKind`

```rust
enum SearcherRevKind {
    Empty,
    OneByte {
        needle: u8,
    },
    TwoWay {
        finder: twoway::FinderRev,
    },
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:459-463`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L459-L463)*

The kind of the reverse searcher.

For the reverse case, we don't do any SIMD acceleration or prefilters.
There is no specific technical reason why we don't, but rather don't do it
because it's not clear it's worth the extra code to do so. If you have a
use case for it, please file an issue.

We also don't do the union trick as we do with the forward case and
prefilters. Basically for the same reason we don't have prefilters or
vector algorithms for reverse searching: it's not clear it's worth doing.
Please file an issue if you have a compelling use case for fast reverse
substring search.

#### Trait Implementations

##### `impl Any for SearcherRevKind`

- <span id="searcherrevkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SearcherRevKind`

- <span id="searcherrevkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SearcherRevKind`

- <span id="searcherrevkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SearcherRevKind`

- <span id="searcherrevkind-clone"></span>`fn clone(&self) -> SearcherRevKind` — [`SearcherRevKind`](#searcherrevkind)

##### `impl CloneToUninit for SearcherRevKind`

- <span id="searcherrevkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SearcherRevKind`

- <span id="searcherrevkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SearcherRevKind`

- <span id="searcherrevkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SearcherRevKind`

- <span id="searcherrevkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SearcherRevKind`

- <span id="searcherrevkind-toowned-type-owned"></span>`type Owned = T`

- <span id="searcherrevkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searcherrevkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SearcherRevKind`

- <span id="searcherrevkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searcherrevkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SearcherRevKind`

- <span id="searcherrevkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searcherrevkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PrefilterConfig`

```rust
enum PrefilterConfig {
    None,
    Auto,
}
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:544-551`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L544-L551)*

Prefilter controls whether heuristics are used to accelerate searching.

A prefilter refers to the idea of detecting candidate matches very quickly,
and then confirming whether those candidates are full matches. This
idea can be quite effective since it's often the case that looking for
candidates can be a lot faster than running a complete substring search
over the entire input. Namely, looking for candidates can be done with
extremely fast vectorized code.

The downside of a prefilter is that it assumes false positives (which are
candidates generated by a prefilter that aren't matches) are somewhat rare
relative to the frequency of full matches. That is, if a lot of false
positives are generated, then it's possible for search time to be worse
than if the prefilter wasn't enabled in the first place.

Another downside of a prefilter is that it can result in highly variable
performance, where some cases are extraordinarily fast and others aren't.
Typically, variable performance isn't a problem, but it may be for your use
case.

The use of prefilters in this implementation does use a heuristic to detect
when a prefilter might not be carrying its weight, and will dynamically
disable its use. Nevertheless, this configuration option gives callers
the ability to disable prefilters if you have knowledge that they won't be
useful.

#### Variants

- **`None`**

  Never used a prefilter in substring search.

- **`Auto`**

  Automatically detect whether a heuristic prefilter should be used. If
  it is used, then heuristics will be used to dynamically disable the
  prefilter if it is believed to not be carrying its weight.

#### Implementations

- <span id="prefilterconfig-is-none"></span>`fn is_none(&self) -> bool`

  Returns true when this prefilter is set to the `None` variant.

#### Trait Implementations

##### `impl Any for PrefilterConfig`

- <span id="prefilterconfig-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PrefilterConfig`

- <span id="prefilterconfig-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PrefilterConfig`

- <span id="prefilterconfig-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PrefilterConfig`

- <span id="prefilterconfig-clone"></span>`fn clone(&self) -> PrefilterConfig` — [`PrefilterConfig`](#prefilterconfig)

##### `impl CloneToUninit for PrefilterConfig`

- <span id="prefilterconfig-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PrefilterConfig`

##### `impl Debug for PrefilterConfig`

- <span id="prefilterconfig-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PrefilterConfig`

- <span id="prefilterconfig-default"></span>`fn default() -> PrefilterConfig` — [`PrefilterConfig`](#prefilterconfig)

##### `impl<T> From for PrefilterConfig`

- <span id="prefilterconfig-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PrefilterConfig`

- <span id="prefilterconfig-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for PrefilterConfig`

- <span id="prefilterconfig-toowned-type-owned"></span>`type Owned = T`

- <span id="prefilterconfig-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prefilterconfig-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PrefilterConfig`

- <span id="prefilterconfig-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prefilterconfig-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PrefilterConfig`

- <span id="prefilterconfig-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prefilterconfig-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `searcher_kind_empty`

```rust
unsafe fn searcher_kind_empty(_searcher: &Searcher, _prestate: &mut PrefilterState, _haystack: &[u8], _needle: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:286-293`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L286-L293)*

Reads from the `empty` field of `SearcherKind` to handle the case of
searching for the empty needle. Works on all platforms.

# Safety

Callers must ensure that the `searcher.kind.empty` union field is set.

### `searcher_kind_one_byte`

```rust
unsafe fn searcher_kind_one_byte(searcher: &Searcher, _prestate: &mut PrefilterState, haystack: &[u8], _needle: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:301-309`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L301-L309)*

Reads from the `one_byte` field of `SearcherKind` to handle the case of
searching for a single byte needle. Works on all platforms.

# Safety

Callers must ensure that the `searcher.kind.one_byte` union field is set.

### `searcher_kind_two_way`

```rust
unsafe fn searcher_kind_two_way(searcher: &Searcher, _prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:318-329`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L318-L329)*

Reads from the `two_way` field of `SearcherKind` to handle the case of
searching for an arbitrary needle without prefilter acceleration. Works on
all platforms.

# Safety

Callers must ensure that the `searcher.kind.two_way` union field is set.

### `searcher_kind_two_way_with_prefilter`

```rust
unsafe fn searcher_kind_two_way_with_prefilter(searcher: &Searcher, prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:339-353`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L339-L353)*

Reads from the `two_way_with_prefilter` field of `SearcherKind` to handle
the case of searching for an arbitrary needle with prefilter acceleration.
Works on all platforms.

# Safety

Callers must ensure that the `searcher.kind.two_way_with_prefilter` union
field is set.

### `searcher_kind_sse2`

```rust
unsafe fn searcher_kind_sse2(searcher: &Searcher, _prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:362-374`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L362-L374)*

Reads from the `sse2` field of `SearcherKind` to execute the x86_64 SSE2
vectorized substring search implementation.

# Safety

Callers must ensure that the `searcher.kind.sse2` union field is set.

### `searcher_kind_avx2`

```rust
unsafe fn searcher_kind_avx2(searcher: &Searcher, _prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:383-395`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L383-L395)*

Reads from the `avx2` field of `SearcherKind` to execute the x86_64 AVX2
vectorized substring search implementation.

# Safety

Callers must ensure that the `searcher.kind.avx2` union field is set.

### `prefilter_kind_fallback`

```rust
unsafe fn prefilter_kind_fallback(strat: &Prefilter, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:785-790`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L785-L790)*

Reads from the `fallback` field of `PrefilterKind` to execute the fallback
prefilter. Works on all platforms.

# Safety

Callers must ensure that the `strat.kind.fallback` union field is set.

### `prefilter_kind_sse2`

```rust
unsafe fn prefilter_kind_sse2(strat: &Prefilter, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:799-809`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L799-L809)*

Reads from the `sse2` field of `PrefilterKind` to execute the x86_64 SSE2
prefilter.

# Safety

Callers must ensure that the `strat.kind.sse2` union field is set.

### `prefilter_kind_avx2`

```rust
unsafe fn prefilter_kind_avx2(strat: &Prefilter, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:818-828`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L818-L828)*

Reads from the `avx2` field of `PrefilterKind` to execute the x86_64 AVX2
prefilter.

# Safety

Callers must ensure that the `strat.kind.avx2` union field is set.

### `do_packed_search`

```rust
fn do_packed_search(needle: &[u8]) -> bool
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:996-1030`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L996-L1030)*

Returns true if the needle has the right characteristics for a vector
algorithm to handle the entirety of substring search.

Vector algorithms can be used for prefilters for other substring search
algorithms (like Two-Way), but they can also be used for substring search
on their own. When used for substring search, vector algorithms will
quickly identify candidate match positions (just like in the prefilter
case), but instead of returning the candidate position they will try to
confirm the match themselves. Confirmation happens via `memcmp`. This
works well for short needles, but can break down when many false candidate
positions are generated for large needles. Thus, we only permit vector
algorithms to own substring search when the needle is of a certain length.

## Type Aliases

### `SearcherKindFn`

```rust
type SearcherKindFn = fn(&Searcher, &mut PrefilterState, &[u8], &[u8]) -> Option<usize>;
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:273-278`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L273-L278)*

The type of a substring search function.

# Safety

When using a function of this type, callers must ensure that the correct
function is paired with the value populated in `SearcherKind` union.

### `PrefilterKindFn`

```rust
type PrefilterKindFn = fn(&Prefilter, &[u8]) -> Option<usize>;
```

*Defined in [`memchr-2.7.6/src/memmem/searcher.rs:776-777`](../../../../.source_1765633015/memchr-2.7.6/src/memmem/searcher.rs#L776-L777)*

The type of a prefilter function.

# Safety

When using a function of this type, callers must ensure that the correct
function is paired with the value populated in `PrefilterKind` union.

