*[memchr](../../../../index.md) / [arch](../../../index.md) / [x86_64](../../index.md) / [sse2](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

A 128-bit vector implementation of the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A "packed pair" finder that uses 128-bit vector operations. |

## Structs

### `Finder`

```rust
struct Finder(packedpair::Finder<core::arch::x86_64::__m128i>);
```

A "packed pair" finder that uses 128-bit vector operations.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`](../../../all/packedpair/index.md) of bytes given matches.

#### Implementations

- <span id="finder-new"></span>`fn new(needle: &[u8]) -> Option<Finder>` — [`Finder`](#finder)

- <span id="finder-with-pair"></span>`fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>` — [`Pair`](../../../all/packedpair/index.md), [`Finder`](#finder)

- <span id="finder-with-pair-impl"></span>`unsafe fn with_pair_impl(needle: &[u8], pair: Pair) -> Finder` — [`Pair`](../../../all/packedpair/index.md), [`Finder`](#finder)

- <span id="finder-is-available"></span>`fn is_available() -> bool`

- <span id="finder-find"></span>`fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- <span id="finder-find-prefilter"></span>`fn find_prefilter(&self, haystack: &[u8]) -> Option<usize>`

- <span id="finder-find-impl"></span>`unsafe fn find_impl(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- <span id="finder-find-prefilter-impl"></span>`unsafe fn find_prefilter_impl(&self, haystack: &[u8]) -> Option<usize>`

- <span id="finder-pair"></span>`fn pair(&self) -> &Pair` — [`Pair`](../../../all/packedpair/index.md)

- <span id="finder-min-haystack-len"></span>`fn min_haystack_len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Finder`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder` — [`Finder`](#finder)

##### `impl Copy for Finder`

##### `impl Debug for Finder`

- <span id="finder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

