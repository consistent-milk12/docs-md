*[memchr](../../../../index.md) / [arch](../../../index.md) / [x86_64](../../index.md) / [sse2](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

A 128-bit vector implementation of the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.

[generic SIMD]: http://0x80.pl/articles/simd-strfind.html#first-and-last

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

- `fn new(needle: &[u8]) -> Option<Finder>` — [`Finder`](../../../../../arch/x86_64/sse2/packedpair/index.md)

- `fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>` — [`Pair`](../../../../../arch/all/packedpair/index.md), [`Finder`](../../../../../arch/x86_64/sse2/packedpair/index.md)

- `unsafe fn with_pair_impl(needle: &[u8], pair: Pair) -> Finder` — [`Pair`](../../../../../arch/all/packedpair/index.md), [`Finder`](../../../../../arch/x86_64/sse2/packedpair/index.md)

- `fn is_available() -> bool`

- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- `fn find_prefilter(self: &Self, haystack: &[u8]) -> Option<usize>`

- `unsafe fn find_impl(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- `unsafe fn find_prefilter_impl(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn pair(self: &Self) -> &Pair` — [`Pair`](../../../../../arch/all/packedpair/index.md)

- `fn min_haystack_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Finder` — [`Finder`](../../../../../arch/x86_64/sse2/packedpair/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

