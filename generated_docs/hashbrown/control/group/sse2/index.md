*[hashbrown](../../../index.md) / [control](../../index.md) / [group](../index.md) / [sse2](index.md)*

---

# Module `sse2`

## Structs

### `Group`

```rust
struct Group(x86::__m128i);
```

Abstraction over a group of control tags which can be scanned in
parallel.

This implementation uses a 128-bit SSE value.

#### Implementations

- `const WIDTH: usize`

- `const fn static_empty() -> &'static [Tag; 16]` — [`Tag`](../../tag/index.md)

- `unsafe fn load(ptr: *const Tag) -> Self` — [`Tag`](../../tag/index.md)

- `unsafe fn load_aligned(ptr: *const Tag) -> Self` — [`Tag`](../../tag/index.md)

- `unsafe fn store_aligned(self: Self, ptr: *mut Tag)` — [`Tag`](../../tag/index.md)

- `fn match_tag(self: Self, tag: Tag) -> BitMask` — [`Tag`](../../tag/index.md), [`BitMask`](../../bitmask/index.md)

- `fn match_empty(self: Self) -> BitMask` — [`BitMask`](../../bitmask/index.md)

- `fn match_empty_or_deleted(self: Self) -> BitMask` — [`BitMask`](../../bitmask/index.md)

- `fn match_full(self: &Self) -> BitMask` — [`BitMask`](../../bitmask/index.md)

- `fn convert_special_to_empty_and_full_to_deleted(self: Self) -> Self`

#### Trait Implementations

##### `impl Clone for Group`

- `fn clone(self: &Self) -> Group` — [`Group`](#group)

##### `impl Copy for Group`

## Type Aliases

### `BitMaskWord`

```rust
type BitMaskWord = u16;
```

### `NonZeroBitMaskWord`

```rust
type NonZeroBitMaskWord = core::num::NonZeroU16;
```

## Constants

### `BITMASK_STRIDE`

```rust
const BITMASK_STRIDE: usize = 1usize;
```

### `BITMASK_MASK`

```rust
const BITMASK_MASK: u16 = 65_535u16;
```

### `BITMASK_ITER_MASK`

```rust
const BITMASK_ITER_MASK: u16 = 65_535u16;
```

