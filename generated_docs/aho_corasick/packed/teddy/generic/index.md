*[aho_corasick](../../../index.md) / [packed](../../index.md) / [teddy](../index.md) / [generic](index.md)*

---

# Module `generic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Match`](#match) | struct | A match type specialized to the Teddy implementations below. |
| [`Slim`](#slim) | struct | A "slim" Teddy implementation that is generic over both the vector type and the minimum length of the patterns being searched for. |
| [`Fat`](#fat) | struct | A "fat" Teddy implementation that is generic over both the vector type and the minimum length of the patterns being searched for. |
| [`Teddy`](#teddy) | struct | The common elements of all "slim" and "fat" Teddy search implementations. |
| [`Mask`](#mask) | struct | A vector generic mask for the low and high nybbles in a set of patterns. |
| [`SlimMaskBuilder`](#slimmaskbuilder) | struct | Represents the low and high nybble masks that will be used during search. |
| [`FatMaskBuilder`](#fatmaskbuilder) | struct | Represents the low and high nybble masks that will be used during "fat" Teddy search. |

## Structs

### `Match`

```rust
struct Match {
    pid: crate::PatternID,
    start: *const u8,
    end: *const u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:26-30`](../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L26-L30)*

A match type specialized to the Teddy implementations below.

Essentially, instead of representing a match at byte offsets, we use
raw pointers. This is because the implementations below operate on raw
pointers, and so this is a more natural return type based on how the
implementation works.

Also, the `PatternID` used here is a `u16`.

#### Implementations

- <span id="match-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](../../../util/primitives/index.md#patternid)

- <span id="match-start"></span>`fn start(&self) -> *const u8`

- <span id="match-end"></span>`fn end(&self) -> *const u8`

#### Trait Implementations

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Match` — [`Match`](#match)

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Slim<V, const BYTES: usize>`

```rust
struct Slim<V, const BYTES: usize> {
    teddy: Teddy<8>,
    masks: [Mask<V>; BYTES],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:54-60`](../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L54-L60)*

A "slim" Teddy implementation that is generic over both the vector type
and the minimum length of the patterns being searched for.

Only 1, 2, 3 and 4 bytes are supported as minimum lengths.

#### Fields

- **`teddy`**: `Teddy<8>`

  A generic data structure for doing "slim" Teddy verification.

- **`masks`**: `[Mask<V>; BYTES]`

  The masks used as inputs to the shuffle operation to generate
  candidates (which are fed into the verification routines).

#### Implementations

- <span id="slim-new"></span>`unsafe fn new(patterns: Arc<Patterns>) -> Slim<V, BYTES>` — [`Patterns`](../../pattern/index.md#patterns), [`Slim`](#slim)

- <span id="slim-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="slim-minimum-len"></span>`fn minimum_len(&self) -> usize`

#### Trait Implementations

##### `impl<V: clone::Clone> Clone for Slim<V, BYTES>`

- <span id="slim-clone"></span>`fn clone(&self) -> Slim<V, BYTES>` — [`Slim`](#slim)

##### `impl<V: fmt::Debug> Debug for Slim<V, BYTES>`

- <span id="slim-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Fat<V, const BYTES: usize>`

```rust
struct Fat<V, const BYTES: usize> {
    teddy: Teddy<16>,
    masks: [Mask<V>; BYTES],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:387-393`](../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L387-L393)*

A "fat" Teddy implementation that is generic over both the vector type
and the minimum length of the patterns being searched for.

Only 1, 2, 3 and 4 bytes are supported as minimum lengths.

#### Fields

- **`teddy`**: `Teddy<16>`

  A generic data structure for doing "fat" Teddy verification.

- **`masks`**: `[Mask<V>; BYTES]`

  The masks used as inputs to the shuffle operation to generate
  candidates (which are fed into the verification routines).

#### Implementations

- <span id="fat-new"></span>`unsafe fn new(patterns: Arc<Patterns>) -> Fat<V, BYTES>` — [`Patterns`](../../pattern/index.md#patterns), [`Fat`](#fat)

- <span id="fat-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="fat-minimum-len"></span>`fn minimum_len(&self) -> usize`

#### Trait Implementations

##### `impl<V: clone::Clone> Clone for Fat<V, BYTES>`

- <span id="fat-clone"></span>`fn clone(&self) -> Fat<V, BYTES>` — [`Fat`](#fat)

##### `impl<V: fmt::Debug> Debug for Fat<V, BYTES>`

- <span id="fat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Teddy<const BUCKETS: usize>`

```rust
struct Teddy<const BUCKETS: usize> {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    buckets: [alloc::vec::Vec<crate::PatternID>; BUCKETS],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:728-747`](../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L728-L747)*

The common elements of all "slim" and "fat" Teddy search implementations.

Essentially, this contains the patterns and the buckets. Namely, it
contains enough to implement the verification step after candidates are
identified via the shuffle masks.

It is generic over the number of buckets used. In general, the number of
buckets is either 8 (for "slim" Teddy) or 16 (for "fat" Teddy). The generic
parameter isn't really meant to be instantiated for any value other than
8 or 16, although it is technically possible. The main hiccup is that there
is some bit-shifting done in the critical part of verification that could
be quite expensive if `N` is not a multiple of 2.

#### Fields

- **`patterns`**: `alloc::sync::Arc<crate::packed::pattern::Patterns>`

  The patterns we are searching for.
  
  A pattern string can be found by its `PatternID`.

- **`buckets`**: `[alloc::vec::Vec<crate::PatternID>; BUCKETS]`

  The allocation of patterns in buckets. This only contains the IDs of
  patterns. In order to do full verification, callers must provide the
  actual patterns when using Teddy.

#### Implementations

- <span id="teddy-new"></span>`fn new(patterns: Arc<Patterns>) -> Teddy<BUCKETS>` — [`Patterns`](../../pattern/index.md#patterns), [`Teddy`](#teddy)

- <span id="teddy-verify64"></span>`unsafe fn verify64(&self, cur: *const u8, end: *const u8, candidate_chunk: u64) -> Option<Match>` — [`Match`](#match)

- <span id="teddy-verify-bucket"></span>`unsafe fn verify_bucket(&self, cur: *const u8, end: *const u8, bucket: usize) -> Option<Match>` — [`Match`](#match)

- <span id="teddy-mask-len"></span>`fn mask_len(&self) -> usize`

- <span id="teddy-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Teddy<BUCKETS>`

- <span id="teddy-clone"></span>`fn clone(&self) -> Teddy<BUCKETS>` — [`Teddy`](#teddy)

##### `impl Debug for Teddy<BUCKETS>`

- <span id="teddy-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Mask<V>`

```rust
struct Mask<V> {
    lo: V,
    hi: V,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:1016-1019`](../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L1016-L1019)*

A vector generic mask for the low and high nybbles in a set of patterns.
Each 8-bit lane `j` in a vector corresponds to a bitset where the `i`th bit
is set if and only if the nybble `j` is in the bucket `i` at a particular
position.

This is slightly tweaked dependending on whether Slim or Fat Teddy is being
used. For Slim Teddy, the bitsets in the lower half are the same as the
bitsets in the higher half, so that we can search `V::BYTES` bytes at a
time. (Remember, the nybbles in the haystack are used as indices into these
masks, and 256-bit shuffles only operate on 128-bit lanes.)

For Fat Teddy, the bitsets are not repeated, but instead, the high half
bits correspond to an addition 8 buckets. So that a bitset `00100010` has
buckets 1 and 5 set if it's in the lower half, but has buckets 9 and 13 set
if it's in the higher half.

#### Implementations

- <span id="mask-members1"></span>`unsafe fn members1(chunk: V, masks: [Mask<V>; 1]) -> V` — [`Mask`](#mask)

- <span id="mask-members2"></span>`unsafe fn members2(chunk: V, masks: [Mask<V>; 2]) -> (V, V)` — [`Mask`](#mask)

- <span id="mask-members3"></span>`unsafe fn members3(chunk: V, masks: [Mask<V>; 3]) -> (V, V, V)` — [`Mask`](#mask)

- <span id="mask-members4"></span>`unsafe fn members4(chunk: V, masks: [Mask<V>; 4]) -> (V, V, V, V)` — [`Mask`](#mask)

#### Trait Implementations

##### `impl<V: clone::Clone> Clone for Mask<V>`

- <span id="mask-clone"></span>`fn clone(&self) -> Mask<V>` — [`Mask`](#mask)

##### `impl<V: marker::Copy> Copy for Mask<V>`

##### `impl<V: fmt::Debug> Debug for Mask<V>`

- <span id="mask-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SlimMaskBuilder`

```rust
struct SlimMaskBuilder {
    lo: [u8; 32],
    hi: [u8; 32],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:1178-1181`](../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L1178-L1181)*

Represents the low and high nybble masks that will be used during
search. Each mask is 32 bytes wide, although only the first 16 bytes are
used for 128-bit vectors.

Each byte in the mask corresponds to a 8-bit bitset, where bit `i` is set
if and only if the corresponding nybble is in the ith bucket. The index of
the byte (0-15, inclusive) corresponds to the nybble.

Each mask is used as the target of a shuffle, where the indices for the
shuffle are taken from the haystack. AND'ing the shuffles for both the
low and high masks together also results in 8-bit bitsets, but where bit
`i` is set if and only if the correspond *byte* is in the ith bucket.

#### Implementations

- <span id="slimmaskbuilder-add"></span>`fn add(&mut self, bucket: usize, byte: u8)`

- <span id="slimmaskbuilder-build"></span>`unsafe fn build<V: Vector>(&self) -> Mask<V>` — [`Mask`](#mask)

- <span id="slimmaskbuilder-from-teddy"></span>`unsafe fn from_teddy<const BYTES: usize, V: Vector>(teddy: &Teddy<8>) -> [Mask<V>; BYTES]` — [`Teddy`](#teddy), [`Mask`](#mask)

#### Trait Implementations

##### `impl Clone for SlimMaskBuilder`

- <span id="slimmaskbuilder-clone"></span>`fn clone(&self) -> SlimMaskBuilder` — [`SlimMaskBuilder`](#slimmaskbuilder)

##### `impl Debug for SlimMaskBuilder`

- <span id="slimmaskbuilder-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for SlimMaskBuilder`

- <span id="slimmaskbuilder-default"></span>`fn default() -> SlimMaskBuilder` — [`SlimMaskBuilder`](#slimmaskbuilder)

### `FatMaskBuilder`

```rust
struct FatMaskBuilder {
    lo: [u8; 32],
    hi: [u8; 32],
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/generic.rs:1288-1291`](../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/generic.rs#L1288-L1291)*

Represents the low and high nybble masks that will be used during "fat"
Teddy search.

Each mask is 32 bytes wide, and at the time of writing, only 256-bit vectors
support fat Teddy.

A fat Teddy mask is like a slim Teddy mask, except that instead of
repeating the bitsets in the high and low 128-bits in 256-bit vectors, the
high and low 128-bit halves each represent distinct buckets. (Bringing the
total to 16 instead of 8.) This permits spreading the patterns out a bit
more and thus putting less pressure on verification to be fast.

Each byte in the mask corresponds to a 8-bit bitset, where bit `i` is set
if and only if the corresponding nybble is in the ith bucket. The index of
the byte (0-15, inclusive) corresponds to the nybble.

#### Implementations

- <span id="fatmaskbuilder-add"></span>`fn add(&mut self, bucket: usize, byte: u8)`

- <span id="fatmaskbuilder-build"></span>`unsafe fn build<V: Vector>(&self) -> Mask<V>` — [`Mask`](#mask)

- <span id="fatmaskbuilder-from-teddy"></span>`unsafe fn from_teddy<const BYTES: usize, V: Vector>(teddy: &Teddy<16>) -> [Mask<V>; BYTES]` — [`Teddy`](#teddy), [`Mask`](#mask)

#### Trait Implementations

##### `impl Clone for FatMaskBuilder`

- <span id="fatmaskbuilder-clone"></span>`fn clone(&self) -> FatMaskBuilder` — [`FatMaskBuilder`](#fatmaskbuilder)

##### `impl Copy for FatMaskBuilder`

##### `impl Debug for FatMaskBuilder`

- <span id="fatmaskbuilder-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for FatMaskBuilder`

- <span id="fatmaskbuilder-default"></span>`fn default() -> FatMaskBuilder` — [`FatMaskBuilder`](#fatmaskbuilder)

