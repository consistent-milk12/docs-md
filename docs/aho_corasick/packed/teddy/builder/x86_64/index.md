*[aho_corasick](../../../../index.md) / [packed](../../../index.md) / [teddy](../../index.md) / [builder](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

## Structs

### `SlimSSSE3<const BYTES: usize>`

```rust
struct SlimSSSE3<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
}
```

#### Implementations

- `fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

- `unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

#### Trait Implementations

##### `impl<const BYTES: usize> Clone for SlimSSSE3<BYTES>`

- `fn clone(self: &Self) -> SlimSSSE3<BYTES>` — [`SlimSSSE3`](#slimssse3)

##### `impl<const BYTES: usize> Debug for SlimSSSE3<BYTES>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl SearcherT for SlimSSSE3<4>`

- `unsafe fn find(self: &Self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md)

### `SlimAVX2<const BYTES: usize>`

```rust
struct SlimAVX2<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
    slim256: generic::Slim<core::arch::x86_64::__m256i, BYTES>,
}
```

#### Implementations

- `fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

- `unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

#### Trait Implementations

##### `impl<const BYTES: usize> Clone for SlimAVX2<BYTES>`

- `fn clone(self: &Self) -> SlimAVX2<BYTES>` — [`SlimAVX2`](#slimavx2)

##### `impl<const BYTES: usize> Debug for SlimAVX2<BYTES>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl SearcherT for SlimAVX2<1>`

- `unsafe fn find(self: &Self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md)

### `FatAVX2<const BYTES: usize>`

```rust
struct FatAVX2<const BYTES: usize> {
    fat256: generic::Fat<core::arch::x86_64::__m256i, BYTES>,
}
```

#### Implementations

- `fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

- `unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md), [`Searcher`](../index.md)

#### Trait Implementations

##### `impl<const BYTES: usize> Clone for FatAVX2<BYTES>`

- `fn clone(self: &Self) -> FatAVX2<BYTES>` — [`FatAVX2`](#fatavx2)

##### `impl<const BYTES: usize> Debug for FatAVX2<BYTES>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl SearcherT for FatAVX2<2>`

- `unsafe fn find(self: &Self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md)

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

