*[memchr](../../../index.md) / [arch](../../index.md) / [generic](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

Generic crate-internal routines for the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.


## Structs

### `Finder<V>`

```rust
struct Finder<V> {
    pair: crate::arch::all::packedpair::Pair,
    v1: V,
    v2: V,
    min_haystack_len: usize,
}
```

A generic architecture dependent "packed pair" finder.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`](../../all/packedpair/index.md) of bytes given matches.

This is architecture dependent because it uses specific vector operations
to look for occurrences of the pair of bytes.

This type is not meant to be exported and is instead meant to be used as
the implementation for architecture specific facades. Why? Because it's a
bit of a quirky API that requires `inline(always)` annotations. And pretty
much everything has safety obligations due (at least) to the caller needing
to inline calls into routines marked with
`#[target_feature(enable = "...")]`.

#### Implementations

- `unsafe fn new(needle: &[u8], pair: Pair) -> Finder<V>` — [`Pair`](../../all/packedpair/index.md), [`Finder`](#finder)

- `unsafe fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- `unsafe fn find_prefilter(self: &Self, haystack: &[u8]) -> Option<usize>`

- `unsafe fn find_in_chunk(self: &Self, needle: &[u8], cur: *const u8, end: *const u8, mask: <V as >::Mask) -> Option<usize>` — [`Vector`](../../../vector/index.md)

- `unsafe fn find_prefilter_in_chunk(self: &Self, cur: *const u8) -> Option<usize>`

- `fn pair(self: &Self) -> &Pair` — [`Pair`](../../all/packedpair/index.md)

- `fn min_haystack_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl<V: $crate::clone::Clone> Clone for Finder<V>`

- `fn clone(self: &Self) -> Finder<V>` — [`Finder`](#finder)

##### `impl<V: $crate::marker::Copy> Copy for Finder<V>`

##### `impl<V: $crate::fmt::Debug> Debug for Finder<V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `matched`

```rust
unsafe fn matched(start: *const u8, cur: *const u8, chunki: usize) -> usize
```

Accepts a chunk-relative offset and returns a haystack relative offset.

This used to be marked `#[cold]` and `#[inline(never)]`, but I couldn't
observe a consistent measureable difference between that and just inlining
it. So we go with inlining it.

# Safety

Same at `ptr::offset_from` in addition to `cur >= start`.

