*[aho_corasick](../../../../index.md) / [packed](../../../index.md) / [teddy](../../index.md) / [builder](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

## Contents

- [Structs](#structs)
  - [`SlimSSSE3`](#slimssse3)
  - [`SlimAVX2`](#slimavx2)
  - [`FatAVX2`](#fatavx2)
- [Functions](#functions)
  - [`is_available_ssse3`](#is-available-ssse3)
  - [`is_available_avx2`](#is-available-avx2)
- [Macros](#macros)
  - [`slim_ssse3!`](#slim-ssse3)
  - [`slim_avx2!`](#slim-avx2)
  - [`fat_avx2!`](#fat-avx2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SlimSSSE3`](#slimssse3) | struct |  |
| [`SlimAVX2`](#slimavx2) | struct |  |
| [`FatAVX2`](#fatavx2) | struct |  |
| [`is_available_ssse3`](#is-available-ssse3) | fn |  |
| [`is_available_avx2`](#is-available-avx2) | fn |  |
| [`slim_ssse3!`](#slim-ssse3) | macro |  |
| [`slim_avx2!`](#slim-avx2) | macro |  |
| [`fat_avx2!`](#fat-avx2) | macro |  |

## Structs

### `SlimSSSE3<const BYTES: usize>`

```rust
struct SlimSSSE3<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:465-467`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L465-L467)*

#### Implementations

- <span id="slimssse3-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

- <span id="slimssse3-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

#### Trait Implementations

##### `impl Clone for SlimSSSE3<BYTES>`

- <span id="slimssse3-clone"></span>`fn clone(&self) -> SlimSSSE3<BYTES>` — [`SlimSSSE3`](#slimssse3)

##### `impl Debug for SlimSSSE3<BYTES>`

- <span id="slimssse3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SearcherT for SlimSSSE3<1>`

- <span id="slimssse3-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md#match)

### `SlimAVX2<const BYTES: usize>`

```rust
struct SlimAVX2<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
    slim256: generic::Slim<core::arch::x86_64::__m256i, BYTES>,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:528-531`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L528-L531)*

#### Implementations

- <span id="slimavx2-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

- <span id="slimavx2-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

#### Trait Implementations

##### `impl Clone for SlimAVX2<BYTES>`

- <span id="slimavx2-clone"></span>`fn clone(&self) -> SlimAVX2<BYTES>` — [`SlimAVX2`](#slimavx2)

##### `impl Debug for SlimAVX2<BYTES>`

- <span id="slimavx2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SearcherT for SlimAVX2<1>`

- <span id="slimavx2-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md#match)

### `FatAVX2<const BYTES: usize>`

```rust
struct FatAVX2<const BYTES: usize> {
    fat256: generic::Fat<core::arch::x86_64::__m256i, BYTES>,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:601-603`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L601-L603)*

#### Implementations

- <span id="fatavx2-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

- <span id="fatavx2-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

#### Trait Implementations

##### `impl Clone for FatAVX2<BYTES>`

- <span id="fatavx2-clone"></span>`fn clone(&self) -> FatAVX2<BYTES>` — [`FatAVX2`](#fatavx2)

##### `impl Debug for FatAVX2<BYTES>`

- <span id="fatavx2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SearcherT for FatAVX2<1>`

- <span id="fatavx2-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md#match)

## Functions

### `is_available_ssse3`

```rust
fn is_available_ssse3() -> bool
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:664-687`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L664-L687)*

### `is_available_avx2`

```rust
fn is_available_avx2() -> bool
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:690-713`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L690-L713)*

## Macros

### `slim_ssse3!`

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:470-520`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L470-L520)*

### `slim_avx2!`

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:534-593`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L534-L593)*

### `fat_avx2!`

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:606-656`](../../../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L606-L656)*

