*[memchr](../../index.md) / [memmem](../index.md) / [searcher](index.md)*

---

# Module `searcher`

## Structs

### `Searcher`

```rust
struct Searcher {
    call: fn(&Searcher, &mut PrefilterState, &[u8], &[u8]) -> Option<usize>,
    kind: SearcherKind,
    rabinkarp: rabinkarp::Finder,
}
```

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

- `fn new<R: HeuristicFrequencyRank>(prefilter: PrefilterConfig, ranker: R, needle: &[u8]) -> Searcher` — [`PrefilterConfig`](#prefilterconfig), [`Searcher`](#searcher)

- `fn twoway(needle: &[u8], rabinkarp: rabinkarp::Finder, prestrat: Option<Prefilter>) -> Searcher` — [`Finder`](../../arch/all/rabinkarp/index.md), [`Prefilter`](#prefilter), [`Searcher`](#searcher)

- `fn find(self: &Self, prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>` — [`PrefilterState`](#prefilterstate)

#### Trait Implementations

##### `impl Clone for Searcher`

- `fn clone(self: &Self) -> Searcher` — [`Searcher`](#searcher)

##### `impl Debug for Searcher`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `TwoWayWithPrefilter`

```rust
struct TwoWayWithPrefilter {
    finder: twoway::Finder,
    prestrat: Prefilter,
}
```

A two-way substring searcher with a prefilter.

#### Trait Implementations

##### `impl Clone for TwoWayWithPrefilter`

- `fn clone(self: &Self) -> TwoWayWithPrefilter` — [`TwoWayWithPrefilter`](#twowaywithprefilter)

##### `impl Copy for TwoWayWithPrefilter`

##### `impl Debug for TwoWayWithPrefilter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SearcherRev`

```rust
struct SearcherRev {
    kind: SearcherRevKind,
    rabinkarp: rabinkarp::FinderRev,
}
```

A reverse substring searcher.

#### Implementations

- `fn new(needle: &[u8]) -> SearcherRev` — [`SearcherRev`](#searcherrev)

- `fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for SearcherRev`

- `fn clone(self: &Self) -> SearcherRev` — [`SearcherRev`](#searcherrev)

##### `impl Debug for SearcherRev`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Prefilter`

```rust
struct Prefilter {
    call: fn(&Prefilter, &[u8]) -> Option<usize>,
    kind: PrefilterKind,
    rarest_byte: u8,
    rarest_offset: u8,
}
```

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

- `fn fallback<R: HeuristicFrequencyRank>(ranker: R, pair: Pair, needle: &[u8]) -> Option<Prefilter>` — [`Pair`](../../arch/all/packedpair/index.md), [`Prefilter`](#prefilter)

- `fn sse2(finder: sse2::Finder, needle: &[u8]) -> Prefilter` — [`Finder`](../../arch/x86_64/sse2/packedpair/index.md), [`Prefilter`](#prefilter)

- `fn avx2(finder: avx2::Finder, needle: &[u8]) -> Prefilter` — [`Finder`](../../arch/x86_64/avx2/packedpair/index.md), [`Prefilter`](#prefilter)

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn find_simple(self: &Self, haystack: &[u8]) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for Prefilter`

- `fn clone(self: &Self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl Copy for Prefilter`

##### `impl Debug for Prefilter`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `PrefilterState`

```rust
struct PrefilterState {
    skips: u32,
    skipped: u32,
}
```

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

- `const MIN_SKIPS: u32`

- `const MIN_SKIP_BYTES: u32`

- `fn new() -> PrefilterState` — [`PrefilterState`](#prefilterstate)

- `fn update(self: &mut Self, skipped: usize)`

- `fn is_effective(self: &mut Self) -> bool`

- `fn is_inert(self: &Self) -> bool`

- `fn skips(self: &Self) -> u32`

#### Trait Implementations

##### `impl Clone for PrefilterState`

- `fn clone(self: &Self) -> PrefilterState` — [`PrefilterState`](#prefilterstate)

##### `impl Copy for PrefilterState`

##### `impl Debug for PrefilterState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Pre<'a>`

```rust
struct Pre<'a> {
    prestate: &'a mut PrefilterState,
    prestrat: &'a Prefilter,
}
```

A combination of prefilter effectiveness state and the prefilter itself.

#### Fields

- **`prestate`**: `&'a mut PrefilterState`

  State that tracks the effectiveness of a prefilter.

- **`prestrat`**: `&'a Prefilter`

  The actual prefilter.

#### Implementations

- `fn find(self: &mut Self, haystack: &[u8]) -> Option<usize>`

- `fn is_effective(self: &mut Self) -> bool`

#### Trait Implementations

##### `impl<'a> Debug for Pre<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

##### `impl Clone for SearcherRevKind`

- `fn clone(self: &Self) -> SearcherRevKind` — [`SearcherRevKind`](#searcherrevkind)

##### `impl Debug for SearcherRevKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PrefilterConfig`

```rust
enum PrefilterConfig {
    None,
    Auto,
}
```

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

- `fn is_none(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for PrefilterConfig`

- `fn clone(self: &Self) -> PrefilterConfig` — [`PrefilterConfig`](#prefilterconfig)

##### `impl Copy for PrefilterConfig`

##### `impl Debug for PrefilterConfig`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PrefilterConfig`

- `fn default() -> PrefilterConfig` — [`PrefilterConfig`](#prefilterconfig)

## Functions

### `searcher_kind_empty`

```rust
unsafe fn searcher_kind_empty(_searcher: &Searcher, _prestate: &mut PrefilterState, _haystack: &[u8], _needle: &[u8]) -> Option<usize>
```

Reads from the `empty` field of `SearcherKind` to handle the case of
searching for the empty needle. Works on all platforms.

# Safety

Callers must ensure that the `searcher.kind.empty` union field is set.

### `searcher_kind_one_byte`

```rust
unsafe fn searcher_kind_one_byte(searcher: &Searcher, _prestate: &mut PrefilterState, haystack: &[u8], _needle: &[u8]) -> Option<usize>
```

Reads from the `one_byte` field of `SearcherKind` to handle the case of
searching for a single byte needle. Works on all platforms.

# Safety

Callers must ensure that the `searcher.kind.one_byte` union field is set.

### `searcher_kind_two_way`

```rust
unsafe fn searcher_kind_two_way(searcher: &Searcher, _prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

Reads from the `two_way` field of `SearcherKind` to handle the case of
searching for an arbitrary needle without prefilter acceleration. Works on
all platforms.

# Safety

Callers must ensure that the `searcher.kind.two_way` union field is set.

### `searcher_kind_two_way_with_prefilter`

```rust
unsafe fn searcher_kind_two_way_with_prefilter(searcher: &Searcher, prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

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

Reads from the `sse2` field of `SearcherKind` to execute the x86_64 SSE2
vectorized substring search implementation.

# Safety

Callers must ensure that the `searcher.kind.sse2` union field is set.

### `searcher_kind_avx2`

```rust
unsafe fn searcher_kind_avx2(searcher: &Searcher, _prestate: &mut PrefilterState, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

Reads from the `avx2` field of `SearcherKind` to execute the x86_64 AVX2
vectorized substring search implementation.

# Safety

Callers must ensure that the `searcher.kind.avx2` union field is set.

### `prefilter_kind_fallback`

```rust
unsafe fn prefilter_kind_fallback(strat: &Prefilter, haystack: &[u8]) -> Option<usize>
```

Reads from the `fallback` field of `PrefilterKind` to execute the fallback
prefilter. Works on all platforms.

# Safety

Callers must ensure that the `strat.kind.fallback` union field is set.

### `prefilter_kind_sse2`

```rust
unsafe fn prefilter_kind_sse2(strat: &Prefilter, haystack: &[u8]) -> Option<usize>
```

Reads from the `sse2` field of `PrefilterKind` to execute the x86_64 SSE2
prefilter.

# Safety

Callers must ensure that the `strat.kind.sse2` union field is set.

### `prefilter_kind_avx2`

```rust
unsafe fn prefilter_kind_avx2(strat: &Prefilter, haystack: &[u8]) -> Option<usize>
```

Reads from the `avx2` field of `PrefilterKind` to execute the x86_64 AVX2
prefilter.

# Safety

Callers must ensure that the `strat.kind.avx2` union field is set.

### `do_packed_search`

```rust
fn do_packed_search(needle: &[u8]) -> bool
```

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

The type of a substring search function.

# Safety

When using a function of this type, callers must ensure that the correct
function is paired with the value populated in `SearcherKind` union.

### `PrefilterKindFn`

```rust
type PrefilterKindFn = fn(&Prefilter, &[u8]) -> Option<usize>;
```

The type of a prefilter function.

# Safety

When using a function of this type, callers must ensure that the correct
function is paired with the value populated in `PrefilterKind` union.

