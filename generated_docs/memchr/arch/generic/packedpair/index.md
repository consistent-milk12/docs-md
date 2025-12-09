*[memchr](../../../index.md) / [arch](../../index.md) / [generic](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

Generic crate-internal routines for the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A generic architecture dependent "packed pair" finder. |
| [`matched`](#matched) | fn | Accepts a chunk-relative offset and returns a haystack relative offset. |

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

*Defined in [`memchr-2.7.6/src/arch/generic/packedpair.rs:35-40`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/generic/packedpair.rs#L35-L40)*

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

- <span id="finder-new"></span>`unsafe fn new(needle: &[u8], pair: Pair) -> Finder<V>` — [`Pair`](../../all/packedpair/index.md), [`Finder`](#finder)

- <span id="finder-find"></span>`unsafe fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

- <span id="finder-find-prefilter"></span>`unsafe fn find_prefilter(&self, haystack: &[u8]) -> Option<usize>`

- <span id="finder-find-in-chunk"></span>`unsafe fn find_in_chunk(&self, needle: &[u8], cur: *const u8, end: *const u8, mask: <V as >::Mask) -> Option<usize>` — [`Vector`](../../../vector/index.md)

- <span id="finder-find-prefilter-in-chunk"></span>`unsafe fn find_prefilter_in_chunk(&self, cur: *const u8) -> Option<usize>`

- <span id="finder-pair"></span>`fn pair(&self) -> &Pair` — [`Pair`](../../all/packedpair/index.md)

- <span id="finder-min-haystack-len"></span>`fn min_haystack_len(&self) -> usize`

#### Trait Implementations

##### `impl<V: clone::Clone> Clone for Finder<V>`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder<V>` — [`Finder`](#finder)

##### `impl<V: marker::Copy> Copy for Finder<V>`

##### `impl<V: fmt::Debug> Debug for Finder<V>`

- <span id="finder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `matched`

```rust
unsafe fn matched(start: *const u8, cur: *const u8, chunki: usize) -> usize
```

*Defined in [`memchr-2.7.6/src/arch/generic/packedpair.rs:312-314`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/generic/packedpair.rs#L312-L314)*

Accepts a chunk-relative offset and returns a haystack relative offset.

This used to be marked `#[cold]` and `#[inline(never)]`, but I couldn't
observe a consistent measureable difference between that and just inlining
it. So we go with inlining it.

# Safety

Same at `ptr::offset_from` in addition to `cur >= start`.

