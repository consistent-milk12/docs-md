# Crate `foldhash`

This crate provides foldhash, a fast, non-cryptographic, minimally
DoS-resistant hashing algorithm designed for computational uses such as
hashmaps, bloom filters, count sketching, etc.

When should you **not** use foldhash:

- You are afraid of people studying your long-running program's behavior
  to reverse engineer its internal random state and using this knowledge to
  create many colliding inputs for computational complexity attacks.

- You expect foldhash to have a consistent output across versions or
  platforms, such as for persistent file formats or communication protocols.
   
- You are relying on foldhash's properties for any kind of security.
  Foldhash is **not appropriate for any cryptographic purpose**.

Foldhash has two variants, one optimized for speed which is ideal for data
structures such as hash maps and bloom filters, and one optimized for
statistical quality which is ideal for algorithms such as
[HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog) and
[MinHash](https://en.wikipedia.org/wiki/MinHash).

Foldhash can be used in a `#![no_std]` environment by disabling its default
`"std"` feature.

# Usage

The easiest way to use this crate with the standard library `HashMap` or
`HashSet` is to import them from `foldhash` instead, along with the
extension traits to make `HashMap::new` and `HashMap::with_capacity`
work out-of-the-box:

```rust
use foldhash::{HashMap, HashMapExt};

let mut hm = HashMap::new();
hm.insert(42, "hello");
```

You can also avoid the convenience types and do it manually by initializing
a [`RandomState`](fast::RandomState), for example if you are using a different hash map
implementation like [`hashbrown`](https://docs.rs/hashbrown/):

```rust
use hashbrown::HashMap;
use foldhash::fast::RandomState;

let mut hm = HashMap::with_hasher(RandomState::default());
hm.insert("foo", "bar");
```

The above methods are the recommended way to use foldhash, which will
automatically generate a randomly generated hasher instance for you. If you
absolutely must have determinism you can use [`FixedState`](fast::FixedState)
instead, but note that this makes you trivially vulnerable to HashDoS
attacks and might lead to quadratic runtime when moving data from one
hashmap/set into another:

```rust
use std::collections::HashSet;
use foldhash::fast::FixedState;

let mut hm = HashSet::with_hasher(FixedState::with_seed(42));
hm.insert([1, 10, 100]);
```

If you rely on statistical properties of the hash for the correctness of
your algorithm, such as in [HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog),
it is suggested to use the [`RandomState`](quality::RandomState)
or [`FixedState`](quality::FixedState) from the [`quality`](quality/index.md) module instead
of the [`fast`](fast/index.md) module. The latter is optimized purely for speed in hash
tables and has known statistical imperfections.

Finally, you can also directly use the [`RandomState`](quality::RandomState)
or [`FixedState`](quality::FixedState) to manually hash items using the
[`BuildHasher`](std::hash::BuildHasher) trait:
```rust
use std::hash::BuildHasher;
use foldhash::quality::RandomState;

let random_state = RandomState::default();
let hash = random_state.hash_one("hello world");
```

## Seeding

Foldhash relies on a single 8-byte per-hasher seed which should be ideally
be different from each instance to instance, and also a larger
[`SharedSeed`](seed/index.md) which may be shared by many different instances.

To reduce overhead, this [`SharedSeed`](seed/index.md) is typically initialized once and
stored. To prevent each hashmap unnecessarily containing a reference to this
value there are three kinds of [`BuildHasher`](core::hash::BuildHasher)s
foldhash provides (both for [`fast`](fast/index.md) and [`quality`](quality/index.md)):

1. [`RandomState`](fast::RandomState), which always generates a
   random per-hasher seed and implicitly stores a reference to `SharedSeed::global_random`.
2. [`FixedState`](fast::FixedState), which by default uses a fixed
   per-hasher seed and implicitly stores a reference to `SharedSeed::global_fixed`.
3. [`SeedableRandomState`](fast::SeedableRandomState), which works like
   [`RandomState`](fast::RandomState) by default but can be seeded in any manner.
   This state must include an explicit reference to a [`SharedSeed`](seed/index.md), and thus
   this struct is 16 bytes as opposed to just 8 bytes for the previous two.

## Features

This crate has the following features:
- `nightly`, this feature improves string hashing performance
slightly using the nightly-only Rust feature
[`hasher_prefixfree_extras`](https://github.com/rust-lang/rust/issues/96762),
- `std`, this enabled-by-default feature offers convenient aliases for `std`
containers, but can be turned off for `#![no_std]` crates.

## Contents

- [Modules](#modules)
  - [`fast`](#fast)
  - [`quality`](#quality)
  - [`seed`](#seed)
- [Structs](#structs)
  - [`SharedSeed`](#sharedseed)
- [Functions](#functions)
  - [`folded_multiply`](#folded_multiply)
  - [`rotate_right`](#rotate_right)
  - [`cold_path`](#cold_path)
  - [`hash_bytes_short`](#hash_bytes_short)
  - [`load`](#load)
  - [`hash_bytes_long`](#hash_bytes_long)
- [Constants](#constants)
  - [`ARBITRARY0`](#arbitrary0)
  - [`ARBITRARY1`](#arbitrary1)
  - [`ARBITRARY2`](#arbitrary2)
  - [`ARBITRARY3`](#arbitrary3)
  - [`ARBITRARY4`](#arbitrary4)
  - [`ARBITRARY5`](#arbitrary5)
  - [`ARBITRARY6`](#arbitrary6)
  - [`ARBITRARY7`](#arbitrary7)
  - [`ARBITRARY8`](#arbitrary8)
  - [`ARBITRARY9`](#arbitrary9)
  - [`ARBITRARY10`](#arbitrary10)
  - [`ARBITRARY11`](#arbitrary11)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fast`](#fast) | mod | The foldhash implementation optimized for speed. |
| [`quality`](#quality) | mod | The foldhash implementation optimized for quality. |
| [`seed`](#seed) | mod |  |
| [`SharedSeed`](#sharedseed) | struct |  |
| [`folded_multiply`](#folded_multiply) | fn |  |
| [`rotate_right`](#rotate_right) | fn |  |
| [`cold_path`](#cold_path) | fn |  |
| [`hash_bytes_short`](#hash_bytes_short) | fn | Hashes strings <= 16 bytes, has unspecified behavior when bytes.len() > 16. |
| [`load`](#load) | fn | Load 8 bytes into a u64 word at the given offset. |
| [`hash_bytes_long`](#hash_bytes_long) | fn | Hashes strings > 16 bytes. |
| [`ARBITRARY0`](#arbitrary0) | const |  |
| [`ARBITRARY1`](#arbitrary1) | const |  |
| [`ARBITRARY2`](#arbitrary2) | const |  |
| [`ARBITRARY3`](#arbitrary3) | const |  |
| [`ARBITRARY4`](#arbitrary4) | const |  |
| [`ARBITRARY5`](#arbitrary5) | const |  |
| [`ARBITRARY6`](#arbitrary6) | const |  |
| [`ARBITRARY7`](#arbitrary7) | const |  |
| [`ARBITRARY8`](#arbitrary8) | const |  |
| [`ARBITRARY9`](#arbitrary9) | const |  |
| [`ARBITRARY10`](#arbitrary10) | const |  |
| [`ARBITRARY11`](#arbitrary11) | const |  |

## Modules

- [`fast`](fast/index.md) — The foldhash implementation optimized for speed.
- [`quality`](quality/index.md) — The foldhash implementation optimized for quality.
- [`seed`](seed/index.md)

## Structs

### `SharedSeed`

```rust
struct SharedSeed {
    seeds: [u64; 6],
}
```

*Defined in [`foldhash-0.2.0/src/seed.rs:78-80`](../../.source_1765210505/foldhash-0.2.0/src/seed.rs#L78-L80)*

A random seed intended to be shared by many different foldhash instances.

This seed is consumed by [`FoldHasher::with_seed`](crate::fast::FoldHasher::with_seed),
and [`SeedableRandomState::with_seed`](crate::fast::SeedableRandomState::with_seed).

#### Implementations

- <span id="sharedseed-global-random"></span>`fn global_random() -> &'static SharedSeed` — [`SharedSeed`](seed/index.md)

- <span id="sharedseed-global-fixed"></span>`const fn global_fixed() -> &'static SharedSeed` — [`SharedSeed`](seed/index.md)

- <span id="sharedseed-from-u64"></span>`const fn from_u64(seed: u64) -> Self`

#### Trait Implementations

##### `impl Clone for SharedSeed`

- <span id="sharedseed-clone"></span>`fn clone(&self) -> SharedSeed` — [`SharedSeed`](seed/index.md)

##### `impl Debug for SharedSeed`

- <span id="sharedseed-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `folded_multiply`

```rust
const fn folded_multiply(x: u64, y: u64) -> u64
```

*Defined in [`foldhash-0.2.0/src/lib.rs:143-206`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L143-L206)*

### `rotate_right`

```rust
const fn rotate_right(x: u64, r: u32) -> u64
```

*Defined in [`foldhash-0.2.0/src/lib.rs:209-233`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L209-L233)*

### `cold_path`

```rust
fn cold_path()
```

*Defined in [`foldhash-0.2.0/src/lib.rs:236`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L236)*

### `hash_bytes_short`

```rust
fn hash_bytes_short(bytes: &[u8], accumulator: u64, seeds: &[u64; 6]) -> u64
```

*Defined in [`foldhash-0.2.0/src/lib.rs:240-259`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L240-L259)*

Hashes strings <= 16 bytes, has unspecified behavior when bytes.len() > 16.

### `load`

```rust
unsafe fn load(bytes: &[u8], offset: usize) -> u64
```

*Defined in [`foldhash-0.2.0/src/lib.rs:266-271`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L266-L271)*

Load 8 bytes into a u64 word at the given offset.

# Safety
You must ensure that offset + 8 <= bytes.len().

### `hash_bytes_long`

```rust
unsafe fn hash_bytes_long(v: &[u8], accumulator: u64, seeds: &[u64; 6]) -> u64
```

*Defined in [`foldhash-0.2.0/src/lib.rs:279-350`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L279-L350)*

Hashes strings > 16 bytes.

# Safety
v.len() must be > 16 bytes.

## Constants

### `ARBITRARY0`
```rust
const ARBITRARY0: u64 = 2_611_923_443_488_327_891u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:129`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L129)*

### `ARBITRARY1`
```rust
const ARBITRARY1: u64 = 1_376_283_091_369_227_076u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:130`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L130)*

### `ARBITRARY2`
```rust
const ARBITRARY2: u64 = 11_820_040_416_388_919_760u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:131`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L131)*

### `ARBITRARY3`
```rust
const ARBITRARY3: u64 = 589_684_135_938_649_225u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:132`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L132)*

### `ARBITRARY4`
```rust
const ARBITRARY4: u64 = 4_983_270_260_364_809_079u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:133`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L133)*

### `ARBITRARY5`
```rust
const ARBITRARY5: u64 = 13_714_699_805_381_954_668u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:134`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L134)*

### `ARBITRARY6`
```rust
const ARBITRARY6: u64 = 13_883_517_620_612_518_109u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:135`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L135)*

### `ARBITRARY7`
```rust
const ARBITRARY7: u64 = 4_577_018_097_722_394_903u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:136`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L136)*

### `ARBITRARY8`
```rust
const ARBITRARY8: u64 = 10_526_836_309_316_205_339u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:137`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L137)*

### `ARBITRARY9`
```rust
const ARBITRARY9: u64 = 15_073_842_237_943_035_308u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:138`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L138)*

### `ARBITRARY10`
```rust
const ARBITRARY10: u64 = 3_458_046_377_305_235_383u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:139`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L139)*

### `ARBITRARY11`
```rust
const ARBITRARY11: u64 = 13_322_122_606_961_655_446u64;
```

*Defined in [`foldhash-0.2.0/src/lib.rs:140`](../../.source_1765210505/foldhash-0.2.0/src/lib.rs#L140)*

