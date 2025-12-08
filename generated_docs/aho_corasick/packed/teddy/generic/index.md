*[aho_corasick](../../../index.md) / [packed](../../index.md) / [teddy](../index.md) / [generic](index.md)*

---

# Module `generic`

## Structs

### `Match`

```rust
struct Match {
    pid: crate::PatternID,
    start: *const u8,
    end: *const u8,
}
```

A match type specialized to the Teddy implementations below.

Essentially, instead of representing a match at byte offsets, we use
raw pointers. This is because the implementations below operate on raw
pointers, and so this is a more natural return type based on how the
implementation works.

Also, the `PatternID` used here is a `u16`.

#### Implementations

- `fn pattern(self: &Self) -> PatternID` — [`PatternID`](../../../index.md)

- `fn start(self: &Self) -> *const u8`

- `fn end(self: &Self) -> *const u8`

#### Trait Implementations

##### `impl Clone for Match`

- `fn clone(self: &Self) -> Match` — [`Match`](#match)

##### `impl Copy for Match`

##### `impl Debug for Match`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Slim<V, const BYTES: usize>`

```rust
struct Slim<V, const BYTES: usize> {
    teddy: Teddy<8>,
    masks: [Mask<V>; BYTES],
}
```

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

- `unsafe fn new(patterns: Arc<Patterns>) -> Slim<V, BYTES>` — [`Patterns`](../../pattern/index.md), [`Slim`](#slim)

- `fn memory_usage(self: &Self) -> usize`

- `fn minimum_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl<V: $crate::clone::Clone, const BYTES: usize> Clone for Slim<V, BYTES>`

- `fn clone(self: &Self) -> Slim<V, BYTES>` — [`Slim`](#slim)

##### `impl<V: $crate::fmt::Debug, const BYTES: usize> Debug for Slim<V, BYTES>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Fat<V, const BYTES: usize>`

```rust
struct Fat<V, const BYTES: usize> {
    teddy: Teddy<16>,
    masks: [Mask<V>; BYTES],
}
```

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

- `unsafe fn new(patterns: Arc<Patterns>) -> Fat<V, BYTES>` — [`Patterns`](../../pattern/index.md), [`Fat`](#fat)

- `fn memory_usage(self: &Self) -> usize`

- `fn minimum_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl<V: $crate::clone::Clone, const BYTES: usize> Clone for Fat<V, BYTES>`

- `fn clone(self: &Self) -> Fat<V, BYTES>` — [`Fat`](#fat)

##### `impl<V: $crate::fmt::Debug, const BYTES: usize> Debug for Fat<V, BYTES>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Teddy<const BUCKETS: usize>`

```rust
struct Teddy<const BUCKETS: usize> {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    buckets: [alloc::vec::Vec<crate::PatternID>; BUCKETS],
}
```

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

- `fn new(patterns: Arc<Patterns>) -> Teddy<BUCKETS>` — [`Patterns`](../../pattern/index.md), [`Teddy`](#teddy)

- `unsafe fn verify64(self: &Self, cur: *const u8, end: *const u8, candidate_chunk: u64) -> Option<Match>` — [`Match`](#match)

- `unsafe fn verify_bucket(self: &Self, cur: *const u8, end: *const u8, bucket: usize) -> Option<Match>` — [`Match`](#match)

- `fn mask_len(self: &Self) -> usize`

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl<const BUCKETS: usize> Clone for Teddy<BUCKETS>`

- `fn clone(self: &Self) -> Teddy<BUCKETS>` — [`Teddy`](#teddy)

##### `impl<const BUCKETS: usize> Debug for Teddy<BUCKETS>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Mask<V>`

```rust
struct Mask<V> {
    lo: V,
    hi: V,
}
```

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

- `unsafe fn members1(chunk: V, masks: [Mask<V>; 1]) -> V` — [`Mask`](#mask)

- `unsafe fn members2(chunk: V, masks: [Mask<V>; 2]) -> (V, V)` — [`Mask`](#mask)

- `unsafe fn members3(chunk: V, masks: [Mask<V>; 3]) -> (V, V, V)` — [`Mask`](#mask)

- `unsafe fn members4(chunk: V, masks: [Mask<V>; 4]) -> (V, V, V, V)` — [`Mask`](#mask)

#### Trait Implementations

##### `impl<V: $crate::clone::Clone> Clone for Mask<V>`

- `fn clone(self: &Self) -> Mask<V>` — [`Mask`](#mask)

##### `impl<V: $crate::marker::Copy> Copy for Mask<V>`

##### `impl<V: $crate::fmt::Debug> Debug for Mask<V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SlimMaskBuilder`

```rust
struct SlimMaskBuilder {
    lo: [u8; 32],
    hi: [u8; 32],
}
```

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

- `fn add(self: &mut Self, bucket: usize, byte: u8)`

- `unsafe fn build<V: Vector>(self: &Self) -> Mask<V>` — [`Mask`](#mask)

- `unsafe fn from_teddy<const BYTES: usize, V: Vector>(teddy: &Teddy<8>) -> [Mask<V>; BYTES]` — [`Teddy`](#teddy), [`Mask`](#mask)

#### Trait Implementations

##### `impl Clone for SlimMaskBuilder`

- `fn clone(self: &Self) -> SlimMaskBuilder` — [`SlimMaskBuilder`](#slimmaskbuilder)

##### `impl Debug for SlimMaskBuilder`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for SlimMaskBuilder`

- `fn default() -> SlimMaskBuilder` — [`SlimMaskBuilder`](#slimmaskbuilder)

### `FatMaskBuilder`

```rust
struct FatMaskBuilder {
    lo: [u8; 32],
    hi: [u8; 32],
}
```

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

- `fn add(self: &mut Self, bucket: usize, byte: u8)`

- `unsafe fn build<V: Vector>(self: &Self) -> Mask<V>` — [`Mask`](#mask)

- `unsafe fn from_teddy<const BYTES: usize, V: Vector>(teddy: &Teddy<16>) -> [Mask<V>; BYTES]` — [`Teddy`](#teddy), [`Mask`](#mask)

#### Trait Implementations

##### `impl Clone for FatMaskBuilder`

- `fn clone(self: &Self) -> FatMaskBuilder` — [`FatMaskBuilder`](#fatmaskbuilder)

##### `impl Copy for FatMaskBuilder`

##### `impl Debug for FatMaskBuilder`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for FatMaskBuilder`

- `fn default() -> FatMaskBuilder` — [`FatMaskBuilder`](#fatmaskbuilder)

