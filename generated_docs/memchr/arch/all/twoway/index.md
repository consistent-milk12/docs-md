*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [twoway](index.md)*

---

# Module `twoway`

An implementation of the [Two-Way substring search algorithm][two-way].

[`Finder`](#finder) can be built for forward searches, while [`FinderRev`](#finderrev) can be built
for reverse searches.

Two-Way makes for a nice general purpose substring search algorithm because of
its time and space complexity properties. It also performs well in practice.
Namely, with `m = len(needle)` and `n = len(haystack)`, Two-Way takes `O(m)`
time to create a finder, `O(1)` space and `O(n)` search time. In other words,
the preprocessing step is quick, doesn't require any heap memory and the worst
case search time is guaranteed to be linear in the haystack regardless of the
size of the needle.

While vector algorithms will usually beat Two-Way handedly, vector algorithms
also usually have pathological or edge cases that are better handled by Two-Way.
Moreover, not all targets support vector algorithms or implementations for them
simply may not exist yet.

Two-Way can be found in the `memmem` implementations in at least [GNU libc] and
[musl].




## Contents

- [Structs](#structs)
  - [`Finder`](#finder)
  - [`FinderRev`](#finderrev)
  - [`TwoWay`](#twoway)
  - [`Suffix`](#suffix)
  - [`ApproximateByteSet`](#approximatebyteset)
- [Enums](#enums)
  - [`Shift`](#shift)
  - [`SuffixKind`](#suffixkind)
  - [`SuffixOrdering`](#suffixordering)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A forward substring searcher that uses the Two-Way algorithm. |
| [`FinderRev`](#finderrev) | struct | A reverse substring searcher that uses the Two-Way algorithm. |
| [`TwoWay`](#twoway) | struct | An implementation of the TwoWay substring search algorithm. |
| [`Suffix`](#suffix) | struct | A suffix extracted from a needle along with its period. |
| [`ApproximateByteSet`](#approximatebyteset) | struct | A bitset used to track whether a particular byte exists in a needle or not. |
| [`Shift`](#shift) | enum | A representation of the amount we're allowed to shift by during Two-Way search. |
| [`SuffixKind`](#suffixkind) | enum | The kind of suffix to extract. |
| [`SuffixOrdering`](#suffixordering) | enum | The result of comparing corresponding bytes between two suffixes. |

## Structs

### `Finder`

```rust
struct Finder(TwoWay);
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:37`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L37)*

A forward substring searcher that uses the Two-Way algorithm.

#### Implementations

- <span id="finder-new"></span>`fn new(needle: &[u8]) -> Finder` — [`Finder`](#finder)

- <span id="finder-find"></span>`fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- <span id="finder-find-with-prefilter"></span>`fn find_with_prefilter(&self, pre: Option<Pre<'_>>, haystack: &[u8], needle: &[u8]) -> Option<usize>` — [`Pre`](../../../memmem/searcher/index.md)

- <span id="finder-find-small-imp"></span>`fn find_small_imp(&self, pre: Option<Pre<'_>>, haystack: &[u8], needle: &[u8], period: usize) -> Option<usize>` — [`Pre`](../../../memmem/searcher/index.md)

- <span id="finder-find-large-imp"></span>`fn find_large_imp(&self, pre: Option<Pre<'_>>, haystack: &[u8], needle: &[u8], shift: usize) -> Option<usize>` — [`Pre`](../../../memmem/searcher/index.md)

#### Trait Implementations

##### `impl Clone for Finder`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder` — [`Finder`](#finder)

##### `impl Copy for Finder`

##### `impl Debug for Finder`

- <span id="finder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FinderRev`

```rust
struct FinderRev(TwoWay);
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:41`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L41)*

A reverse substring searcher that uses the Two-Way algorithm.

#### Implementations

- <span id="finderrev-new"></span>`fn new(needle: &[u8]) -> FinderRev` — [`FinderRev`](#finderrev)

- <span id="finderrev-rfind"></span>`fn rfind(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- <span id="finderrev-rfind-small-imp"></span>`fn rfind_small_imp(&self, haystack: &[u8], needle: &[u8], period: usize) -> Option<usize>`

- <span id="finderrev-rfind-large-imp"></span>`fn rfind_large_imp(&self, haystack: &[u8], needle: &[u8], shift: usize) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for FinderRev`

- <span id="finderrev-clone"></span>`fn clone(&self) -> FinderRev` — [`FinderRev`](#finderrev)

##### `impl Copy for FinderRev`

##### `impl Debug for FinderRev`

- <span id="finderrev-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TwoWay`

```rust
struct TwoWay {
    byteset: ApproximateByteSet,
    critical_pos: usize,
    shift: Shift,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:80-106`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L80-L106)*

An implementation of the TwoWay substring search algorithm.

This searcher supports forward and reverse search, although not
simultaneously. It runs in `O(n + m)` time and `O(1)` space, where
`n ~ len(needle)` and `m ~ len(haystack)`.

The implementation here roughly matches that which was developed by
Crochemore and Perrin in their 1991 paper "Two-way string-matching." The
changes in this implementation are 1) the use of zero-based indices, 2) a
heuristic skip table based on the last byte (borrowed from Rust's standard
library) and 3) the addition of heuristics for a fast skip loop. For (3),
callers can pass any kind of prefilter they want, but usually it's one
based on a heuristic that uses an approximate background frequency of bytes
to choose rare bytes to quickly look for candidate match positions. Note
though that currently, this prefilter functionality is not exposed directly
in the public API. (File an issue if you want it and provide a use case
please.)

The heuristic for fast skipping is automatically shut off if it's
detected to be ineffective at search time. Generally, this only occurs in
pathological cases. But this is generally necessary in order to preserve
a `O(n + m)` time bound.

The code below is fairly complex and not obviously correct at all. It's
likely necessary to read the Two-Way paper cited above in order to fully
grok this code. The essence of it is:

1. Do something to detect a "critical" position in the needle.
2. For the current position in the haystack, look if `needle[critical..]`
matches at that position.
3. If so, look if `needle[..critical]` matches.
4. If a mismatch occurs, shift the search by some amount based on the
critical position and a pre-computed shift.

This type is wrapped in the forward and reverse finders that expose
consistent forward or reverse APIs.

#### Fields

- **`byteset`**: `ApproximateByteSet`

  A small bitset used as a quick prefilter (in addition to any prefilter
  given by the caller). Namely, a bit `i` is set if and only if `b%64==i`
  for any `b == needle[i]`.
  
  When used as a prefilter, if the last byte at the current candidate
  position is NOT in this set, then we can skip that entire candidate
  position (the length of the needle). This is essentially the shift
  trick found in Boyer-Moore, but only applied to bytes that don't appear
  in the needle.
  
  N.B. This trick was inspired by something similar in std's
  implementation of Two-Way.

- **`critical_pos`**: `usize`

  A critical position in needle. Specifically, this position corresponds
  to beginning of either the minimal or maximal suffix in needle. (N.B.
  See SuffixType below for why "minimal" isn't quite the correct word
  here.)
  
  This is the position at which every search begins. Namely, search
  starts by scanning text to the right of this position, and only if
  there's a match does the text to the left of this position get scanned.

- **`shift`**: `Shift`

  The amount we shift by in the Two-Way search algorithm. This
  corresponds to the "small period" and "large period" cases.

#### Trait Implementations

##### `impl Clone for TwoWay`

- <span id="twoway-clone"></span>`fn clone(&self) -> TwoWay` — [`TwoWay`](#twoway)

##### `impl Copy for TwoWay`

##### `impl Debug for TwoWay`

- <span id="twoway-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Suffix`

```rust
struct Suffix {
    pos: usize,
    period: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:483-497`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L483-L497)*

A suffix extracted from a needle along with its period.

#### Fields

- **`pos`**: `usize`

  The starting position of this suffix.
  
  If this is a forward suffix, then `&bytes[pos..]` can be used. If this
  is a reverse suffix, then `&bytes[..pos]` can be used. That is, for
  forward suffixes, this is an inclusive starting position, where as for
  reverse suffixes, this is an exclusive ending position.

- **`period`**: `usize`

  The period of this suffix.
  
  Note that this is NOT necessarily the period of the string from which
  this suffix comes from. (It is always less than or equal to the period
  of the original string.)

#### Implementations

- <span id="suffix-forward"></span>`fn forward(needle: &[u8], kind: SuffixKind) -> Suffix` — [`SuffixKind`](#suffixkind), [`Suffix`](#suffix)

- <span id="suffix-reverse"></span>`fn reverse(needle: &[u8], kind: SuffixKind) -> Suffix` — [`SuffixKind`](#suffixkind), [`Suffix`](#suffix)

#### Trait Implementations

##### `impl Debug for Suffix`

- <span id="suffix-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ApproximateByteSet`

```rust
struct ApproximateByteSet(u64);
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:651`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L651)*

A bitset used to track whether a particular byte exists in a needle or not.

Namely, bit 'i' is set if and only if byte%64==i for any byte in the
needle. If a particular byte in the haystack is NOT in this set, then one
can conclude that it is also not in the needle, and thus, one can advance
in the haystack by needle.len() bytes.

#### Implementations

- <span id="approximatebyteset-new"></span>`fn new(needle: &[u8]) -> ApproximateByteSet` — [`ApproximateByteSet`](#approximatebyteset)

- <span id="approximatebyteset-contains"></span>`fn contains(&self, byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for ApproximateByteSet`

- <span id="approximatebyteset-clone"></span>`fn clone(&self) -> ApproximateByteSet` — [`ApproximateByteSet`](#approximatebyteset)

##### `impl Copy for ApproximateByteSet`

##### `impl Debug for ApproximateByteSet`

- <span id="approximatebyteset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Shift`

```rust
enum Shift {
    Small {
        period: usize,
    },
    Large {
        shift: usize,
    },
}
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:428-431`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L428-L431)*

A representation of the amount we're allowed to shift by during Two-Way
search.

When computing a critical factorization of the needle, we find the position
of the critical factorization by finding the needle's maximal (or minimal)
suffix, along with the period of that suffix. It turns out that the period
of that suffix is a lower bound on the period of the needle itself.

This lower bound is equivalent to the actual period of the needle in
some cases. To describe that case, we denote the needle as `x` where
`x = uv` and `v` is the lexicographic maximal suffix of `v`. The lower
bound given here is always the period of `v`, which is `<= period(x)`. The
case where `period(v) == period(x)` occurs when `len(u) < (len(x) / 2)` and
where `u` is a suffix of `v[0..period(v)]`.

This case is important because the search algorithm for when the
periods are equivalent is slightly different than the search algorithm
for when the periods are not equivalent. In particular, when they aren't
equivalent, we know that the period of the needle is no less than half its
length. In this case, we shift by an amount less than or equal to the
period of the needle (determined by the maximum length of the components
of the critical factorization of `x`, i.e., `max(len(u), len(v))`)..

The above two cases are represented by the variants below. Each entails
a different instantiation of the Two-Way search algorithm.

N.B. If we could find a way to compute the exact period in all cases,
then we could collapse this case analysis and simplify the algorithm. The
Two-Way paper suggests this is possible, but more reading is required to
grok why the authors didn't pursue that path.

#### Implementations

- <span id="shift-forward"></span>`fn forward(needle: &[u8], period_lower_bound: usize, critical_pos: usize) -> Shift` — [`Shift`](#shift)

- <span id="shift-reverse"></span>`fn reverse(needle: &[u8], period_lower_bound: usize, critical_pos: usize) -> Shift` — [`Shift`](#shift)

#### Trait Implementations

##### `impl Clone for Shift`

- <span id="shift-clone"></span>`fn clone(&self) -> Shift` — [`Shift`](#shift)

##### `impl Copy for Shift`

##### `impl Debug for Shift`

- <span id="shift-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SuffixKind`

```rust
enum SuffixKind {
    Minimal,
    Maximal,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:590-605`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L590-L605)*

The kind of suffix to extract.

#### Variants

- **`Minimal`**

  Extract the smallest lexicographic suffix from a string.
  
  Technically, this doesn't actually pick the smallest lexicographic
  suffix. e.g., Given the choice between `a` and `aa`, this will choose
  the latter over the former, even though `a < aa`. The reasoning for
  this isn't clear from the paper, but it still smells like a minimal
  suffix.

- **`Maximal`**

  Extract the largest lexicographic suffix from a string.
  
  Unlike `Minimal`, this really does pick the maximum suffix. e.g., Given
  the choice between `z` and `zz`, this will choose the latter over the
  former.

#### Implementations

- <span id="suffixkind-cmp"></span>`fn cmp(self, current: u8, candidate: u8) -> SuffixOrdering` — [`SuffixOrdering`](#suffixordering)

#### Trait Implementations

##### `impl Clone for SuffixKind`

- <span id="suffixkind-clone"></span>`fn clone(&self) -> SuffixKind` — [`SuffixKind`](#suffixkind)

##### `impl Copy for SuffixKind`

##### `impl Debug for SuffixKind`

- <span id="suffixkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SuffixOrdering`

```rust
enum SuffixOrdering {
    Accept,
    Skip,
    Push,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/twoway.rs:609-624`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/twoway.rs#L609-L624)*

The result of comparing corresponding bytes between two suffixes.

#### Variants

- **`Accept`**

  This occurs when the given candidate byte indicates that the candidate
  suffix is better than the current maximal (or minimal) suffix. That is,
  the current candidate suffix should supplant the current maximal (or
  minimal) suffix.

- **`Skip`**

  This occurs when the given candidate byte excludes the candidate suffix
  from being better than the current maximal (or minimal) suffix. That
  is, the current candidate suffix should be dropped and the next one
  should be considered.

- **`Push`**

  This occurs when no decision to accept or skip the candidate suffix
  can be made, e.g., when corresponding bytes are equivalent. In this
  case, the next corresponding bytes should be compared.

#### Trait Implementations

##### `impl Clone for SuffixOrdering`

- <span id="suffixordering-clone"></span>`fn clone(&self) -> SuffixOrdering` — [`SuffixOrdering`](#suffixordering)

##### `impl Copy for SuffixOrdering`

##### `impl Debug for SuffixOrdering`

- <span id="suffixordering-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

