*[hashbrown](../../../index.md) / [control](../../index.md) / [group](../index.md) / [sse2](index.md)*

---

# Module `sse2`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Group`](#group) | struct | Abstraction over a group of control tags which can be scanned in parallel. |
| [`BitMaskWord`](#bitmaskword) | type |  |
| [`NonZeroBitMaskWord`](#nonzerobitmaskword) | type |  |
| [`BITMASK_STRIDE`](#bitmask-stride) | const |  |
| [`BITMASK_MASK`](#bitmask-mask) | const |  |
| [`BITMASK_ITER_MASK`](#bitmask-iter-mask) | const |  |

## Structs

### `Group`

```rust
struct Group(x86::__m128i);
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:21`](../../../../../.source_1765210505/hashbrown-0.16.1/src/control/group/sse2.rs#L21)*

Abstraction over a group of control tags which can be scanned in
parallel.

This implementation uses a 128-bit SSE value.

#### Implementations

- <span id="group-const-width"></span>`const WIDTH: usize`

- <span id="group-static-empty"></span>`const fn static_empty() -> &'static [Tag; 16]` — [`Tag`](../../tag/index.md#tag)

- <span id="group-load"></span>`unsafe fn load(ptr: *const Tag) -> Self` — [`Tag`](../../tag/index.md#tag)

- <span id="group-load-aligned"></span>`unsafe fn load_aligned(ptr: *const Tag) -> Self` — [`Tag`](../../tag/index.md#tag)

- <span id="group-store-aligned"></span>`unsafe fn store_aligned(self, ptr: *mut Tag)` — [`Tag`](../../tag/index.md#tag)

- <span id="group-match-tag"></span>`fn match_tag(self, tag: Tag) -> BitMask` — [`Tag`](../../tag/index.md#tag), [`BitMask`](../../bitmask/index.md#bitmask)

- <span id="group-match-empty"></span>`fn match_empty(self) -> BitMask` — [`BitMask`](../../bitmask/index.md#bitmask)

- <span id="group-match-empty-or-deleted"></span>`fn match_empty_or_deleted(self) -> BitMask` — [`BitMask`](../../bitmask/index.md#bitmask)

- <span id="group-match-full"></span>`fn match_full(&self) -> BitMask` — [`BitMask`](../../bitmask/index.md#bitmask)

- <span id="group-convert-special-to-empty-and-full-to-deleted"></span>`fn convert_special_to_empty_and_full_to_deleted(self) -> Self`

#### Trait Implementations

##### `impl Clone for Group`

- <span id="group-clone"></span>`fn clone(&self) -> Group` — [`Group`](#group)

##### `impl Copy for Group`

## Type Aliases

### `BitMaskWord`

```rust
type BitMaskWord = u16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:10`](../../../../../.source_1765210505/hashbrown-0.16.1/src/control/group/sse2.rs#L10)*

### `NonZeroBitMaskWord`

```rust
type NonZeroBitMaskWord = core::num::NonZeroU16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:11`](../../../../../.source_1765210505/hashbrown-0.16.1/src/control/group/sse2.rs#L11)*

## Constants

### `BITMASK_STRIDE`
```rust
const BITMASK_STRIDE: usize = 1usize;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:12`](../../../../../.source_1765210505/hashbrown-0.16.1/src/control/group/sse2.rs#L12)*

### `BITMASK_MASK`
```rust
const BITMASK_MASK: u16 = 65_535u16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:13`](../../../../../.source_1765210505/hashbrown-0.16.1/src/control/group/sse2.rs#L13)*

### `BITMASK_ITER_MASK`
```rust
const BITMASK_ITER_MASK: u16 = 65_535u16;
```

*Defined in [`hashbrown-0.16.1/src/control/group/sse2.rs:14`](../../../../../.source_1765210505/hashbrown-0.16.1/src/control/group/sse2.rs#L14)*

