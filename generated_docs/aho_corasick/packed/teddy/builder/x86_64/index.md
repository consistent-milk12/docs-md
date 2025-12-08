*[aho_corasick](../../../../index.md) / [packed](../../../index.md) / [teddy](../../index.md) / [builder](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

## Contents

- [Structs](#structs)
  - [`SlimSSSE3`](#slimssse3)
  - [`SlimAVX2`](#slimavx2)
  - [`FatAVX2`](#fatavx2)
- [Functions](#functions)
  - [`is_available_ssse3`](#is_available_ssse3)
  - [`is_available_avx2`](#is_available_avx2)
- [Macros](#macros)
  - [`slim_ssse3!`](#slim_ssse3)
  - [`slim_avx2!`](#slim_avx2)
  - [`fat_avx2!`](#fat_avx2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SlimSSSE3`](#slimssse3) | struct |  |
| [`SlimAVX2`](#slimavx2) | struct |  |
| [`FatAVX2`](#fatavx2) | struct |  |
| [`is_available_ssse3`](#is_available_ssse3) | fn |  |
| [`is_available_avx2`](#is_available_avx2) | fn |  |
| [`slim_ssse3!`](#slim_ssse3) | macro |  |
| [`slim_avx2!`](#slim_avx2) | macro |  |
| [`fat_avx2!`](#fat_avx2) | macro |  |

## Structs

### `SlimSSSE3<const BYTES: usize>`

```rust
struct SlimSSSE3<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
}
```

#### Implementations

- <span id="slimssse3-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

- <span id="slimssse3-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

#### Trait Implementations

##### `impl<const BYTES: usize> Clone for SlimSSSE3<BYTES>`

- <span id="slimssse3-clone"></span>`fn clone(&self) -> SlimSSSE3<BYTES>` — [`SlimSSSE3`](#slimssse3)

##### `impl<const BYTES: usize> Debug for SlimSSSE3<BYTES>`

- <span id="slimssse3-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SearcherT for SlimSSSE3<3>`

- <span id="slimssse3-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md)

### `SlimAVX2<const BYTES: usize>`

```rust
struct SlimAVX2<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
    slim256: generic::Slim<core::arch::x86_64::__m256i, BYTES>,
}
```

#### Implementations

- <span id="slimavx2-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

- <span id="slimavx2-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

#### Trait Implementations

##### `impl<const BYTES: usize> Clone for SlimAVX2<BYTES>`

- <span id="slimavx2-clone"></span>`fn clone(&self) -> SlimAVX2<BYTES>` — [`SlimAVX2`](#slimavx2)

##### `impl<const BYTES: usize> Debug for SlimAVX2<BYTES>`

- <span id="slimavx2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SearcherT for SlimAVX2<2>`

- <span id="slimavx2-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md)

### `FatAVX2<const BYTES: usize>`

```rust
struct FatAVX2<const BYTES: usize> {
    fat256: generic::Fat<core::arch::x86_64::__m256i, BYTES>,
}
```

#### Implementations

- <span id="fatavx2-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

- <span id="fatavx2-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

#### Trait Implementations

##### `impl<const BYTES: usize> Clone for FatAVX2<BYTES>`

- <span id="fatavx2-clone"></span>`fn clone(&self) -> FatAVX2<BYTES>` — [`FatAVX2`](#fatavx2)

##### `impl<const BYTES: usize> Debug for FatAVX2<BYTES>`

- <span id="fatavx2-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl SearcherT for FatAVX2<1>`

- <span id="fatavx2-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md)

## Functions

### `is_available_ssse3`

```rust
fn is_available_ssse3() -> bool
```

### `is_available_avx2`

```rust
fn is_available_avx2() -> bool
```

## Macros

### `slim_ssse3!`

### `slim_avx2!`

### `fat_avx2!`

