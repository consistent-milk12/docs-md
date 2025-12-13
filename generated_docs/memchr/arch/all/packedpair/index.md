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

  Create a new prefilter that reports possible locations where the given

  needle matches.

- <span id="finder-with-pair"></span>`fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>` — [`Pair`](#pair), [`Finder`](#finder)

  Create a new prefilter using the pair given.

  

  If the prefilter could not be constructed, then `None` is returned.

  

  This constructor permits callers to control precisely which pair of

  bytes is used as a predicate.

- <span id="finder-find-prefilter"></span>`fn find_prefilter(&self, haystack: &[u8]) -> Option<usize>`

  Run this finder on the given haystack as a prefilter.

  

  If a candidate match is found, then an offset where the needle *could*

  begin in the haystack is returned.

- <span id="finder-pair"></span>`fn pair(&self) -> &Pair` — [`Pair`](#pair)

  Returns the pair of offsets (into the needle) used to check as a

  predicate before confirming whether a needle exists at a particular

  position.

#### Trait Implementations

##### `impl Any for Finder`

- <span id="finder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Finder`

- <span id="finder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Finder`

- <span id="finder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Finder`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder` — [`Finder`](#finder)

##### `impl CloneToUninit for Finder`

- <span id="finder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Finder`

##### `impl Debug for Finder`

- <span id="finder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Finder`

- <span id="finder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Finder`

- <span id="finder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Finder`

- <span id="finder-toowned-type-owned"></span>`type Owned = T`

- <span id="finder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="finder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Finder`

- <span id="finder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="finder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Finder`

- <span id="finder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="finder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new pair of offsets from the given needle.

  

  If a pair could not be created (for example, if the needle is too

  short), then `None` is returned.

  

  This chooses the pair in the needle that is believed to be as

  predictive of an overall match of the needle as possible.

- <span id="pair-with-ranker"></span>`fn with_ranker<R: HeuristicFrequencyRank>(needle: &[u8], ranker: R) -> Option<Pair>` — [`Pair`](#pair)

  Create a new pair of offsets from the given needle and ranker.

  

  This permits the caller to choose a background frequency distribution

  with which bytes are selected. The idea is to select a pair of bytes

  that is believed to strongly predict a match in the haystack. This

  usually means selecting bytes that occur rarely in a haystack.

  

  If a pair could not be created (for example, if the needle is too

  short), then `None` is returned.

- <span id="pair-with-indices"></span>`fn with_indices(needle: &[u8], index1: u8, index2: u8) -> Option<Pair>` — [`Pair`](#pair)

  Create a new pair using the offsets given for the needle given.

  

  This bypasses any sort of heuristic process for choosing the offsets

  and permits the caller to choose the offsets themselves.

  

  Indices are limited to valid `u8` values so that a `Pair` uses less

  memory. It is not possible to create a `Pair` with offsets bigger than

  `u8::MAX`. It's likely that such a thing is not needed, but if it is,

  it's suggested to build your own bespoke algorithm because you're

  likely working on a very niche case. (File an issue if this suggestion

  does not make sense to you.)

  

  If a pair could not be created (for example, if the needle is too

  short), then `None` is returned.

- <span id="pair-index1"></span>`fn index1(&self) -> u8`

  Returns the first offset of the pair.

- <span id="pair-index2"></span>`fn index2(&self) -> u8`

  Returns the second offset of the pair.

#### Trait Implementations

##### `impl Any for Pair`

- <span id="pair-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pair`

- <span id="pair-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pair`

- <span id="pair-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Pair`

- <span id="pair-clone"></span>`fn clone(&self) -> Pair` — [`Pair`](#pair)

##### `impl CloneToUninit for Pair`

- <span id="pair-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Pair`

##### `impl Debug for Pair`

- <span id="pair-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Pair`

- <span id="pair-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Pair`

- <span id="pair-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Pair`

- <span id="pair-toowned-type-owned"></span>`type Owned = T`

- <span id="pair-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pair-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Pair`

- <span id="pair-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pair-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pair`

- <span id="pair-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pair-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DefaultFrequencyRank`

```rust
struct DefaultFrequencyRank;
```

*Defined in [`memchr-2.7.6/src/arch/all/packedpair/mod.rs:321`](../../../../../.source_1765521767/memchr-2.7.6/src/arch/all/packedpair/mod.rs#L321)*

The default byte frequency heuristic that is good for most haystacks.

#### Trait Implementations

##### `impl Any for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl HeuristicFrequencyRank for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-heuristicfrequencyrank-rank"></span>`fn rank(&self, byte: u8) -> u8`

##### `impl<U> Into for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="defaultfrequencyrank-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DefaultFrequencyRank`

- <span id="defaultfrequencyrank-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="defaultfrequencyrank-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

