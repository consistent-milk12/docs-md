*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

Provides an architecture independent implementation of the "packed pair"
algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for. Note that
this module provides an architecture independent version that doesn't do as
good of a job keeping the search for candidates inside a SIMD hot path. It
however can be good enough in many circumstances.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`default_rank`](#default-rank) | mod |  |
| [`Finder`](#finder) | struct | An architecture independent "packed pair" finder. |
| [`Pair`](#pair) | struct | A pair of byte offsets into a needle to use as a predicate. |
| [`DefaultFrequencyRank`](#defaultfrequencyrank) | struct | The default byte frequency heuristic that is good for most haystacks. |
| [`HeuristicFrequencyRank`](#heuristicfrequencyrank) | trait | This trait allows the user to customize the heuristic used to determine the relative frequency of a given byte in the dataset being searched. |

## Modules

- [`default_rank`](default_rank/index.md)

## Structs

### `Finder`

```rust
struct Finder {
    pair: Pair,
    byte1: u8,
    byte2: u8,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/packedpair/mod.rs:35-39`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/all/packedpair/mod.rs#L35-L39)*

An architecture independent "packed pair" finder.

This finder picks two bytes that it believes have high predictive power for
indicating an overall match of a needle. At search time, it reports offsets
where the needle could match based on whether the pair of bytes it chose
match.

This is architecture independent because it utilizes `memchr` to find the
occurrence of one of the bytes in the pair, and then checks whether the
second byte matches. If it does, in the case of `Finder::find_prefilter`,
the location at which the needle could match is returned.

It is generally preferred to use architecture specific routines for a
"packed pair" prefilter, but this can be a useful fallback when the
architecture independent routines are unavailable.

#### Implementations

- <span id="finder-new"></span>`fn new(needle: &[u8]) -> Option<Finder>` — [`Finder`](#finder)

- <span id="finder-with-pair"></span>`fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>` — [`Pair`](#pair), [`Finder`](#finder)

- <span id="finder-find-prefilter"></span>`fn find_prefilter(&self, haystack: &[u8]) -> Option<usize>`

- <span id="finder-pair"></span>`fn pair(&self) -> &Pair` — [`Pair`](#pair)

#### Trait Implementations

##### `impl Clone for Finder`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder` — [`Finder`](#finder)

##### `impl Copy for Finder`

##### `impl Debug for Finder`

- <span id="finder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Pair`

```rust
struct Pair {
    index1: u8,
    index2: u8,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/packedpair/mod.rs:135-138`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/all/packedpair/mod.rs#L135-L138)*

A pair of byte offsets into a needle to use as a predicate.

This pair is used as a predicate to quickly filter out positions in a
haystack in which a needle cannot match. In some cases, this pair can even
be used in vector algorithms such that the vector algorithm only switches
over to scalar code once this pair has been found.

A pair of offsets can be used in both substring search implementations and
in prefilters. The former will report matches of a needle in a haystack
where as the latter will only report possible matches of a needle.

The offsets are limited each to a maximum of 255 to keep memory usage low.
Moreover, it's rarely advantageous to create a predicate using offsets
greater than 255 anyway.

The only guarantee enforced on the pair of offsets is that they are not
equivalent. It is not necessarily the case that `index1 < index2` for
example. By convention, `index1` corresponds to the byte in the needle
that is believed to be most the predictive. Note also that because of the
requirement that the indices be both valid for the needle used to build
the pair and not equal, it follows that a pair can only be constructed for
needles with length at least 2.

#### Implementations

- <span id="pair-new"></span>`fn new(needle: &[u8]) -> Option<Pair>` — [`Pair`](#pair)

- <span id="pair-with-ranker"></span>`fn with_ranker<R: HeuristicFrequencyRank>(needle: &[u8], ranker: R) -> Option<Pair>` — [`Pair`](#pair)

- <span id="pair-with-indices"></span>`fn with_indices(needle: &[u8], index1: u8, index2: u8) -> Option<Pair>` — [`Pair`](#pair)

- <span id="pair-index1"></span>`fn index1(&self) -> u8`

- <span id="pair-index2"></span>`fn index2(&self) -> u8`

#### Trait Implementations

##### `impl Clone for Pair`

- <span id="pair-clone"></span>`fn clone(&self) -> Pair` — [`Pair`](#pair)

##### `impl Copy for Pair`

##### `impl Debug for Pair`

- <span id="pair-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DefaultFrequencyRank`

```rust
struct DefaultFrequencyRank;
```

*Defined in [`memchr-2.7.6/src/arch/all/packedpair/mod.rs:321`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/all/packedpair/mod.rs#L321)*

The default byte frequency heuristic that is good for most haystacks.

#### Trait Implementations

##### `impl HeuristicFrequencyRank for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-rank"></span>`fn rank(&self, byte: u8) -> u8`

## Traits

### `HeuristicFrequencyRank`

```rust
trait HeuristicFrequencyRank { ... }
```

*Defined in [`memchr-2.7.6/src/arch/all/packedpair/mod.rs:309-318`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/all/packedpair/mod.rs#L309-L318)*

This trait allows the user to customize the heuristic used to determine the
relative frequency of a given byte in the dataset being searched.

The use of this trait can have a dramatic impact on performance depending
on the type of data being searched. The details of why are explained in the
docs of [`crate::memmem::Prefilter`](../../../memmem/searcher/index.md). To summarize, the core algorithm uses
a prefilter to quickly identify candidate matches that are later verified
more slowly. This prefilter is implemented in terms of trying to find
`rare` bytes at specific offsets that will occur less frequently in the
dataset. While the concept of a `rare` byte is similar for most datasets,
there are some specific datasets (like binary executables) that have
dramatically different byte distributions. For these datasets customizing
the byte frequency heuristic can have a massive impact on performance, and
might even need to be done at runtime.

The default implementation of `HeuristicFrequencyRank` reads from the
static frequency table defined in `src/memmem/byte_frequencies.rs`. This
is optimal for most inputs, so if you are unsure of the impact of using a
custom `HeuristicFrequencyRank` you should probably just use the default.

# Example

```rust
use memchr::{
    arch::all::packedpair::HeuristicFrequencyRank,
    memmem::FinderBuilder,
};

/// A byte-frequency table that is good for scanning binary executables.
struct Binary;

impl HeuristicFrequencyRank for Binary {
    fn rank(&self, byte: u8) -> u8 {
        const TABLE: [u8; 256] = [
            255, 128, 61, 43, 50, 41, 27, 28, 57, 15, 21, 13, 24, 17, 17,
            89, 58, 16, 11, 7, 14, 23, 7, 6, 24, 9, 6, 5, 9, 4, 7, 16,
            68, 11, 9, 6, 88, 7, 4, 4, 23, 9, 4, 8, 8, 5, 10, 4, 30, 11,
            9, 24, 11, 5, 5, 5, 19, 11, 6, 17, 9, 9, 6, 8,
            48, 58, 11, 14, 53, 40, 9, 9, 254, 35, 3, 6, 52, 23, 6, 6, 27,
            4, 7, 11, 14, 13, 10, 11, 11, 5, 2, 10, 16, 12, 6, 19,
            19, 20, 5, 14, 16, 31, 19, 7, 14, 20, 4, 4, 19, 8, 18, 20, 24,
            1, 25, 19, 58, 29, 10, 5, 15, 20, 2, 2, 9, 4, 3, 5,
            51, 11, 4, 53, 23, 39, 6, 4, 13, 81, 4, 186, 5, 67, 3, 2, 15,
            0, 0, 1, 3, 2, 0, 0, 5, 0, 0, 0, 2, 0, 0, 0,
            12, 2, 1, 1, 3, 1, 1, 1, 6, 1, 2, 1, 3, 1, 1, 2, 9, 1, 1, 0,
            2, 2, 4, 4, 11, 6, 7, 3, 6, 9, 4, 5,
            46, 18, 8, 18, 17, 3, 8, 20, 16, 10, 3, 7, 175, 4, 6, 7, 13,
            3, 7, 3, 3, 1, 3, 3, 10, 3, 1, 5, 2, 0, 1, 2,
            16, 3, 5, 1, 6, 1, 1, 2, 58, 20, 3, 14, 12, 2, 1, 3, 16, 3, 5,
            8, 3, 1, 8, 6, 17, 6, 5, 3, 8, 6, 13, 175,
        ];
        TABLE[byte as usize]
    }
}
// Create a new finder with the custom heuristic.
let finder = FinderBuilder::new()
    .build_forward_with_ranker(Binary, b"\x00\x00\xdd\xdd");
// Find needle with custom heuristic.
assert!(finder.find(b"\x00\x00\x00\xdd\xdd").is_some());
```

#### Required Methods

- `fn rank(&self, byte: u8) -> u8`

  Return the heuristic frequency rank of the given byte. A lower rank

#### Implementors

- [`DefaultFrequencyRank`](#defaultfrequencyrank)
- `&'a R`

